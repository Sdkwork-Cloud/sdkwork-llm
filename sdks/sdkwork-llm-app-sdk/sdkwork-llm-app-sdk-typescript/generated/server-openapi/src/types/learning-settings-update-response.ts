import type { LlmLearningSettings } from './llm-learning-settings';

export interface LearningSettingsUpdateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
