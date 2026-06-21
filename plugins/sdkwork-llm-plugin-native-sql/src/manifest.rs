use sdkwork_llm_spi::LlmPluginManifest;

pub const NATIVE_SQL_PLUGIN_ID: &str = "sdkwork-llm-plugin-native-sql";

pub fn native_sql_manifest() -> LlmPluginManifest {
    LlmPluginManifest::native_sql_baseline()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeSqlPortBuilder {
    pub port_name: &'static str,
    pub builder_name: &'static str,
    pub ready: bool,
}

pub fn build_native_sql_record_store() -> NativeSqlPortBuilder {
    NativeSqlPortBuilder {
        port_name: "LlmRecordStorePort",
        builder_name: "build_native_sql_record_store",
        ready: true,
    }
}

pub fn build_native_sql_event_store() -> NativeSqlPortBuilder {
    NativeSqlPortBuilder {
        port_name: "LlmEventStorePort",
        builder_name: "build_native_sql_event_store",
        ready: true,
    }
}

pub fn build_native_sql_audit_store() -> NativeSqlPortBuilder {
    NativeSqlPortBuilder {
        port_name: "LlmAuditStorePort",
        builder_name: "build_native_sql_audit_store",
        ready: true,
    }
}

pub fn build_native_sql_outbox_store() -> NativeSqlPortBuilder {
    NativeSqlPortBuilder {
        port_name: "LlmOutboxStorePort",
        builder_name: "build_native_sql_outbox_store",
        ready: true,
    }
}

pub fn build_native_sql_candidate_store() -> NativeSqlPortBuilder {
    NativeSqlPortBuilder {
        port_name: "LlmCandidateStorePort",
        builder_name: "build_native_sql_candidate_store",
        ready: true,
    }
}

pub fn build_native_sql_habit_store() -> NativeSqlPortBuilder {
    NativeSqlPortBuilder {
        port_name: "LlmHabitStorePort",
        builder_name: "build_native_sql_habit_store",
        ready: true,
    }
}

pub fn build_native_sql_retrieval_trace_store() -> NativeSqlPortBuilder {
    NativeSqlPortBuilder {
        port_name: "LlmRetrievalTraceStorePort",
        builder_name: "build_native_sql_retrieval_trace_store",
        ready: true,
    }
}
