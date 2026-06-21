export interface LlmRecordSource {
  sourceId: string;
  recordId: string;
  eventId: string;
  sourceRole: string;
  confidenceDelta?: number | null;
  createdAt: string;
}
