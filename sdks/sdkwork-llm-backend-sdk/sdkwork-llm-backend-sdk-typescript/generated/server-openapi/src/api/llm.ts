import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { LlmAuditLog, LlmCandidate, LlmEvalRun, LlmEvalRunRequest, LlmEvent, LlmExtractionRequest, LlmImplementationProfile, LlmImplementationProfileRequest, LlmIndex, LlmIndexRequest, LlmLearningJob, LlmMigrationJobRequest, LlmProviderBinding, LlmProviderBindingRequest, LlmProviderHealth, LlmRecord, LlmRecordRequest, LlmRetentionJobRequest, LlmRetrievalProfile, LlmRetrievalProfileRequest, LlmRetrievalTrace, LlmReviewRequest, LlmSpace, LlmSpaceRequest, PageInfo } from '../types';


export interface LlmMigrationJobsCreateParams {
  idempotencyKey?: string;
}

export class LlmMigrationJobsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async create(body: LlmMigrationJobRequest, params?: LlmMigrationJobsCreateParams): Promise<LlmLearningJob> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmLearningJob>(backendApiPath(`/llm/migration_jobs`), body, undefined, requestHeaders, 'application/json');
  }

async retrieve(migrationJobId: string): Promise<LlmLearningJob> {
    return this.client.get<LlmLearningJob>(backendApiPath(`/llm/migration_jobs/${serializePathParameter(migrationJobId, { name: 'migrationJobId', style: 'simple', explode: false })}`));
  }
}

export interface LlmRetentionJobsCreateParams {
  idempotencyKey?: string;
}

export class LlmRetentionJobsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async create(body: LlmRetentionJobRequest, params?: LlmRetentionJobsCreateParams): Promise<LlmLearningJob> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmLearningJob>(backendApiPath(`/llm/retention_jobs`), body, undefined, requestHeaders, 'application/json');
  }
}

export interface LlmAuditLogsListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
}

export class LlmAuditLogsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(params?: LlmAuditLogsListParams): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<Record<string, unknown>>(appendQueryString(backendApiPath(`/llm/audit_logs`), query));
  }
}

export interface LlmRetrievalTracesListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
}

export class LlmRetrievalTracesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(params?: LlmRetrievalTracesListParams): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<Record<string, unknown>>(appendQueryString(backendApiPath(`/llm/retrieval_traces`), query));
  }

async retrieve(traceId: string): Promise<LlmRetrievalTrace> {
    return this.client.get<LlmRetrievalTrace>(backendApiPath(`/llm/retrieval_traces/${serializePathParameter(traceId, { name: 'traceId', style: 'simple', explode: false })}`));
  }
}

export interface LlmEvalRunsListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
}

export interface LlmEvalRunsCreateParams {
  idempotencyKey?: string;
}

export class LlmEvalRunsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(params?: LlmEvalRunsListParams): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<Record<string, unknown>>(appendQueryString(backendApiPath(`/llm/eval_runs`), query));
  }

async create(body: LlmEvalRunRequest, params?: LlmEvalRunsCreateParams): Promise<LlmEvalRun> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmEvalRun>(backendApiPath(`/llm/eval_runs`), body, undefined, requestHeaders, 'application/json');
  }

async retrieve(evalRunId: string): Promise<LlmEvalRun> {
    return this.client.get<LlmEvalRun>(backendApiPath(`/llm/eval_runs/${serializePathParameter(evalRunId, { name: 'evalRunId', style: 'simple', explode: false })}`));
  }
}

export class LlmProviderHealthApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async retrieve(): Promise<LlmProviderHealth> {
    return this.client.get<LlmProviderHealth>(backendApiPath(`/llm/provider_health`));
  }
}

export interface LlmProviderBindingsListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
}

export interface LlmProviderBindingsCreateParams {
  idempotencyKey?: string;
}

export class LlmProviderBindingsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(params?: LlmProviderBindingsListParams): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<Record<string, unknown>>(appendQueryString(backendApiPath(`/llm/provider_bindings`), query));
  }

async create(body: LlmProviderBindingRequest, params?: LlmProviderBindingsCreateParams): Promise<LlmProviderBinding> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmProviderBinding>(backendApiPath(`/llm/provider_bindings`), body, undefined, requestHeaders, 'application/json');
  }

