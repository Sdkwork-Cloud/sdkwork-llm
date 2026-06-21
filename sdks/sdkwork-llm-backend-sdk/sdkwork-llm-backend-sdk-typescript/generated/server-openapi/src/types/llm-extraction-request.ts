export interface LlmExtractionRequest {
  spaceId: string;
  inputEvents: string[];
  extractionMode?: 'deterministic' | 'llm_assisted' | 'hybrid';
  reviewRequired?: boolean;
  metadata?: Record<string, unknown> | null;
}
