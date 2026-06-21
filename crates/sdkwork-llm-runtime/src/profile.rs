use sdkwork_llm_spi::{LlmImplementationKind, MemoryPluginRegistry, LlmSpiError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryImplementationProfileDraft {
    pub profile_id: String,
    pub implementation_kind: LlmImplementationKind,
    pub primary_plugin_id: String,
    pub required_ports: Vec<String>,
    pub safe_config_json: Value,
}

impl MemoryImplementationProfileDraft {
    pub fn native_sql_phase1() -> Self {
        Self {
            profile_id: "native-sql-phase1".to_string(),
            implementation_kind: LlmImplementationKind::NativeSql,
            primary_plugin_id: "sdkwork-llm-plugin-native-sql".to_string(),
            required_ports: native_sql_store_ports(),
            safe_config_json: Value::Object(Default::default()),
        }
    }

    pub fn local_embedded_phase1() -> Self {
        Self {
            profile_id: "local-embedded-phase1".to_string(),
            implementation_kind: LlmImplementationKind::LocalEmbedded,
            primary_plugin_id: "sdkwork-llm-plugin-native-sql".to_string(),
            required_ports: native_sql_store_ports(),
            safe_config_json: Value::Object(Default::default()),
        }
    }

    pub fn event_sourced_phase1() -> Self {
        reference_profile(
            "event-sourced-phase1",
            LlmImplementationKind::EventSourced,
            vec![
                "LlmEventStorePort",
                "LlmRecordStorePort",
                "LlmAuditStorePort",
                "LlmOutboxStorePort",
            ],
        )
    }

    pub fn search_first_phase1() -> Self {
        reference_profile(
            "search-first-phase1",
            LlmImplementationKind::SearchFirst,
            vec![
                "LlmRecordStorePort",
                "LlmRetrieverPort",
                "LlmIndexPort",
                "LlmAuditStorePort",
            ],
        )
    }

    pub fn graph_temporal_phase1() -> Self {
        reference_profile(
            "graph-temporal-phase1",
            LlmImplementationKind::GraphTemporal,
            vec![
                "LlmRecordStorePort",
                "LlmRetrieverPort",
                "LlmIndexPort",
                "LlmContextAssemblerPort",
            ],
        )
    }

    pub fn external_provider_bridge_eval() -> Self {
        reference_profile(
            "external-provider-bridge-eval",
            LlmImplementationKind::ExternalProviderBridge,
            vec![
                "ExternalLlmBridgePort",
                "LlmAuditStorePort",
                "LlmOutboxStorePort",
            ],
        )
    }

    pub fn hybrid_platform_phase1() -> Self {
        reference_profile(
            "hybrid-platform-phase1",
            LlmImplementationKind::HybridPlatform,
            vec![
                "LlmRecordStorePort",
                "LlmEventStorePort",
                "LlmAuditStorePort",
                "LlmOutboxStorePort",
                "LlmRetrieverPort",
                "LlmIndexPort",
                "ExternalLlmBridgePort",
                "LlmContextAssemblerPort",
                "LlmEvaluationPort",
            ],
        )
    }

    pub fn phase1_family_baselines() -> Vec<Self> {
        vec![
            Self::native_sql_phase1(),
            Self::local_embedded_phase1(),
            Self::event_sourced_phase1(),
            Self::search_first_phase1(),
            Self::graph_temporal_phase1(),
            Self::external_provider_bridge_eval(),
            Self::hybrid_platform_phase1(),
        ]
    }
}

fn reference_profile(
    profile_id: &str,
    implementation_kind: LlmImplementationKind,
    required_ports: Vec<&str>,
) -> MemoryImplementationProfileDraft {
    let mut required_ports = required_ports
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    for required_port in learning_and_trace_ports() {
        if !required_ports.contains(&required_port.to_string()) {
            required_ports.push(required_port.to_string());
        }
    }

    MemoryImplementationProfileDraft {
        profile_id: profile_id.to_string(),
        implementation_kind,
        primary_plugin_id: "sdkwork-llm-plugin-reference-profiles".to_string(),
        required_ports,
        safe_config_json: Value::Object(Default::default()),
    }
}

fn native_sql_store_ports() -> Vec<String> {
    [
        "LlmRecordStorePort".to_string(),
        "LlmEventStorePort".to_string(),
        "LlmAuditStorePort".to_string(),
        "LlmOutboxStorePort".to_string(),
        "LlmCandidateStorePort".to_string(),
        "LlmHabitStorePort".to_string(),
        "LlmRetrievalTraceStorePort".to_string(),
    ]
    .to_vec()
}

fn learning_and_trace_ports() -> [&'static str; 3] {
    [
        "LlmCandidateStorePort",
        "LlmHabitStorePort",
        "LlmRetrievalTraceStorePort",
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedMemoryImplementationProfile {
    pub profile_id: String,
    pub implementation_kind: LlmImplementationKind,
    pub primary_plugin_id: String,
}

#[derive(Debug)]
pub struct MemoryRuntimeProfileResolver<'a> {
    registry: &'a MemoryPluginRegistry,
}

impl<'a> MemoryRuntimeProfileResolver<'a> {
    pub fn new(registry: &'a MemoryPluginRegistry) -> Self {
        Self { registry }
    }

    pub fn resolve(
        &self,
        profile: MemoryImplementationProfileDraft,
    ) -> Result<ResolvedMemoryImplementationProfile, MemoryRuntimeError> {
        let manifest = self
            .registry
            .get(&profile.primary_plugin_id)
            .ok_or_else(|| {
                MemoryRuntimeError::PrimaryPluginMissing(profile.primary_plugin_id.clone())
            })?;

        if !manifest
            .implementation_kinds
            .contains(&profile.implementation_kind)
        {
            return Err(MemoryRuntimeError::ImplementationKindUnsupported {
                plugin_id: profile.primary_plugin_id,
                implementation_kind: format!("{:?}", profile.implementation_kind),
            });
        }

        reject_literal_secrets(&profile.safe_config_json)?;

        let required_ports: Vec<&str> = profile.required_ports.iter().map(String::as_str).collect();
        self.registry
            .validate_required_ports(&profile.primary_plugin_id, &required_ports)
            .map_err(MemoryRuntimeError::from)?;

        Ok(ResolvedMemoryImplementationProfile {
            profile_id: profile.profile_id,
            implementation_kind: profile.implementation_kind,
            primary_plugin_id: profile.primary_plugin_id,
        })
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MemoryRuntimeError {
    #[error("primary memory plugin is missing: {0}")]
    PrimaryPluginMissing(String),
    #[error(
        "memory plugin {plugin_id} does not support implementation kind {implementation_kind}"
    )]
    ImplementationKindUnsupported {
        plugin_id: String,
        implementation_kind: String,
    },
    #[error("memory runtime profile is missing required port {port} on plugin {plugin_id}")]
    RequiredPortMissing { plugin_id: String, port: String },
    #[error("memory runtime safe config contains a literal secret-like value at {0}")]
    UnsafeConfigSecret(String),
    #[error("memory runtime SPI error: {0}")]
    Spi(String),
}

impl From<LlmSpiError> for MemoryRuntimeError {
    fn from(value: LlmSpiError) -> Self {
        match value {
            LlmSpiError::RequiredPortMissing { plugin_id, port } => {
                MemoryRuntimeError::RequiredPortMissing { plugin_id, port }
            }
            other => MemoryRuntimeError::Spi(other.to_string()),
        }
    }
}

fn reject_literal_secrets(value: &Value) -> Result<(), MemoryRuntimeError> {
    reject_literal_secrets_at("$", value)
}

fn reject_literal_secrets_at(path: &str, value: &Value) -> Result<(), MemoryRuntimeError> {
    match value {
        Value::String(text) if looks_like_secret_value(text) => {
            Err(MemoryRuntimeError::UnsafeConfigSecret(path.to_string()))
        }
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                reject_literal_secrets_at(&format!("{path}[{index}]"), item)?;
            }
            Ok(())
        }
        Value::Object(map) => {
            for (key, item) in map {
                if looks_like_secret_key(key) && item.is_string() {
                    return Err(MemoryRuntimeError::UnsafeConfigSecret(format!(
                        "{path}.{key}"
                    )));
                }
                reject_literal_secrets_at(&format!("{path}.{key}"), item)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn looks_like_secret_key(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("token")
        || lower.contains("password")
        || lower.contains("api_key")
        || lower.contains("private_key")
        || lower.contains("secret")
}

fn looks_like_secret_value(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("literal")
        || lower.contains("password")
        || lower.contains("api_key")
        || lower.contains("private_key")
        || lower.contains("access_token")
        || lower.contains("refresh_token")
        || lower.contains("bearer ")
        || lower.contains("sk-")
        || lower.contains("token-secret")
}
