export interface LlmEvalRunRequest {
  evalType: string;
  datasetRef?: string | null;
  profileRef?: string | null;
  config?: Record<string, unknown> | null;
}
