import type { LlmRetrievalProfile } from './llm-retrieval-profile';

export interface RetrievalProfilesRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
