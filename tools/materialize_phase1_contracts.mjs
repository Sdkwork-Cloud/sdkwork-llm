import fs from "node:fs";
import path from "node:path";

const root = process.cwd();
const owner = "sdkwork-llm";
const domain = "intelligence";
const capability = "llm";
const version = "0.1.0";
const standardVersion = "2026-06-10";
const llmOpenApiPrefix = "/llm/v3/api";
const llmOpenApiSchemaUrl = "/llm/v3/openapi.json";

function writeText(relativePath, content) {
  const target = path.join(root, relativePath);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.writeFileSync(target, content.replace(/\r\n/g, "\n"), "utf8");
  console.log(`wrote ${relativePath}`);
}

function writeJson(relativePath, value) {
  writeText(relativePath, `${JSON.stringify(value, null, 2)}\n`);
}

const packageByLanguage = {
  appbaseApp: {
    typescript: "@sdkwork/appbase-app-sdk",
    rust: "sdkwork-appbase-app-sdk",
    java: "com.sdkwork:sdkwork-appbase-app-sdk",
    python: "sdkwork-appbase-app-sdk",
    go: "github.com/sdkwork/sdkwork-appbase-app-sdk"
  },
  appbaseBackend: {
    typescript: "@sdkwork/appbase-backend-sdk",
    rust: "sdkwork-appbase-backend-sdk",
    java: "com.sdkwork:sdkwork-appbase-backend-sdk",
    python: "sdkwork-appbase-backend-sdk",
    go: "github.com/sdkwork/sdkwork-appbase-backend-sdk"
  },
  driveApp: {
    typescript: "@sdkwork/drive-app-sdk",
    rust: "sdkwork-drive-app-sdk",
    java: "com.sdkwork:sdkwork-drive-app-sdk",
    python: "sdkwork-drive-app-sdk",
    go: "github.com/sdkwork/sdkwork-drive-app-sdk"
  },
  driveBackend: {
    typescript: "@sdkwork/drive-backend-sdk",
    rust: "sdkwork-drive-backend-sdk",
    java: "com.sdkwork:sdkwork-drive-backend-sdk",
    python: "sdkwork-drive-backend-sdk",
    go: "github.com/sdkwork/sdkwork-drive-backend-sdk"
  },
  knowledgebaseApp: {
    typescript: "@sdkwork/knowledgebase-app-sdk",
    rust: "sdkwork-knowledgebase-app-sdk",
    java: "com.sdkwork:sdkwork-knowledgebase-app-sdk",
    python: "sdkwork-knowledgebase-app-sdk",
    go: "github.com/sdkwork/sdkwork-knowledgebase-app-sdk"
  },
  knowledgebaseBackend: {
    typescript: "@sdkwork/knowledgebase-backend-sdk",
    rust: "sdkwork-knowledgebase-backend-sdk",
    java: "com.sdkwork:sdkwork-knowledgebase-backend-sdk",
    python: "sdkwork-knowledgebase-backend-sdk",
    go: "github.com/sdkwork/sdkwork-knowledgebase-backend-sdk"
  }
};

const appSdkDependencies = [
  {
    workspace: "sdkwork-appbase-app-sdk",
    role: "appbase-identity-and-session-capability",
    required: true,
    dependencyMode: "consumer-sdk",
    apiPrefix: "/app/v3/api",
    apiAuthority: "sdkwork-appbase-app-api",
    generatedTransportImportPolicy: "forbidden",
    packageByLanguage: packageByLanguage.appbaseApp
  },
  {
    workspace: "sdkwork-drive-app-sdk",
    role: "drive-memory-export-import-capability",
    required: false,
    dependencyMode: "consumer-sdk",
    apiPrefix: "/app/v3/api",
    apiAuthority: "sdkwork-drive.app",
    generatedTransportImportPolicy: "forbidden",
    packageByLanguage: packageByLanguage.driveApp
  },
  {
    workspace: "sdkwork-knowledgebase-app-sdk",
    role: "knowledgebase-context-composition-capability",
    required: false,
    dependencyMode: "consumer-sdk",
    apiPrefix: "/app/v3/api",
    apiAuthority: "sdkwork-knowledgebase.app",
    generatedTransportImportPolicy: "forbidden",
    packageByLanguage: packageByLanguage.knowledgebaseApp
  }
];

const backendSdkDependencies = [
  {
    workspace: "sdkwork-appbase-backend-sdk",
    role: "appbase-backend-management-capability",
    required: true,
    dependencyMode: "consumer-sdk",
    apiPrefix: "/backend/v3/api",
    apiAuthority: "sdkwork-appbase-backend-api",
    generatedTransportImportPolicy: "forbidden",
    packageByLanguage: packageByLanguage.appbaseBackend
  },
  {
    workspace: "sdkwork-drive-backend-sdk",
    role: "drive-backend-memory-export-and-retention-capability",
    required: false,
    dependencyMode: "consumer-sdk",
    apiPrefix: "/backend/v3/api",
    apiAuthority: "sdkwork-drive.backend",
    generatedTransportImportPolicy: "forbidden",
    packageByLanguage: packageByLanguage.driveBackend
  },
  {
    workspace: "sdkwork-knowledgebase-backend-sdk",
    role: "knowledgebase-backend-index-and-eval-capability",
    required: false,
    dependencyMode: "consumer-sdk",
    apiPrefix: "/backend/v3/api",
    apiAuthority: "sdkwork-knowledgebase.backend",
    generatedTransportImportPolicy: "forbidden",
    packageByLanguage: packageByLanguage.knowledgebaseBackend
  }
];

const platformWorkspaceDependencies = [
  {
    workspace: "sdkwork-web-framework",
    role: "http-web-framework-runtime",
    required: true,
    dependencyMode: "platform-framework",
    generatedTransportImportPolicy: "forbidden"
  },
  {
    workspace: "sdkwork-database",
    role: "database-runtime",
    required: true,
    dependencyMode: "platform-framework",
    generatedTransportImportPolicy: "forbidden"
  },
  {
    workspace: "sdkwork-appbase",
    role: "appbase-platform-runtime",
    required: true,
    dependencyMode: "platform-framework",
    generatedTransportImportPolicy: "forbidden"
  },
  {
    workspace: "sdkwork-utils",
    role: "cross-language-utility-runtime",
    required: true,
    dependencyMode: "platform-framework",
    generatedTransportImportPolicy: "forbidden"
  },
  {
    workspace: "sdkwork-id",
    role: "id-generation-runtime",
    required: true,
    dependencyMode: "platform-framework",
    generatedTransportImportPolicy: "forbidden"
  },
  {
    workspace: "sdkwork-sdk-generator",
    role: "sdk-generation-tooling",
    required: true,
    dependencyMode: "platform-tooling",
    generatedTransportImportPolicy: "forbidden"
  }
];

const rootCanonicalSpecs = [
  ["SOUL.md", "../sdkwork-specs/SOUL.md", "SDKWork execution soul."],
  ["COMPONENT_SPEC.md", "../sdkwork-specs/COMPONENT_SPEC.md", "Component-local contract and discovery rules."],
  ["NAMING_SPEC.md", "../sdkwork-specs/NAMING_SPEC.md", "Canonical naming for domains, APIs, SDKs, tables, and events."],
  ["DOMAIN_SPEC.md", "../sdkwork-specs/DOMAIN_SPEC.md", "Canonical domain and capability boundaries."],
  ["API_SPEC.md", "../sdkwork-specs/API_SPEC.md", "HTTP/OpenAPI and generated SDK contract rules."],
  ["SDK_SPEC.md", "../sdkwork-specs/SDK_SPEC.md", "SDK generation, SDK dependency, and SDK integration rules."],
  ["SDK_WORKSPACE_GENERATION_SPEC.md", "../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md", "SDK workspace and OpenAPI authority placement rules."],
  ["WEB_FRAMEWORK_SPEC.md", "../sdkwork-specs/WEB_FRAMEWORK_SPEC.md", "Mandatory sdkwork-web-framework integration for HTTP *-api surfaces."],
  ["WEB_BACKEND_SPEC.md", "../sdkwork-specs/WEB_BACKEND_SPEC.md", "Backend route, service, repository, provider, and request-context boundaries."],
  ["DEPLOYMENT_SPEC.md", "../sdkwork-specs/DEPLOYMENT_SPEC.md", "Standalone/cloud deployment profile parity and runtime packaging."],
  ["DATABASE_SPEC.md", "../sdkwork-specs/DATABASE_SPEC.md", "Schema registry, table contract, migration, and storage rules."],
  ["EVENT_SPEC.md", "../sdkwork-specs/EVENT_SPEC.md", "Domain event and outbox contract rules."],
  ["PRIVACY_SPEC.md", "../sdkwork-specs/PRIVACY_SPEC.md", "Memory privacy, sensitive data, retention, export, and deletion rules."],
  ["OBSERVABILITY_SPEC.md", "../sdkwork-specs/OBSERVABILITY_SPEC.md", "Request, retrieval, provider, job, audit, and evaluation observability rules."],
  ["TEST_SPEC.md", "../sdkwork-specs/TEST_SPEC.md", "Verification, contract testing, and evidence-before-completion rules."]
].map(([file, specPath, purpose]) => ({ file, path: specPath, purpose }));

const sdkCanonicalSpecs = [
  ["API_SPEC.md", "../sdkwork-specs/API_SPEC.md", "HTTP/OpenAPI and generated SDK contract rules."],
  ["SDK_SPEC.md", "../sdkwork-specs/SDK_SPEC.md", "SDK generation, SDK dependency, and SDK integration rules."],
  ["SDK_WORKSPACE_GENERATION_SPEC.md", "../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md", "SDK workspace, SDK family naming, API authority naming, and OpenAPI generation rules."],
  ["COMPONENT_SPEC.md", "../sdkwork-specs/COMPONENT_SPEC.md", "SDK family component spec and discovery rules."]
].map(([file, specPath, purpose]) => ({ file, path: specPath, purpose }));

function writeAgentEntrypoints() {
  writeText("AGENTS.md", `# Repository Guidelines

<!-- SDKWORK-AGENTS-GENERATED: v2 -->

## SDKWORK Soul

Read \`../sdkwork-specs/SOUL.md\` before executing tasks in this root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this root:

- \`../sdkwork-specs/README.md\`
- \`../sdkwork-specs/SOUL.md\`
- \`../sdkwork-specs/AGENTS_SPEC.md\`
- \`../sdkwork-specs/PNPM_SCRIPT_SPEC.md\`
- \`../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md\`
- \`../sdkwork-specs/CODE_STYLE_SPEC.md\`
- \`../sdkwork-specs/NAMING_SPEC.md\`

Do not copy root standard text into this repository. If these relative paths do not resolve, stop and report the broken workspace layout.

## Application Identity

Read \`sdkwork.app.config.json\` only when the task touches LLM application behavior, runtime config, SDK wiring, release metadata, app-owned capabilities, packaging, or deployment. For unrelated documentation or tooling work, do not expand into the full app manifest unless evidence requires it.

## Local Dictionary Structure

- \`AGENTS.md\`: repository agent entrypoint and relative SDKWork spec index.
- \`CLAUDE.md\`, \`GEMINI.md\`, \`CODEX.md\`: compatibility shims that point to \`AGENTS.md\` and must not duplicate rules.
- \`sdkwork.app.config.json\`: LLM application identity, runtime, release, and capability metadata.
- \`sdkwork.workflow.json\`: GitHub packaging/release workflow manifest governed by \`GITHUB_WORKFLOW_SPEC.md\`.
- \`.github/workflows/package.yml\`: thin reusable workflow call only.
- \`.sdkwork/\`: repository/application AI workspace metadata, local skills, local plugins, and manifests.
- \`specs/\`: local application/component contracts and narrowing rules.
- \`apis/\`: LLM-owned API contract sources and materialized OpenAPI inputs.
- \`apps/\`: reserved for future client application roots.
- \`crates/\`: reusable Rust service, repository, route, and API server crates.
- \`sdks/\`: SDK families, SDK generation manifests, route manifests, and generated SDK artifacts.
- \`database/\`: database contract, baseline DDL, migrations, seeds, and drift policy.
- \`configs/\`, \`deployments/\`, \`scripts/\`, \`tools/\`, \`docs/\`, \`tests/\`: config templates, deployment descriptors, thin command entrypoints, validators, documentation, and verification assets.
- \`package.json\`, \`Cargo.toml\`: language/build manifests.

## Spec Resolution Order

Use dynamic progressive loading:

1. Read this \`AGENTS.md\` and any nearer component-level \`AGENTS.md\`.
2. Read \`sdkwork.app.config.json\` only when app behavior, runtime config, SDK wiring, release, packaging, or app-owned capabilities are touched.
3. Read local \`specs/README.md\` and \`specs/component.spec.json\` only when the task touches that local contract.
4. Read local \`.sdkwork/README.md\`, \`.sdkwork/skills/\`, and \`.sdkwork/plugins/\` only when local agent extensions are relevant.
5. Read \`../sdkwork-specs/README.md\`, then only the task-specific root specs.
6. Inspect implementation files after the dictionary and relevant specs are clear.

Do not load the whole repository or every root spec before identifying the task surface.

## Required Specs By Task Type

- Agent/workflow changes: \`../sdkwork-specs/SOUL.md\`, \`../sdkwork-specs/AGENTS_SPEC.md\`, \`../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md\`, \`../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md\`, and \`../sdkwork-specs/TEST_SPEC.md\`.
- Package script changes: \`../sdkwork-specs/PNPM_SCRIPT_SPEC.md\`, \`../sdkwork-specs/APP_RUNTIME_TOPOLOGY_SPEC.md\`, \`../sdkwork-specs/CONFIG_SPEC.md\`, and \`../sdkwork-specs/TEST_SPEC.md\`.
- Any code change: \`../sdkwork-specs/CODE_STYLE_SPEC.md\`, \`../sdkwork-specs/NAMING_SPEC.md\`, plus only the touched language/framework spec.
- Rust code: \`../sdkwork-specs/RUST_CODE_SPEC.md\`; add \`../sdkwork-specs/RUST_RPC_SPEC.md\` when RPC is touched.
- API/SDK changes: \`../sdkwork-specs/API_SPEC.md\`, \`../sdkwork-specs/WEB_FRAMEWORK_SPEC.md\`, \`../sdkwork-specs/WEB_BACKEND_SPEC.md\`, \`../sdkwork-specs/SDK_SPEC.md\`, \`../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md\`, and \`../sdkwork-specs/TEST_SPEC.md\`.
- Database changes: \`../sdkwork-specs/DATABASE_SPEC.md\`, \`../sdkwork-specs/DATABASE_FRAMEWORK_SPEC.md\`, \`../sdkwork-specs/PRIVACY_SPEC.md\`, and \`../sdkwork-specs/TEST_SPEC.md\`.
- Runtime/deployment/release changes: \`../sdkwork-specs/CONFIG_SPEC.md\`, \`../sdkwork-specs/ENVIRONMENT_SPEC.md\`, \`../sdkwork-specs/DEPLOYMENT_SPEC.md\`, \`../sdkwork-specs/RELEASE_SPEC.md\`, and \`../sdkwork-specs/GITHUB_WORKFLOW_SPEC.md\`.
- Provider/integration changes: \`../sdkwork-specs/INTEGRATION_SPEC.md\`, \`../sdkwork-specs/SECURITY_SPEC.md\`, and \`../sdkwork-specs/PRIVACY_SPEC.md\`.

Language-specific specs are on-demand; do not load Rust, Java, TypeScript, and frontend specs for unrelated tasks.

## Code Style Rules

Read \`../sdkwork-specs/CODE_STYLE_SPEC.md\` and \`../sdkwork-specs/NAMING_SPEC.md\` before code changes. Generated SDK output under \`generated/server-openapi\` must not be hand-edited. Fix OpenAPI, route manifests, generator input, or approved composed facades, then regenerate. Use \`sdkwork-utils-rust\` and \`sdkwork-id-core\` for shared helpers instead of duplicating utility logic locally.

## Build, Test, and Verification

Use canonical root package scripts from \`PNPM_SCRIPT_SPEC.md\`:

\`\`\`powershell
pnpm verify
pnpm check
pnpm topology:validate
pnpm db:validate
\`\`\`

## Agent Execution Rules

Do not rely on memory when a relevant SDKWork spec exists. Do not replace generated SDK calls with raw HTTP. Stop when the relative specs path, app identity, component spec, API authority, SDK family, table prefix, or provider ownership is ambiguous.

## Human Review Rules

Human review is required for breaking public API changes, schema migrations, privacy/security exceptions, generated SDK ownership changes, provider lock-in decisions, and destructive filesystem or data operations.
`);

  const shim = (toolName) => `# ${toolName} Entry

This file is a compatibility shim for ${toolName}. The authoritative SDKWork agent entrypoint is \`AGENTS.md\`.

Read in this order:

1. \`AGENTS.md\`
2. \`../sdkwork-specs/SOUL.md\`
3. \`../sdkwork-specs/AGENTS_SPEC.md\`
4. Task-specific files from \`../sdkwork-specs/README.md\`

Do not duplicate or override SDKWork rules here.
`;
  writeText("CODEX.md", shim("Codex"));
  writeText("CLAUDE.md", shim("Claude Code"));
  writeText("GEMINI.md", shim("Gemini CLI"));

  writeText(".sdkwork/README.md", `# SDKWork LLM Workspace Metadata

This directory is the source-controlled SDKWork workspace metadata root for \`sdkwork-llm\`.

It is distinct from generated SDK output \`.sdkwork/\` directories under \`generated/server-openapi\`.
Generated output must not store repository skills, plugins, runtime files, databases, logs, caches, or secrets.

Canonical standards:

- \`../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md\`
- \`../sdkwork-specs/AGENTS_SPEC.md\`
- \`../sdkwork-specs/COMPONENT_SPEC.md\`
`);
  writeText(".sdkwork/skills/README.md", `# SDKWork LLM Local Skills

No repository-local skills are defined yet.

Add local skills only when they encode repeatable LLM-specific workflows that should live with this repository. Do not copy root SDKWork standards into skills.
`);
  writeText(".sdkwork/plugins/README.md", `# SDKWork LLM Local Plugins

No repository-local plugins are defined yet.

Add local plugins only when this repository needs checked-in plugin metadata. Do not place generated SDK control-plane files here.
`);
}

