# sdkwork-llm

SDKWork LLM service and SDK families for embedding-optional AI LLM memory, self-learning, habit LLM memory, and provider-switchable retrieval.

## Standards

- Repository instructions: `AGENTS.md`
- Local component specs: `specs/README.md`
- Root SDKWork standards: `../sdkwork-specs/README.md`

## Verification

```powershell
node tools/materialize_phase1_contracts.mjs
powershell -ExecutionPolicy Bypass -File tools/verify_phase1.ps1
```
