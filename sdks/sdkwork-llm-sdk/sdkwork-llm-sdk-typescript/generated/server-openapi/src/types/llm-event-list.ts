import type { LlmEvent } from './llm-event';
import type { LlmPageInfo } from './llm-page-info';

export interface LlmEventList {
  items: LlmEvent[];
  pageInfo: LlmPageInfo;
}
