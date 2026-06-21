import type { LlmAuditLog } from './llm-audit-log';
import type { LlmPageInfo } from './llm-page-info';

export interface LlmAuditLogList {
  items: LlmAuditLog[];
  pageInfo: LlmPageInfo;
}
