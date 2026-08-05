//! Portable SQLx Any queries with PostgreSQL-compatible numbered placeholders.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

use crate::pool_backend::LlmSqlDialect;

pub(crate) use ::sqlx::{any, AnyPool, Error, Row};

const MAX_CACHED_QUERY_SHAPES: usize = 1024;
static NORMALIZED_SQL: OnceLock<RwLock<HashMap<String, &'static str>>> = OnceLock::new();

pub(crate) fn query<'q, DB>(
    sql: &str,
) -> ::sqlx::query::Query<'q, DB, <DB as ::sqlx::Database>::Arguments>
where
    DB: ::sqlx::Database,
{
    ::sqlx::query(normalized_sql(sql))
}

pub(crate) fn query_for<'q, DB>(
    dialect: LlmSqlDialect,
    sql: &str,
) -> ::sqlx::query::Query<'q, DB, <DB as ::sqlx::Database>::Arguments>
where
    DB: ::sqlx::Database,
{
    ::sqlx::query(normalized_sql_for(dialect, sql))
}

pub(crate) fn query_scalar<'q, DB, O>(
    sql: &str,
) -> ::sqlx::query::QueryScalar<'q, DB, O, <DB as ::sqlx::Database>::Arguments>
where
    DB: ::sqlx::Database,
    (O,): for<'row> ::sqlx::FromRow<'row, DB::Row>,
{
    ::sqlx::query_scalar(normalized_sql(sql))
}

fn normalized_sql(sql: &str) -> &'static str {
    normalized_sql_for(LlmSqlDialect::Sqlite, sql)
}

fn normalized_sql_for(dialect: LlmSqlDialect, sql: &str) -> &'static str {
    let cache = NORMALIZED_SQL.get_or_init(|| RwLock::new(HashMap::new()));
    let cache_key = format!("{dialect:?}\0{sql}");
    if let Some(value) = cache.read().expect("SQL cache read lock").get(&cache_key) {
        return value;
    }

    let normalized = number_placeholders(&expand_dialect_markers(dialect, sql));
    let mut write = cache.write().expect("SQL cache write lock");
    if let Some(value) = write.get(&cache_key) {
        return value;
    }
    assert!(
        write.len() < MAX_CACHED_QUERY_SHAPES,
        "LLM native SQL query-shape cache exceeded its static bound"
    );
    let value = Box::leak(normalized.into_boxed_str());
    write.insert(cache_key, value);
    value
}

fn expand_dialect_markers(dialect: LlmSqlDialect, sql: &str) -> String {
    let postgres = matches!(dialect, LlmSqlDialect::Postgres);
    let sql = sql
        .replace(
            "SDKWORK_TIMESTAMP_BIND(?)",
            if postgres {
                "CAST(? AS TIMESTAMPTZ)"
            } else {
                "?"
            },
        )
        .replace(
            "SDKWORK_JSON_BIND(?)",
            if postgres { "CAST(? AS JSONB)" } else { "?" },
        )
        .replace(
            "SDKWORK_BOOL_BIND(?)",
            if postgres { "(? <> 0)" } else { "?" },
        );
    let sql = expand_column_marker(&sql, "SDKWORK_JSON_TEXT(", |column| {
        if postgres {
            format!("CAST({column} AS TEXT)")
        } else {
            column.to_string()
        }
    });
    let sql = expand_column_marker(&sql, "SDKWORK_TIMESTAMP_TEXT(", |column| {
        if postgres {
            format!("to_char({column} AT TIME ZONE 'UTC', 'YYYY-MM-DD\"T\"HH24:MI:SS.MS\"Z\"')")
        } else {
            column.to_string()
        }
    });
    let sql = expand_column_marker(&sql, "SDKWORK_FLOAT(", |column| {
        if postgres {
            format!("CAST({column} AS DOUBLE PRECISION)")
        } else {
            column.to_string()
        }
    });
    expand_column_marker(&sql, "SDKWORK_BOOL_INT(", |column| {
        if postgres {
            format!("CASE WHEN {column} THEN 1 ELSE 0 END")
        } else {
            column.to_string()
        }
    })
}

fn expand_column_marker(sql: &str, marker: &str, replacement: impl Fn(&str) -> String) -> String {
    let mut output = String::with_capacity(sql.len());
    let mut remaining = sql;
    while let Some(start) = remaining.find(marker) {
        output.push_str(&remaining[..start]);
        let expression = &remaining[start + marker.len()..];
        let end = expression
            .find(')')
            .unwrap_or_else(|| panic!("unterminated SQL dialect marker `{marker}`"));
        output.push_str(&replacement(expression[..end].trim()));
        remaining = &expression[end + 1..];
    }
    output.push_str(remaining);
    output
}

