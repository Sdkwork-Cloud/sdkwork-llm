import type { LlmCandidate } from './llm-candidate';

export interface CandidatesRejectResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
