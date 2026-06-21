import type { LlmIndex } from './llm-index';
import type { LlmPageInfo } from './llm-page-info';

export interface LlmIndexList {
  items: LlmIndex[];
  pageInfo: LlmPageInfo;
}
