import type { LlmProviderBinding } from './llm-provider-binding';
import type { PageInfo } from './page-info';

export interface ProviderBindingsListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