function writeAppManifest() {
  writeJson("sdkwork.app.config.json", {
    schemaVersion: 3,
    kind: "sdkwork.app",
    app: {
      key: owner,
      name: "SDKWork LLM",
      displayName: "SDKWork LLM",
      description: "SDKWork LLM service and SDK families for embedding-optional AI LLM memory, self-learning, habit LLM memory, and provider-switchable retrieval.",
      vendor: "SDKWork",
      officialWebsiteUrl: "https://sdkwork.com/apps/sdkwork-llm",
      supportUrl: "https://sdkwork.com/support",
      privacyPolicyUrl: "https://sdkwork.com/privacy",
      termsOfServiceUrl: "https://sdkwork.com/terms",
      iconUrl: "https://cdn.sdkwork.com/apps/sdkwork-llm/assets/icon-1024.png",
      appType: "SDK",
      versionSource: "manifest",
      identifiers: {
        packageName: null,
        bundleId: null,
        desktopAppId: null,
        containerImage: "registry.sdkwork.com/apps/sdkwork-llm"
      }
    },
    backend: {
      profileKey: "backend-root-admin",
      ownerMode: "tenant",
      grantMode: "current",
      platform: "API",
      appId: null,
      tenantId: "100001",
      organizationId: "0"
    },
    runtime: {
      family: "server",
      framework: "rust-service",
      runtimes: ["API"],
      deliveryModes: ["CONTAINER_IMAGE"],
      defaultPlatform: "API",
      defaultArchitecture: "x64"
    },
    media: {
      icons: {
        primary: {
          id: "sdkwork-llm-primary-icon",
          type: "ICON",
          purpose: "PRIMARY",
          url: "https://cdn.sdkwork.com/apps/sdkwork-llm/assets/icon-1024.png",
          platform: "API",
          locale: "en-US",
          width: 1024,
          height: 1024,
          format: "PNG",
          fileSizeBytes: 524288,
          alphaChannel: false,
          sortOrder: 0,
          enabled: true,
          metadata: {
            generatedPlaceholder: true
          }
        },
        platform: [],
        metadata: {
          generatedBy: "tools/materialize_phase1_contracts.mjs"
        }
      },
      screenshots: [],
      previews: [
        {
          id: "sdkwork-llm-catalog-preview",
          type: "PREVIEW_IMAGE",
          purpose: "CATALOG_PREVIEW",
          url: "https://cdn.sdkwork.com/apps/sdkwork-llm/media/preview-cover.png",
          platform: "API",
          locale: "en-US",
          width: 1600,
          height: 900,
          format: "PNG",
          fileSizeBytes: 786432,
          alphaChannel: false,
          caption: "SDKWork LLM service preview.",
          sortOrder: 0,
          enabled: true,
          metadata: {
            generatedPlaceholder: true,
            altText: "SDKWork LLM service preview cover."
          }
        }
      ],
      metadata: {
        assetVersion: version,
        defaultLocale: "en-US"
      }
    },
    publish: {
      status: "DRAFT",
      installSkill: {
        name: "sdkwork-skills-app"
      },
      platforms: ["API"],
      installPlatforms: ["API"],
      defaultPackageId: "container-x64-server-docker-image",
      storeUrl: "https://sdkwork.com/apps/sdkwork-llm",
      stores: [],
      config: {
        workspaceRoot: "sdkwork-llm",
        framework: "rust-service",
        managedBy: "tools/materialize_phase1_contracts.mjs"
      }
    },
    environments: {
      development: {
        accessUrl: "https://api-dev.sdkwork.com/apps/sdkwork-llm",
        deployUrl: "https://api-dev.sdkwork.com/apps/sdkwork-llm",
        deployEnv: "dev"
      },
      test: {
        accessUrl: "https://api-test.sdkwork.com/apps/sdkwork-llm",
        deployUrl: "https://api-test.sdkwork.com/apps/sdkwork-llm",
        deployEnv: "test"
      },
      production: {
        accessUrl: "https://api.sdkwork.com/apps/sdkwork-llm",
        deployUrl: "https://api.sdkwork.com/apps/sdkwork-llm",
        deployEnv: "production"
      }
    },
    artifacts: {
      installConfig: {
        defaultPackageId: "container-x64-server-docker-image",
        installCommand: "sdkwork install sdkwork-llm",
        launchCommand: "sdkwork open sdkwork-llm",
        uninstallCommand: "sdkwork uninstall sdkwork-llm",
        packages: [
          {
            id: "container-x64-server-docker-image",
            name: "SDKWork LLM Server Container",
            sourceType: "CONTAINER_IMAGE",
            packageFormat: "DOCKER_IMAGE",
            platform: "API",
            url: "https://registry.sdkwork.com/v2/apps/sdkwork-llm/manifests/0.1.0",
            enabled: false,
            metadata: {
              image: "registry.sdkwork.com/apps/sdkwork-llm:0.1.0",
              digestRequiredBeforeRelease: true
            },
            architecture: "x64",
            checksumAlgorithm: "SHA-256",
            checksum: "4d656d6f72794472616674436f6e74726163744f6e6c794e6f7452656c65617365"
          }
        ],
        metadata: {
          workspaceRoot: "sdkwork-llm",
          framework: "rust-service",
          packageManager: "cargo",
          contractPhase: "phase1-draft"
        }
      }
    },
    release: {
      currentVersion: version,
      defaultChannel: "DEV",
      latest: {
        DEV: version
      },
      notes: [
        {
          version,
          releaseChannel: "DEV",
          title: "SDKWork LLM 0.1.0 Draft",
          summary: "Initial SDKWork LLM contract skeleton.",
          content: "Initial SDKWork LLM standard contracts for schema registry, app-api, backend-api, and SDK family metadata.",
          highlights: [
            "Provider-routable LLM execution architecture",
            "Multi-protocol LLM provider routing",
            "SDKWork app-api and backend-api authority drafts"
          ],
          packageIds: ["container-x64-server-docker-image"],
          publishedAt: "2026-06-10T00:00:00Z",
          current: true,
          forceUpdate: false,
          minSupportedVersion: version,
          metadata: {
            draft: true
          }
        }
      ]
    },
    security: {
      checksumRequired: false,
      signatureRequired: false,
      sbomRequired: false
    },
    devApp: {
      build: {
        targets: []
      },
      sourceRoot: "sdkwork-llm"
    },
    metadata: {
      standardOwner: "sdkwork-platform",
      initializedAt: "2026-06-10T00:00:00Z",
      managedBy: "tools/materialize_phase1_contracts.mjs"
    }
  });
}

function writeRootSpecs() {
  writeText("specs/README.md", `# SDKWork LLM Component Specs

This directory is the local component contract entrypoint for \`sdkwork-llm\`.

Authoritative root standards remain in \`../sdkwork-specs/\`. Local specs may narrow or instantiate those standards for Memory, but they must not redefine them.

Primary standards for this component:

- \`../sdkwork-specs/SOUL.md\`
- \`../sdkwork-specs/COMPONENT_SPEC.md\`
- \`../sdkwork-specs/NAMING_SPEC.md\`
- \`../sdkwork-specs/DOMAIN_SPEC.md\`
- \`../sdkwork-specs/API_SPEC.md\`
- \`../sdkwork-specs/SDK_SPEC.md\`
- \`../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md\`
- \`../sdkwork-specs/WEB_BACKEND_SPEC.md\`
- \`../sdkwork-specs/DATABASE_SPEC.md\`
- \`../sdkwork-specs/EVENT_SPEC.md\`
- \`../sdkwork-specs/PRIVACY_SPEC.md\`
- \`../sdkwork-specs/OBSERVABILITY_SPEC.md\`

Local design authority:

- \`docs/superpowers/specs/2026-06-10-ai-llm-architecture-design.md\`
- \`docs/superpowers/specs/2026-06-10-llm-spi-plugin-architecture-design.md\`

Draft contract artifacts:

- \`docs/schema-registry/tables/*.yaml\`
- \`sdks/sdkwork-llm-sdk/openapi/llm-open-api.openapi.json\`
- \`sdks/sdkwork-llm-app-sdk/openapi/llm-app-api.openapi.json\`
- \`sdks/sdkwork-llm-backend-sdk/openapi/llm-backend-api.openapi.json\`
- \`sdks/sdkwork-llm-sdk/.sdkwork-assembly.json\`
- \`sdks/sdkwork-llm-app-sdk/.sdkwork-assembly.json\`
- \`sdks/sdkwork-llm-backend-sdk/.sdkwork-assembly.json\`

Phase 1 verification:

\`\`\`powershell
node tools/materialize_phase1_contracts.mjs
powershell -ExecutionPolicy Bypass -File tools/verify_phase1.ps1
\`\`\`
`);

  writeJson("specs/component.spec.json", {
    schemaVersion: 1,
    kind: "sdkwork.component.spec",
    component: {
      name: owner,
      displayName: "SDKWork LLM",
      version,
      type: "web-backend-service",
      root: "sdkwork-llm",
      domain,
      capability,
      surface: "backend-service",
      languages: ["rust", "typescript"],
      generated: false,
      status: "draft",
      manifests: [
        "sdkwork.app.config.json",
        "AGENTS.md",
        "specs/component.spec.json",
        "sdks/sdkwork-llm-sdk/.sdkwork-assembly.json",
        "sdks/sdkwork-llm-app-sdk/.sdkwork-assembly.json",
        "sdks/sdkwork-llm-backend-sdk/.sdkwork-assembly.json"
      ]
    },
    canonicalSpecs: rootCanonicalSpecs,
    contracts: {
      publicExports: [],
      runtimeEntrypoints: [],
      routeManifest: null,
      apiAuthorities: [
        {
          name: "sdkwork-llm-open-api",
          prefix: llmOpenApiPrefix,
          authorityOpenApi: "sdks/sdkwork-llm-sdk/openapi/llm-open-api.openapi.json",
          sdkFamily: "sdkwork-llm-sdk"
        },
        {
          name: "sdkwork-llm.app",
          prefix: "/app/v3/api",
          authorityOpenApi: "sdks/sdkwork-llm-app-sdk/openapi/llm-app-api.openapi.json",
          sdkFamily: "sdkwork-llm-app-sdk"
        },
        {
          name: "sdkwork-llm.backend",
          prefix: "/backend/v3/api",
          authorityOpenApi: "sdks/sdkwork-llm-backend-sdk/openapi/llm-backend-api.openapi.json",
          sdkFamily: "sdkwork-llm-backend-sdk"
        }
      ],
      sdkClients: [
        "SdkworkLlmOpenClient",
        "SdkworkLlmAppClient",
        "SdkworkLlmBackendClient"
      ],
      sdkDependencies: [
        ...platformWorkspaceDependencies,
        ...appSdkDependencies,
        ...backendSdkDependencies
      ],
      dependencyApiExports: [],
      dependencyApiSurfaces: [],
      events: [
        "llm.space.created",
        "llm.event.appended",
        "llm.record.created",
        "llm.record.updated",
        "llm.record.deleted",
        "llm.record.superseded",
        "llm.candidate.created",
        "llm.candidate.approved",
        "llm.candidate.rejected",
        "llm.habit.promoted",
        "llm.habit.decayed",
        "memory.index.rebuild_requested",
        "memory.index.rebuild_completed",
        "memory.context_pack.created",
        "memory.retention.deleted",
        "memory.provider.health_changed"
      ],
      configKeys: []
    },
    integration: {
      authority: "Root SDKWork specs remain authoritative. Local specs may extend but must not contradict them.",
      memoryPolicy: "Canonical llm_record and llm_event tables are the source of truth; all indexes are derived and rebuildable.",
      implementationPolicy: "Memory implementation profiles select native_sql, event_sourced, graph_temporal, search_first, local_embedded, external_provider_bridge, or hybrid_platform without changing app/backend API contracts.",
      embeddingPolicy: "Embedding is an optional retriever/index provider, not a required storage dependency.",
      sdkPolicy: "Generated SDK clients are consumed through generated SDKs or approved composed wrappers; no raw HTTP fallback."
    },
    verification: {
      commands: [
        "node tools/materialize_phase1_contracts.mjs",
        "powershell -ExecutionPolicy Bypass -File tools/verify_phase1.ps1"
      ]
    },
    metadata: {
      standardVersion,
      status: "draft",
      design: "docs/superpowers/specs/2026-06-10-ai-llm-architecture-design.md",
      managedBy: "tools/materialize_phase1_contracts.mjs"
    }
  });
}

const sdkSurfaceProfiles = {
  open: {
    family: "sdkwork-llm-sdk",
    packageName: "@sdkwork/llm-sdk",
    schemaUrl: llmOpenApiSchemaUrl,
    sdkTarget: "open-api",
    componentSurface: "open-api"
  },
  app: {
    family: "sdkwork-llm-app-sdk",
    packageName: "@sdkwork/llm-app-sdk",
    schemaUrl: "/app/v3/openapi.json",
    sdkTarget: "app",
    componentSurface: "app"
  },
  backend: {
    family: "sdkwork-llm-backend-sdk",
    packageName: "@sdkwork/llm-backend-sdk",
    schemaUrl: "/backend/v3/openapi.json",
    sdkTarget: "backend",
    componentSurface: "backend-admin"
  }
};

function sdkSurfaceProfile(surface) {
  const profile = sdkSurfaceProfiles[surface];
  if (!profile) {
    throw new Error(`Unsupported SDK surface: ${surface}`);
  }
  return profile;
}

function sdkFamilyAssembly({ surface, prefix, title, authority, openapiFile, client, dependencies }) {
  const profile = sdkSurfaceProfile(surface);
  const family = profile.family;
  const languageWorkspace = `${family}-typescript`;
  return {
    workspace: family,
    title,
    apiVersion: version,
    openapiVersion: "3.1.2",
    authoritySpec: `openapi/${openapiFile}`,
    generationInputSpec: `openapi/${openapiFile}`,
    derivedSpecs: {
      default: `openapi/${openapiFile}`
    },
    apiAuthority: authority,
    discoverySurface: {
      sdkTarget: profile.sdkTarget,
      apiPrefix: prefix,
      schemaUrl: profile.schemaUrl,
      generatedProtocols: ["http-openapi"],
      manualTransports: []
    },
    languages: [
      {
        language: "typescript",
        workspace: languageWorkspace,
        generationState: "declared",
        releaseState: "not_published",
        generatedPath: `${languageWorkspace}/generated/server-openapi`,
        manifestPath: `${languageWorkspace}/generated/server-openapi/package.json`,
        name: profile.packageName,
        version,
        description: `Generator-owned TypeScript transport SDK for ${authority}.`,
        consumerSurface: {
          primaryClient: client,
          apiPrefix: prefix
        }
      }
    ],
    sdkOwner: owner,
    sdkDependencies: dependencies,
    dependencyApiExports: [],
    dependencyApiSurfaces: [],
    metadata: {
      standardVersion,
      ownerOnlyOperationCount: null,
      materializationState: "authority-draft",
      managedBy: "tools/materialize_phase1_contracts.mjs"
    }
  };
}

function sdkComponentSpec({ surface, prefix, title, authority, openapiFile, client, dependencies }) {
  const profile = sdkSurfaceProfile(surface);
  const family = profile.family;
  return {
    schemaVersion: 1,
    kind: "sdkwork.component.spec",
    component: {
      name: family,
      displayName: title,
      version,
      type: "sdk-family",
      root: `sdkwork-llm/sdks/${family}`,
      domain,
      capability,
      surface: profile.componentSurface,
      status: "draft",
      languages: ["typescript"],
      generated: true,
      private: false,
      manifests: [
        ".sdkwork-assembly.json",
        "sdk-manifest.json"
      ]
    },
    canonicalSpecs: sdkCanonicalSpecs,
    contracts: {
      apiAuthority: {
        name: authority,
        prefix,
        authorityOpenApi: `openapi/${openapiFile}`,
        derivedOpenApi: [`openapi/${openapiFile}`],
        owner,
        standard: "../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md"
      },
      publicExports: [],
      runtimeEntrypoints: [
        ".sdkwork-assembly.json"
      ],
      sdkDependencies: dependencies,
      dependencyApiExports: [],
      dependencyApiSurfaces: [],
      sdkClients: [client],
      events: [],
      configKeys: [
        ".sdkwork-assembly.json",
        "sdk-manifest.json"
      ]
    },
    integration: {
      authority: "Root SDKWork specs remain authoritative. Local specs may extend but must not contradict them.",
      dependencyPolicy: "Dependency capabilities are consumed through declared SDK dependencies and are not copied into generated Memory transports.",
      sdkPolicy: "Generated SDK clients are injected through service/runtime boundaries; consumers must not create raw HTTP clients or manual auth headers.",
      languagePolicy: "TypeScript is the first declared generated package. Additional languages must use the same owner-only OpenAPI input and sdkDependencies."
    },
    verification: {
      commands: [
        "powershell -ExecutionPolicy Bypass -File tools/verify_phase1.ps1"
      ]
    },
    metadata: {
      managedBy: "tools/materialize_phase1_contracts.mjs",
      standardVersion
    }
  };
}

function sdkManifest({ surface, prefix, authority, openapiFile, dependencies }) {
  const profile = sdkSurfaceProfile(surface);
  const family = profile.family;
  return {
    schemaVersion: 1,
    sdkName: family,
    packageName: profile.packageName,
    sdkOwner: owner,
    apiAuthority: authority,
    sdkFamily: family,
    sdkType: surface,
    sdkSurface: surface,
    language: "typescript",
    apiPrefix: prefix,
    generationInputSpec: `openapi/${openapiFile}`,
    generatedOutput: `${family}-typescript/generated/server-openapi`,
    standardProfile: "sdkwork-v3",
    sdkDependencies: dependencies,
    dependencyApiExports: [],
    dependencyApiSurfaces: [],
    ownerOnlyOperationCount: null,
    standardVersion,
    managedBy: "tools/materialize_phase1_contracts.mjs"
  };
}

function writeSdkgenConfig({ surface, prefix, authority, openapiFile }) {
  const profile = sdkSurfaceProfile(surface);
  const family = profile.family;
  const sdkgenFile = openapiFile.replace(".openapi.json", ".sdkgen.yaml");
  const apiSurface =
    surface === "open" ? "open-api" : surface === "app" ? "app-api" : "backend-api";
  writeText(
    `sdks/${family}/openapi/${sdkgenFile}`,
    `schemaVersion: 1
kind: sdkwork.sdkgen.config
input: ${openapiFile}
output: ../${family}-typescript/generated/server-openapi
sdkOwner: ${owner}
apiAuthority: ${authority}
sdkFamily: ${family}
standardProfile: sdkwork-v3
languageTargets:
  - typescript
ownerOnly: true
domain: ${domain}
capability: ${capability}
prefix: ${prefix}
surface: ${apiSurface}
`
  );
}

function writeSdkFamily({ surface, prefix, title, authority, openapiFile, client, dependencies }) {
  const profile = sdkSurfaceProfile(surface);
  const family = profile.family;
  writeText(`sdks/${family}/README.md`, `# ${title}

This is the SDK family root for the \`${authority}\` OpenAPI authority.

- SDK family: \`${family}\`
- API authority: \`${authority}\`
- API prefix: \`${prefix}\`
- Owner: \`${owner}\`
- Standard profile: \`sdkwork-v3\`
- Authority OpenAPI: \`openapi/${openapiFile}\`
- Declared generated client: \`${client}\`

Generated transport output, when materialized, belongs under:

- \`${family}-typescript/generated/server-openapi\`

Generated files must not be hand-edited. Fix OpenAPI, route manifests, generator input, or approved composed facades, then regenerate with the canonical SDKWork generator.

Credential mode:

- Open API SDKs use an API key credential provider for protected operations.
- App and backend SDKs use SDKWork dual-token credential injection.
`);
  writeText(`sdks/${family}/specs/README.md`, `# ${title} Component Specs

This directory is the local component contract entrypoint for \`${family}\`.

Root standards remain authoritative:

- \`../sdkwork-specs/API_SPEC.md\`
- \`../sdkwork-specs/SDK_SPEC.md\`
- \`../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md\`
- \`../sdkwork-specs/COMPONENT_SPEC.md\`

Local authority:

- \`../.sdkwork-assembly.json\`
- \`../sdk-manifest.json\`
- \`../openapi/${openapiFile}\`
`);
  writeJson(`sdks/${family}/.sdkwork-assembly.json`, sdkFamilyAssembly({ surface, prefix, title, authority, openapiFile, client, dependencies }));
  writeJson(`sdks/${family}/specs/component.spec.json`, sdkComponentSpec({ surface, prefix, title, authority, openapiFile, client, dependencies }));
  writeJson(`sdks/${family}/sdk-manifest.json`, sdkManifest({ surface, prefix, authority, openapiFile, dependencies }));
  writeSdkgenConfig({ surface, prefix, authority, openapiFile });
}

function writeSdkFamilies() {
  writeText("sdks/README.md", `# SDKWork LLM SDK Workspace

This directory owns SDKWork LLM SDK families and authority OpenAPI documents.

SDK families:

- \`sdkwork-llm-sdk\` for \`sdkwork-llm-open-api\` and \`${llmOpenApiPrefix}\`
- \`sdkwork-llm-app-sdk\` for \`sdkwork-llm.app\` and \`/app/v3/api\`
- \`sdkwork-llm-backend-sdk\` for \`sdkwork-llm.backend\` and \`/backend/v3/api\`

Protected Open API clients use \`X-API-Key\` through generated SDK credential providers. They must not join app/backend token-manager client lists.

RPC SDK families are deferred until high-throughput backend/native RPC integration is needed.
`);
  writeSdkFamily({
    surface: "open",
    prefix: llmOpenApiPrefix,
    title: "SDKWork LLM Open API SDK",
    authority: "sdkwork-llm-open-api",
    openapiFile: "llm-open-api.openapi.json",
    client: "SdkworkLlmOpenClient",
    dependencies: []
  });
  writeSdkFamily({
    surface: "app",
    prefix: "/app/v3/api",
    title: "SDKWork LLM App API SDK",
    authority: "sdkwork-llm.app",
    openapiFile: "llm-app-api.openapi.json",
    client: "SdkworkLlmAppClient",
    dependencies: appSdkDependencies
  });
  writeSdkFamily({
    surface: "backend",
    prefix: "/backend/v3/api",
    title: "SDKWork LLM Backend API SDK",
    authority: "sdkwork-llm.backend",
    openapiFile: "llm-backend-api.openapi.json",
    client: "SdkworkLlmBackendClient",
    dependencies: backendSdkDependencies
  });
}

