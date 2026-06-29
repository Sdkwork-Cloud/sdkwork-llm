import type { LlmEvalRun } from './llm-eval-run';

export interface EvalRunsCreateResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
