import type { LlmPageInfo } from './llm-page-info';
import type { LlmSpace } from './llm-space';

export interface LlmSpaceList {
  items: LlmSpace[];
  pageInfo: LlmPageInfo;
}
