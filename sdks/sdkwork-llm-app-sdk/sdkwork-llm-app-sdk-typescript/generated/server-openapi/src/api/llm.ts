import { appApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { LlmCandidate, LlmCandidateList, LlmContextPack, LlmContextPackRequest, LlmEvent, LlmEventRequest, LlmExportJob, LlmExportRequest, LlmExtractionRequest, LlmFeedback, LlmFeedbackRequest, LlmForgetJob, LlmForgetRequest, LlmHabit, LlmHabitList, LlmHabitRequest, LlmLearningJob, LlmLearningSettings, LlmLearningSettingsRequest, LlmRecord, LlmRecordList, LlmRecordRequest, LlmRecordSourceList, LlmRetrievalRequest, LlmRetrievalResult, LlmReviewRequest, LlmSpace, LlmSpaceList, LlmSpaceRequest } from '../types';


export class LlmLearningSettingsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async retrieve(): Promise<LlmLearningSettings> {
    return this.client.get<LlmLearningSettings>(appApiPath(`/llm/learning_settings`));
  }

async update(body: LlmLearningSettingsRequest): Promise<LlmLearningSettings> {
    return this.client.patch<LlmLearningSettings>(appApiPath(`/llm/learning_settings`), body, undefined, undefined, 'application/json');
  }
}

export interface LlmExportJobsCreateParams {
  idempotencyKey?: string;
}

export class LlmExportJobsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async create(body: LlmExportRequest, params?: LlmExportJobsCreateParams): Promise<LlmExportJob> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmExportJob>(appApiPath(`/llm/export_jobs`), body, undefined, requestHeaders, 'application/json');
  }

async retrieve(exportJobId: string): Promise<LlmExportJob> {
    return this.client.get<LlmExportJob>(appApiPath(`/llm/export_jobs/${serializePathParameter(exportJobId, { name: 'exportJobId', style: 'simple', explode: false })}`));
  }
}

export interface LlmFeedbackCreateParams {
  idempotencyKey?: string;
}

export class LlmFeedbackApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async create(body: LlmFeedbackRequest, params?: LlmFeedbackCreateParams): Promise<LlmFeedback> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmFeedback>(appApiPath(`/llm/feedback`), body, undefined, requestHeaders, 'application/json');
  }
}

export interface LlmContextPacksCreateParams {
  idempotencyKey?: string;
}

export class LlmContextPacksApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async create(body: LlmContextPackRequest, params?: LlmContextPacksCreateParams): Promise<LlmContextPack> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmContextPack>(appApiPath(`/llm/context_packs`), body, undefined, requestHeaders, 'application/json');
  }

async retrieve(contextPackId: string): Promise<LlmContextPack> {
    return this.client.get<LlmContextPack>(appApiPath(`/llm/context_packs/${serializePathParameter(contextPackId, { name: 'contextPackId', style: 'simple', explode: false })}`));
  }
}

export interface LlmRetrievalsCreateParams {
  idempotencyKey?: string;
}

export class LlmRetrievalsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async create(body: LlmRetrievalRequest, params?: LlmRetrievalsCreateParams): Promise<LlmRetrievalResult> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmRetrievalResult>(appApiPath(`/llm/retrievals`), body, undefined, requestHeaders, 'application/json');
  }

async retrieve(retrievalId: string): Promise<LlmRetrievalResult> {
    return this.client.get<LlmRetrievalResult>(appApiPath(`/llm/retrievals/${serializePathParameter(retrievalId, { name: 'retrievalId', style: 'simple', explode: false })}`));
  }
}

export interface LlmHabitsListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
  stage?: string;
}

export interface LlmHabitsConfirmParams {
  idempotencyKey?: string;
}

export interface LlmHabitsRejectParams {
  idempotencyKey?: string;
}

export class LlmHabitsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(params?: LlmHabitsListParams): Promise<LlmHabitList> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'stage', value: params?.stage, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<LlmHabitList>(appendQueryString(appApiPath(`/llm/habits`), query));
  }

async retrieve(habitId: string): Promise<LlmHabit> {
    return this.client.get<LlmHabit>(appApiPath(`/llm/habits/${serializePathParameter(habitId, { name: 'habitId', style: 'simple', explode: false })}`));
  }

