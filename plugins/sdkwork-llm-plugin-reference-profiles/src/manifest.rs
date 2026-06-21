use sdkwork_llm_spi::LlmPluginManifest;

pub const REFERENCE_PROFILES_PLUGIN_ID: &str = "sdkwork-llm-plugin-reference-profiles";

pub fn reference_profiles_manifest() -> LlmPluginManifest {
    LlmPluginManifest::reference_profiles_baseline()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceProfilePortBuilder {
    pub port_name: &'static str,
    pub builder_name: &'static str,
    pub ready: bool,
    pub fail_closed: bool,
}

pub fn build_reference_record_store() -> ReferenceProfilePortBuilder {
    ready_builder("LlmRecordStorePort", "build_reference_record_store")
}

pub fn build_reference_event_store() -> ReferenceProfilePortBuilder {
    ready_builder("LlmEventStorePort", "build_reference_event_store")
}

pub fn build_reference_audit_store() -> ReferenceProfilePortBuilder {
    ready_builder("LlmAuditStorePort", "build_reference_audit_store")
}

pub fn build_reference_outbox_store() -> ReferenceProfilePortBuilder {
    ready_builder("LlmOutboxStorePort", "build_reference_outbox_store")
}

pub fn build_reference_candidate_store() -> ReferenceProfilePortBuilder {
    ready_builder(
        "LlmCandidateStorePort",
        "build_reference_candidate_store",
    )
}

pub fn build_reference_habit_store() -> ReferenceProfilePortBuilder {
    ready_builder("LlmHabitStorePort", "build_reference_habit_store")
}

pub fn build_reference_retrieval_trace_store() -> ReferenceProfilePortBuilder {
    ready_builder(
        "LlmRetrievalTraceStorePort",
        "build_reference_retrieval_trace_store",
    )
}

pub fn build_reference_retriever() -> ReferenceProfilePortBuilder {
    ready_builder("LlmRetrieverPort", "build_reference_retriever")
}

pub fn build_reference_index() -> ReferenceProfilePortBuilder {
    ready_builder("LlmIndexPort", "build_reference_index")
}

pub fn build_reference_external_bridge() -> ReferenceProfilePortBuilder {
    ReferenceProfilePortBuilder {
        port_name: "ExternalLlmBridgePort",
        builder_name: "build_reference_external_bridge",
        ready: true,
        fail_closed: true,
    }
}

pub fn build_reference_context_assembler() -> ReferenceProfilePortBuilder {
    ready_builder(
        "LlmContextAssemblerPort",
        "build_reference_context_assembler",
    )
}

pub fn build_reference_evaluation() -> ReferenceProfilePortBuilder {
    ready_builder("LlmEvaluationPort", "build_reference_evaluation")
}

fn ready_builder(
    port_name: &'static str,
    builder_name: &'static str,
) -> ReferenceProfilePortBuilder {
    ReferenceProfilePortBuilder {
        port_name,
        builder_name,
        ready: true,
        fail_closed: false,
    }
}
