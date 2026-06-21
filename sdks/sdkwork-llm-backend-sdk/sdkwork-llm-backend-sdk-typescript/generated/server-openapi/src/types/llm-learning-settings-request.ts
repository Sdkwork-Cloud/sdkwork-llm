export interface LlmLearningSettingsRequest {
  autoExtractEnabled?: boolean;
  autoApproveThreshold?: number;
  reviewRequiredBelowThreshold?: boolean;
  habitPromotionThreshold?: number;
  retentionPolicyRef?: string | null;
  version?: string | null;
}