async update(providerBindingId: string, body: LlmProviderBindingRequest): Promise<LlmProviderBinding> {
    return this.client.patch<LlmProviderBinding>(backendApiPath(`/llm/provider_bindings/${serializePathParameter(providerBindingId, { name: 'providerBindingId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export interface LlmImplementationProfilesListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
}

export interface LlmImplementationProfilesCreateParams {
  idempotencyKey?: string;
}

export class LlmImplementationProfilesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(params?: LlmImplementationProfilesListParams): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<Record<string, unknown>>(appendQueryString(backendApiPath(`/llm/implementation_profiles`), query));
  }

async create(body: LlmImplementationProfileRequest, params?: LlmImplementationProfilesCreateParams): Promise<LlmImplementationProfile> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmImplementationProfile>(backendApiPath(`/llm/implementation_profiles`), body, undefined, requestHeaders, 'application/json');
  }

async retrieve(implementationProfileId: string): Promise<LlmImplementationProfile> {
    return this.client.get<LlmImplementationProfile>(backendApiPath(`/llm/implementation_profiles/${serializePathParameter(implementationProfileId, { name: 'implementationProfileId', style: 'simple', explode: false })}`));
  }

async update(implementationProfileId: string, body: LlmImplementationProfileRequest): Promise<LlmImplementationProfile> {
    return this.client.patch<LlmImplementationProfile>(backendApiPath(`/llm/implementation_profiles/${serializePathParameter(implementationProfileId, { name: 'implementationProfileId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export interface LlmRetrievalProfilesListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
}

export interface LlmRetrievalProfilesCreateParams {
  idempotencyKey?: string;
}

export class LlmRetrievalProfilesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(params?: LlmRetrievalProfilesListParams): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<Record<string, unknown>>(appendQueryString(backendApiPath(`/llm/retrieval_profiles`), query));
  }

async create(body: LlmRetrievalProfileRequest, params?: LlmRetrievalProfilesCreateParams): Promise<LlmRetrievalProfile> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmRetrievalProfile>(backendApiPath(`/llm/retrieval_profiles`), body, undefined, requestHeaders, 'application/json');
  }

async retrieve(profileId: string): Promise<LlmRetrievalProfile> {
    return this.client.get<LlmRetrievalProfile>(backendApiPath(`/llm/retrieval_profiles/${serializePathParameter(profileId, { name: 'profileId', style: 'simple', explode: false })}`));
  }

async update(profileId: string, body: LlmRetrievalProfileRequest): Promise<LlmRetrievalProfile> {
    return this.client.patch<LlmRetrievalProfile>(backendApiPath(`/llm/retrieval_profiles/${serializePathParameter(profileId, { name: 'profileId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export interface LlmIndexesListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
}

export interface LlmIndexesCreateParams {
  idempotencyKey?: string;
}

export interface LlmIndexesRebuildParams {
  idempotencyKey?: string;
}

export class LlmIndexesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(params?: LlmIndexesListParams): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<Record<string, unknown>>(appendQueryString(backendApiPath(`/llm/indexes`), query));
  }

async create(body: LlmIndexRequest, params?: LlmIndexesCreateParams): Promise<LlmIndex> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmIndex>(backendApiPath(`/llm/indexes`), body, undefined, requestHeaders, 'application/json');
  }

async retrieve(indexId: string): Promise<LlmIndex> {
    return this.client.get<LlmIndex>(backendApiPath(`/llm/indexes/${serializePathParameter(indexId, { name: 'indexId', style: 'simple', explode: false })}`));
  }

async update(indexId: string, body: LlmIndexRequest): Promise<LlmIndex> {
    return this.client.patch<LlmIndex>(backendApiPath(`/llm/indexes/${serializePathParameter(indexId, { name: 'indexId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

async rebuild(indexId: string, body: LlmReviewRequest, params?: LlmIndexesRebuildParams): Promise<LlmLearningJob> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmLearningJob>(backendApiPath(`/llm/indexes/${serializePathParameter(indexId, { name: 'indexId', style: 'simple', explode: false })}/rebuild`), body, undefined, requestHeaders, 'application/json');
  }
}

export interface LlmConsolidationJobsCreateParams {
  idempotencyKey?: string;
}

export class LlmConsolidationJobsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async create(body: LlmExtractionRequest, params?: LlmConsolidationJobsCreateParams): Promise<LlmLearningJob> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmLearningJob>(backendApiPath(`/llm/consolidation_jobs`), body, undefined, requestHeaders, 'application/json');
  }
}

export interface LlmExtractionJobsCreateParams {
  idempotencyKey?: string;
}

export class LlmExtractionJobsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async create(body: LlmExtractionRequest, params?: LlmExtractionJobsCreateParams): Promise<LlmLearningJob> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmLearningJob>(backendApiPath(`/llm/extraction_jobs`), body, undefined, requestHeaders, 'application/json');
  }

async retrieve(jobId: string): Promise<LlmLearningJob> {
    return this.client.get<LlmLearningJob>(backendApiPath(`/llm/extraction_jobs/${serializePathParameter(jobId, { name: 'jobId', style: 'simple', explode: false })}`));
  }
}

export interface LlmCandidatesListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
}

export interface LlmCandidatesApproveParams {
  idempotencyKey?: string;
}

export interface LlmCandidatesRejectParams {
  idempotencyKey?: string;
}

export class LlmCandidatesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(params?: LlmCandidatesListParams): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<Record<string, unknown>>(appendQueryString(backendApiPath(`/llm/candidates`), query));
  }

async approve(candidateId: string, body: LlmReviewRequest, params?: LlmCandidatesApproveParams): Promise<LlmCandidate> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmCandidate>(backendApiPath(`/llm/candidates/${serializePathParameter(candidateId, { name: 'candidateId', style: 'simple', explode: false })}/approve`), body, undefined, requestHeaders, 'application/json');
  }

