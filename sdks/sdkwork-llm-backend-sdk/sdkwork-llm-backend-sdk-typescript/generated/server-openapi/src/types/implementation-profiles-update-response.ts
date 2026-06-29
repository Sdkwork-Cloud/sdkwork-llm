import type { LlmImplementationProfile } from './llm-implementation-profile';

export interface ImplementationProfilesUpdateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
