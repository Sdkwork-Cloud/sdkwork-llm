//! SDKWork LLM test support and conformance helpers.

use sdkwork_llm_spi::LlmPluginManifest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConformanceCheckStatus {
    Passed,
    Failed,
    Pending,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceCheck {
    pub name: String,
    pub status: ConformanceCheckStatus,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryPluginConformanceReport {
    pub plugin_id: String,
    pub checks: Vec<ConformanceCheck>,
}

impl MemoryPluginConformanceReport {
    pub fn has_status(&self, name: &str, status: ConformanceCheckStatus) -> bool {
        self.checks
            .iter()
            .any(|check| check.name == name && check.status == status)
    }
}

#[derive(Debug, Default)]
pub struct MemoryPluginConformanceHarness;

impl MemoryPluginConformanceHarness {
    pub fn verify_manifest_skeleton(
        &self,
        manifest: &LlmPluginManifest,
    ) -> MemoryPluginConformanceReport {
        let mut checks = Vec::new();

        checks.push(check(
            "manifest_validation",
            if manifest.validate().is_ok() {
                ConformanceCheckStatus::Passed
            } else {
                ConformanceCheckStatus::Failed
            },
            "Manifest must satisfy SDKWork LLM plugin manifest rules.",
        ));
        checks.push(check(
            "required_port_declarations",
            if manifest.port_exports.is_empty() {
                ConformanceCheckStatus::Failed
            } else {
                ConformanceCheckStatus::Passed
            },
            "Plugin must declare executable port exports.",
        ));
        checks.push(check(
            "no_embedding_profile",
            if manifest.capabilities.embedding_required {
                ConformanceCheckStatus::Failed
            } else {
                ConformanceCheckStatus::Passed
            },
            "First native SQL profile must not require embeddings.",
        ));
        checks.push(check(
            "secret_redaction",
            if manifest.validate().is_ok() {
                ConformanceCheckStatus::Passed
            } else {
                ConformanceCheckStatus::Failed
            },
            "Manifest may contain secret references but no secret values.",
        ));
        checks.push(check(
            "runtime_plugin_path_separation",
            if manifest.package_name.contains(".sdkwork/plugins")
                || manifest.package_name.contains(".sdkwork\\plugins")
            {
                ConformanceCheckStatus::Failed
            } else {
                ConformanceCheckStatus::Passed
            },
            "Runtime plugins must live under plugins/, not .sdkwork/plugins/.",
        ));

        checks.push(port_backed_check(
            manifest,
            "store_crud",
            &["LlmRecordStorePort", "LlmEventStorePort"],
            "Record and event store ports are required for executable store CRUD.",
        ));
        checks.push(port_backed_check(
            manifest,
            "tenant_isolation",
            &["LlmRecordStorePort", "LlmEventStorePort"],
            "Tenant isolation is executable only when scoped record and event ports exist.",
        ));
        checks.push(capability_port_check(
            manifest.capabilities.deletion_propagation,
            manifest,
            "deletion_propagation",
            &["LlmRecordStorePort"],
            "Deletion propagation requires a record store delete path.",
        ));
        checks.push(port_backed_check(
            manifest,
            "retriever_and_index",
            &["LlmRetrieverPort", "LlmIndexPort"],
            "Retriever and index baseline requires retriever and index ports.",
        ));
        checks.push(capability_port_check(
            manifest.capabilities.retrieval_trace,
            manifest,
            "retrieval_trace",
            &["LlmRetrievalTraceStorePort"],
            "Retrieval trace requires an executable retrieval trace store port.",
        ));
        checks.push(capability_port_check(
            manifest.capabilities.audit_log && manifest.capabilities.outbox_log,
            manifest,
            "audit_and_outbox",
            &["LlmAuditStorePort", "LlmOutboxStorePort"],
            "Audit and outbox behavior requires audit and outbox store ports.",
        ));
        checks.push(capability_port_check(
            manifest.capabilities.candidate_lifecycle,
            manifest,
            "candidate_lifecycle",
            &["LlmCandidateStorePort"],
            "Candidate lifecycle is pending until an executable candidate store port exists.",
        ));
        checks.push(capability_port_check(
            manifest.capabilities.habit_learning,
            manifest,
            "habit_learning",
            &["LlmHabitStorePort"],
            "Habit learning is pending until an executable habit store port exists.",
        ));
        checks.push(port_backed_check(
            manifest,
            "external_bridge",
            &["ExternalLlmBridgePort"],
            "External bridge baseline requires an explicit fail-closed bridge port.",
        ));

        MemoryPluginConformanceReport {
            plugin_id: manifest.plugin_id.clone(),
            checks,
        }
    }
}

fn capability_port_check(
    capability_enabled: bool,
    manifest: &LlmPluginManifest,
    name: impl Into<String>,
    ports: &[&str],
    message: impl Into<String>,
) -> ConformanceCheck {
    if !capability_enabled {
        return check(name, ConformanceCheckStatus::Pending, message);
    }

    port_backed_check(manifest, name, ports, message)
}

fn port_backed_check(
    manifest: &LlmPluginManifest,
    name: impl Into<String>,
    ports: &[&str],
    message: impl Into<String>,
) -> ConformanceCheck {
    let status = if ports.iter().all(|port| {
        manifest
            .port_exports
            .iter()
            .any(|export| export.port == *port)
    }) {
        ConformanceCheckStatus::Passed
    } else {
        ConformanceCheckStatus::Pending
    };

    check(name, status, message)
}

fn check(
    name: impl Into<String>,
    status: ConformanceCheckStatus,
    message: impl Into<String>,
) -> ConformanceCheck {
    ConformanceCheck {
        name: name.into(),
        status,
        message: message.into(),
    }
}