async reject(candidateId: string, body: LlmReviewRequest, params?: LlmCandidatesRejectParams): Promise<LlmCandidate> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmCandidate>(backendApiPath(`/llm/candidates/${serializePathParameter(candidateId, { name: 'candidateId', style: 'simple', explode: false })}/reject`), body, undefined, requestHeaders, 'application/json');
  }
}

export interface LlmEventsListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
}

export class LlmEventsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(params?: LlmEventsListParams): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<Record<string, unknown>>(appendQueryString(backendApiPath(`/llm/events`), query));
  }

async retrieve(eventId: string): Promise<LlmEvent> {
    return this.client.get<LlmEvent>(backendApiPath(`/llm/events/${serializePathParameter(eventId, { name: 'eventId', style: 'simple', explode: false })}`));
  }
}

export interface LlmRecordsListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
}

export interface LlmRecordsSupersedeParams {
  idempotencyKey?: string;
}

export class LlmRecordsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(params?: LlmRecordsListParams): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<Record<string, unknown>>(appendQueryString(backendApiPath(`/llm/records`), query));
  }

async retrieve(recordId: string): Promise<LlmRecord> {
    return this.client.get<LlmRecord>(backendApiPath(`/llm/records/${serializePathParameter(recordId, { name: 'recordId', style: 'simple', explode: false })}`));
  }

