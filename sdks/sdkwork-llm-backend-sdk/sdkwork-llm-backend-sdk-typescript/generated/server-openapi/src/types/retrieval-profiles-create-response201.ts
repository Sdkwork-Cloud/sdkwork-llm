import type { LlmRetrievalProfile } from './llm-retrieval-profile';

export interface RetrievalProfilesCreateResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