function writeSchemaRegistry() {
  writeText("docs/schema-registry/README.md", `# SDKWork LLM Schema Registry

Memory database contracts are defined here before migrations or ORM entities are created.

Rules:

- Physical table names use the \`llm_\` prefix.
- \`llm_record\` and \`llm_event\` are canonical source-of-truth tables.
- Index, retrieval, vector, graph, grep/file, and provider states are derived or governed tables and must be rebuildable from canonical records and events when possible.
- PostgreSQL is the production/server target; SQLite is allowed for local/private/test parity where feasible.
- 64-bit identifiers are serialized as strings in API/SDK contracts.
`);

  writeText("docs/schema-registry/tables/001-llm-core.yaml", `module: memory
owner: sdkwork-llm
domain: intelligence
bounded_context: memory-core
description: Canonical memory spaces, events, records, sources, entities, and edges. These tables are the durable source of truth for embedding-optional memory.
tables:
  - table: llm_space
    domain: intelligence
    profile: tenant_entity
    compliance_level: L3
    system_of_record: true
    description: Tenant-scoped memory namespace owned by a user, organization, app, project, agent, or external subject.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: organization_id, type: bigint, nullable: true }
      - { name: owner_subject_type, type: varchar(32), constraints: ["enum: [user, organization, app, project, agent, session, external]"] }
      - { name: owner_subject_id, type: varchar(128), constraints: [required] }
      - { name: space_type, type: varchar(32), constraints: ["enum: [personal, agent, team, app, project, session, imported, external_shadow]"] }
      - { name: display_name, type: varchar(200), constraints: [required] }
      - { name: default_scope, type: varchar(32), constraints: ["enum: [user, organization, app, agent, session, global]"] }
      - { name: lifecycle_status, type: varchar(32), constraints: ["enum: [active, archived, deleted]"] }
      - { name: metadata_json, type: json, nullable: true }
      - { name: policy_json, type: json, nullable: true }
      - { name: created_by, type: bigint, nullable: true }
      - { name: updated_by, type: bigint, nullable: true }
      - { name: created_at, type: timestamp, constraints: [required] }
      - { name: updated_at, type: timestamp, constraints: [required] }
      - { name: deleted_at, type: timestamp, nullable: true }
      - { name: version, type: bigint, constraints: [required, "min: 0"] }
    indexes:
      - { name: uk_llm_space_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: uk_llm_space_owner_type, unique: true, columns: [tenant_id, owner_subject_type, owner_subject_id, space_type] }
      - { name: idx_llm_space_tenant_status, columns: [tenant_id, lifecycle_status, updated_at] }
  - table: llm_event
    domain: intelligence
    profile: event_log
    compliance_level: L3
    system_of_record: true
    description: Append-first memory evidence event, including conversation facts, tool events, feedback, external imports, and deletion requests.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: space_id, type: bigint, constraints: [required], references: llm_space.id }
      - { name: user_id, type: bigint, nullable: true }
      - { name: actor_type, type: varchar(32), constraints: ["enum: [user, agent, backend, system, import, external_provider]"] }
      - { name: actor_id, type: varchar(128), nullable: true }
      - { name: session_id, type: varchar(128), nullable: true }
      - { name: trace_id, type: varchar(128), nullable: true }
      - { name: request_id, type: varchar(64), nullable: true }
      - { name: idempotency_key, type: varchar(128), nullable: true }
      - { name: event_type, type: varchar(64), constraints: [required] }
      - { name: source_type, type: varchar(64), constraints: ["enum: [conversation, tool, feedback, import, api, file, system, external_provider]"] }
      - { name: source_ref, type: varchar(256), nullable: true }
      - { name: event_time, type: timestamp, constraints: [required] }
      - { name: payload_json, type: json, constraints: [required] }
      - { name: payload_hash, type: varchar(128), constraints: [required] }
      - { name: sensitivity_level, type: varchar(32), constraints: ["enum: [public, internal, private, sensitive, restricted]"] }
      - { name: ingestion_status, type: varchar(32), constraints: ["enum: [received, processed, rejected, redacted, deleted]"] }
      - { name: created_at, type: timestamp, constraints: [required] }
    indexes:
      - { name: uk_llm_event_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: uk_llm_event_idempotency, unique: true, columns: [tenant_id, idempotency_key], predicate: "idempotency_key IS NOT NULL" }
      - { name: idx_llm_event_space_time, columns: [tenant_id, space_id, event_time, id] }
      - { name: idx_llm_event_session_time, columns: [tenant_id, session_id, event_time] }
      - { name: idx_llm_event_type_time, columns: [tenant_id, event_type, event_time] }
      - { name: idx_llm_event_hash, columns: [tenant_id, payload_hash] }
  - table: llm_record
    domain: intelligence
    profile: core_entity
    compliance_level: L3
    system_of_record: true
    description: Canonical durable memory fact, preference, procedure, habit reference, relationship, or episode. All retrieval indexes derive from this table.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: space_id, type: bigint, constraints: [required], references: llm_space.id }
      - { name: user_id, type: bigint, nullable: true }
      - { name: scope, type: varchar(32), constraints: ["enum: [user, organization, app, agent, session, global]"] }
      - { name: record_type, type: varchar(32), constraints: ["enum: [working, session, semantic, episodic, procedural, habit, relationship, domain_knowledge]"] }
      - { name: subject, type: varchar(256), nullable: true }
      - { name: predicate, type: varchar(128), nullable: true }
      - { name: object_text, type: text, constraints: [required] }
      - { name: canonical_text, type: text, constraints: [required] }
      - { name: summary_text, type: text, nullable: true }
      - { name: language, type: varchar(16), nullable: true }
      - { name: confidence, type: decimal(5,4), constraints: ["range: 0..1"] }
      - { name: evidence_count, type: int32, constraints: [required, "min: 0"] }
      - { name: contradiction_count, type: int32, constraints: [required, "min: 0"] }
      - { name: importance_score, type: decimal(5,4), constraints: ["range: 0..1"] }
      - { name: recency_score, type: decimal(5,4), constraints: ["range: 0..1"] }
      - { name: habit_strength, type: decimal(5,4), nullable: true }
      - { name: valid_from, type: timestamp, nullable: true }
      - { name: valid_to, type: timestamp, nullable: true }
      - { name: expires_at, type: timestamp, nullable: true }
      - { name: status, type: varchar(32), constraints: ["enum: [candidate, active, inactive, superseded, deleted, rejected]"] }
      - { name: sensitivity_level, type: varchar(32), constraints: ["enum: [public, internal, private, sensitive, restricted]"] }
      - { name: metadata_json, type: json, nullable: true }
      - { name: tags_json, type: json, nullable: true }
      - { name: supersedes_record_id, type: bigint, nullable: true, references: llm_record.id }
      - { name: superseded_by_record_id, type: bigint, nullable: true, references: llm_record.id }
      - { name: created_by, type: bigint, nullable: true }
      - { name: updated_by, type: bigint, nullable: true }
      - { name: created_at, type: timestamp, constraints: [required] }
      - { name: updated_at, type: timestamp, constraints: [required] }
      - { name: deleted_at, type: timestamp, nullable: true }
      - { name: version, type: bigint, constraints: [required, "min: 0"] }
    indexes:
      - { name: uk_llm_record_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: idx_llm_record_scope_type_status, columns: [tenant_id, space_id, scope, record_type, status, updated_at] }
      - { name: idx_llm_record_user_type, columns: [tenant_id, user_id, record_type, status, updated_at] }
      - { name: idx_llm_record_subject_predicate, columns: [tenant_id, space_id, subject, predicate, status] }
      - { name: idx_llm_record_validity, columns: [tenant_id, valid_from, valid_to, expires_at] }
      - { name: idx_llm_record_supersession, columns: [tenant_id, supersedes_record_id, superseded_by_record_id] }
  - table: llm_record_source
    domain: intelligence
    profile: relation_entity
    compliance_level: L3
    system_of_record: true
    description: Evidence link from a memory record to one or more source events.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: record_id, type: bigint, constraints: [required], references: llm_record.id }
      - { name: event_id, type: bigint, constraints: [required], references: llm_event.id }
      - { name: source_role, type: varchar(32), constraints: ["enum: [supporting, contradicting, originating, deletion, correction]"] }
      - { name: confidence_delta, type: decimal(5,4), nullable: true }
      - { name: created_at, type: timestamp, constraints: [required] }
    indexes:
      - { name: uk_llm_record_source_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: uk_llm_record_source_pair, unique: true, columns: [tenant_id, record_id, event_id, source_role] }
      - { name: idx_llm_record_source_event, columns: [tenant_id, event_id] }
  - table: llm_entity
    domain: intelligence
    profile: dictionary_entity
    compliance_level: L2
    system_of_record: true
    description: Entity dictionary used by graph, dictionary, and deterministic retrieval without requiring embeddings.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: space_id, type: bigint, constraints: [required], references: llm_space.id }
      - { name: entity_type, type: varchar(64), constraints: [required] }
      - { name: canonical_name, type: varchar(256), constraints: [required] }
      - { name: aliases_json, type: json, nullable: true }
      - { name: attributes_json, type: json, nullable: true }
      - { name: status, type: varchar(32), constraints: ["enum: [active, merged, deleted]"] }
      - { name: created_at, type: timestamp, constraints: [required] }
      - { name: updated_at, type: timestamp, constraints: [required] }
      - { name: version, type: bigint, constraints: [required, "min: 0"] }
    indexes:
      - { name: uk_llm_entity_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: uk_llm_entity_name, unique: true, columns: [tenant_id, space_id, entity_type, canonical_name] }
      - { name: idx_llm_entity_type_status, columns: [tenant_id, space_id, entity_type, status] }
  - table: llm_edge
    domain: intelligence
    profile: relation_entity
    compliance_level: L2
    system_of_record: true
    description: Relationship edge between entities or memories for graph-temporal retrieval.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: space_id, type: bigint, constraints: [required], references: llm_space.id }
      - { name: source_entity_id, type: bigint, constraints: [required], references: llm_entity.id }
      - { name: target_entity_id, type: bigint, constraints: [required], references: llm_entity.id }
      - { name: relation_type, type: varchar(64), constraints: [required] }
      - { name: weight, type: decimal(8,4), nullable: true }
      - { name: source_record_id, type: bigint, nullable: true, references: llm_record.id }
      - { name: valid_from, type: timestamp, nullable: true }
      - { name: valid_to, type: timestamp, nullable: true }
      - { name: status, type: varchar(32), constraints: ["enum: [active, inactive, deleted]"] }
      - { name: metadata_json, type: json, nullable: true }
      - { name: created_at, type: timestamp, constraints: [required] }
      - { name: updated_at, type: timestamp, constraints: [required] }
      - { name: version, type: bigint, constraints: [required, "min: 0"] }
    indexes:
      - { name: uk_llm_edge_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: idx_llm_edge_source, columns: [tenant_id, space_id, source_entity_id, relation_type, status] }
      - { name: idx_llm_edge_target, columns: [tenant_id, space_id, target_entity_id, relation_type, status] }
      - { name: idx_llm_edge_validity, columns: [tenant_id, valid_from, valid_to] }
serialization:
  int64: string
  decimal: string
  time: iso8601_utc
`);

  writeText("docs/schema-registry/tables/002-llm-learning.yaml", `module: memory
owner: sdkwork-llm
domain: intelligence
bounded_context: memory-learning
description: Self-learning candidates, habit lifecycle, evidence signals, and learning jobs.
tables:
  - table: llm_candidate
    profile: core_entity
    compliance_level: L3
    system_of_record: true
    description: Reviewable memory candidate created by deterministic rules, LLM extraction, feedback, consolidation, or external provider import.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: space_id, type: bigint, constraints: [required], references: llm_space.id }
      - { name: user_id, type: bigint, nullable: true }
      - { name: candidate_type, type: varchar(32), constraints: ["enum: [create, update, delete, supersede, promote_habit, decay_habit]"] }
      - { name: record_type, type: varchar(32), constraints: ["enum: [semantic, episodic, procedural, habit, relationship, domain_knowledge]"] }
      - { name: proposed_text, type: text, constraints: [required] }
      - { name: proposed_payload_json, type: json, nullable: true }
      - { name: target_record_id, type: bigint, nullable: true, references: llm_record.id }
      - { name: evidence_json, type: json, nullable: true }
      - { name: confidence, type: decimal(5,4), constraints: ["range: 0..1"] }
      - { name: novelty_score, type: decimal(5,4), nullable: true }
      - { name: risk_score, type: decimal(5,4), nullable: true }
      - { name: decision_state, type: varchar(32), constraints: ["enum: [pending, auto_approved, approved, rejected, expired, superseded]"] }
      - { name: decision_reason, type: varchar(256), nullable: true }
      - { name: decided_by, type: bigint, nullable: true }
      - { name: decided_at, type: timestamp, nullable: true }
      - { name: created_at, type: timestamp, constraints: [required] }
      - { name: updated_at, type: timestamp, constraints: [required] }
      - { name: version, type: bigint, constraints: [required, "min: 0"] }
    indexes:
      - { name: uk_llm_candidate_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: idx_llm_candidate_state, columns: [tenant_id, space_id, decision_state, updated_at] }
      - { name: idx_llm_candidate_target, columns: [tenant_id, target_record_id] }
  - table: llm_habit
    profile: core_entity
    compliance_level: L3
    system_of_record: true
    description: Habit-forming memory state promoted from repeated behavior signals and optional user confirmations.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: space_id, type: bigint, constraints: [required], references: llm_space.id }
      - { name: user_id, type: bigint, constraints: [required] }
      - { name: habit_key, type: varchar(160), constraints: [required] }
      - { name: habit_type, type: varchar(64), constraints: [required] }
      - { name: description, type: text, constraints: [required] }
      - { name: stage, type: varchar(32), constraints: ["enum: [observing, emerging, confirmed, decaying, inactive, rejected]"] }
      - { name: strength, type: decimal(5,4), constraints: ["range: 0..1"] }
      - { name: confidence, type: decimal(5,4), constraints: ["range: 0..1"] }
      - { name: support_count, type: int32, constraints: [required, "min: 0"] }
      - { name: last_signal_at, type: timestamp, nullable: true }
      - { name: promoted_record_id, type: bigint, nullable: true, references: llm_record.id }
      - { name: decay_after, type: timestamp, nullable: true }
      - { name: metadata_json, type: json, nullable: true }
      - { name: created_at, type: timestamp, constraints: [required] }
      - { name: updated_at, type: timestamp, constraints: [required] }
      - { name: version, type: bigint, constraints: [required, "min: 0"] }
    indexes:
      - { name: uk_llm_habit_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: uk_llm_habit_key, unique: true, columns: [tenant_id, space_id, user_id, habit_key] }
      - { name: idx_llm_habit_stage, columns: [tenant_id, space_id, stage, confidence, updated_at] }
  - table: llm_habit_signal
    profile: event_log
    compliance_level: L2
    system_of_record: true
    description: Evidence signal attached to habit scoring and promotion/decay decisions.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: habit_id, type: bigint, constraints: [required], references: llm_habit.id }
      - { name: event_id, type: bigint, nullable: true, references: llm_event.id }
      - { name: signal_type, type: varchar(64), constraints: [required] }
      - { name: signal_strength, type: decimal(5,4), constraints: ["range: 0..1"] }
      - { name: observed_at, type: timestamp, constraints: [required] }
      - { name: payload_json, type: json, nullable: true }
      - { name: created_at, type: timestamp, constraints: [required] }
    indexes:
      - { name: uk_llm_habit_signal_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: idx_llm_habit_signal_habit, columns: [tenant_id, habit_id, observed_at] }
      - { name: idx_llm_habit_signal_event, columns: [tenant_id, event_id] }
  - table: llm_learning_job
    profile: event_log
    compliance_level: L2
    system_of_record: true
    description: Asynchronous extraction, consolidation, habit scoring, and review materialization job.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: space_id, type: bigint, nullable: true, references: llm_space.id }
      - { name: job_type, type: varchar(64), constraints: ["enum: [extract, consolidate, habit_score, index_sync, retention, migration]"] }
      - { name: state, type: varchar(32), constraints: ["enum: [queued, running, succeeded, failed, cancelled]"] }
      - { name: priority, type: int32, constraints: [required] }
      - { name: idempotency_key, type: varchar(128), nullable: true }
      - { name: input_json, type: json, nullable: true }
      - { name: result_json, type: json, nullable: true }
      - { name: error_json, type: json, nullable: true }
      - { name: started_at, type: timestamp, nullable: true }
      - { name: finished_at, type: timestamp, nullable: true }
      - { name: created_at, type: timestamp, constraints: [required] }
      - { name: updated_at, type: timestamp, constraints: [required] }
      - { name: version, type: bigint, constraints: [required, "min: 0"] }
    indexes:
      - { name: uk_llm_learning_job_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: uk_llm_learning_job_idempotency, unique: true, columns: [tenant_id, job_type, idempotency_key], predicate: "idempotency_key IS NOT NULL" }
      - { name: idx_llm_learning_job_state, columns: [tenant_id, job_type, state, priority, created_at] }
serialization: { int64: string, decimal: string, time: iso8601_utc }
`);

  writeText("docs/schema-registry/tables/003-llm-retrieval.yaml", `module: memory
owner: sdkwork-llm
domain: intelligence
bounded_context: memory-retrieval
description: Derived indexes, retrieval profiles, traces, hits, and context packs. Embedding/vector state is optional and represented as one index kind among many.
tables:
  - table: llm_index
    profile: projection
    compliance_level: L2
    system_of_record: false
    description: Provider-neutral index definition for sql, keyword, dictionary, time, event, vector, graph, grep_file, or custom retrievers.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: space_id, type: bigint, nullable: true, references: llm_space.id }
      - { name: index_kind, type: varchar(32), constraints: ["enum: [sql, keyword, dictionary, time, event, vector, graph, grep_file, custom]"] }
      - { name: implementation_profile_id, type: bigint, nullable: true }
      - { name: provider_binding_id, type: bigint, nullable: true }
      - { name: schema_version, type: varchar(32), constraints: [required] }
      - { name: status, type: varchar(32), constraints: ["enum: [active, rebuilding, degraded, disabled, deleted]"] }
      - { name: rebuild_cursor, type: varchar(256), nullable: true }
      - { name: config_json, type: json, nullable: true }
      - { name: last_rebuilt_at, type: timestamp, nullable: true }
      - { name: created_at, type: timestamp, constraints: [required] }
      - { name: updated_at, type: timestamp, constraints: [required] }
      - { name: version, type: bigint, constraints: [required, "min: 0"] }
    indexes:
      - { name: uk_llm_index_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: uk_llm_index_kind_space, unique: true, columns: [tenant_id, space_id, index_kind, schema_version] }
      - { name: idx_llm_index_status, columns: [tenant_id, space_id, index_kind, status] }
  - table: llm_index_entry
    profile: projection
    compliance_level: L2
    system_of_record: false
    description: Provider-neutral pointer to derived index state; vector payloads may be externalized while canonical memory stays in llm_record.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: index_id, type: bigint, constraints: [required], references: llm_index.id }
      - { name: record_id, type: bigint, nullable: true, references: llm_record.id }
      - { name: event_id, type: bigint, nullable: true, references: llm_event.id }
      - { name: entry_kind, type: varchar(32), constraints: ["enum: [memory, event, entity, edge, file_line, external_ref]"] }
      - { name: entry_hash, type: varchar(128), constraints: [required] }
      - { name: external_ref, type: varchar(512), nullable: true }
      - { name: payload_json, type: json, nullable: true }
      - { name: indexed_at, type: timestamp, constraints: [required] }
    indexes:
      - { name: uk_llm_index_entry_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: uk_llm_index_entry_memory, unique: true, columns: [tenant_id, index_id, record_id, entry_kind], predicate: "record_id IS NOT NULL" }
      - { name: idx_llm_index_entry_hash, columns: [tenant_id, index_id, entry_hash] }
  - table: llm_retrieval_profile
    profile: dictionary_entity
    compliance_level: L2
    system_of_record: true
    description: Runtime retrieval policy describing retriever selection, fusion, rerank, context budget, and degraded-mode behavior.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: space_id, type: bigint, nullable: true, references: llm_space.id }
      - { name: name, type: varchar(160), constraints: [required] }
      - { name: strategy, type: varchar(64), constraints: ["enum: [deterministic, semantic, graph, file, hybrid, custom]"] }
      - { name: retrievers_json, type: json, constraints: [required] }
      - { name: fusion_policy_json, type: json, nullable: true }
      - { name: rerank_policy_json, type: json, nullable: true }
      - { name: top_k, type: int32, constraints: [required] }
      - { name: context_budget_tokens, type: int32, constraints: [required] }
      - { name: status, type: varchar(32), constraints: ["enum: [active, disabled, deleted]"] }
      - { name: created_at, type: timestamp, constraints: [required] }
      - { name: updated_at, type: timestamp, constraints: [required] }
      - { name: version, type: bigint, constraints: [required, "min: 0"] }
    indexes:
      - { name: uk_llm_retrieval_profile_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: idx_llm_retrieval_profile_scope, columns: [tenant_id, space_id, status, updated_at] }
  - table: llm_retrieval_trace
    profile: event_log
    compliance_level: L2
    system_of_record: true
    description: Trace for each retrieval orchestration, including selected retrievers, degraded mode, and scoring metadata.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: space_id, type: bigint, nullable: true, references: llm_space.id }
      - { name: retrieval_profile_id, type: bigint, nullable: true, references: llm_retrieval_profile.id }
      - { name: actor_id, type: varchar(128), nullable: true }
      - { name: query_text, type: text, nullable: true }
      - { name: query_hash, type: varchar(128), constraints: [required] }
      - { name: retrievers_json, type: json, nullable: true }
      - { name: latency_ms, type: int32, nullable: true }
      - { name: result_count, type: int32, constraints: [required] }
      - { name: degraded, type: boolean, constraints: [required] }
      - { name: metadata_json, type: json, nullable: true }
      - { name: created_at, type: timestamp, constraints: [required] }
    indexes:
      - { name: uk_llm_retrieval_trace_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: idx_llm_retrieval_trace_profile_created, columns: [tenant_id, retrieval_profile_id, created_at] }
      - { name: idx_llm_retrieval_trace_actor_created, columns: [tenant_id, actor_id, created_at] }
  - table: llm_retrieval_hit
    profile: projection
    compliance_level: L2
    system_of_record: true
    description: Ranked hit materialized from a retrieval trace for explainability, evaluation, and feedback.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: retrieval_trace_id, type: bigint, constraints: [required], references: llm_retrieval_trace.id }
      - { name: record_id, type: bigint, nullable: true, references: llm_record.id }
      - { name: retriever_name, type: varchar(64), constraints: [required] }
      - { name: result_rank, type: int32, constraints: [required] }
      - { name: raw_score, type: decimal(10,6), nullable: true }
      - { name: fused_score, type: decimal(10,6), nullable: true }
      - { name: explanation_json, type: json, nullable: true }
      - { name: status, type: varchar(32), constraints: ["enum: [included, filtered, suppressed]"] }
      - { name: created_at, type: timestamp, constraints: [required] }
    indexes:
      - { name: uk_llm_retrieval_hit_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: idx_llm_retrieval_hit_trace_rank, columns: [tenant_id, retrieval_trace_id, result_rank] }
      - { name: idx_llm_retrieval_hit_memory, columns: [tenant_id, record_id, status] }
  - table: llm_context_pack
    profile: snapshot
    compliance_level: L2
    system_of_record: true
    description: LLM-ready context assembled from one retrieval trace with citations and token budget metadata.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: retrieval_trace_id, type: bigint, nullable: true, references: llm_retrieval_trace.id }
      - { name: actor_id, type: varchar(128), nullable: true }
      - { name: query_text, type: text, nullable: true }
      - { name: pack_json, type: json, constraints: [required] }
      - { name: estimated_tokens, type: int32, constraints: [required] }
      - { name: truncated, type: boolean, constraints: [required] }
      - { name: created_at, type: timestamp, constraints: [required] }
    indexes:
      - { name: uk_llm_context_pack_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: idx_llm_context_pack_trace, columns: [tenant_id, retrieval_trace_id] }
      - { name: idx_llm_context_pack_actor_created, columns: [tenant_id, actor_id, created_at] }
serialization: { int64: string, decimal: string, time: iso8601_utc }
`);

  writeText("docs/schema-registry/tables/004-llm-provider.yaml", `module: memory
owner: sdkwork-llm
domain: intelligence
bounded_context: memory-provider-policy
description: Implementation profiles, provider bindings, and policy definitions for provider-switchable memory.
tables:
  - table: llm_implementation_profile
    profile: dictionary_entity
    compliance_level: L3
    system_of_record: true
    description: Selects the concrete Memory runtime implementation behind stable app/backend APIs.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: name, type: varchar(160), constraints: [required] }
      - { name: implementation_kind, type: varchar(64), constraints: ["enum: [native_sql, event_sourced, graph_temporal, search_first, local_embedded, external_provider_bridge, hybrid_platform]"] }
      - { name: role, type: varchar(32), constraints: ["enum: [primary, shadow, migration_source, migration_target, eval_only]"] }
      - { name: status, type: varchar(32), constraints: ["enum: [draft, active, paused, deprecated, deleted]"] }
      - { name: capability_json, type: json, constraints: [required] }
      - { name: config_json, type: json, nullable: true }
      - { name: rollout_json, type: json, nullable: true }
      - { name: created_at, type: timestamp, constraints: [required] }
      - { name: updated_at, type: timestamp, constraints: [required] }
      - { name: version, type: bigint, constraints: [required, "min: 0"] }
    indexes:
      - { name: uk_llm_implementation_profile_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: idx_llm_implementation_profile_kind, columns: [tenant_id, implementation_kind, status] }
  - table: llm_provider_binding
    profile: dictionary_entity
    compliance_level: L3
    system_of_record: true
    description: Abstract binding for LLM, embedding, rerank, graph, search, file, and external memory providers. Secrets are referenced, never stored here.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: provider_kind, type: varchar(32), constraints: ["enum: [llm, embedding, rerank, vector, graph, search, file, memory, external]"] }
      - { name: provider_code, type: varchar(128), constraints: [required] }
      - { name: display_name, type: varchar(160), constraints: [required] }
      - { name: endpoint_ref, type: varchar(256), nullable: true }
      - { name: secret_ref, type: varchar(256), nullable: true }
      - { name: model_ref, type: varchar(256), nullable: true }
      - { name: capabilities_json, type: json, constraints: [required] }
      - { name: config_json, type: json, nullable: true }
      - { name: health_state, type: varchar(32), constraints: ["enum: [unknown, healthy, degraded, unhealthy, disabled]"] }
      - { name: last_health_at, type: timestamp, nullable: true }
      - { name: created_at, type: timestamp, constraints: [required] }
      - { name: updated_at, type: timestamp, constraints: [required] }
      - { name: version, type: bigint, constraints: [required, "min: 0"] }
    indexes:
      - { name: uk_llm_provider_binding_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: uk_llm_provider_binding_code, unique: true, columns: [tenant_id, provider_kind, provider_code] }
      - { name: idx_llm_provider_binding_health, columns: [tenant_id, provider_kind, health_state, updated_at] }
  - table: llm_policy
    profile: dictionary_entity
    compliance_level: L3
    system_of_record: true
    description: Retention, privacy, review, learning, retrieval, and provider governance policy.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: policy_type, type: varchar(64), constraints: ["enum: [retention, privacy, review, learning, retrieval, provider, export, deletion]"] }
      - { name: scope, type: varchar(32), constraints: ["enum: [tenant, organization, user, space, app, agent, global]"] }
      - { name: scope_ref, type: varchar(128), nullable: true }
      - { name: status, type: varchar(32), constraints: ["enum: [active, disabled, deleted]"] }
      - { name: policy_json, type: json, constraints: [required] }
      - { name: created_at, type: timestamp, constraints: [required] }
      - { name: updated_at, type: timestamp, constraints: [required] }
      - { name: version, type: bigint, constraints: [required, "min: 0"] }
    indexes:
      - { name: uk_llm_policy_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: idx_llm_policy_type_scope, columns: [tenant_id, policy_type, scope, status] }
serialization: { int64: string, decimal: string, time: iso8601_utc }
`);

  writeText("docs/schema-registry/tables/005-llm-governance.yaml", `module: memory
owner: sdkwork-llm
domain: intelligence
bounded_context: memory-governance
description: Audit, evaluation, and outbox contracts for governed memory operations.
tables:
  - table: llm_audit_log
    profile: audit_log
    compliance_level: L3
    system_of_record: true
    description: Immutable audit log for memory read, write, provider, retention, export, deletion, and admin operations.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: actor_type, type: varchar(32), constraints: [required] }
      - { name: actor_id, type: varchar(128), nullable: true }
      - { name: action, type: varchar(128), constraints: [required] }
      - { name: resource_type, type: varchar(64), constraints: [required] }
      - { name: resource_id, type: varchar(128), nullable: true }
      - { name: request_id, type: varchar(64), nullable: true }
      - { name: trace_id, type: varchar(128), nullable: true }
      - { name: result, type: varchar(32), constraints: ["enum: [success, denied, failed, partial]"] }
      - { name: reason, type: varchar(256), nullable: true }
      - { name: metadata_json, type: json, nullable: true }
      - { name: created_at, type: timestamp, constraints: [required] }
    indexes:
      - { name: uk_llm_audit_log_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: idx_llm_audit_actor_time, columns: [tenant_id, actor_type, actor_id, created_at] }
      - { name: idx_llm_audit_resource_time, columns: [tenant_id, resource_type, resource_id, created_at] }
      - { name: idx_llm_audit_action_time, columns: [tenant_id, action, created_at] }
  - table: llm_eval_run
    profile: event_log
    compliance_level: L2
    system_of_record: true
    description: Offline or online evaluation run for write quality, retrieval quality, habit quality, and provider switching.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: eval_type, type: varchar(64), constraints: ["enum: [write_quality, retrieval_quality, habit_quality, provider_switch, regression]"] }
      - { name: state, type: varchar(32), constraints: ["enum: [queued, running, succeeded, failed, cancelled]"] }
      - { name: dataset_ref, type: varchar(256), nullable: true }
      - { name: profile_ref, type: varchar(256), nullable: true }
      - { name: metrics_json, type: json, nullable: true }
      - { name: result_json, type: json, nullable: true }
      - { name: started_at, type: timestamp, nullable: true }
      - { name: finished_at, type: timestamp, nullable: true }
      - { name: created_at, type: timestamp, constraints: [required] }
      - { name: updated_at, type: timestamp, constraints: [required] }
    indexes:
      - { name: uk_llm_eval_run_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: idx_llm_eval_run_type_state, columns: [tenant_id, eval_type, state, created_at] }
  - table: llm_outbox_event
    profile: outbox_event
    compliance_level: L3
    system_of_record: true
    description: Transactional outbox for memory.* domain events.
    columns:
      - { name: id, type: bigint, constraints: [primary_key, snowflake] }
      - { name: uuid, type: varchar(64), constraints: [required, public_id] }
      - { name: tenant_id, type: bigint, constraints: [required] }
      - { name: aggregate_type, type: varchar(64), constraints: [required] }
      - { name: aggregate_id, type: varchar(128), constraints: [required] }
      - { name: event_type, type: varchar(128), constraints: [required] }
      - { name: event_version, type: varchar(32), constraints: [required] }
      - { name: payload_json, type: json, constraints: [required] }
      - { name: publish_state, type: varchar(32), constraints: ["enum: [pending, published, failed, skipped]"] }
      - { name: published_at, type: timestamp, nullable: true }
      - { name: retry_count, type: int32, constraints: [required, "min: 0"] }
      - { name: created_at, type: timestamp, constraints: [required] }
      - { name: updated_at, type: timestamp, constraints: [required] }
    indexes:
      - { name: uk_llm_outbox_event_uuid, unique: true, columns: [tenant_id, uuid] }
      - { name: idx_llm_outbox_state, columns: [tenant_id, publish_state, created_at] }
serialization: { int64: string, decimal: string, time: iso8601_utc }
`);
}

