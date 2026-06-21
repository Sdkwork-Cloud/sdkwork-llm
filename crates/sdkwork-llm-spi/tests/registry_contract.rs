use sdkwork_llm_spi::{
    LlmImplementationKind, LlmPluginManifest, MemoryPluginRegistry, LlmSpiError,
};

#[test]
fn registry_registers_plugins_and_finds_implementation_kinds() {
    let mut registry = MemoryPluginRegistry::default();
    let manifest = LlmPluginManifest::native_sql_for_test();

    registry.register(manifest).unwrap();

    let native_sql = registry.plugins_for_implementation(LlmImplementationKind::NativeSql);
    assert_eq!(native_sql.len(), 1);
    assert_eq!(native_sql[0].plugin_id, "sdkwork-llm-plugin-native-sql");
}

#[test]
fn registry_registers_phase1_baseline_plugins_for_every_implementation_kind() {
    let mut registry = MemoryPluginRegistry::default();
    for manifest in LlmPluginManifest::phase1_baseline_manifests_for_test() {
        registry.register(manifest).unwrap();
    }

    for implementation_kind in [
        LlmImplementationKind::NativeSql,
        LlmImplementationKind::LocalEmbedded,
        LlmImplementationKind::EventSourced,
        LlmImplementationKind::SearchFirst,
        LlmImplementationKind::GraphTemporal,
        LlmImplementationKind::ExternalProviderBridge,
        LlmImplementationKind::HybridPlatform,
    ] {
        assert_eq!(
            registry
                .plugins_for_implementation(implementation_kind.clone())
                .len(),
            1,
            "expected exactly one phase1 baseline plugin for {implementation_kind:?}"
        );
    }
}

#[test]
fn registry_rejects_duplicate_plugin_ids() {
    let mut registry = MemoryPluginRegistry::default();
    let manifest = LlmPluginManifest::native_sql_for_test();

    registry.register(manifest.clone()).unwrap();
    let err = registry.register(manifest).unwrap_err();

    assert!(matches!(err, LlmSpiError::DuplicatePluginId(_)));
}

#[test]
fn registry_validates_required_port_exports_before_runtime_serves() {
    let mut registry = MemoryPluginRegistry::default();
    registry
        .register(LlmPluginManifest::native_sql_for_test())
        .unwrap();

    registry
        .validate_required_ports(
            "sdkwork-llm-plugin-native-sql",
            &[
                "LlmRecordStorePort",
                "LlmEventStorePort",
                "LlmAuditStorePort",
                "LlmOutboxStorePort",
                "LlmCandidateStorePort",
                "LlmHabitStorePort",
                "LlmRetrievalTraceStorePort",
            ],
        )
        .unwrap();

    let err = registry
        .validate_required_ports(
            "sdkwork-llm-plugin-native-sql",
            &[
                "LlmRecordStorePort",
                "LlmEventStorePort",
                "LlmAuditStorePort",
                "LlmOutboxStorePort",
                "LlmCandidateStorePort",
                "LlmHabitStorePort",
                "LlmRetrievalTraceStorePort",
                "MemoryPolicyStorePort",
            ],
        )
        .unwrap_err();
    assert!(matches!(err, LlmSpiError::RequiredPortMissing { .. }));
}
