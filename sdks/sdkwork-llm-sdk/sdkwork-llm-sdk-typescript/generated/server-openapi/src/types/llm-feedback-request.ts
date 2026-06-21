export interface LlmFeedbackRequest {
  targetType: 'retrieval' | 'hit' | 'memory' | 'candidate' | 'habit';
  targetId: string;
  feedbackType: string;
  rating?: number | null;
  comment?: string | null;
  metadata?: Record<string, unknown> | null;
}
