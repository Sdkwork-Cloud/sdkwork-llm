# Docker Deployment

Build and run the SDKWork LLM API server container from the repository root:

```powershell
docker build -f deployments/docker/Dockerfile -t sdkwork-llm:local .
docker run --rm -p 8080:8080 `
  -e SDKWORK_LLM_ENVIRONMENT=development `
  -e SDKWORK_LLM_DEV_AUTH_BYPASS=true `
  -e SDKWORK_DATABASE_URL=postgresql://sdkwork_ai_dev:change-me@host.docker.internal:5432/sdkwork_ai_dev `
  -e SDKWORK_DATABASE_SCHEMA=sdkwork_ai_dev `
  sdkwork-llm:local
```

The image exposes `SDKWORK_LLM_APPLICATION_PUBLIC_INGRESS_BIND` on `0.0.0.0:8080`, ships `/app/database` lifecycle assets, and defaults `SDKWORK_LLM_ENVIRONMENT=production` when no overrides are supplied. LLM and IAM modules reuse the same `SDKWORK_DATABASE_*` identity.

For local development without Docker, use `pnpm dev`, which loads `configs/topology/standalone.development.env` through `scripts/llm-dev.mjs`.