async update(recordId: string, body: LlmRecordRequest): Promise<LlmRecord> {
    return this.client.patch<LlmRecord>(backendApiPath(`/llm/records/${serializePathParameter(recordId, { name: 'recordId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

async supersede(recordId: string, body: LlmRecordRequest, params?: LlmRecordsSupersedeParams): Promise<LlmRecord> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmRecord>(backendApiPath(`/llm/records/${serializePathParameter(recordId, { name: 'recordId', style: 'simple', explode: false })}/supersede`), body, undefined, requestHeaders, 'application/json');
  }
}

export interface LlmSpacesListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
}

export class LlmSpacesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(params?: LlmSpacesListParams): Promise<Record<string, unknown>> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<Record<string, unknown>>(appendQueryString(backendApiPath(`/llm/spaces`), query));
  }

async retrieve(spaceId: string): Promise<LlmSpace> {
    return this.client.get<LlmSpace>(backendApiPath(`/llm/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}`));
  }

async update(spaceId: string, body: LlmSpaceRequest): Promise<LlmSpace> {
    return this.client.patch<LlmSpace>(backendApiPath(`/llm/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export class LlmApi {
  private client: HttpClient;
  public readonly spaces: LlmSpacesApi;
  public readonly records: LlmRecordsApi;
  public readonly events: LlmEventsApi;
  public readonly candidates: LlmCandidatesApi;
  public readonly extractionJobs: LlmExtractionJobsApi;
  public readonly consolidationJobs: LlmConsolidationJobsApi;
  public readonly indexes: LlmIndexesApi;
  public readonly retrievalProfiles: LlmRetrievalProfilesApi;
  public readonly implementationProfiles: LlmImplementationProfilesApi;
  public readonly providerBindings: LlmProviderBindingsApi;
  public readonly providerHealth: LlmProviderHealthApi;
  public readonly evalRuns: LlmEvalRunsApi;
  public readonly retrievalTraces: LlmRetrievalTracesApi;
  public readonly auditLogs: LlmAuditLogsApi;
  public readonly retentionJobs: LlmRetentionJobsApi;
  public readonly migrationJobs: LlmMigrationJobsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.spaces = new LlmSpacesApi(client);
    this.records = new LlmRecordsApi(client);
    this.events = new LlmEventsApi(client);
    this.candidates = new LlmCandidatesApi(client);
    this.extractionJobs = new LlmExtractionJobsApi(client);
    this.consolidationJobs = new LlmConsolidationJobsApi(client);
    this.indexes = new LlmIndexesApi(client);
    this.retrievalProfiles = new LlmRetrievalProfilesApi(client);
    this.implementationProfiles = new LlmImplementationProfilesApi(client);
    this.providerBindings = new LlmProviderBindingsApi(client);
    this.providerHealth = new LlmProviderHealthApi(client);
    this.evalRuns = new LlmEvalRunsApi(client);
    this.retrievalTraces = new LlmRetrievalTracesApi(client);
    this.auditLogs = new LlmAuditLogsApi(client);
    this.retentionJobs = new LlmRetentionJobsApi(client);
    this.migrationJobs = new LlmMigrationJobsApi(client);
  }

}

export function createLlmApi(client: HttpClient): LlmApi {
  return new LlmApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}

interface PathParameterSpec {
  name: string;
  style: string;
  explode: boolean;
}

function serializePathParameter(value: unknown, spec: PathParameterSpec): string {
  if (value === undefined || value === null) {
    return '';
  }

  const style = spec.style || 'simple';
  if (Array.isArray(value)) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (typeof value === 'object') {
    return serializePathObject(spec.name, value as Record<string, unknown>, style, spec.explode);
  }
  return pathPrefix(spec.name, style, false) + encodePathValue(serializePathPrimitive(value));
}

function serializePathArray(name: string, values: unknown[], style: string, explode: boolean): string {
  const serialized = values
    .filter((item) => item !== undefined && item !== null)
    .map((item) => encodePathValue(serializePathPrimitive(item)));
  if (serialized.length === 0) {
    return pathPrefix(name, style, false);
  }
  if (style === 'matrix') {
    return explode
      ? serialized.map((item) => `;${name}=${item}`).join('')
      : `;${name}=${serialized.join(',')}`;
  }
  return pathPrefix(name, style, false) + serialized.join(explode ? '.' : ',');
}

function serializePathObject(name: string, value: Record<string, unknown>, style: string, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return pathPrefix(name, style, true);
  }
  if (style === 'matrix') {
    return explode
      ? entries.map(([key, entryValue]) => `;${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join('')
      : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',')}`;
  }
  const serialized = explode
    ? entries.map(([key, entryValue]) => `${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join(style === 'label' ? '.' : ',')
    : entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
  return pathPrefix(name, style, true) + serialized;
}

function pathPrefix(name: string, style: string, _objectValue: boolean): string {
  if (style === 'label') return '.';
  if (style === 'matrix') return `;${name}`;
  return '';
}

function encodePathValue(value: string): string {
  return encodeURIComponent(value);
}

function serializePathPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}
interface QueryParameterSpec {
  name: string;
  value: unknown;
  style: string;
  explode: boolean;
  allowReserved: boolean;
  contentType?: string;
}

function buildQueryString(parameters: QueryParameterSpec[]): string {
  const pairs: string[] = [];
  for (const parameter of parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

function appendSerializedParameter(pairs: string[], parameter: QueryParameterSpec): void {
  if (parameter.value === undefined || parameter.value === null) {
    return;
  }

  if (parameter.contentType) {
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(JSON.stringify(parameter.value), parameter.allowReserved)}`);
    return;
  }

  const style = parameter.style || 'form';
  if (style === 'deepObject') {
    appendDeepObjectParameter(pairs, parameter.name, parameter.value, parameter.allowReserved);
    return;
  }

  if (Array.isArray(parameter.value)) {
    appendArrayParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
    return;
  }

  if (typeof parameter.value === 'object') {
    appendObjectParameter(pairs, parameter.name, parameter.value as Record<string, unknown>, style, parameter.explode, parameter.allowReserved);
    return;
  }

  pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}

