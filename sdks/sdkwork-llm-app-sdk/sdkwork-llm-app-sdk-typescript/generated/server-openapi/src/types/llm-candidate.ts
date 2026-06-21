export interface LlmCandidate {
  candidateId: string;
  spaceId: string;
  userId?: string | null;
  candidateType: string;
  recordType: 'working' | 'session' | 'semantic' | 'episodic' | 'procedural' | 'habit' | 'relationship' | 'domain_knowledge';
  proposedText: string;
  proposedPayload?: Record<string, unknown> | null;
  targetRecordId?: string | null;
  evidence?: Record<string, unknown> | null;
  confidence: number;
  noveltyScore?: number | null;
  riskScore?: number | null;
  decisionState: 'pending' | 'auto_approved' | 'approved' | 'rejected' | 'expired' | 'superseded';
  decisionReason?: string | null;
  createdAt: string;
  updatedAt: string;
  version?: string;
}
