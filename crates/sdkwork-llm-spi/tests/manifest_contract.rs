use sdkwork_llm_spi::{LlmImplementationKind, LlmPluginManifest, MemoryPluginRole};

#[test]
fn native_sql_manifest_deserializes_and_declares_no_embedding_baseline() {
    let manifest: LlmPluginManifest = serde_json::from_str(
        r#"{
          "schemaVersion": 1,
          "kind": "sdkwork.llm.plugin",
          "pluginId": "sdkwork-llm-plugin-native-sql",
          "packageName": "sdkwork-llm-plugin-native-sql",
          "displayName": "SDKWork LLM Native SQL Plugin",
          "version": "0.1.0",
          "owner": "sdkwork-llm",
          "implementationKinds": ["native_sql", "local_embedded"],
          "pluginRoles": ["implementation", "store"],
          "deploymentModes": ["server", "container", "private", "local", "test"],
          "portExports": [
            {"port": "LlmRecordStorePort", "builder": "build_native_sql_record_store"},
            {"port": "LlmEventStorePort", "builder": "build_native_sql_event_store"},
            {"port": "LlmAuditStorePort", "builder": "build_native_sql_audit_store"},
            {"port": "LlmOutboxStorePort", "builder": "build_native_sql_outbox_store"},
            {"port": "LlmCandidateStorePort", "builder": "build_native_sql_candidate_store"},
            {"port": "LlmHabitStorePort", "builder": "build_native_sql_habit_store"},
            {"port": "LlmRetrievalTraceStorePort", "builder": "build_native_sql_retrieval_trace_store"}
          ],
          "providerKinds": [],
          "retrieverKinds": [],
          "indexKinds": [],
          "requiredCoreVersion": "0.1.0",
          "secretRefs": [],
          "dataClasses": ["tenant", "personal"],
          "capabilities": {
            "canonicalStore": true,
            "eventLog": true,
            "candidateLifecycle": true,
            "habitLearning": true,
            "retrievalTrace": true,
            "deletionPropagation": true,
            "auditLog": true,
            "outboxLog": true,
            "embeddingRequired": false
          },
          "degradation": {"mode": "fail_required_degrade_optional", "returnsStaleHits": false},
          "migration": {"exportSupported": true, "importSupported": true, "dualWriteSupported": false, "shadowReadSupported": true},
          "observability": {"metricsPrefix": "sdkwork_llm_native_sql", "redactsPayloads": true},
          "conformance": {"suite": "sdkwork-llm-plugin-conformance", "suiteVersion": "0.1.0"}
        }"#,
    )
    .unwrap();

    assert_eq!(manifest.schema_version, 1);
    assert!(manifest
        .implementation_kinds
        .contains(&LlmImplementationKind::NativeSql));
    assert!(manifest
        .implementation_kinds
        .contains(&LlmImplementationKind::LocalEmbedded));
    assert!(manifest
        .plugin_roles
        .contains(&MemoryPluginRole::Implementation));
    assert!(!manifest.capabilities.embedding_required);
    assert!(manifest.validate().is_ok());
}

#[test]
fn phase1_baseline_manifests_cover_all_implementation_families() {
    let manifests = LlmPluginManifest::phase1_baseline_manifests_for_test();
    let covered_kinds = manifests
        .iter()
        .flat_map(|manifest| manifest.implementation_kinds.iter())
        .collect::<Vec<_>>();

    for implementation_kind in [
        LlmImplementationKind::NativeSql,
        LlmImplementationKind::LocalEmbedded,
        LlmImplementationKind::EventSourced,
        LlmImplementationKind::SearchFirst,
        LlmImplementationKind::GraphTemporal,
        LlmImplementationKind::ExternalProviderBridge,
        LlmImplementationKind::HybridPlatform,
    ] {
        assert!(
            covered_kinds.contains(&&implementation_kind),
            "phase1 baseline manifests must cover {implementation_kind:?}"
        );
    }

    for manifest in manifests {
        assert!(manifest.validate().is_ok());
    }
}

#[test]
fn manifest_rejects_secret_values_and_agent_plugin_paths() {
    let mut manifest = LlmPluginManifest::native_sql_for_test();
    manifest
        .secret_refs
        .push("literal-token-secret".to_string());
    assert!(manifest.validate().is_err());

    let mut manifest = LlmPluginManifest::native_sql_for_test();
    manifest.package_name = ".sdkwork/plugins/sdkwork-llm-plugin-native-sql".to_string();
    assert!(manifest.validate().is_err());
}

#[test]
fn manifest_rejects_enabled_capabilities_without_required_ports() {
    let mut manifest = LlmPluginManifest::native_sql_for_test();
    manifest
        .port_exports
        .retain(|export| export.port != "LlmAuditStorePort");

    assert!(manifest.validate().is_err());

    let mut manifest = LlmPluginManifest::native_sql_for_test();
    manifest
        .port_exports
        .retain(|export| export.port != "LlmOutboxStorePort");

    assert!(manifest.validate().is_err());
}