function appendArrayParameter(
  pairs: string[],
  name: string,
  value: unknown[],
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const values = value
    .filter((item) => item !== undefined && item !== null)
    .map((item) => serializePrimitive(item));
  if (values.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const item of values) {
      pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(item, allowReserved)}`);
    }
    return;
  }

  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(values.join(','), allowReserved)}`);
}

function appendObjectParameter(
  pairs: string[],
  name: string,
  value: Record<string, unknown>,
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const [key, entryValue] of entries) {
      pairs.push(`${encodeQueryComponent(key)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
    return;
  }

  const serialized = entries.flatMap(([key, entryValue]) => [key, serializePrimitive(entryValue)]).join(',');
  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serialized, allowReserved)}`);
}

function appendDeepObjectParameter(
  pairs: string[],
  name: string,
  value: unknown,
  allowReserved: boolean,
): void {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
    return;
  }

  for (const [key, entryValue] of Object.entries(value as Record<string, unknown>)) {
    if (entryValue === undefined || entryValue === null) {
      continue;
    }
    pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
  }
}

function serializePrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}

function encodeQueryComponent(value: string): string {
  return encodeURIComponent(value);
}

function encodeQueryValue(value: string, allowReserved: boolean): string {
  const encoded = encodeURIComponent(value);
  if (!allowReserved) {
    return encoded;
  }
  return encoded.replace(/%3A/gi, ':')
    .replace(/%2F/gi, '/')
    .replace(/%3F/gi, '?')
    .replace(/%23/gi, '#')
    .replace(/%5B/gi, '[')
    .replace(/%5D/gi, ']')
    .replace(/%40/gi, '@')
    .replace(/%21/gi, '!')
    .replace(/%24/gi, '$')
    .replace(/%26/gi, '&')
    .replace(/%27/gi, "'")
    .replace(/%28/gi, '(')
    .replace(/%29/gi, ')')
    .replace(/%2A/gi, '*')
    .replace(/%2B/gi, '+')
    .replace(/%2C/gi, ',')
    .replace(/%3B/gi, ';')
    .replace(/%3D/gi, '=');
}
function buildRequestHeaders(
  headers: Record<string, HeaderParameterSpec | undefined>,
  cookies: Record<string, HeaderParameterSpec | undefined> = {},
): Record<string, string> | undefined {
  const requestHeaders: Record<string, string> = {};

  for (const [name, parameter] of Object.entries(headers)) {
    const serialized = serializeParameterValue(parameter);
    if (serialized !== undefined) {
      requestHeaders[name] = serialized;
    }
  }

  const cookieHeader = buildCookieHeader(cookies);
  if (cookieHeader) {
    requestHeaders.Cookie = requestHeaders.Cookie
      ? `${requestHeaders.Cookie}; ${cookieHeader}`
      : cookieHeader;
  }

  return Object.keys(requestHeaders).length > 0 ? requestHeaders : undefined;
}

interface HeaderParameterSpec {
  value: unknown;
  style: string;
  explode: boolean;
  contentType?: string;
}

function buildCookieHeader(cookies: Record<string, HeaderParameterSpec | undefined>): string | undefined {
  const pairs: string[] = [];
  for (const [name, parameter] of Object.entries(cookies)) {
    const serialized = serializeParameterValue(parameter);
    if (serialized !== undefined) {
      pairs.push(`${encodeURIComponent(name)}=${encodeURIComponent(serialized)}`);
    }
  }
  return pairs.length > 0 ? pairs.join('; ') : undefined;
}

function serializeParameterValue(parameter: HeaderParameterSpec | undefined): string | undefined {
  const value = parameter?.value;
  if (value === undefined || value === null) {
    return undefined;
  }
  if (parameter?.contentType) {
    return JSON.stringify(value);
  }
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (Array.isArray(value)) {
    return value.map((item) => serializeHeaderPrimitive(item)).join(',');
  }
  if (typeof value === 'object' && value !== null) {
    return serializeHeaderObject(value as Record<string, unknown>, parameter?.explode === true);
  }
  return serializeHeaderPrimitive(value);
}

function serializeHeaderObject(value: Record<string, unknown>, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (explode) {
    return entries.map(([key, entryValue]) => `${key}=${serializeHeaderPrimitive(entryValue)}`).join(',');
  }
  return entries.flatMap(([key, entryValue]) => [key, serializeHeaderPrimitive(entryValue)]).join(',');
}

function serializeHeaderPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  return String(value);
}
