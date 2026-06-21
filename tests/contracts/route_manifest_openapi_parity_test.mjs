import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), 'utf8'));
}

function countOpenApiOperations(openapiPath) {
  const openapi = readJson(openapiPath);
  let count = 0;
  for (const pathItem of Object.values(openapi.paths ?? {})) {
    for (const method of Object.keys(pathItem ?? {})) {
      if (['get', 'post', 'patch', 'delete'].includes(method)) {
        count += 1;
      }
    }
  }
  return count;
}

const surfaces = [
  {
    openapiPath: 'sdks/sdkwork-llm-sdk/openapi/llm-open-api.openapi.json',
    routeManifestPath: 'sdks/_route-manifests/open-api/sdkwork-router-llm-open-api.route-manifest.json',
    httpRouteManifestPath: 'crates/sdkwork-router-llm-open-api/src/http_route_manifest.rs',
  },
  {
    openapiPath: 'sdks/sdkwork-llm-app-sdk/openapi/llm-app-api.openapi.json',
    routeManifestPath: 'sdks/_route-manifests/app-api/sdkwork-router-llm-app-api.route-manifest.json',
    httpRouteManifestPath: 'crates/sdkwork-router-llm-app-api/src/http_route_manifest.rs',
  },
  {
    openapiPath: 'sdks/sdkwork-llm-backend-sdk/openapi/llm-backend-api.openapi.json',
    routeManifestPath: 'sdks/_route-manifests/backend-api/sdkwork-router-llm-backend-api.route-manifest.json',
    httpRouteManifestPath: 'crates/sdkwork-router-llm-backend-api/src/http_route_manifest.rs',
  },
];

for (const surface of surfaces) {
  test(`${surface.openapiPath} route manifest parity`, () => {
    const openapiCount = countOpenApiOperations(surface.openapiPath);
    const routeManifest = readJson(surface.routeManifestPath);
    assert.equal(routeManifest.routes.length, openapiCount);
    const rustManifest = fs.readFileSync(path.join(repoRoot, surface.httpRouteManifestPath), 'utf8');
    const rustRouteCount = (rustManifest.match(/HttpRoute::/g) ?? []).length;
    assert.equal(rustRouteCount, openapiCount);
  });
}

test('apis authority manifest mirrors sdk openapi paths', () => {
  const authorityManifest = readJson('apis/authority-manifest.json');
  for (const surface of authorityManifest.surfaces ?? []) {
    assert.ok(fs.existsSync(path.join(repoRoot, surface.authorityPath)));
    assert.ok(fs.existsSync(path.join(repoRoot, surface.sdkPath)));
  }
});

test('apis authority openapi content matches sdk openapi authority copies', () => {
  const authorityManifest = readJson('apis/authority-manifest.json');
  for (const surface of authorityManifest.surfaces ?? []) {
    const authority = fs.readFileSync(path.join(repoRoot, surface.authorityPath), 'utf8');
    const sdkCopy = fs.readFileSync(path.join(repoRoot, surface.sdkPath), 'utf8');
    assert.equal(
      authority,
      sdkCopy,
      `${surface.authorityPath} must match ${surface.sdkPath}`,
    );
  }
});
