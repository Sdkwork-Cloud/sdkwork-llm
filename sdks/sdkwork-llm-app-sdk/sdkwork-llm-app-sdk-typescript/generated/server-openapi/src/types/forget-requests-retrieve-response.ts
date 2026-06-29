import type { LlmForgetJob } from './llm-forget-job';

export interface ForgetRequestsRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
