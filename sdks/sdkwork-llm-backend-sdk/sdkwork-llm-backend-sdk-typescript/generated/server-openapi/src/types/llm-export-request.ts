export interface LlmExportRequest {
  spaceIds: string[];
  format: 'json' | 'jsonl' | 'markdown';
  includeEvents?: boolean;
  driveTargetRef?: string | null;
  metadata?: Record<string, unknown> | null;
}
