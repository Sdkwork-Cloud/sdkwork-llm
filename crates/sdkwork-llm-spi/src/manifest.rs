use serde::{Deserialize, Serialize};

use crate::LlmSpiError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmPluginManifest {
    pub schema_version: u32,
    pub kind: String,
    pub plugin_id: String,
    pub package_name: String,
    pub display_name: String,
    pub version: String,
    pub owner: String,
    pub implementation_kinds: Vec<LlmImplementationKind>,
    pub plugin_roles: Vec<MemoryPluginRole>,
    pub deployment_modes: Vec<MemoryDeploymentMode>,
    pub port_exports: Vec<MemoryPluginPortExport>,
    pub provider_kinds: Vec<MemoryProviderKind>,
    pub retriever_kinds: Vec<LlmRetrieverKind>,
    pub index_kinds: Vec<MemoryIndexKind>,
    pub required_core_version: String,
    #[serde(default)]
    pub config_schema_ref: Option<String>,
    #[serde(default)]
    pub secret_refs: Vec<String>,
    #[serde(default)]
    pub data_classes: Vec<MemoryPluginDataClass>,
    pub capabilities: MemoryPluginCapabilities,
    pub degradation: MemoryPluginDegradationPolicy,
    pub migration: MemoryPluginMigrationCapabilities,
    pub observability: MemoryPluginObservabilityContract,
    pub conformance: MemoryPluginConformanceContract,
}

impl LlmPluginManifest {
    pub fn validate(&self) -> Result<(), LlmSpiError> {
        if self.schema_version != 1 {
            return Err(LlmSpiError::ManifestInvalid(
                "schemaVersion must be 1".to_string(),
            ));
        }

        if self.kind != "sdkwork.llm.plugin" {
            return Err(LlmSpiError::ManifestInvalid(
                "kind must be sdkwork.llm.plugin".to_string(),
            ));
        }

        if !self.plugin_id.starts_with("sdkwork-llm-plugin-") {
            return Err(LlmSpiError::ManifestInvalid(
                "pluginId must start with sdkwork-llm-plugin-".to_string(),
            ));
        }

        for path_like in [&self.plugin_id, &self.package_name] {
            if path_like.contains(".sdkwork/plugins") || path_like.contains(".sdkwork\\plugins") {
                return Err(LlmSpiError::ManifestInvalid(
                    "runtime plugins must not live under .sdkwork/plugins".to_string(),
                ));
            }
        }

        if self.port_exports.is_empty() {
            return Err(LlmSpiError::ManifestInvalid(
                "portExports must not be empty".to_string(),
            ));
        }

        for secret_ref in &self.secret_refs {
            if looks_like_secret_value(secret_ref) {
                return Err(LlmSpiError::ManifestInvalid(
                    "secretRefs must contain references, not literal secret values".to_string(),
                ));
            }
        }

        if self
            .implementation_kinds
            .contains(&LlmImplementationKind::NativeSql)
            && self.capabilities.embedding_required
        {
            return Err(LlmSpiError::ManifestInvalid(
                "native_sql must not require embeddings".to_string(),
            ));
        }

        self.require_capability_port(
            self.capabilities.canonical_store,
            "LlmRecordStorePort",
            "canonicalStore",
        )?;
        self.require_capability_port(
            self.capabilities.event_log,
            "LlmEventStorePort",
            "eventLog",
        )?;
        self.require_capability_port(
            self.capabilities.audit_log,
            "LlmAuditStorePort",
            "auditLog",
        )?;
        self.require_capability_port(
            self.capabilities.outbox_log,
            "LlmOutboxStorePort",
            "outboxLog",
        )?;
        self.require_capability_port(
            self.capabilities.candidate_lifecycle,
            "LlmCandidateStorePort",
            "candidateLifecycle",
        )?;
        self.require_capability_port(
            self.capabilities.habit_learning,
            "LlmHabitStorePort",
            "habitLearning",
        )?;
        self.require_capability_port(
            self.capabilities.retrieval_trace,
            "LlmRetrievalTraceStorePort",
            "retrievalTrace",
        )?;

        Ok(())
    }

    fn require_capability_port(
        &self,
        capability_enabled: bool,
        required_port: &str,
        capability_name: &str,
    ) -> Result<(), LlmSpiError> {
        if capability_enabled
            && !self
                .port_exports
                .iter()
                .any(|export| export.port == required_port)
        {
            return Err(LlmSpiError::ManifestInvalid(format!(
                "{capability_name}=true requires {required_port}"
            )));
        }

        Ok(())
    }