fn number_placeholders(sql: &str) -> String {
    let chars = sql.as_bytes();
    let mut output = String::with_capacity(sql.len() + 16);
    let mut index = 0usize;
    let mut placeholder = 1usize;
    let mut state = SqlLexState::Normal;

    while index < chars.len() {
        match state {
            SqlLexState::Normal => {
                if chars[index] == b'\'' {
                    state = SqlLexState::SingleQuoted;
                    output.push('\'');
                    index += 1;
                } else if chars[index] == b'"' {
                    state = SqlLexState::DoubleQuoted;
                    output.push('"');
                    index += 1;
                } else if chars[index..].starts_with(b"--") {
                    state = SqlLexState::LineComment;
                    output.push_str("--");
                    index += 2;
                } else if chars[index..].starts_with(b"/*") {
                    state = SqlLexState::BlockComment;
                    output.push_str("/*");
                    index += 2;
                } else if chars[index] == b'$' {
                    if let Some(end) = dollar_quote_tag_end(chars, index) {
                        let tag = sql[index..=end].to_string();
                        output.push_str(&tag);
                        index = end + 1;
                        state = SqlLexState::DollarQuoted(tag);
                    } else {
                        output.push('$');
                        index += 1;
                    }
                } else if chars[index] == b'?'
                    && !matches!(chars.get(index + 1), Some(b'|') | Some(b'&'))
                {
                    output.push('$');
                    output.push_str(&placeholder.to_string());
                    placeholder += 1;
                    index += 1;
                } else {
                    output.push(chars[index] as char);
                    index += 1;
                }
            }
            SqlLexState::SingleQuoted => {
                output.push(chars[index] as char);
                if chars[index] == b'\'' {
                    if chars.get(index + 1) == Some(&b'\'') {
                        output.push('\'');
                        index += 2;
                    } else {
                        index += 1;
                        state = SqlLexState::Normal;
                    }
                } else {
                    index += 1;
                }
            }
            SqlLexState::DoubleQuoted => {
                output.push(chars[index] as char);
                if chars[index] == b'"' {
                    if chars.get(index + 1) == Some(&b'"') {
                        output.push('"');
                        index += 2;
                    } else {
                        index += 1;
                        state = SqlLexState::Normal;
                    }
                } else {
                    index += 1;
                }
            }
            SqlLexState::LineComment => {
                output.push(chars[index] as char);
                if chars[index] == b'\n' {
                    state = SqlLexState::Normal;
                }
                index += 1;
            }
            SqlLexState::BlockComment => {
                if chars[index..].starts_with(b"*/") {
                    output.push_str("*/");
                    index += 2;
                    state = SqlLexState::Normal;
                } else {
                    output.push(chars[index] as char);
                    index += 1;
                }
            }
            SqlLexState::DollarQuoted(ref tag) => {
                if sql[index..].starts_with(tag) {
                    output.push_str(tag);
                    index += tag.len();
                    state = SqlLexState::Normal;
                } else {
                    output.push(chars[index] as char);
                    index += 1;
                }
            }
        }
    }
    output
}

fn dollar_quote_tag_end(chars: &[u8], start: usize) -> Option<usize> {
    let mut index = start + 1;
    while index < chars.len() && (chars[index].is_ascii_alphanumeric() || chars[index] == b'_') {
        index += 1;
    }
    (chars.get(index) == Some(&b'$')).then_some(index)
}

#[derive(Clone)]
enum SqlLexState {
    Normal,
    SingleQuoted,
    DoubleQuoted,
    LineComment,
    BlockComment,
    DollarQuoted(String),
}

#[cfg(test)]
mod tests {
    use super::{expand_dialect_markers, number_placeholders};
    use crate::pool_backend::LlmSqlDialect;

    #[test]
    fn numbers_bind_markers_without_touching_literals_or_comments() {
        let sql =
            "SELECT '?' AS literal, value FROM sample -- ? comment\nWHERE a = ? AND b = ? /* ? */";
        assert_eq!(
            number_placeholders(sql),
            "SELECT '?' AS literal, value FROM sample -- ? comment\nWHERE a = $1 AND b = $2 /* ? */"
        );
    }

    #[test]
    fn preserves_postgres_json_and_dollar_quoted_operators() {
        let sql = "SELECT payload ?| array['a'] FROM sample WHERE id = ?; $$ ? $$";
        assert_eq!(
            number_placeholders(sql),
            "SELECT payload ?| array['a'] FROM sample WHERE id = $1; $$ ? $$"
        );
    }

    #[test]
    fn expands_postgres_rich_type_markers() {
        let sql = "SELECT SDKWORK_JSON_TEXT(payload), SDKWORK_TIMESTAMP_TEXT(created_at), \
                   SDKWORK_FLOAT(score), SDKWORK_BOOL_INT(enabled) \
                   FROM sample WHERE payload = SDKWORK_JSON_BIND(?) \
                   AND created_at = SDKWORK_TIMESTAMP_BIND(?) \
                   AND enabled = SDKWORK_BOOL_BIND(?)";
        let expanded = expand_dialect_markers(LlmSqlDialect::Postgres, sql);
        assert!(expanded.contains("CAST(payload AS TEXT)"));
        assert!(expanded.contains("to_char(created_at AT TIME ZONE 'UTC'"));
        assert!(expanded.contains("CAST(score AS DOUBLE PRECISION)"));
        assert!(expanded.contains("CASE WHEN enabled THEN 1 ELSE 0 END"));
        assert!(expanded.contains("CAST(? AS JSONB)"));
        assert!(expanded.contains("CAST(? AS TIMESTAMPTZ)"));
        assert!(expanded.contains("(? <> 0)"));
    }

    #[test]
    fn removes_rich_type_markers_for_sqlite() {
        let sql = "SELECT SDKWORK_JSON_TEXT(payload), SDKWORK_TIMESTAMP_TEXT(created_at), \
                   SDKWORK_FLOAT(score), SDKWORK_BOOL_INT(enabled) \
                   FROM sample WHERE payload = SDKWORK_JSON_BIND(?) \
                   AND created_at = SDKWORK_TIMESTAMP_BIND(?) \
                   AND enabled = SDKWORK_BOOL_BIND(?)";
        assert_eq!(
            expand_dialect_markers(LlmSqlDialect::Sqlite, sql),
            "SELECT payload, created_at, score, enabled FROM sample WHERE payload = ? AND created_at = ? AND enabled = ?"
        );
    }
}