const idSchema = {
  type: "string",
  pattern: "^[0-9]+$",
  "x-sdkwork-int64-string": true
};
const nullableIdSchema = { anyOf: [idSchema, { type: "null" }] };
const nullableString = { anyOf: [{ type: "string" }, { type: "null" }] };
const jsonObject = { type: "object", additionalProperties: true };
const nullableJsonObject = { anyOf: [jsonObject, { type: "null" }] };
const instant = { type: "string", format: "date-time" };
const nullableInstant = { anyOf: [instant, { type: "null" }] };

function schemaRef(name) {
  return { $ref: `#/components/schemas/${name}` };
}

function pageSchema(itemSchemaName) {
  return {
    type: "object",
    required: ["items", "pageInfo"],
    properties: {
      items: { type: "array", items: schemaRef(itemSchemaName) },
      pageInfo: schemaRef("LlmPageInfo")
    }
  };
}

function errorResponses() {
  const problem = {
    description: "Problem detail",
    content: {
      "application/problem+json": {
        schema: schemaRef("ProblemDetails")
      }
    }
  };
  return {
    "400": problem,
    "401": problem,
    "403": problem,
    "404": problem,
    "409": problem
  };
}

function successResponse(status, schemaName, description = status === "201" ? "Created" : "OK") {
  if (status === "204") {
    return { description: "No content" };
  }
  return {
    description,
    content: {
      "application/json": {
        schema: schemaRef(schemaName)
      }
    }
  };
}

function pathParam(name) {
  return {
    name,
    in: "path",
    required: true,
    schema: idSchema
  };
}

function listParams(extra = []) {
  return [
    { name: "q", in: "query", required: false, schema: { type: "string" } },
    { name: "cursor", in: "query", required: false, schema: { type: "string" } },
    { name: "page_size", in: "query", required: false, schema: { type: "integer", format: "int32", minimum: 1, maximum: 100 } },
    ...extra
  ];
}

function idempotencyParam() {
  return {
    name: "Idempotency-Key",
    in: "header",
    required: false,
    schema: { type: "string", minLength: 1, maxLength: 128 },
    description: "Client retry idempotency key scoped by tenant, principal, method, and path."
  };
}

function resolveApiSurface({ authority, authMode, apiSurface }) {
  if (apiSurface) {
    return apiSurface;
  }
  if (authMode === "api-key") {
    return "open-api";
  }
  if (authority === "sdkwork-llm.backend") {
    return "backend-api";
  }
  return "app-api";
}

function operation({
  method,
  operationId,
  tag = "llm",
  authority,
  permission,
  auditEvent,
  pathParams = [],
  queryParams = [],
  requestSchema,
  responseSchema,
  status,
  resource,
  idempotent = false,
  authMode = "dual-token",
  apiSurface
}) {
  const resolvedApiSurface = resolveApiSurface({ authority, authMode, apiSurface });
  const responses = {
    [status ?? (method === "post" ? "201" : method === "delete" ? "204" : "200")]: successResponse(status ?? (method === "post" ? "201" : method === "delete" ? "204" : "200"), responseSchema),
    ...errorResponses()
  };
  const parameters = [...pathParams, ...queryParams];
  if (idempotent) {
    parameters.push(idempotencyParam());
  }
  const op = {
    operationId,
    tags: [tag],
    parameters,
    responses,
    security: authMode === "api-key" ? [{ ApiKey: [] }] : [{ AuthToken: [], AccessToken: [] }],
    "x-sdkwork-owner": owner,
    "x-sdkwork-api-authority": authority,
    "x-sdkwork-api-surface": resolvedApiSurface,
    "x-sdkwork-request-context": "WebRequestContext",
    "x-sdkwork-domain": domain,
    "x-sdkwork-resource": resource ?? operationId.split(".")[0],
    "x-sdkwork-permission": permission,
    "x-sdkwork-tenant-scope": "tenant",
    "x-sdkwork-data-scope": "owner",
    "x-sdkwork-auth-mode": authMode,
    "x-sdkwork-audit-event": auditEvent,
    "x-sdkwork-idempotent": idempotent
  };
  if (requestSchema) {
    op.requestBody = {
      required: true,
      content: {
        "application/json": {
          schema: schemaRef(requestSchema)
        }
      }
    };
  }
  return op;
}

function addPath(paths, pathKey, method, op) {
  paths[pathKey] ??= {};
  paths[pathKey][method] = op;
}

