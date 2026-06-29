import type { LlmLearningJob } from './llm-learning-job';

export interface ExtractionJobsRetrieveResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
