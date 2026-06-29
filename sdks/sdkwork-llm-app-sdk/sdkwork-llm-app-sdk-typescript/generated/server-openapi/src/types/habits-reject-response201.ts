import type { LlmHabit } from './llm-habit';

export interface HabitsRejectResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
