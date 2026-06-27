# Kubernetes deployment

Owner: sdkwork-llm

Unified-process Memory API server manifests for cloud-hosted deployment.

## Files

- `deployment.yaml` — `sdkwork-llm-standalone-gateway` Deployment with health probes on `/healthz`
- `service.yaml` — ClusterIP service exposing port 8080

## Prerequisites

- Container image built from `deployments/docker/Dockerfile` (ships `/app/database` lifecycle assets)
- Secret `sdkwork-llm-database` with key `database-url` for Memory PostgreSQL runtime
- Secret `sdkwork-llm-iam-database` with key `database-url` for IAM PostgreSQL auth resolution

## Apply

```bash
kubectl apply -f deployments/kubernetes/
```

## Notes

Memory Phase 1 runs open, app, and backend API surfaces in a single unified process. Production auth always uses IAM database resolver; `SDKWORK_LLM_DEV_AUTH_BYPASS` is development-only.
