export interface LlmForgetRequest {
  scope: 'memory' | 'space' | 'user' | 'query';
  recordIds?: string[] | null;
  spaceId?: string | null;
  query?: string | null;
  reason: string;
  metadata?: Record<string, unknown> | null;
}
