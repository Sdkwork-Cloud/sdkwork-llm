import type { LlmIndex } from './llm-index';
import type { PageInfo } from './page-info';

export interface IndexesListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
