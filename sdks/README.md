# SDKWork LLM SDK Workspace

This directory owns SDKWork LLM SDK families and authority OpenAPI documents.

SDK families:

- `sdkwork-llm-sdk` for `sdkwork-llm-open-api` and `/llm/v3/api`
- `sdkwork-llm-app-sdk` for `sdkwork-llm.app` and `/app/v3/api`
- `sdkwork-llm-backend-sdk` for `sdkwork-llm.backend` and `/backend/v3/api`

Protected Open API clients use `X-API-Key` through generated SDK credential providers. They must not join app/backend token-manager client lists.

RPC SDK families are deferred until high-throughput backend/native RPC integration is needed.