    pub fn native_sql_baseline() -> Self {
        Self {
            schema_version: 1,
            kind: "sdkwork.llm.plugin".to_string(),
            plugin_id: "sdkwork-llm-plugin-native-sql".to_string(),
            package_name: "sdkwork-llm-plugin-native-sql".to_string(),
            display_name: "SDKWork LLM Native SQL Plugin".to_string(),
            version: "0.1.0".to_string(),
            owner: "sdkwork-llm".to_string(),
            implementation_kinds: vec![
                LlmImplementationKind::NativeSql,
                LlmImplementationKind::LocalEmbedded,
            ],
            plugin_roles: vec![MemoryPluginRole::Implementation, MemoryPluginRole::Store],
            deployment_modes: vec![
                MemoryDeploymentMode::Server,
                MemoryDeploymentMode::Container,
                MemoryDeploymentMode::Private,
                MemoryDeploymentMode::Local,
                MemoryDeploymentMode::Test,
            ],
            port_exports: vec![
                MemoryPluginPortExport {
                    port: "LlmRecordStorePort".to_string(),
                    builder: "build_native_sql_record_store".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmEventStorePort".to_string(),
                    builder: "build_native_sql_event_store".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmAuditStorePort".to_string(),
                    builder: "build_native_sql_audit_store".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmOutboxStorePort".to_string(),
                    builder: "build_native_sql_outbox_store".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmCandidateStorePort".to_string(),
                    builder: "build_native_sql_candidate_store".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmHabitStorePort".to_string(),
                    builder: "build_native_sql_habit_store".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmRetrievalTraceStorePort".to_string(),
                    builder: "build_native_sql_retrieval_trace_store".to_string(),
                },
            ],
            provider_kinds: vec![],
            retriever_kinds: vec![],
            index_kinds: vec![],
            required_core_version: "0.1.0".to_string(),
            config_schema_ref: None,
            secret_refs: vec![],
            data_classes: vec![
                MemoryPluginDataClass::Tenant,
                MemoryPluginDataClass::Personal,
            ],
            capabilities: MemoryPluginCapabilities {
                canonical_store: true,
                event_log: true,
                candidate_lifecycle: true,
                habit_learning: true,
                retrieval_trace: true,
                deletion_propagation: true,
                audit_log: true,
                outbox_log: true,
                embedding_required: false,
            },
            degradation: MemoryPluginDegradationPolicy {
                mode: "fail_required_degrade_optional".to_string(),
                returns_stale_hits: false,
            },
            migration: MemoryPluginMigrationCapabilities {
                export_supported: true,
                import_supported: true,
                dual_write_supported: false,
                shadow_read_supported: true,
            },
            observability: MemoryPluginObservabilityContract {
                metrics_prefix: "sdkwork_llm_native_sql".to_string(),
                redacts_payloads: true,
            },
            conformance: MemoryPluginConformanceContract {
                suite: "sdkwork-llm-plugin-conformance".to_string(),
                suite_version: "0.1.0".to_string(),
            },
        }
    }

