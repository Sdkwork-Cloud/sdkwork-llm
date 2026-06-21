export interface LlmRetrievalRequest {
  query: string;
  spaceIds: string[];
  actorId?: string | null;
  retrievalProfileId?: string | null;
  recordTypes?: ('working' | 'session' | 'semantic' | 'episodic' | 'procedural' | 'habit' | 'relationship' | 'domain_knowledge')[] | null;
  filters?: Record<string, unknown> | null;
  topK: number;
  contextBudgetTokens: number;
  includeTrace?: boolean;
}
