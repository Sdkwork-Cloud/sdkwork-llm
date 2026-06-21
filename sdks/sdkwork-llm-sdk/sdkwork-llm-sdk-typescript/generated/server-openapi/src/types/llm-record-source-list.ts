import type { LlmPageInfo } from './llm-page-info';
import type { LlmRecordSource } from './llm-record-source';

export interface LlmRecordSourceList {
  items: LlmRecordSource[];
  pageInfo: LlmPageInfo;
}
