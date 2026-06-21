import type { LlmRetrievalHit } from './llm-retrieval-hit';
import type { LlmRetrievalTrace } from './llm-retrieval-trace';

export interface LlmRetrievalResult {
  retrievalId: string;
  trace?: LlmRetrievalTrace | null;
  hits: LlmRetrievalHit[];
  degraded: boolean;
}
