use sdkwork_router_llm_open_api::{llm_open_api_public_path_prefixes, open_route_manifest};

#[test]
fn open_route_manifest_resolves_contract_routes() {
    let manifest = open_route_manifest();
    let route = manifest
        .match_route("GET", "/llm/v3/api/llm/capabilities")
        .expect("capabilities route");
    assert_eq!(route.operation_id, "capabilities.retrieve");
    manifest
        .validate_public_path_prefixes(&llm_open_api_public_path_prefixes())
        .expect("public prefixes must not cover protected routes");
}