function baseSchemas() {
  const enumSchema = (values) => ({ type: "string", enum: values });
  const recordType = enumSchema(["working", "session", "semantic", "episodic", "procedural", "habit", "relationship", "domain_knowledge"]);
  const memoryStatus = enumSchema(["candidate", "active", "inactive", "superseded", "deleted", "rejected"]);
  const candidateState = enumSchema(["pending", "auto_approved", "approved", "rejected", "expired", "superseded"]);
  const habitStage = enumSchema(["observing", "emerging", "confirmed", "decaying", "inactive", "rejected"]);
  return {
    ProblemDetails: {
      type: "object",
      required: ["type", "title", "status"],
      properties: {
        type: { type: "string" },
        title: { type: "string" },
        status: { type: "integer", format: "int32" },
        detail: nullableString,
        instance: nullableString,
        code: nullableString,
        requestId: nullableString,
        traceId: nullableString,
        retryable: { anyOf: [{ type: "boolean" }, { type: "null" }] }
      },
      additionalProperties: true
    },
    LlmPageInfo: {
      type: "object",
      required: ["hasMore"],
      properties: {
        nextCursor: nullableString,
        hasMore: { type: "boolean" },
        pageSize: { type: "integer", format: "int32" }
      }
    },
    LlmSpace: {
      type: "object",
      required: ["spaceId", "tenantId", "ownerSubjectType", "ownerSubjectId", "spaceType", "displayName", "lifecycleStatus", "createdAt", "updatedAt", "version"],
      properties: {
        spaceId: idSchema,
        uuid: { type: "string" },
        tenantId: idSchema,
        organizationId: nullableIdSchema,
        ownerSubjectType: { type: "string" },
        ownerSubjectId: { type: "string" },
        spaceType: { type: "string" },
        displayName: { type: "string" },
        defaultScope: { type: "string" },
        lifecycleStatus: { type: "string" },
        metadata: nullableJsonObject,
        policy: nullableJsonObject,
        createdAt: instant,
        updatedAt: instant,
        version: idSchema
      }
    },
    LlmSpaceRequest: {
      type: "object",
      required: ["ownerSubjectType", "ownerSubjectId", "spaceType", "displayName"],
      properties: {
        organizationId: nullableIdSchema,
        ownerSubjectType: { type: "string" },
        ownerSubjectId: { type: "string" },
        spaceType: { type: "string" },
        displayName: { type: "string" },
        defaultScope: { type: "string" },
        metadata: nullableJsonObject,
        policy: nullableJsonObject,
        version: nullableIdSchema
      }
    },
    LlmSpaceList: pageSchema("LlmSpace"),
    LlmEvent: {
      type: "object",
      required: ["eventId", "spaceId", "eventType", "sourceType", "eventTime", "payloadHash", "ingestionStatus", "createdAt"],
      properties: {
        eventId: idSchema,
        uuid: { type: "string" },
        spaceId: idSchema,
        userId: nullableIdSchema,
        actorType: { type: "string" },
        actorId: nullableString,
        sessionId: nullableString,
        traceId: nullableString,
        eventType: { type: "string" },
        sourceType: { type: "string" },
        sourceRef: nullableString,
        eventTime: instant,
        payload: jsonObject,
        payloadHash: { type: "string" },
        sensitivityLevel: { type: "string" },
        ingestionStatus: { type: "string" },
        createdAt: instant
      }
    },
    LlmEventRequest: {
      type: "object",
      required: ["spaceId", "eventType", "sourceType", "eventTime", "payload"],
      properties: {
        spaceId: idSchema,
        userId: nullableIdSchema,
        actorType: { type: "string" },
        actorId: nullableString,
        sessionId: nullableString,
        traceId: nullableString,
        eventType: { type: "string" },
        sourceType: { type: "string" },
        sourceRef: nullableString,
        eventTime: instant,
        payload: jsonObject,
        sensitivityLevel: { type: "string" }
      }
    },
    LlmEventList: pageSchema("LlmEvent"),
    LlmRecord: {
      type: "object",
      required: ["recordId", "spaceId", "scope", "recordType", "canonicalText", "confidence", "status", "createdAt", "updatedAt", "version"],
      properties: {
        recordId: idSchema,
        uuid: { type: "string" },
        spaceId: idSchema,
        userId: nullableIdSchema,
        scope: { type: "string" },
        recordType,
        subject: nullableString,
        predicate: nullableString,
        objectText: { type: "string" },
        canonicalText: { type: "string" },
        summaryText: nullableString,
        language: nullableString,
        confidence: { type: "number", format: "double" },
        evidenceCount: { type: "integer", format: "int32" },
        contradictionCount: { type: "integer", format: "int32" },
        importanceScore: { type: "number", format: "double" },
        recencyScore: { type: "number", format: "double" },
        habitStrength: { anyOf: [{ type: "number", format: "double" }, { type: "null" }] },
        validFrom: nullableInstant,
        validTo: nullableInstant,
        expiresAt: nullableInstant,
        status: memoryStatus,
        sensitivityLevel: { type: "string" },
        metadata: nullableJsonObject,
        tags: { anyOf: [{ type: "array", items: { type: "string" } }, { type: "null" }] },
        supersedesRecordId: nullableIdSchema,
        supersededRecordId: nullableIdSchema,
        createdAt: instant,
        updatedAt: instant,
        version: idSchema
      }
    },
    LlmRecordRequest: {
      type: "object",
      required: ["spaceId", "scope", "recordType", "canonicalText"],
      properties: {
        spaceId: idSchema,
        userId: nullableIdSchema,
        scope: { type: "string" },
        recordType,
        subject: nullableString,
        predicate: nullableString,
        objectText: nullableString,
        canonicalText: { type: "string" },
        summaryText: nullableString,
        confidence: { anyOf: [{ type: "number", format: "double" }, { type: "null" }] },
        validFrom: nullableInstant,
        validTo: nullableInstant,
        expiresAt: nullableInstant,
        sensitivityLevel: { type: "string" },
        metadata: nullableJsonObject,
        tags: { anyOf: [{ type: "array", items: { type: "string" } }, { type: "null" }] },
        version: nullableIdSchema
      }
    },
    LlmRecordList: pageSchema("LlmRecord"),
    LlmRecordSource: {
      type: "object",
      required: ["sourceId", "recordId", "eventId", "sourceRole", "createdAt"],
      properties: {
        sourceId: idSchema,
        recordId: idSchema,
        eventId: idSchema,
        sourceRole: { type: "string" },
        confidenceDelta: { anyOf: [{ type: "number", format: "double" }, { type: "null" }] },
        createdAt: instant
      }
    },
    LlmRecordSourceList: pageSchema("LlmRecordSource"),
    LlmCandidate: {
      type: "object",
      required: ["candidateId", "spaceId", "candidateType", "recordType", "proposedText", "confidence", "decisionState", "createdAt", "updatedAt"],
      properties: {
        candidateId: idSchema,
        spaceId: idSchema,
        userId: nullableIdSchema,
        candidateType: { type: "string" },
        recordType,
        proposedText: { type: "string" },
        proposedPayload: nullableJsonObject,
        targetRecordId: nullableIdSchema,
        evidence: nullableJsonObject,
        confidence: { type: "number", format: "double" },
        noveltyScore: { anyOf: [{ type: "number", format: "double" }, { type: "null" }] },
        riskScore: { anyOf: [{ type: "number", format: "double" }, { type: "null" }] },
        decisionState: candidateState,
        decisionReason: nullableString,
        createdAt: instant,
        updatedAt: instant,
        version: idSchema
      }
    },
    LlmCandidateList: pageSchema("LlmCandidate"),
    LlmReviewRequest: {
      type: "object",
      properties: {
        reason: nullableString,
        reviewerNote: nullableString,
        metadata: nullableJsonObject
      }
    },
    LlmHabit: {
      type: "object",
      required: ["habitId", "spaceId", "userId", "habitKey", "habitType", "description", "stage", "strength", "confidence", "supportCount", "createdAt", "updatedAt", "version"],
      properties: {
        habitId: idSchema,
        spaceId: idSchema,
        userId: idSchema,
        habitKey: { type: "string" },
        habitType: { type: "string" },
        description: { type: "string" },
        stage: habitStage,
        strength: { type: "number", format: "double" },
        confidence: { type: "number", format: "double" },
        supportCount: { type: "integer", format: "int32" },
        lastSignalAt: nullableInstant,
        promotedRecordId: nullableIdSchema,
        decayAfter: nullableInstant,
        metadata: nullableJsonObject,
        createdAt: instant,
        updatedAt: instant,
        version: idSchema
      }
    },
    LlmHabitRequest: {
      type: "object",
      properties: {
        description: nullableString,
        stage: habitStage,
        metadata: nullableJsonObject,
        version: nullableIdSchema
      }
    },
    LlmHabitList: pageSchema("LlmHabit"),
    LlmExtractionRequest: {
      type: "object",
      required: ["spaceId", "inputEvents"],
      properties: {
        spaceId: idSchema,
        inputEvents: { type: "array", items: idSchema },
        extractionMode: { type: "string", enum: ["deterministic", "llm_assisted", "hybrid"] },
        reviewRequired: { type: "boolean" },
        metadata: nullableJsonObject
      }
    },
    LlmLearningJob: {
      type: "object",
      required: ["jobId", "jobType", "state", "priority", "createdAt", "updatedAt"],
      properties: {
        jobId: idSchema,
        spaceId: nullableIdSchema,
        jobType: { type: "string" },
        state: { type: "string" },
        priority: { type: "integer", format: "int32" },
        result: nullableJsonObject,
        error: nullableJsonObject,
        startedAt: nullableInstant,
        finishedAt: nullableInstant,
        createdAt: instant,
        updatedAt: instant,
        version: idSchema
      }
    },
    LlmRetrievalRequest: {
      type: "object",
      required: ["query", "spaceIds", "topK", "contextBudgetTokens"],
      properties: {
        query: { type: "string" },
        spaceIds: { type: "array", items: idSchema },
        actorId: nullableString,
        retrievalProfileId: nullableIdSchema,
        recordTypes: { anyOf: [{ type: "array", items: recordType }, { type: "null" }] },
        filters: nullableJsonObject,
        topK: { type: "integer", format: "int32", minimum: 1, maximum: 100 },
        contextBudgetTokens: { type: "integer", format: "int32", minimum: 1 },
        includeTrace: { type: "boolean" }
      }
    },
    LlmRetrievalResult: {
      type: "object",
      required: ["retrievalId", "hits", "degraded"],
      properties: {
        retrievalId: idSchema,
        trace: { anyOf: [schemaRef("LlmRetrievalTrace"), { type: "null" }] },
        hits: { type: "array", items: schemaRef("LlmRetrievalHit") },
        degraded: { type: "boolean" }
      }
    },
    LlmRetrievalTrace: {
      type: "object",
      required: ["traceId", "queryHash", "resultCount", "degraded", "createdAt"],
      properties: {
        traceId: idSchema,
        spaceId: nullableIdSchema,
        retrievalProfileId: nullableIdSchema,
        actorId: nullableString,
        queryText: nullableString,
        queryHash: { type: "string" },
        retrievers: nullableJsonObject,
        latencyMs: { anyOf: [{ type: "integer", format: "int32" }, { type: "null" }] },
        resultCount: { type: "integer", format: "int32" },
        degraded: { type: "boolean" },
        metadata: nullableJsonObject,
        createdAt: instant
      }
    },
    LlmRetrievalTraceList: pageSchema("LlmRetrievalTrace"),
    LlmRetrievalHit: {
      type: "object",
      required: ["hitId", "retrieverName", "resultRank", "status"],
      properties: {
        hitId: idSchema,
        memory: { anyOf: [schemaRef("LlmRecord"), { type: "null" }] },
        recordId: nullableIdSchema,
        retrieverName: { type: "string" },
        resultRank: { type: "integer", format: "int32" },
        rawScore: { anyOf: [{ type: "number", format: "double" }, { type: "null" }] },
        fusedScore: { anyOf: [{ type: "number", format: "double" }, { type: "null" }] },
        explanation: nullableJsonObject,
        status: { type: "string" }
      }
    },
    LlmContextPackRequest: {
      type: "object",
      required: ["query", "spaceIds", "contextBudgetTokens"],
      properties: {
        query: { type: "string" },
        spaceIds: { type: "array", items: idSchema },
        actorId: nullableString,
        retrievalProfileId: nullableIdSchema,
        contextBudgetTokens: { type: "integer", format: "int32" },
        includeCitations: { type: "boolean" },
        filters: nullableJsonObject
      }
    },
    LlmContextPack: {
      type: "object",
      required: ["contextPackId", "pack", "estimatedTokens", "truncated", "createdAt"],
      properties: {
        contextPackId: idSchema,
        retrievalId: nullableIdSchema,
        query: nullableString,
        pack: jsonObject,
        estimatedTokens: { type: "integer", format: "int32" },
        truncated: { type: "boolean" },
        createdAt: instant
      }
    },
    LlmFeedbackRequest: {
      type: "object",
      required: ["targetType", "targetId", "feedbackType"],
      properties: {
        targetType: { type: "string", enum: ["retrieval", "hit", "memory", "candidate", "habit"] },
        targetId: idSchema,
        feedbackType: { type: "string" },
        rating: { anyOf: [{ type: "integer", format: "int32" }, { type: "null" }] },
        comment: nullableString,
        metadata: nullableJsonObject
      }
    },
    LlmFeedback: {
      type: "object",
      required: ["feedbackId", "targetType", "targetId", "feedbackType", "createdAt"],
      properties: {
        feedbackId: idSchema,
        targetType: { type: "string" },
        targetId: idSchema,
        feedbackType: { type: "string" },
        rating: { anyOf: [{ type: "integer", format: "int32" }, { type: "null" }] },
        comment: nullableString,
        createdAt: instant
      }
    },
    LlmForgetRequest: {
      type: "object",
      required: ["scope", "reason"],
      properties: {
        scope: { type: "string", enum: ["memory", "space", "user", "query"] },
        recordIds: { anyOf: [{ type: "array", items: idSchema }, { type: "null" }] },
        spaceId: nullableIdSchema,
        query: nullableString,
        reason: { type: "string" },
        metadata: nullableJsonObject
      }
    },
    LlmForgetJob: {
      type: "object",
      required: ["forgetRequestId", "state", "createdAt", "updatedAt"],
      properties: {
        forgetRequestId: idSchema,
        state: { type: "string", enum: ["queued", "running", "succeeded", "failed", "cancelled"] },
        result: nullableJsonObject,
        createdAt: instant,
        updatedAt: instant
      }
    },
    LlmExportRequest: {
      type: "object",
      required: ["spaceIds", "format"],
      properties: {
        spaceIds: { type: "array", items: idSchema },
        format: { type: "string", enum: ["json", "jsonl", "markdown"] },
        includeEvents: { type: "boolean" },
        driveTargetRef: nullableString,
        metadata: nullableJsonObject
      }
    },
    LlmExportJob: {
      type: "object",
      required: ["exportJobId", "state", "format", "createdAt", "updatedAt"],
      properties: {
        exportJobId: idSchema,
        state: { type: "string" },
        format: { type: "string" },
        driveObjectRef: nullableString,
        result: nullableJsonObject,
        createdAt: instant,
        updatedAt: instant
      }
    },
    LlmLearningSettings: {
      type: "object",
      required: ["autoExtractEnabled", "autoApproveThreshold", "reviewRequiredBelowThreshold", "habitPromotionThreshold", "updatedAt", "version"],
      properties: {
        autoExtractEnabled: { type: "boolean" },
        autoApproveThreshold: { type: "number", format: "double" },
        reviewRequiredBelowThreshold: { type: "boolean" },
        habitPromotionThreshold: { type: "number", format: "double" },
        retentionPolicyRef: nullableString,
        updatedAt: instant,
        version: idSchema
      }
    },
    LlmLearningSettingsRequest: {
      type: "object",
      properties: {
        autoExtractEnabled: { type: "boolean" },
        autoApproveThreshold: { type: "number", format: "double" },
        reviewRequiredBelowThreshold: { type: "boolean" },
        habitPromotionThreshold: { type: "number", format: "double" },
        retentionPolicyRef: nullableString,
        version: nullableIdSchema
      }
    },
    LlmIndex: {
      type: "object",
      required: ["indexId", "indexKind", "schemaVersion", "status", "createdAt", "updatedAt", "version"],
      properties: {
        indexId: idSchema,
        spaceId: nullableIdSchema,
        indexKind: { type: "string", enum: ["sql", "keyword", "dictionary", "time", "event", "vector", "graph", "grep_file", "custom"] },
        implementationProfileId: nullableIdSchema,
        providerBindingId: nullableIdSchema,
        schemaVersion: { type: "string" },
        status: { type: "string" },
        config: nullableJsonObject,
        lastRebuiltAt: nullableInstant,
        createdAt: instant,
        updatedAt: instant,
        version: idSchema
      }
    },
    LlmIndexRequest: {
      type: "object",
      required: ["indexKind", "schemaVersion"],
      properties: {
        spaceId: nullableIdSchema,
        indexKind: { type: "string" },
        implementationProfileId: nullableIdSchema,
        providerBindingId: nullableIdSchema,
        schemaVersion: { type: "string" },
        config: nullableJsonObject,
        status: { type: "string" },
        version: nullableIdSchema
      }
    },
    LlmIndexList: pageSchema("LlmIndex"),
    LlmRetrievalProfile: {
      type: "object",
      required: ["retrievalProfileId", "name", "strategy", "retrievers", "topK", "contextBudgetTokens", "status", "createdAt", "updatedAt", "version"],
      properties: {
        retrievalProfileId: idSchema,
        spaceId: nullableIdSchema,
        name: { type: "string" },
        strategy: { type: "string" },
        retrievers: jsonObject,
        fusionPolicy: nullableJsonObject,
        rerankPolicy: nullableJsonObject,
        topK: { type: "integer", format: "int32" },
        contextBudgetTokens: { type: "integer", format: "int32" },
        status: { type: "string" },
        createdAt: instant,
        updatedAt: instant,
        version: idSchema
      }
    },
    LlmRetrievalProfileRequest: {
      type: "object",
      required: ["name", "strategy", "retrievers", "topK", "contextBudgetTokens"],
      properties: {
        spaceId: nullableIdSchema,
        name: { type: "string" },
        strategy: { type: "string" },
        retrievers: jsonObject,
        fusionPolicy: nullableJsonObject,
        rerankPolicy: nullableJsonObject,
        topK: { type: "integer", format: "int32" },
        contextBudgetTokens: { type: "integer", format: "int32" },
        status: { type: "string" },
        version: nullableIdSchema
      }
    },
    LlmRetrievalProfileList: pageSchema("LlmRetrievalProfile"),
    LlmImplementationProfile: {
      type: "object",
      required: ["implementationProfileId", "name", "implementationKind", "role", "status", "capabilities", "createdAt", "updatedAt", "version"],
      properties: {
        implementationProfileId: idSchema,
        name: { type: "string" },
        implementationKind: { type: "string", enum: ["native_sql", "event_sourced", "graph_temporal", "search_first", "local_embedded", "external_provider_bridge", "hybrid_platform"] },
        role: { type: "string" },
        status: { type: "string" },
        capabilities: jsonObject,
        config: nullableJsonObject,
        rollout: nullableJsonObject,
        createdAt: instant,
        updatedAt: instant,
        version: idSchema
      }
    },
    LlmImplementationProfileRequest: {
      type: "object",
      required: ["name", "implementationKind", "role", "capabilities"],
      properties: {
        name: { type: "string" },
        implementationKind: { type: "string" },
        role: { type: "string" },
        status: { type: "string" },
        capabilities: jsonObject,
        config: nullableJsonObject,
        rollout: nullableJsonObject,
        version: nullableIdSchema
      }
    },
    LlmImplementationProfileList: pageSchema("LlmImplementationProfile"),
    LlmProviderBinding: {
      type: "object",
      required: ["providerBindingId", "providerKind", "providerCode", "displayName", "capabilities", "healthState", "createdAt", "updatedAt", "version"],
      properties: {
        providerBindingId: idSchema,
        providerKind: { type: "string" },
        providerCode: { type: "string" },
        displayName: { type: "string" },
        endpointRef: nullableString,
        secretRef: nullableString,
        modelRef: nullableString,
        capabilities: jsonObject,
        config: nullableJsonObject,
        healthState: { type: "string" },
        lastHealthAt: nullableInstant,
        createdAt: instant,
        updatedAt: instant,
        version: idSchema
      }
    },
    LlmProviderBindingRequest: {
      type: "object",
      required: ["providerKind", "providerCode", "displayName", "capabilities"],
      properties: {
        providerKind: { type: "string" },
        providerCode: { type: "string" },
        displayName: { type: "string" },
        endpointRef: nullableString,
        secretRef: nullableString,
        modelRef: nullableString,
        capabilities: jsonObject,
        config: nullableJsonObject,
        healthState: { type: "string" },
        version: nullableIdSchema
      }
    },
    LlmProviderBindingList: pageSchema("LlmProviderBinding"),
    LlmProviderHealth: {
      type: "object",
      required: ["status", "checkedAt", "providers"],
      properties: {
        status: { type: "string", enum: ["healthy", "degraded", "unhealthy", "unknown"] },
        checkedAt: instant,
        providers: { type: "array", items: schemaRef("LlmProviderBinding") }
      }
    },
    LlmCapabilities: {
      type: "object",
      required: ["embeddingOptional", "retrievers", "providerInterfaces", "implementationKinds", "openApiPrefix", "sdkFamily", "checkedAt"],
      properties: {
        embeddingOptional: { type: "boolean" },
        retrievers: {
          type: "array",
          items: {
            type: "string",
            enum: ["sql", "keyword", "dictionary", "time", "event", "vector", "graph", "grep_file", "custom"]
          }
        },
        providerInterfaces: {
          type: "array",
          items: {
            type: "string",
            enum: ["llm", "embedding", "rerank", "tokenizer", "graph", "search", "file", "memory"]
          }
        },
        implementationKinds: {
          type: "array",
          items: {
            type: "string",
            enum: ["native_sql", "event_sourced", "graph_temporal", "search_first", "local_embedded", "external_provider_bridge", "hybrid_platform"]
          }
        },
        openApiPrefix: { type: "string" },
        sdkFamily: { type: "string" },
        checkedAt: instant,
        metadata: nullableJsonObject
      }
    },
    LlmEvalRun: {
      type: "object",
      required: ["evalRunId", "evalType", "state", "createdAt", "updatedAt"],
      properties: {
        evalRunId: idSchema,
        evalType: { type: "string" },
        state: { type: "string" },
        datasetRef: nullableString,
        profileRef: nullableString,
        metrics: nullableJsonObject,
        result: nullableJsonObject,
        startedAt: nullableInstant,
        finishedAt: nullableInstant,
        createdAt: instant,
        updatedAt: instant
      }
    },
    LlmEvalRunRequest: {
      type: "object",
      required: ["evalType"],
      properties: {
        evalType: { type: "string" },
        datasetRef: nullableString,
        profileRef: nullableString,
        config: nullableJsonObject
      }
    },
    LlmEvalRunList: pageSchema("LlmEvalRun"),
    LlmAuditLog: {
      type: "object",
      required: ["auditLogId", "actorType", "action", "resourceType", "result", "createdAt"],
      properties: {
        auditLogId: idSchema,
        actorType: { type: "string" },
        actorId: nullableString,
        action: { type: "string" },
        resourceType: { type: "string" },
        resourceId: nullableString,
        requestId: nullableString,
        traceId: nullableString,
        result: { type: "string" },
        reason: nullableString,
        metadata: nullableJsonObject,
        createdAt: instant
      }
    },
    LlmAuditLogList: pageSchema("LlmAuditLog"),
    LlmRetentionJobRequest: {
      type: "object",
      required: ["scope"],
      properties: {
        scope: { type: "string" },
        spaceId: nullableIdSchema,
        dryRun: { type: "boolean" },
        policyRef: nullableString,
        metadata: nullableJsonObject
      }
    },
    LlmMigrationJobRequest: {
      type: "object",
      required: ["sourceImplementationProfileId", "targetImplementationProfileId", "mode"],
      properties: {
        sourceImplementationProfileId: idSchema,
        targetImplementationProfileId: idSchema,
        mode: { type: "string", enum: ["shadow", "dual_write", "backfill", "cutover", "rollback"] },
        spaceIds: { anyOf: [{ type: "array", items: idSchema }, { type: "null" }] },
        dryRun: { type: "boolean" },
        metadata: nullableJsonObject
      }
    }
  };
}

function securitySchemesFor(authMode) {
  if (authMode === "api-key") {
    return {
      ApiKey: {
        type: "apiKey",
        in: "header",
        name: "X-API-Key"
      }
    };
  }

  return {
    AuthToken: {
      type: "http",
      scheme: "bearer",
      bearerFormat: "JWT"
    },
    AccessToken: {
      type: "apiKey",
      in: "header",
      name: "Access-Token"
    }
  };
}