async update(habitId: string, body: LlmHabitRequest): Promise<LlmHabit> {
    return this.client.patch<LlmHabit>(appApiPath(`/llm/habits/${serializePathParameter(habitId, { name: 'habitId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

async confirm(habitId: string, body: LlmReviewRequest, params?: LlmHabitsConfirmParams): Promise<LlmHabit> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmHabit>(appApiPath(`/llm/habits/${serializePathParameter(habitId, { name: 'habitId', style: 'simple', explode: false })}/confirm`), body, undefined, requestHeaders, 'application/json');
  }

async reject(habitId: string, body: LlmReviewRequest, params?: LlmHabitsRejectParams): Promise<LlmHabit> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmHabit>(appApiPath(`/llm/habits/${serializePathParameter(habitId, { name: 'habitId', style: 'simple', explode: false })}/reject`), body, undefined, requestHeaders, 'application/json');
  }
}

export interface LlmCandidatesListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
  decisionState?: string;
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


async list(params?: LlmCandidatesListParams): Promise<LlmCandidateList> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'decision_state', value: params?.decisionState, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<LlmCandidateList>(appendQueryString(appApiPath(`/llm/candidates`), query));
  }

async retrieve(candidateId: string): Promise<LlmCandidate> {
    return this.client.get<LlmCandidate>(appApiPath(`/llm/candidates/${serializePathParameter(candidateId, { name: 'candidateId', style: 'simple', explode: false })}`));
  }

async approve(candidateId: string, body: LlmReviewRequest, params?: LlmCandidatesApproveParams): Promise<LlmCandidate> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmCandidate>(appApiPath(`/llm/candidates/${serializePathParameter(candidateId, { name: 'candidateId', style: 'simple', explode: false })}/approve`), body, undefined, requestHeaders, 'application/json');
  }

async reject(candidateId: string, body: LlmReviewRequest, params?: LlmCandidatesRejectParams): Promise<LlmCandidate> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmCandidate>(appApiPath(`/llm/candidates/${serializePathParameter(candidateId, { name: 'candidateId', style: 'simple', explode: false })}/reject`), body, undefined, requestHeaders, 'application/json');
  }
}

export interface LlmExtractionsCreateParams {
  idempotencyKey?: string;
}

export class LlmExtractionsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async create(body: LlmExtractionRequest, params?: LlmExtractionsCreateParams): Promise<LlmLearningJob> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmLearningJob>(appApiPath(`/llm/extractions`), body, undefined, requestHeaders, 'application/json');
  }
}

export interface LlmForgetRequestsCreateParams {
  idempotencyKey?: string;
}

export class LlmForgetRequestsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async create(body: LlmForgetRequest, params?: LlmForgetRequestsCreateParams): Promise<LlmForgetJob> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmForgetJob>(appApiPath(`/llm/forget_requests`), body, undefined, requestHeaders, 'application/json');
  }

async retrieve(forgetRequestId: string): Promise<LlmForgetJob> {
    return this.client.get<LlmForgetJob>(appApiPath(`/llm/forget_requests/${serializePathParameter(forgetRequestId, { name: 'forgetRequestId', style: 'simple', explode: false })}`));
  }
}

export interface LlmRecordsSourcesListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
}

export class LlmRecordsSourcesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(recordId: string, params?: LlmRecordsSourcesListParams): Promise<LlmRecordSourceList> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<LlmRecordSourceList>(appendQueryString(appApiPath(`/llm/records/${serializePathParameter(recordId, { name: 'recordId', style: 'simple', explode: false })}/sources`), query));
  }
}

export interface LlmRecordsListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
  spaceId?: string;
  recordType?: string;
}

export interface LlmRecordsCreateParams {
  idempotencyKey?: string;
}

export class LlmRecordsApi {
  private client: HttpClient;
  public readonly sources: LlmRecordsSourcesApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.sources = new LlmRecordsSourcesApi(client);
  }


async list(params?: LlmRecordsListParams): Promise<LlmRecordList> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'space_id', value: params?.spaceId, style: 'form', explode: true, allowReserved: false },
      { name: 'record_type', value: params?.recordType, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<LlmRecordList>(appendQueryString(appApiPath(`/llm/records`), query));
  }

async create(body: LlmRecordRequest, params?: LlmRecordsCreateParams): Promise<LlmRecord> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmRecord>(appApiPath(`/llm/records`), body, undefined, requestHeaders, 'application/json');
  }

async retrieve(recordId: string): Promise<LlmRecord> {
    return this.client.get<LlmRecord>(appApiPath(`/llm/records/${serializePathParameter(recordId, { name: 'recordId', style: 'simple', explode: false })}`));
  }

async update(recordId: string, body: LlmRecordRequest): Promise<LlmRecord> {
    return this.client.patch<LlmRecord>(appApiPath(`/llm/records/${serializePathParameter(recordId, { name: 'recordId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

async delete(recordId: string): Promise<void> {
    return this.client.delete<void>(appApiPath(`/llm/records/${serializePathParameter(recordId, { name: 'recordId', style: 'simple', explode: false })}`));
  }
}

export interface LlmEventsCreateParams {
  idempotencyKey?: string;
}

export class LlmEventsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async create(body: LlmEventRequest, params?: LlmEventsCreateParams): Promise<LlmEvent> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmEvent>(appApiPath(`/llm/events`), body, undefined, requestHeaders, 'application/json');
  }

async retrieve(eventId: string): Promise<LlmEvent> {
    return this.client.get<LlmEvent>(appApiPath(`/llm/events/${serializePathParameter(eventId, { name: 'eventId', style: 'simple', explode: false })}`));
  }
}

export interface LlmSpacesListParams {
  q?: string;
  cursor?: string;
  pageSize?: number;
}

export interface LlmSpacesCreateParams {
  idempotencyKey?: string;
}

export class LlmSpacesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(params?: LlmSpacesListParams): Promise<LlmSpaceList> {
    const query = buildQueryString([
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<LlmSpaceList>(appendQueryString(appApiPath(`/llm/spaces`), query));
  }

async create(body: LlmSpaceRequest, params?: LlmSpacesCreateParams): Promise<LlmSpace> {
    const requestHeaders = buildRequestHeaders(
      {
        'Idempotency-Key': { value: params?.idempotencyKey, style: 'simple', explode: false },
      },
      {}
    );
    return this.client.post<LlmSpace>(appApiPath(`/llm/spaces`), body, undefined, requestHeaders, 'application/json');
  }

async retrieve(spaceId: string): Promise<LlmSpace> {
    return this.client.get<LlmSpace>(appApiPath(`/llm/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}`));
  }

async update(spaceId: string, body: LlmSpaceRequest): Promise<LlmSpace> {
    return this.client.patch<LlmSpace>(appApiPath(`/llm/spaces/${serializePathParameter(spaceId, { name: 'spaceId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export class LlmApi {
  private client: HttpClient;
  public readonly spaces: LlmSpacesApi;
  public readonly events: LlmEventsApi;
  public readonly records: LlmRecordsApi;
  public readonly forgetRequests: LlmForgetRequestsApi;
  public readonly extractions: LlmExtractionsApi;
  public readonly candidates: LlmCandidatesApi;
  public readonly habits: LlmHabitsApi;
  public readonly retrievals: LlmRetrievalsApi;
  public readonly contextPacks: LlmContextPacksApi;
  public readonly feedback: LlmFeedbackApi;
  public readonly exportJobs: LlmExportJobsApi;
  public readonly learningSettings: LlmLearningSettingsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.spaces = new LlmSpacesApi(client);
    this.events = new LlmEventsApi(client);
    this.records = new LlmRecordsApi(client);
    this.forgetRequests = new LlmForgetRequestsApi(client);
    this.extractions = new LlmExtractionsApi(client);
    this.candidates = new LlmCandidatesApi(client);
    this.habits = new LlmHabitsApi(client);
    this.retrievals = new LlmRetrievalsApi(client);
    this.contextPacks = new LlmContextPacksApi(client);
    this.feedback = new LlmFeedbackApi(client);
    this.exportJobs = new LlmExportJobsApi(client);
    this.learningSettings = new LlmLearningSettingsApi(client);
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
