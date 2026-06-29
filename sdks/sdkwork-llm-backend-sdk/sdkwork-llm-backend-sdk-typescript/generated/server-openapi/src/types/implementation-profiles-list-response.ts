import type { LlmImplementationProfile } from './llm-implementation-profile';
import type { PageInfo } from './page-info';

export interface ImplementationProfilesListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