function createOpenApi({ title, authority, prefix, sdkFamily, paths, authMode = "dual-token" }) {
  return {
    openapi: "3.1.2",
    info: {
      title,
      version
    },
    servers: [
      { url: "https://api.sdkwork.com", description: "SDKWork production API" },
      { url: "http://localhost:8080", description: "Local/private API" }
    ],
    paths,
    components: {
      schemas: baseSchemas(),
      securitySchemes: securitySchemesFor(authMode)
    },
    "x-sdkwork-owner": owner,
    "x-sdkwork-api-authority": authority,
    "x-sdkwork-sdk-family": sdkFamily,
    "x-sdkwork-owner-only-input": true,
    "x-sdkwork-standard-version": standardVersion,
    "x-sdkwork-domain": domain,
    "x-sdkwork-api-prefix": prefix
  };
}

function openOperation(args) {
  return operation({ ...args, authMode: "api-key" });
}

function writeOpenApi() {
  const paths = {};
  const authority = "sdkwork-llm-open-api";
  const P = `${llmOpenApiPrefix}/llm`;

  addPath(paths, `${P}/capabilities`, "get", openOperation({
    method: "get",
    authority,
    operationId: "capabilities.retrieve",
    permission: "llm.open.capabilities.read",
    auditEvent: "llm.open.capabilities.read",
    responseSchema: "LlmCapabilities"
  }));

  addPath(paths, `${P}/events`, "post", openOperation({
    method: "post",
    authority,
    operationId: "events.create",
    permission: "llm.open.events.write",
    auditEvent: "llm.open.event.appended",
    requestSchema: "LlmEventRequest",
    responseSchema: "LlmEvent",
    idempotent: true
  }));
  addPath(paths, `${P}/events/{eventId}`, "get", openOperation({
    method: "get",
    authority,
    operationId: "events.retrieve",
    permission: "llm.open.events.read",
    auditEvent: "llm.open.event.read",
    pathParams: [pathParam("eventId")],
    responseSchema: "LlmEvent"
  }));

  addPath(paths, `${P}/records`, "get", openOperation({
    method: "get",
    authority,
    operationId: "records.list",
    permission: "llm.open.records.read",
    auditEvent: "llm.open.record.list",
    queryParams: listParams([
      { name: "space_id", in: "query", schema: idSchema },
      { name: "record_type", in: "query", schema: { type: "string" } },
      { name: "external_subject_ref", in: "query", schema: { type: "string" } }
    ]),
    responseSchema: "LlmRecordList"
  }));
  addPath(paths, `${P}/records`, "post", openOperation({
    method: "post",
    authority,
    operationId: "records.create",
    permission: "llm.open.records.write",
    auditEvent: "llm.open.record.created",
    requestSchema: "LlmRecordRequest",
    responseSchema: "LlmRecord",
    idempotent: true
  }));
  addPath(paths, `${P}/records/{recordId}`, "get", openOperation({
    method: "get",
    authority,
    operationId: "records.retrieve",
    permission: "llm.open.records.read",
    auditEvent: "llm.open.record.read",
    pathParams: [pathParam("recordId")],
    responseSchema: "LlmRecord"
  }));
  addPath(paths, `${P}/records/{recordId}`, "patch", openOperation({
    method: "patch",
    authority,
    operationId: "records.update",
    permission: "llm.open.records.write",
    auditEvent: "llm.open.record.updated",
    pathParams: [pathParam("recordId")],
    requestSchema: "LlmRecordRequest",
    responseSchema: "LlmRecord"
  }));
  addPath(paths, `${P}/records/{recordId}`, "delete", openOperation({
    method: "delete",
    authority,
    operationId: "records.delete",
    permission: "llm.open.records.write",
    auditEvent: "llm.open.record.deleted",
    pathParams: [pathParam("recordId")],
    responseSchema: "LlmRecord",
    status: "204"
  }));

  addPath(paths, `${P}/retrievals`, "post", openOperation({
    method: "post",
    authority,
    operationId: "retrievals.create",
    permission: "llm.open.retrievals.write",
    auditEvent: "llm.open.retrieval.created",
    requestSchema: "LlmRetrievalRequest",
    responseSchema: "LlmRetrievalResult",
    idempotent: true
  }));
  addPath(paths, `${P}/retrievals/{retrievalId}`, "get", openOperation({
    method: "get",
    authority,
    operationId: "retrievals.retrieve",
    permission: "llm.open.retrievals.read",
    auditEvent: "llm.open.retrieval.read",
    pathParams: [pathParam("retrievalId")],
    responseSchema: "LlmRetrievalResult"
  }));

  addPath(paths, `${P}/context_packs`, "post", openOperation({
    method: "post",
    authority,
    operationId: "contextPacks.create",
    permission: "llm.open.contextPacks.write",
    auditEvent: "llm.open.context_pack.created",
    requestSchema: "LlmContextPackRequest",
    responseSchema: "LlmContextPack",
    idempotent: true
  }));
  addPath(paths, `${P}/context_packs/{contextPackId}`, "get", openOperation({
    method: "get",
    authority,
    operationId: "contextPacks.retrieve",
    permission: "llm.open.contextPacks.read",
    auditEvent: "llm.open.context_pack.read",
    pathParams: [pathParam("contextPackId")],
    responseSchema: "LlmContextPack"
  }));

  addPath(paths, `${P}/feedback`, "post", openOperation({
    method: "post",
    authority,
    operationId: "feedback.create",
    permission: "llm.open.feedback.write",
    auditEvent: "llm.open.feedback.created",
    requestSchema: "LlmFeedbackRequest",
    responseSchema: "LlmFeedback",
    idempotent: true
  }));

  addPath(paths, `${P}/extractions`, "post", openOperation({
    method: "post",
    authority,
    operationId: "extractions.create",
    permission: "llm.open.learning.write",
    auditEvent: "llm.open.extraction.requested",
    requestSchema: "LlmExtractionRequest",
    responseSchema: "LlmLearningJob",
    idempotent: true
  }));

  addPath(paths, `${P}/candidates`, "get", openOperation({
    method: "get",
    authority,
    operationId: "candidates.list",
    permission: "llm.open.candidates.read",
    auditEvent: "llm.open.candidate.list",
    queryParams: listParams([{ name: "decision_state", in: "query", schema: { type: "string" } }]),
    responseSchema: "LlmCandidateList"
  }));
  addPath(paths, `${P}/candidates/{candidateId}`, "get", openOperation({
    method: "get",
    authority,
    operationId: "candidates.retrieve",
    permission: "llm.open.candidates.read",
    auditEvent: "llm.open.candidate.read",
    pathParams: [pathParam("candidateId")],
    responseSchema: "LlmCandidate"
  }));

  addPath(paths, `${P}/provider_health`, "get", openOperation({
    method: "get",
    authority,
    operationId: "providerHealth.retrieve",
    permission: "llm.open.providerHealth.read",
    auditEvent: "llm.open.provider_health.read",
    responseSchema: "LlmProviderHealth"
  }));

  writeJson("sdks/sdkwork-llm-sdk/openapi/llm-open-api.openapi.json", createOpenApi({
    title: "SDKWork LLM Open API",
    authority,
    prefix: llmOpenApiPrefix,
    sdkFamily: "sdkwork-llm-sdk",
    paths,
    authMode: "api-key"
  }));
}

function writeAppOpenApi() {
  const paths = {};
  const authority = "sdkwork-llm.app";
  const P = "/app/v3/api/llm";
  addPath(paths, `${P}/spaces`, "get", operation({ method: "get", authority, operationId: "spaces.list", permission: "llm.spaces.read", auditEvent: "llm.space.list", queryParams: listParams(), responseSchema: "LlmSpaceList" }));
  addPath(paths, `${P}/spaces`, "post", operation({ method: "post", authority, operationId: "spaces.create", permission: "llm.spaces.write", auditEvent: "llm.space.created", requestSchema: "LlmSpaceRequest", responseSchema: "LlmSpace", idempotent: true }));
  addPath(paths, `${P}/spaces/{spaceId}`, "get", operation({ method: "get", authority, operationId: "spaces.retrieve", permission: "llm.spaces.read", auditEvent: "llm.space.read", pathParams: [pathParam("spaceId")], responseSchema: "LlmSpace" }));
  addPath(paths, `${P}/spaces/{spaceId}`, "patch", operation({ method: "patch", authority, operationId: "spaces.update", permission: "llm.spaces.write", auditEvent: "llm.space.updated", pathParams: [pathParam("spaceId")], requestSchema: "LlmSpaceRequest", responseSchema: "LlmSpace" }));

  addPath(paths, `${P}/events`, "post", operation({ method: "post", authority, operationId: "events.create", permission: "llm.events.write", auditEvent: "llm.event.appended", requestSchema: "LlmEventRequest", responseSchema: "LlmEvent", idempotent: true }));
  addPath(paths, `${P}/events/{eventId}`, "get", operation({ method: "get", authority, operationId: "events.retrieve", permission: "llm.events.read", auditEvent: "llm.event.read", pathParams: [pathParam("eventId")], responseSchema: "LlmEvent" }));

  addPath(paths, `${P}/records`, "get", operation({ method: "get", authority, operationId: "records.list", permission: "llm.records.read", auditEvent: "llm.record.list", queryParams: listParams([{ name: "space_id", in: "query", schema: idSchema }, { name: "record_type", in: "query", schema: { type: "string" } }]), responseSchema: "LlmRecordList" }));
  addPath(paths, `${P}/records`, "post", operation({ method: "post", authority, operationId: "records.create", permission: "llm.records.write", auditEvent: "llm.record.created", requestSchema: "LlmRecordRequest", responseSchema: "LlmRecord", idempotent: true }));
  addPath(paths, `${P}/records/{recordId}`, "get", operation({ method: "get", authority, operationId: "records.retrieve", permission: "llm.records.read", auditEvent: "llm.record.read", pathParams: [pathParam("recordId")], responseSchema: "LlmRecord" }));
  addPath(paths, `${P}/records/{recordId}`, "patch", operation({ method: "patch", authority, operationId: "records.update", permission: "llm.records.write", auditEvent: "llm.record.updated", pathParams: [pathParam("recordId")], requestSchema: "LlmRecordRequest", responseSchema: "LlmRecord" }));
  addPath(paths, `${P}/records/{recordId}`, "delete", operation({ method: "delete", authority, operationId: "records.delete", permission: "llm.records.write", auditEvent: "llm.record.deleted", pathParams: [pathParam("recordId")], responseSchema: "LlmRecord", status: "204" }));
  addPath(paths, `${P}/records/{recordId}/sources`, "get", operation({ method: "get", authority, operationId: "records.sources.list", permission: "llm.records.read", auditEvent: "llm.record.sources.list", pathParams: [pathParam("recordId")], queryParams: listParams(), responseSchema: "LlmRecordSourceList" }));

  addPath(paths, `${P}/forget_requests`, "post", operation({ method: "post", authority, operationId: "forgetRequests.create", permission: "llm.forget.write", auditEvent: "llm.forget.requested", requestSchema: "LlmForgetRequest", responseSchema: "LlmForgetJob", idempotent: true }));
  addPath(paths, `${P}/forget_requests/{forgetRequestId}`, "get", operation({ method: "get", authority, operationId: "forgetRequests.retrieve", permission: "llm.forget.read", auditEvent: "llm.forget.read", pathParams: [pathParam("forgetRequestId")], responseSchema: "LlmForgetJob" }));

  addPath(paths, `${P}/extractions`, "post", operation({ method: "post", authority, operationId: "extractions.create", permission: "llm.learning.write", auditEvent: "memory.extraction.requested", requestSchema: "LlmExtractionRequest", responseSchema: "LlmLearningJob", idempotent: true }));

  addPath(paths, `${P}/candidates`, "get", operation({ method: "get", authority, operationId: "candidates.list", permission: "llm.candidates.read", auditEvent: "llm.candidate.list", queryParams: listParams([{ name: "decision_state", in: "query", schema: { type: "string" } }]), responseSchema: "LlmCandidateList" }));
  addPath(paths, `${P}/candidates/{candidateId}`, "get", operation({ method: "get", authority, operationId: "candidates.retrieve", permission: "llm.candidates.read", auditEvent: "llm.candidate.read", pathParams: [pathParam("candidateId")], responseSchema: "LlmCandidate" }));
  addPath(paths, `${P}/candidates/{candidateId}/approve`, "post", operation({ method: "post", authority, operationId: "candidates.approve", permission: "llm.candidates.write", auditEvent: "llm.candidate.approved", pathParams: [pathParam("candidateId")], requestSchema: "LlmReviewRequest", responseSchema: "LlmCandidate", idempotent: true }));
  addPath(paths, `${P}/candidates/{candidateId}/reject`, "post", operation({ method: "post", authority, operationId: "candidates.reject", permission: "llm.candidates.write", auditEvent: "llm.candidate.rejected", pathParams: [pathParam("candidateId")], requestSchema: "LlmReviewRequest", responseSchema: "LlmCandidate", idempotent: true }));

  addPath(paths, `${P}/habits`, "get", operation({ method: "get", authority, operationId: "habits.list", permission: "llm.habits.read", auditEvent: "llm.habit.list", queryParams: listParams([{ name: "stage", in: "query", schema: { type: "string" } }]), responseSchema: "LlmHabitList" }));
  addPath(paths, `${P}/habits/{habitId}`, "get", operation({ method: "get", authority, operationId: "habits.retrieve", permission: "llm.habits.read", auditEvent: "llm.habit.read", pathParams: [pathParam("habitId")], responseSchema: "LlmHabit" }));
  addPath(paths, `${P}/habits/{habitId}`, "patch", operation({ method: "patch", authority, operationId: "habits.update", permission: "llm.habits.write", auditEvent: "llm.habit.updated", pathParams: [pathParam("habitId")], requestSchema: "LlmHabitRequest", responseSchema: "LlmHabit" }));
  addPath(paths, `${P}/habits/{habitId}/confirm`, "post", operation({ method: "post", authority, operationId: "habits.confirm", permission: "llm.habits.write", auditEvent: "llm.habit.confirmed", pathParams: [pathParam("habitId")], requestSchema: "LlmReviewRequest", responseSchema: "LlmHabit", idempotent: true }));
  addPath(paths, `${P}/habits/{habitId}/reject`, "post", operation({ method: "post", authority, operationId: "habits.reject", permission: "llm.habits.write", auditEvent: "llm.habit.rejected", pathParams: [pathParam("habitId")], requestSchema: "LlmReviewRequest", responseSchema: "LlmHabit", idempotent: true }));

  addPath(paths, `${P}/retrievals`, "post", operation({ method: "post", authority, operationId: "retrievals.create", permission: "llm.retrievals.write", auditEvent: "llm.retrieval.created", requestSchema: "LlmRetrievalRequest", responseSchema: "LlmRetrievalResult", idempotent: true }));
  addPath(paths, `${P}/retrievals/{retrievalId}`, "get", operation({ method: "get", authority, operationId: "retrievals.retrieve", permission: "llm.retrievals.read", auditEvent: "llm.retrieval.read", pathParams: [pathParam("retrievalId")], responseSchema: "LlmRetrievalResult" }));
  addPath(paths, `${P}/context_packs`, "post", operation({ method: "post", authority, operationId: "contextPacks.create", permission: "llm.contextPacks.write", auditEvent: "memory.context_pack.created", requestSchema: "LlmContextPackRequest", responseSchema: "LlmContextPack", idempotent: true }));
  addPath(paths, `${P}/context_packs/{contextPackId}`, "get", operation({ method: "get", authority, operationId: "contextPacks.retrieve", permission: "llm.contextPacks.read", auditEvent: "memory.context_pack.read", pathParams: [pathParam("contextPackId")], responseSchema: "LlmContextPack" }));
  addPath(paths, `${P}/feedback`, "post", operation({ method: "post", authority, operationId: "feedback.create", permission: "llm.feedback.write", auditEvent: "llm.feedback.created", requestSchema: "LlmFeedbackRequest", responseSchema: "LlmFeedback", idempotent: true }));
  addPath(paths, `${P}/export_jobs`, "post", operation({ method: "post", authority, operationId: "exportJobs.create", permission: "llm.exports.write", auditEvent: "llm.export.requested", requestSchema: "LlmExportRequest", responseSchema: "LlmExportJob", idempotent: true }));
  addPath(paths, `${P}/export_jobs/{exportJobId}`, "get", operation({ method: "get", authority, operationId: "exportJobs.retrieve", permission: "llm.exports.read", auditEvent: "llm.export.read", pathParams: [pathParam("exportJobId")], responseSchema: "LlmExportJob" }));
  addPath(paths, `${P}/learning_settings`, "get", operation({ method: "get", authority, operationId: "learningSettings.retrieve", permission: "llm.learningSettings.read", auditEvent: "llm.learning_settings.read", responseSchema: "LlmLearningSettings" }));
  addPath(paths, `${P}/learning_settings`, "patch", operation({ method: "patch", authority, operationId: "learningSettings.update", permission: "llm.learningSettings.write", auditEvent: "llm.learning_settings.updated", requestSchema: "LlmLearningSettingsRequest", responseSchema: "LlmLearningSettings" }));

  writeJson("sdks/sdkwork-llm-app-sdk/openapi/llm-app-api.openapi.json", createOpenApi({
    title: "SDKWork LLM App API",
    authority,
    prefix: "/app/v3/api",
    sdkFamily: "sdkwork-llm-app-sdk",
    paths
  }));
}

