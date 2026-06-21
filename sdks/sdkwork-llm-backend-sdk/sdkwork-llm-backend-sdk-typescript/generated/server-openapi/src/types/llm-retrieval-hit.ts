import type { LlmRecord } from './llm-record';

export interface LlmRetrievalHit {
  hitId: string;
  memory?: LlmRecord | null;
  recordId?: string | null;
  retrieverName: string;
  resultRank: number;
  rawScore?: number | null;
  fusedScore?: number | null;
  explanation?: Record<string, unknown> | null;
  status: string;
}
