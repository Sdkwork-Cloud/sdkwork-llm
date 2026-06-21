import type { LlmPageInfo } from './llm-page-info';
import type { LlmRecord } from './llm-record';

export interface LlmRecordList {
  items: LlmRecord[];
  pageInfo: LlmPageInfo;
}