function writeBackendOpenApi() {
  const paths = {};
  const authority = "sdkwork-llm.backend";
  const P = "/backend/v3/api/llm";
  addPath(paths, `${P}/spaces`, "get", operation({ method: "get", authority, operationId: "spaces.list", permission: "llm.backend.spaces.read", auditEvent: "llm.backend.space.list", queryParams: listParams(), responseSchema: "LlmSpaceList" }));
  addPath(paths, `${P}/spaces/{spaceId}`, "get", operation({ method: "get", authority, operationId: "spaces.retrieve", permission: "llm.backend.spaces.read", auditEvent: "llm.backend.space.read", pathParams: [pathParam("spaceId")], responseSchema: "LlmSpace" }));
  addPath(paths, `${P}/spaces/{spaceId}`, "patch", operation({ method: "patch", authority, operationId: "spaces.update", permission: "llm.backend.spaces.write", auditEvent: "llm.backend.space.updated", pathParams: [pathParam("spaceId")], requestSchema: "LlmSpaceRequest", responseSchema: "LlmSpace" }));
  addPath(paths, `${P}/records`, "get", operation({ method: "get", authority, operationId: "records.list", permission: "llm.backend.records.read", auditEvent: "llm.backend.record.list", queryParams: listParams(), responseSchema: "LlmRecordList" }));
  addPath(paths, `${P}/records/{recordId}`, "get", operation({ method: "get", authority, operationId: "records.retrieve", permission: "llm.backend.records.read", auditEvent: "llm.backend.record.read", pathParams: [pathParam("recordId")], responseSchema: "LlmRecord" }));
  addPath(paths, `${P}/records/{recordId}`, "patch", operation({ method: "patch", authority, operationId: "records.update", permission: "llm.backend.records.write", auditEvent: "llm.backend.record.updated", pathParams: [pathParam("recordId")], requestSchema: "LlmRecordRequest", responseSchema: "LlmRecord" }));
  addPath(paths, `${P}/records/{recordId}/supersede`, "post", operation({ method: "post", authority, operationId: "records.supersede", permission: "llm.backend.records.write", auditEvent: "llm.backend.record.superseded", pathParams: [pathParam("recordId")], requestSchema: "LlmRecordRequest", responseSchema: "LlmRecord", idempotent: true }));
  addPath(paths, `${P}/events`, "get", operation({ method: "get", authority, operationId: "events.list", permission: "llm.backend.events.read", auditEvent: "llm.backend.event.list", queryParams: listParams(), responseSchema: "LlmEventList" }));
  addPath(paths, `${P}/events/{eventId}`, "get", operation({ method: "get", authority, operationId: "events.retrieve", permission: "llm.backend.events.read", auditEvent: "llm.backend.event.read", pathParams: [pathParam("eventId")], responseSchema: "LlmEvent" }));
  addPath(paths, `${P}/candidates`, "get", operation({ method: "get", authority, operationId: "candidates.list", permission: "llm.backend.candidates.read", auditEvent: "llm.backend.candidate.list", queryParams: listParams(), responseSchema: "LlmCandidateList" }));
  addPath(paths, `${P}/candidates/{candidateId}/approve`, "post", operation({ method: "post", authority, operationId: "candidates.approve", permission: "llm.backend.candidates.write", auditEvent: "llm.backend.candidate.approved", pathParams: [pathParam("candidateId")], requestSchema: "LlmReviewRequest", responseSchema: "LlmCandidate", idempotent: true }));
  addPath(paths, `${P}/candidates/{candidateId}/reject`, "post", operation({ method: "post", authority, operationId: "candidates.reject", permission: "llm.backend.candidates.write", auditEvent: "llm.backend.candidate.rejected", pathParams: [pathParam("candidateId")], requestSchema: "LlmReviewRequest", responseSchema: "LlmCandidate", idempotent: true }));
  addPath(paths, `${P}/extraction_jobs`, "post", operation({ method: "post", authority, operationId: "extractionJobs.create", permission: "llm.backend.learning.write", auditEvent: "llm.backend.extraction_job.created", requestSchema: "LlmExtractionRequest", responseSchema: "LlmLearningJob", idempotent: true }));
  addPath(paths, `${P}/extraction_jobs/{jobId}`, "get", operation({ method: "get", authority, operationId: "extractionJobs.retrieve", permission: "llm.backend.learning.read", auditEvent: "llm.backend.extraction_job.read", pathParams: [pathParam("jobId")], responseSchema: "LlmLearningJob" }));
  addPath(paths, `${P}/consolidation_jobs`, "post", operation({ method: "post", authority, operationId: "consolidationJobs.create", permission: "llm.backend.learning.write", auditEvent: "llm.backend.consolidation_job.created", requestSchema: "LlmExtractionRequest", responseSchema: "LlmLearningJob", idempotent: true }));
  addPath(paths, `${P}/indexes`, "get", operation({ method: "get", authority, operationId: "indexes.list", permission: "llm.backend.indexes.read", auditEvent: "llm.backend.index.list", queryParams: listParams(), responseSchema: "LlmIndexList" }));
  addPath(paths, `${P}/indexes`, "post", operation({ method: "post", authority, operationId: "indexes.create", permission: "llm.backend.indexes.write", auditEvent: "llm.backend.index.created", requestSchema: "LlmIndexRequest", responseSchema: "LlmIndex", idempotent: true }));
  addPath(paths, `${P}/indexes/{indexId}`, "get", operation({ method: "get", authority, operationId: "indexes.retrieve", permission: "llm.backend.indexes.read", auditEvent: "llm.backend.index.read", pathParams: [pathParam("indexId")], responseSchema: "LlmIndex" }));
  addPath(paths, `${P}/indexes/{indexId}`, "patch", operation({ method: "patch", authority, operationId: "indexes.update", permission: "llm.backend.indexes.write", auditEvent: "llm.backend.index.updated", pathParams: [pathParam("indexId")], requestSchema: "LlmIndexRequest", responseSchema: "LlmIndex" }));
  addPath(paths, `${P}/indexes/{indexId}/rebuild`, "post", operation({ method: "post", authority, operationId: "indexes.rebuild", permission: "llm.backend.indexes.write", auditEvent: "llm.backend.index.rebuild_requested", pathParams: [pathParam("indexId")], requestSchema: "LlmReviewRequest", responseSchema: "LlmLearningJob", idempotent: true }));
  addPath(paths, `${P}/retrieval_profiles`, "get", operation({ method: "get", authority, operationId: "retrievalProfiles.list", permission: "llm.backend.retrievalProfiles.read", auditEvent: "llm.backend.retrieval_profile.list", queryParams: listParams(), responseSchema: "LlmRetrievalProfileList" }));
  addPath(paths, `${P}/retrieval_profiles`, "post", operation({ method: "post", authority, operationId: "retrievalProfiles.create", permission: "llm.backend.retrievalProfiles.write", auditEvent: "llm.backend.retrieval_profile.created", requestSchema: "LlmRetrievalProfileRequest", responseSchema: "LlmRetrievalProfile", idempotent: true }));
  addPath(paths, `${P}/retrieval_profiles/{profileId}`, "get", operation({ method: "get", authority, operationId: "retrievalProfiles.retrieve", permission: "llm.backend.retrievalProfiles.read", auditEvent: "llm.backend.retrieval_profile.read", pathParams: [pathParam("profileId")], responseSchema: "LlmRetrievalProfile" }));
  addPath(paths, `${P}/retrieval_profiles/{profileId}`, "patch", operation({ method: "patch", authority, operationId: "retrievalProfiles.update", permission: "llm.backend.retrievalProfiles.write", auditEvent: "llm.backend.retrieval_profile.updated", pathParams: [pathParam("profileId")], requestSchema: "LlmRetrievalProfileRequest", responseSchema: "LlmRetrievalProfile" }));
  addPath(paths, `${P}/implementation_profiles`, "get", operation({ method: "get", authority, operationId: "implementationProfiles.list", permission: "llm.backend.implementationProfiles.read", auditEvent: "llm.backend.implementation_profile.list", queryParams: listParams(), responseSchema: "LlmImplementationProfileList" }));
  addPath(paths, `${P}/implementation_profiles`, "post", operation({ method: "post", authority, operationId: "implementationProfiles.create", permission: "llm.backend.implementationProfiles.write", auditEvent: "llm.backend.implementation_profile.created", requestSchema: "LlmImplementationProfileRequest", responseSchema: "LlmImplementationProfile", idempotent: true }));
  addPath(paths, `${P}/implementation_profiles/{implementationProfileId}`, "get", operation({ method: "get", authority, operationId: "implementationProfiles.retrieve", permission: "llm.backend.implementationProfiles.read", auditEvent: "llm.backend.implementation_profile.read", pathParams: [pathParam("implementationProfileId")], responseSchema: "LlmImplementationProfile" }));
  addPath(paths, `${P}/implementation_profiles/{implementationProfileId}`, "patch", operation({ method: "patch", authority, operationId: "implementationProfiles.update", permission: "llm.backend.implementationProfiles.write", auditEvent: "llm.backend.implementation_profile.updated", pathParams: [pathParam("implementationProfileId")], requestSchema: "LlmImplementationProfileRequest", responseSchema: "LlmImplementationProfile" }));
  addPath(paths, `${P}/provider_bindings`, "get", operation({ method: "get", authority, operationId: "providerBindings.list", permission: "llm.backend.providerBindings.read", auditEvent: "llm.backend.provider_binding.list", queryParams: listParams(), responseSchema: "LlmProviderBindingList" }));
  addPath(paths, `${P}/provider_bindings`, "post", operation({ method: "post", authority, operationId: "providerBindings.create", permission: "llm.backend.providerBindings.write", auditEvent: "llm.backend.provider_binding.created", requestSchema: "LlmProviderBindingRequest", responseSchema: "LlmProviderBinding", idempotent: true }));
  addPath(paths, `${P}/provider_bindings/{providerBindingId}`, "patch", operation({ method: "patch", authority, operationId: "providerBindings.update", permission: "llm.backend.providerBindings.write", auditEvent: "llm.backend.provider_binding.updated", pathParams: [pathParam("providerBindingId")], requestSchema: "LlmProviderBindingRequest", responseSchema: "LlmProviderBinding" }));
  addPath(paths, `${P}/provider_health`, "get", operation({ method: "get", authority, operationId: "providerHealth.retrieve", permission: "llm.backend.providerHealth.read", auditEvent: "llm.backend.provider_health.read", responseSchema: "LlmProviderHealth" }));
  addPath(paths, `${P}/eval_runs`, "get", operation({ method: "get", authority, operationId: "evalRuns.list", permission: "llm.backend.evalRuns.read", auditEvent: "llm.backend.eval_run.list", queryParams: listParams(), responseSchema: "LlmEvalRunList" }));
  addPath(paths, `${P}/eval_runs`, "post", operation({ method: "post", authority, operationId: "evalRuns.create", permission: "llm.backend.evalRuns.write", auditEvent: "llm.backend.eval_run.created", requestSchema: "LlmEvalRunRequest", responseSchema: "LlmEvalRun", idempotent: true }));
  addPath(paths, `${P}/eval_runs/{evalRunId}`, "get", operation({ method: "get", authority, operationId: "evalRuns.retrieve", permission: "llm.backend.evalRuns.read", auditEvent: "llm.backend.eval_run.read", pathParams: [pathParam("evalRunId")], responseSchema: "LlmEvalRun" }));
  addPath(paths, `${P}/retrieval_traces`, "get", operation({ method: "get", authority, operationId: "retrievalTraces.list", permission: "llm.backend.retrievalTraces.read", auditEvent: "llm.backend.retrieval_trace.list", queryParams: listParams(), responseSchema: "LlmRetrievalTraceList" }));
  addPath(paths, `${P}/retrieval_traces/{traceId}`, "get", operation({ method: "get", authority, operationId: "retrievalTraces.retrieve", permission: "llm.backend.retrievalTraces.read", auditEvent: "llm.backend.retrieval_trace.read", pathParams: [pathParam("traceId")], responseSchema: "LlmRetrievalTrace" }));
  addPath(paths, `${P}/audit_logs`, "get", operation({ method: "get", authority, operationId: "auditLogs.list", permission: "llm.backend.auditLogs.read", auditEvent: "llm.backend.audit_log.list", queryParams: listParams(), responseSchema: "LlmAuditLogList" }));
  addPath(paths, `${P}/retention_jobs`, "post", operation({ method: "post", authority, operationId: "retentionJobs.create", permission: "llm.backend.retention.write", auditEvent: "llm.backend.retention_job.created", requestSchema: "LlmRetentionJobRequest", responseSchema: "LlmLearningJob", idempotent: true }));
  addPath(paths, `${P}/migration_jobs`, "post", operation({ method: "post", authority, operationId: "migrationJobs.create", permission: "llm.backend.migrations.write", auditEvent: "llm.backend.migration_job.created", requestSchema: "LlmMigrationJobRequest", responseSchema: "LlmLearningJob", idempotent: true }));
  addPath(paths, `${P}/migration_jobs/{migrationJobId}`, "get", operation({ method: "get", authority, operationId: "migrationJobs.retrieve", permission: "llm.backend.migrations.read", auditEvent: "llm.backend.migration_job.read", pathParams: [pathParam("migrationJobId")], responseSchema: "LlmLearningJob" }));

  writeJson("sdks/sdkwork-llm-backend-sdk/openapi/llm-backend-api.openapi.json", createOpenApi({
    title: "SDKWork LLM Backend API",
    authority,
    prefix: "/backend/v3/api",
    sdkFamily: "sdkwork-llm-backend-sdk",
    paths
  }));
}

const routeSurfaceProfiles = [
  {
    surface: "open-api",
    openapiPath: "sdks/sdkwork-llm-sdk/openapi/llm-open-api.openapi.json",
    apisPath: "apis/open-api/llm-open-api.openapi.json",
    packageName: "sdkwork-router-llm-open-api",
    crateDir: "crates/sdkwork-router-llm-open-api",
    crateImport: "sdkwork_router_llm_open_api",
    manifestFn: "open_route_manifest",
    apiAuthority: "sdkwork-llm-open-api",
    sdkFamily: "sdkwork-llm-sdk",
    prefix: llmOpenApiPrefix,
    routeManifestDir: "sdks/_route-manifests/open-api",
    routeManifestFile: "sdkwork-router-llm-open-api.route-manifest.json"
  },
  {
    surface: "app-api",
    openapiPath: "sdks/sdkwork-llm-app-sdk/openapi/llm-app-api.openapi.json",
    apisPath: "apis/app-api/llm-app-api.openapi.json",
    packageName: "sdkwork-router-llm-app-api",
    crateDir: "crates/sdkwork-router-llm-app-api",
    crateImport: "sdkwork_router_llm_app_api",
    manifestFn: "app_route_manifest",
    apiAuthority: "sdkwork-llm.app",
    sdkFamily: "sdkwork-llm-app-sdk",
    prefix: "/app/v3/api",
    routeManifestDir: "sdks/_route-manifests/app-api",
    routeManifestFile: "sdkwork-router-llm-app-api.route-manifest.json"
  },
  {
    surface: "backend-api",
    openapiPath: "sdks/sdkwork-llm-backend-sdk/openapi/llm-backend-api.openapi.json",
    apisPath: "apis/backend-api/llm-backend-api.openapi.json",
    packageName: "sdkwork-router-llm-backend-api",
    crateDir: "crates/sdkwork-router-llm-backend-api",
    crateImport: "sdkwork_router_llm_backend_api",
    manifestFn: "backend_route_manifest",
    apiAuthority: "sdkwork-llm.backend",
    sdkFamily: "sdkwork-llm-backend-sdk",
    prefix: "/backend/v3/api",
    routeManifestDir: "sdks/_route-manifests/backend-api",
    routeManifestFile: "sdkwork-router-llm-backend-api.route-manifest.json"
  }
];

function readJsonFromDisk(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(root, relativePath), "utf8"));
}

function extractRoutesFromOpenApi(openapi) {
  const routes = [];
  for (const [pathKey, pathItem] of Object.entries(openapi.paths ?? {})) {
    for (const [method, operation] of Object.entries(pathItem ?? {})) {
      if (!["get", "post", "patch", "delete"].includes(method)) {
        continue;
      }
      routes.push({
        method: method.toUpperCase(),
        path: pathKey,
        operationId: operation.operationId,
        tags: operation.tags ?? ["llm"],
        authMode: operation["x-sdkwork-auth-mode"],
        apiSurface: operation["x-sdkwork-api-surface"],
        apiAuthority: operation["x-sdkwork-api-authority"]
      });
    }
  }
  return routes;
}

function httpRouteAuthHelper(authMode) {
  return authMode === "api-key" ? "api_key" : "dual_token";
}

function httpMethodRust(method) {
  const map = { GET: "Get", POST: "Post", PATCH: "Patch", DELETE: "Delete" };
  return map[method];
}

function writeHttpRouteManifestRust(crateDir, fnName, routes) {
  const lines = [
    "// @generated by tools/materialize_phase1_contracts.mjs — do not edit",
    "",
    "use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};",
    "",
    "const HTTP_ROUTES: &[HttpRoute] = &["
  ];
  for (const route of routes) {
    const auth = httpRouteAuthHelper(route.authMode);
    lines.push(`    HttpRoute::${auth}(`);
    lines.push(`        HttpMethod::${httpMethodRust(route.method)},`);
    lines.push(`        "${route.path}",`);
    lines.push(`        "${route.tags[0] ?? "llm"}",`);
    lines.push(`        "${route.operationId}",`);
    lines.push("    ),");
  }
  lines.push(
    "];",
    "",
    `pub fn ${fnName}() -> HttpRouteManifest {`,
    "    HttpRouteManifest::new(HTTP_ROUTES)",
    "}",
    ""
  );
  writeText(`${crateDir}/src/http_route_manifest.rs`, lines.join("\n"));
}

function writeRouteManifestJson(profile, routes) {
  writeJson(`${profile.routeManifestDir}/${profile.routeManifestFile}`, {
    schemaVersion: 1,
    kind: "sdkwork.route.manifest",
    packageName: profile.packageName,
    surface: profile.surface,
    owner,
    domain,
    capability,
    apiAuthority: profile.apiAuthority,
    sdkFamily: profile.sdkFamily,
    prefix: profile.prefix,
    source: {
      crateRoot: profile.crateDir,
      crateImport: profile.crateImport,
      openApiAuthority: profile.openapiPath
    },
    routes: routes.map((route) => ({
      method: route.method,
      path: route.path,
      operationId: route.operationId,
      tags: route.tags,
      auth: {
        mode: route.authMode,
        required: true
      },
      handler: {
        module: "crate::routes",
        name: null
      },
      ownership: {
        owner,
        apiAuthority: route.apiAuthority
      },
      requestContext: "WebRequestContext",
      apiSurface: route.apiSurface
    }))
  });
}

function mirrorApisOpenApi(profiles) {
  for (const profile of profiles) {
    const content = fs.readFileSync(path.join(root, profile.openapiPath), "utf8");
    writeText(profile.apisPath, content);
  }
}

function writeRouteArtifacts() {
  for (const profile of routeSurfaceProfiles) {
    const openapi = readJsonFromDisk(profile.openapiPath);
    const routes = extractRoutesFromOpenApi(openapi);
    writeHttpRouteManifestRust(profile.crateDir, profile.manifestFn, routes);
    writeRouteManifestJson(profile, routes);
    console.log(`materialized ${routes.length} routes for ${profile.packageName}`);
  }
  mirrorApisOpenApi(routeSurfaceProfiles);
  writeJson("apis/authority-manifest.json", {
    schemaVersion: 1,
    kind: "sdkwork.api.authority.manifest",
    surfaces: routeSurfaceProfiles.map((profile) => ({
      authorityPath: profile.apisPath,
      sdkPath: profile.openapiPath
    }))
  });
}