    pub fn reference_profiles_baseline() -> Self {
        Self {
            schema_version: 1,
            kind: "sdkwork.llm.plugin".to_string(),
            plugin_id: "sdkwork-llm-plugin-reference-profiles".to_string(),
            package_name: "sdkwork-llm-plugin-reference-profiles".to_string(),
            display_name: "SDKWork LLM Reference Profiles Plugin".to_string(),
            version: "0.1.0".to_string(),
            owner: "sdkwork-llm".to_string(),
            implementation_kinds: vec![
                LlmImplementationKind::EventSourced,
                LlmImplementationKind::SearchFirst,
                LlmImplementationKind::GraphTemporal,
                LlmImplementationKind::ExternalProviderBridge,
                LlmImplementationKind::HybridPlatform,
            ],
            plugin_roles: vec![
                MemoryPluginRole::Implementation,
                MemoryPluginRole::Store,
                MemoryPluginRole::Retriever,
                MemoryPluginRole::Index,
                MemoryPluginRole::Provider,
                MemoryPluginRole::Context,
                MemoryPluginRole::Evaluation,
            ],
            deployment_modes: vec![
                MemoryDeploymentMode::Server,
                MemoryDeploymentMode::Container,
                MemoryDeploymentMode::Private,
                MemoryDeploymentMode::Local,
                MemoryDeploymentMode::Test,
                MemoryDeploymentMode::EvalOnly,
            ],
            port_exports: vec![
                MemoryPluginPortExport {
                    port: "LlmRecordStorePort".to_string(),
                    builder: "build_reference_record_store".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmEventStorePort".to_string(),
                    builder: "build_reference_event_store".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmAuditStorePort".to_string(),
                    builder: "build_reference_audit_store".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmOutboxStorePort".to_string(),
                    builder: "build_reference_outbox_store".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmCandidateStorePort".to_string(),
                    builder: "build_reference_candidate_store".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmHabitStorePort".to_string(),
                    builder: "build_reference_habit_store".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmRetrievalTraceStorePort".to_string(),
                    builder: "build_reference_retrieval_trace_store".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmRetrieverPort".to_string(),
                    builder: "build_reference_retriever".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmIndexPort".to_string(),
                    builder: "build_reference_index".to_string(),
                },
                MemoryPluginPortExport {
                    port: "ExternalLlmBridgePort".to_string(),
                    builder: "build_reference_external_bridge".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmContextAssemblerPort".to_string(),
                    builder: "build_reference_context_assembler".to_string(),
                },
                MemoryPluginPortExport {
                    port: "LlmEvaluationPort".to_string(),
                    builder: "build_reference_evaluation".to_string(),
                },
            ],
            provider_kinds: vec![
                MemoryProviderKind::SearchEngine,
                MemoryProviderKind::GraphEngine,
                MemoryProviderKind::ExternalMemory,
            ],
            retriever_kinds: vec![
                LlmRetrieverKind::Sql,
                LlmRetrieverKind::Keyword,
                LlmRetrieverKind::Time,
                LlmRetrieverKind::Event,
                LlmRetrieverKind::Graph,
                LlmRetrieverKind::External,
            ],
            index_kinds: vec![
                MemoryIndexKind::Sql,
                MemoryIndexKind::Keyword,
                MemoryIndexKind::Time,
                MemoryIndexKind::Event,
                MemoryIndexKind::Graph,
            ],
            required_core_version: "0.1.0".to_string(),
            config_schema_ref: None,
            secret_refs: vec![],
            data_classes: vec![
                MemoryPluginDataClass::Tenant,
                MemoryPluginDataClass::Personal,
                MemoryPluginDataClass::Internal,
            ],
            capabilities: MemoryPluginCapabilities {
                canonical_store: true,
                event_log: true,
                candidate_lifecycle: true,
                habit_learning: true,
                retrieval_trace: true,
                deletion_propagation: true,
                audit_log: true,
                outbox_log: true,
                embedding_required: false,
            },
            degradation: MemoryPluginDegradationPolicy {
                mode: "fail_closed_reference_baseline".to_string(),
                returns_stale_hits: false,
            },
            migration: MemoryPluginMigrationCapabilities {
                export_supported: false,
                import_supported: false,
                dual_write_supported: false,
                shadow_read_supported: true,
            },
            observability: MemoryPluginObservabilityContract {
                metrics_prefix: "sdkwork_llm_reference_profiles".to_string(),
                redacts_payloads: true,
            },
            conformance: MemoryPluginConformanceContract {
                suite: "sdkwork-llm-plugin-conformance".to_string(),
                suite_version: "0.1.0".to_string(),
            },
        }
    }

    pub fn phase1_baseline_manifests() -> Vec<Self> {
        vec![
            Self::native_sql_baseline(),
            Self::reference_profiles_baseline(),
        ]
    }

    pub fn native_sql_for_test() -> Self {
        Self::native_sql_baseline()
    }

    pub fn reference_profiles_for_test() -> Self {
        Self::reference_profiles_baseline()
    }

    pub fn phase1_baseline_manifests_for_test() -> Vec<Self> {
        Self::phase1_baseline_manifests()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmImplementationKind {
    NativeSql,
    EventSourced,
    SearchFirst,
    GraphTemporal,
    LocalEmbedded,
    ExternalProviderBridge,
    HybridPlatform,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryPluginRole {
    Implementation,
    Store,
    Retriever,
    Index,
    Provider,
    Context,
    Evaluation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryDeploymentMode {
    Server,
    Container,
    Private,
    Local,
    Test,
    EvalOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryProviderKind {
    LanguageModel,
    EmbeddingModel,
    RerankModel,
    SearchEngine,
    GraphEngine,
    ExternalMemory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LlmRetrieverKind {
    Sql,
    Keyword,
    Dictionary,
    Time,
    Event,
    Vector,
    Graph,
    GrepFile,
    External,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryIndexKind {
    Sql,
    Keyword,
    Dictionary,
    Time,
    Event,
    Vector,
    Graph,
    GrepFile,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryPluginDataClass {
    Public,
    Internal,
    Tenant,
    Personal,
    Sensitive,
    Regulated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryPluginPortExport {
    pub port: String,
    pub builder: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryPluginCapabilities {
    pub canonical_store: bool,
    pub event_log: bool,
    pub candidate_lifecycle: bool,
    pub habit_learning: bool,
    pub retrieval_trace: bool,
    pub deletion_propagation: bool,
    pub audit_log: bool,
    pub outbox_log: bool,
    pub embedding_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryPluginDegradationPolicy {
    pub mode: String,
    pub returns_stale_hits: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryPluginMigrationCapabilities {
    pub export_supported: bool,
    pub import_supported: bool,
    pub dual_write_supported: bool,
    pub shadow_read_supported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryPluginObservabilityContract {
    pub metrics_prefix: String,
    pub redacts_payloads: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryPluginConformanceContract {
    pub suite: String,
    pub suite_version: String,
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
