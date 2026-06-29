import type { LlmRecord } from './llm-record';

export interface RecordsSupersedeResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
