export interface LlmLearningSettings {
  autoExtractEnabled: boolean;
  autoApproveThreshold: number;
  reviewRequiredBelowThreshold: boolean;
  habitPromotionThreshold: number;
  retentionPolicyRef?: string | null;
  updatedAt: string;
  version: string;
}