function writeVerification() {
  writeText("tools/verify_phase1.ps1", `$ErrorActionPreference = "Stop"

function Read-JsonFile {
    param([Parameter(Mandatory = $true)][string]$Path)
    if (!(Test-Path $Path)) {
        throw "Missing required file: $Path"
    }
    return Get-Content -Raw $Path | ConvertFrom-Json
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Content,
        [Parameter(Mandatory = $true)][string]$Needle,
        [Parameter(Mandatory = $true)][string]$Path
    )
    if (!$Content.Contains($Needle)) {
        throw "$Path must contain: $Needle"
    }
}

$requiredFiles = @(
    "AGENTS.md",
    "CODEX.md",
    "CLAUDE.md",
    "GEMINI.md",
    ".sdkwork/README.md",
    ".sdkwork/skills/README.md",
    ".sdkwork/plugins/README.md",
    "sdkwork.app.config.json",
    "specs/README.md",
    "specs/component.spec.json",
    "docs/superpowers/specs/2026-06-10-ai-llm-architecture-design.md",
    "docs/superpowers/specs/2026-06-10-llm-spi-plugin-architecture-design.md",
    "docs/schema-registry/README.md",
    "docs/schema-registry/tables/001-llm-core.yaml",
    "docs/schema-registry/tables/002-llm-learning.yaml",
    "docs/schema-registry/tables/003-llm-retrieval.yaml",
    "docs/schema-registry/tables/004-llm-provider.yaml",
    "docs/schema-registry/tables/005-llm-governance.yaml",
    "sdks/README.md",
    "sdks/sdkwork-llm-sdk/README.md",
    "sdks/sdkwork-llm-sdk/.sdkwork-assembly.json",
    "sdks/sdkwork-llm-sdk/sdk-manifest.json",
    "sdks/sdkwork-llm-sdk/specs/README.md",
    "sdks/sdkwork-llm-sdk/specs/component.spec.json",
    "sdks/sdkwork-llm-sdk/openapi/llm-open-api.openapi.json",
    "sdks/sdkwork-llm-app-sdk/README.md",
    "sdks/sdkwork-llm-app-sdk/.sdkwork-assembly.json",
    "sdks/sdkwork-llm-app-sdk/sdk-manifest.json",
    "sdks/sdkwork-llm-app-sdk/specs/README.md",
    "sdks/sdkwork-llm-app-sdk/specs/component.spec.json",
    "sdks/sdkwork-llm-app-sdk/openapi/llm-app-api.openapi.json",
    "sdks/sdkwork-llm-backend-sdk/README.md",
    "sdks/sdkwork-llm-backend-sdk/.sdkwork-assembly.json",
    "sdks/sdkwork-llm-backend-sdk/sdk-manifest.json",
    "sdks/sdkwork-llm-backend-sdk/specs/README.md",
    "sdks/sdkwork-llm-backend-sdk/specs/component.spec.json",
    "sdks/sdkwork-llm-backend-sdk/openapi/llm-backend-api.openapi.json",
    "apis/authority-manifest.json",
    "apis/open-api/llm-open-api.openapi.json",
    "apis/app-api/llm-app-api.openapi.json",
    "apis/backend-api/llm-backend-api.openapi.json",
    "sdks/_route-manifests/open-api/sdkwork-router-llm-open-api.route-manifest.json",
    "sdks/_route-manifests/app-api/sdkwork-router-llm-app-api.route-manifest.json",
    "sdks/_route-manifests/backend-api/sdkwork-router-llm-backend-api.route-manifest.json",
    "package.json",
    "sdkwork.workflow.json",
    ".github/workflows/package.yml"
)

foreach ($file in $requiredFiles) {
    if (!(Test-Path $file)) {
        throw "Missing required phase1 contract artifact: $file"
    }
}

foreach ($forbidden in @("sdks/sdkwork-llm-open-api", "sdks/sdkwork-llm-app-api", "sdks/sdkwork-llm-backend-api", "sdks/memory-open-sdk", "sdks/memory-app-sdk", "sdks/memory-backend-sdk")) {
    if (Test-Path $forbidden) {
        throw "Forbidden SDK/API authority directory exists: $forbidden"
    }
}

$appConfig = Read-JsonFile "sdkwork.app.config.json"
if ($appConfig.schemaVersion -ne 3 -or $appConfig.kind -ne "sdkwork.app") {
    throw "sdkwork.app.config.json must use SDKWork app manifest v3"
}
if ($appConfig.app.key -ne "sdkwork-llm") {
    throw "sdkwork.app.config.json app.key must be sdkwork-llm"
}

$rootSpec = Read-JsonFile "specs/component.spec.json"
if ($rootSpec.component.name -ne "sdkwork-llm") {
    throw "Root component spec must identify sdkwork-llm"
}
if ($rootSpec.component.domain -ne "intelligence" -or $rootSpec.component.capability -ne "llm") {
    throw "Root component spec must use domain=intelligence and capability=llm"
}
if (!$rootSpec.contracts.sdkDependencies -or $rootSpec.contracts.sdkDependencies.Count -eq 0) {
    throw "Root component spec must declare sdkDependencies"
}
if ($null -eq $rootSpec.contracts.dependencyApiExports) {
    throw "Root component spec must explicitly declare dependencyApiExports"
}
if ($null -eq $rootSpec.contracts.dependencyApiSurfaces) {
    throw "Root component spec must explicitly declare dependencyApiSurfaces"
}

foreach ($family in @(
    @{ Path = "sdks/sdkwork-llm-sdk"; Authority = "sdkwork-llm-open-api"; Prefix = "${llmOpenApiPrefix}"; SchemaUrl = "${llmOpenApiSchemaUrl}"; Spec = "openapi/llm-open-api.openapi.json"; Client = "SdkworkLlmOpenClient" },
    @{ Path = "sdks/sdkwork-llm-app-sdk"; Authority = "sdkwork-llm.app"; Prefix = "/app/v3/api"; SchemaUrl = "/app/v3/openapi.json"; Spec = "openapi/llm-app-api.openapi.json"; Client = "SdkworkLlmAppClient" },
    @{ Path = "sdks/sdkwork-llm-backend-sdk"; Authority = "sdkwork-llm.backend"; Prefix = "/backend/v3/api"; SchemaUrl = "/backend/v3/openapi.json"; Spec = "openapi/llm-backend-api.openapi.json"; Client = "SdkworkLlmBackendClient" }
)) {
    $assembly = Read-JsonFile (Join-Path $family.Path ".sdkwork-assembly.json")
    $manifest = Read-JsonFile (Join-Path $family.Path "sdk-manifest.json")
    $component = Read-JsonFile (Join-Path $family.Path "specs/component.spec.json")

    if ($assembly.sdkOwner -ne "sdkwork-llm") {
        throw "$($family.Path) assembly sdkOwner mismatch"
    }
    if ($assembly.apiAuthority -ne $family.Authority -or $manifest.apiAuthority -ne $family.Authority) {
        throw "$($family.Path) apiAuthority mismatch"
    }
    if ($assembly.generationInputSpec -ne $family.Spec -or $manifest.generationInputSpec -ne $family.Spec) {
        throw "$($family.Path) generationInputSpec mismatch"
    }
    if ($assembly.discoverySurface.apiPrefix -ne $family.Prefix -or $manifest.apiPrefix -ne $family.Prefix) {
        throw "$($family.Path) apiPrefix mismatch"
    }
    if ($assembly.discoverySurface.schemaUrl -ne $family.SchemaUrl) {
        throw "$($family.Path) schemaUrl mismatch"
    }
    if ($null -eq $component.contracts.sdkDependencies) {
        throw "$($family.Path) component spec must declare sdkDependencies"
    }
    if ($null -eq $component.contracts.dependencyApiExports) {
        throw "$($family.Path) component spec must explicitly declare dependencyApiExports"
    }
    if (!$component.contracts.sdkClients.Contains($family.Client)) {
        throw "$($family.Path) component spec must declare client $($family.Client)"
    }
}

function Verify-OpenApi {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Prefix,
        [Parameter(Mandatory = $true)][string]$Authority,
        [Parameter(Mandatory = $true)][string]$SdkFamily,
        [Parameter(Mandatory = $true)][ValidateSet("dual-token", "api-key")][string]$AuthMode,
        [Parameter(Mandatory = $true)][ValidateSet("open-api", "app-api", "backend-api")][string]$ExpectedApiSurface,
        [Parameter(Mandatory = $true)][string[]]$RequiredOperationIds,
        [Parameter(Mandatory = $true)][string[]]$RequiredSchemas
    )

    $spec = Read-JsonFile $Path
    if (!$spec.openapi.StartsWith("3.1.")) {
        throw "$Path must use OpenAPI 3.1.x"
    }
    if ($spec.'x-sdkwork-owner' -ne "sdkwork-llm" -or $spec.'x-sdkwork-api-authority' -ne $Authority -or $spec.'x-sdkwork-sdk-family' -ne $SdkFamily) {
        throw "$Path root SDKWork ownership metadata mismatch"
    }
    if (!$spec.components -or !$spec.components.schemas -or !$spec.components.securitySchemes) {
        throw "$Path must define schemas and securitySchemes"
    }
    $authToken = $spec.components.securitySchemes.AuthToken
    $accessToken = $spec.components.securitySchemes.AccessToken
    $apiKey = $spec.components.securitySchemes.ApiKey
    if ($AuthMode -eq "dual-token") {
        if (!$authToken -or $authToken.type -ne "http" -or $authToken.scheme -ne "bearer") {
            throw "$Path must define AuthToken as http bearer"
        }
        if (!$accessToken -or $accessToken.type -ne "apiKey" -or $accessToken.in -ne "header" -or $accessToken.name -ne "Access-Token") {
            throw "$Path must define AccessToken as Access-Token apiKey header"
        }
        if ($apiKey) {
            throw "$Path dual-token API must not declare ApiKey security scheme"
        }
    }
    if ($AuthMode -eq "api-key") {
        if (!$apiKey -or $apiKey.type -ne "apiKey" -or $apiKey.in -ne "header" -or $apiKey.name -ne "X-API-Key") {
            throw "$Path must define ApiKey as X-API-Key apiKey header"
        }
        if ($authToken -or $accessToken) {
            throw "$Path open API must not declare app/backend token security schemes"
        }
    }

    $operationIds = New-Object System.Collections.Generic.HashSet[string]
    foreach ($pathProperty in $spec.paths.PSObject.Properties) {
        if (!$pathProperty.Name.StartsWith($Prefix)) {
            throw "$Path contains non-canonical path prefix: $($pathProperty.Name)"
        }
        if ($AuthMode -eq "api-key" -and ($pathProperty.Name.StartsWith("/app/v3/api") -or $pathProperty.Name.StartsWith("/backend/v3/api"))) {
            throw "$Path open API must not use app/backend prefix: $($pathProperty.Name)"
        }
        if (($AuthMode -eq "api-key" -or $Prefix -eq "/backend/v3/api") -and $pathProperty.Name -match "/auth|/login|/sessions|/refresh|/logout") {
            throw "$Path backend/open API must not expose auth/session path: $($pathProperty.Name)"
        }

        foreach ($methodProperty in $pathProperty.Value.PSObject.Properties) {
            $methodName = [string]$methodProperty.Name
            if ($methodName -notin @("get", "post", "patch", "delete")) {
                continue
            }
            $operation = $methodProperty.Value
            $operationId = [string]$operation.operationId
            if ([string]::IsNullOrWhiteSpace($operationId)) {
                throw "$Path operation missing operationId at $($pathProperty.Name)"
            }
            if ($operationId.Contains("_") -or $operationId.Contains("__")) {
                throw "$Path operationId must use dotted lowerCamelCase style: $operationId"
            }
            [void]$operationIds.Add($operationId)
            if ($operation.'x-sdkwork-owner' -ne "sdkwork-llm" -or $operation.'x-sdkwork-api-authority' -ne $Authority) {
                throw "$Path operation ownership mismatch: $operationId"
            }
            if (!$operation.'x-sdkwork-permission' -or !$operation.'x-sdkwork-audit-event' -or !$operation.'x-sdkwork-auth-mode') {
                throw "$Path operation missing permission/audit/auth metadata: $operationId"
            }
            if ($operation.'x-sdkwork-auth-mode' -ne $AuthMode) {
                throw "$Path operation auth mode mismatch for $operationId"
            }
            if ($operation.'x-sdkwork-api-surface' -ne $ExpectedApiSurface) {
                throw "$Path operation api surface mismatch for $operationId"
            }
            if ($operation.'x-sdkwork-request-context' -ne "WebRequestContext") {
                throw "$Path operation must declare WebRequestContext for $operationId"
            }
            $security = $operation.security
            if (!$security -or $security.Count -eq 0) {
                throw "$Path operation missing security declaration: $operationId"
            }
            $firstSecurity = $security[0]
            if ($AuthMode -eq "dual-token" -and (!$firstSecurity.PSObject.Properties["AuthToken"] -or !$firstSecurity.PSObject.Properties["AccessToken"])) {
                throw "$Path operation must require both AuthToken and AccessToken: $operationId"
            }
            if ($AuthMode -eq "api-key" -and !$firstSecurity.PSObject.Properties["ApiKey"]) {
                throw "$Path operation must require ApiKey: $operationId"
            }
            if ($AuthMode -eq "api-key" -and ($firstSecurity.PSObject.Properties["AuthToken"] -or $firstSecurity.PSObject.Properties["AccessToken"])) {
                throw "$Path open API operation must not require app/backend tokens: $operationId"
            }
            foreach ($errorStatus in @("400", "404")) {
                $responseProperty = $operation.responses.PSObject.Properties[$errorStatus]
                if ($responseProperty) {
                    $content = $responseProperty.Value.content
                    if (!$content -or !$content.PSObject.Properties["application/problem+json"]) {
                        throw "$Path error response $errorStatus must include application/problem+json: $operationId"
                    }
                }
            }
            if ($pathProperty.Name.Contains("{") -and (!$operation.parameters -or $operation.parameters.Count -eq 0)) {
                throw "$Path operation with path parameter has no parameters: $operationId"
            }
            if (($methodName -eq "post" -or $methodName -eq "patch") -and !$operation.requestBody) {
                throw "$Path mutating operation has no requestBody: $operationId"
            }
            if ($methodName -eq "post" -and $operation.'x-sdkwork-idempotent') {
                $hasIdempotency = $false
                foreach ($parameter in $operation.parameters) {
                    if ($parameter.name -eq "Idempotency-Key" -and $parameter.in -eq "header") {
                        $hasIdempotency = $true
                    }
                }
                if (!$hasIdempotency) {
                    throw "$Path idempotent POST missing Idempotency-Key header: $operationId"
                }
            }
        }
    }

    foreach ($requiredId in $RequiredOperationIds) {
        if (!$operationIds.Contains($requiredId)) {
            throw "$Path missing required operationId: $requiredId"
        }
    }

    foreach ($schemaName in $RequiredSchemas) {
        if (!$spec.components.schemas.PSObject.Properties[$schemaName]) {
            throw "$Path missing required schema: $schemaName"
        }
    }

    Write-Host "Verified $Path with $($operationIds.Count) operations."
}

$appOpenApiCheck = @{
    Path = "sdks/sdkwork-llm-app-sdk/openapi/llm-app-api.openapi.json"
    Prefix = "/app/v3/api"
    Authority = "sdkwork-llm.app"
    SdkFamily = "sdkwork-llm-app-sdk"
    AuthMode = "dual-token"
    ExpectedApiSurface = "app-api"
    RequiredOperationIds = @(
        "spaces.create", "spaces.list", "spaces.retrieve", "spaces.update",
        "events.create", "events.retrieve",
        "records.create", "records.list", "records.retrieve", "records.update", "records.delete", "records.sources.list",
        "forgetRequests.create", "forgetRequests.retrieve",
        "extractions.create",
        "candidates.list", "candidates.retrieve", "candidates.approve", "candidates.reject",
        "habits.list", "habits.retrieve", "habits.update", "habits.confirm", "habits.reject",
        "retrievals.create", "retrievals.retrieve",
        "contextPacks.create", "contextPacks.retrieve",
        "feedback.create",
        "exportJobs.create", "exportJobs.retrieve",
        "learningSettings.retrieve", "learningSettings.update"
    )
    RequiredSchemas = @(
        "ProblemDetails", "LlmSpace", "LlmEvent", "LlmRecord", "LlmCandidate", "LlmHabit",
        "LlmRetrievalRequest", "LlmRetrievalResult", "LlmContextPackRequest", "LlmContextPack",
        "LlmLearningSettings", "LlmForgetJob", "LlmExportJob"
    )
}
Verify-OpenApi @appOpenApiCheck

$openApiCheck = @{
    Path = "sdks/sdkwork-llm-sdk/openapi/llm-open-api.openapi.json"
    Prefix = "${llmOpenApiPrefix}"
    Authority = "sdkwork-llm-open-api"
    SdkFamily = "sdkwork-llm-sdk"
    AuthMode = "api-key"
    ExpectedApiSurface = "open-api"
    RequiredOperationIds = @(
        "capabilities.retrieve",
        "events.create", "events.retrieve",
        "records.create", "records.list", "records.retrieve", "records.update", "records.delete",
        "retrievals.create", "retrievals.retrieve",
        "contextPacks.create", "contextPacks.retrieve",
        "feedback.create",
        "extractions.create",
        "candidates.list", "candidates.retrieve",
        "providerHealth.retrieve"
    )
    RequiredSchemas = @(
        "ProblemDetails", "LlmCapabilities", "LlmEvent", "LlmRecord",
        "LlmRetrievalRequest", "LlmRetrievalResult", "LlmContextPackRequest", "LlmContextPack",
        "LlmFeedbackRequest", "LlmFeedback", "LlmExtractionRequest", "LlmLearningJob",
        "LlmCandidate", "LlmProviderHealth"
    )
}
Verify-OpenApi @openApiCheck

$backendOpenApiCheck = @{
    Path = "sdks/sdkwork-llm-backend-sdk/openapi/llm-backend-api.openapi.json"
    Prefix = "/backend/v3/api"
    Authority = "sdkwork-llm.backend"
    SdkFamily = "sdkwork-llm-backend-sdk"
    AuthMode = "dual-token"
    ExpectedApiSurface = "backend-api"
    RequiredOperationIds = @(
        "spaces.list", "spaces.retrieve", "spaces.update",
        "records.list", "records.retrieve", "records.update", "records.supersede",
        "events.list", "events.retrieve",
        "candidates.list", "candidates.approve", "candidates.reject",
        "extractionJobs.create", "extractionJobs.retrieve", "consolidationJobs.create",
        "indexes.create", "indexes.list", "indexes.retrieve", "indexes.update", "indexes.rebuild",
        "retrievalProfiles.create", "retrievalProfiles.list", "retrievalProfiles.retrieve", "retrievalProfiles.update",
        "implementationProfiles.create", "implementationProfiles.list", "implementationProfiles.retrieve", "implementationProfiles.update",
        "providerBindings.create", "providerBindings.list", "providerBindings.update",
        "providerHealth.retrieve",
        "evalRuns.create", "evalRuns.list", "evalRuns.retrieve",
        "retrievalTraces.list", "retrievalTraces.retrieve",
        "auditLogs.list", "retentionJobs.create", "migrationJobs.create", "migrationJobs.retrieve"
    )
    RequiredSchemas = @(
        "ProblemDetails", "LlmIndex", "LlmRetrievalProfile", "LlmImplementationProfile",
        "LlmProviderBinding", "LlmProviderHealth", "LlmEvalRun", "LlmAuditLog",
        "LlmMigrationJobRequest", "LlmRetentionJobRequest"
    )
}
Verify-OpenApi @backendOpenApiCheck

foreach ($routeManifestPath in @(
    "sdks/_route-manifests/open-api/sdkwork-router-llm-open-api.route-manifest.json",
    "sdks/_route-manifests/app-api/sdkwork-router-llm-app-api.route-manifest.json",
    "sdks/_route-manifests/backend-api/sdkwork-router-llm-backend-api.route-manifest.json"
)) {
    $routeManifest = Read-JsonFile $routeManifestPath
    if ($routeManifest.kind -ne "sdkwork.route.manifest") {
        throw "$routeManifestPath must use sdkwork.route.manifest kind"
    }
    foreach ($route in $routeManifest.routes) {
        if ($route.requestContext -ne "WebRequestContext") {
            throw "$routeManifestPath route $($route.method) $($route.path) must declare WebRequestContext"
        }
        if ($route.apiSurface -notin @("open-api", "app-api", "backend-api")) {
            throw "$routeManifestPath route $($route.method) $($route.path) must declare canonical apiSurface"
        }
    }
    Write-Host "Verified $routeManifestPath with $($routeManifest.routes.Count) routes."
}

foreach ($schemaPath in Get-ChildItem -Path "docs/schema-registry/tables" -Filter "*.yaml") {
    $content = Get-Content -Raw $schemaPath.FullName
    Assert-Contains -Content $content -Needle "module: memory" -Path $schemaPath.FullName
    Assert-Contains -Content $content -Needle "owner: sdkwork-llm" -Path $schemaPath.FullName
    Assert-Contains -Content $content -Needle "table: llm_" -Path $schemaPath.FullName
}

$allSchemaText = (Get-ChildItem -Path "docs/schema-registry/tables" -Filter "*.yaml" | ForEach-Object { Get-Content -Raw $_.FullName }) -join [Environment]::NewLine
foreach ($requiredTable in @(
    "llm_space", "llm_event", "llm_record", "llm_record_source", "llm_entity", "llm_edge",
    "llm_candidate", "llm_habit", "llm_habit_signal", "llm_learning_job",
    "llm_index", "llm_index_entry", "llm_retrieval_profile", "llm_retrieval_trace", "llm_retrieval_hit", "llm_context_pack",
    "llm_implementation_profile", "llm_provider_binding", "llm_policy",
    "llm_audit_log", "llm_eval_run", "llm_outbox_event"
)) {
    if (!$allSchemaText.Contains("table: $requiredTable")) {
        throw "Schema registry missing required table: $requiredTable"
    }
}

$design = Get-Content -Raw "docs/superpowers/specs/2026-06-10-ai-llm-architecture-design.md"
foreach ($snippet in @(
    "Embedding Optional",
    "Multi-Implementation Abstraction",
    "Open API Contract Draft",
    "App API Contract Draft",
    "Backend API Contract Draft",
    "Database And Storage Design",
    "llm_"
)) {
    Assert-Contains -Content $design -Needle $snippet -Path "docs/superpowers/specs/2026-06-10-ai-llm-architecture-design.md"
}

$spiDesign = Get-Content -Raw "docs/superpowers/specs/2026-06-10-llm-spi-plugin-architecture-design.md"
foreach ($snippet in @(
    "LlmPluginManifest",
    "LlmRuntimePlugin",
    "LlmCoreRuntime",
    "Stable Core And Plugin Boundaries",
    "SPI Port Families",
    "Runtime Plugin Manifest",
    "Built-In Plugin Families",
    "Conformance suite",
    "0.1.0 Implementation Decisions",
    "Static Rust registration",
    "JSON manifest plus Rust constant",
    "Runtime plugins are not Codex agent plugins",
    'Do not place runtime LLM plugins under \`.sdkwork/plugins/\`',
    "Industry References"
)) {
    Assert-Contains -Content $spiDesign -Needle $snippet -Path "docs/superpowers/specs/2026-06-10-llm-spi-plugin-architecture-design.md"
}

if ($spiDesign.Contains("## 17. Open Decisions")) {
    throw "SPI design must resolve first-landing open decisions before runtime implementation starts."
}

Write-Host "SDKWork LLM phase1 contract verification passed."
`);
}

writeAgentEntrypoints();
writeAppManifest();
writeRootSpecs();
writeSdkFamilies();
writeSchemaRegistry();
writeOpenApi();
writeAppOpenApi();
writeBackendOpenApi();
writeRouteArtifacts();
writeVerification();
