import type { LlmPageInfo } from './llm-page-info';
import type { LlmRetrievalProfile } from './llm-retrieval-profile';

export interface LlmRetrievalProfileList {
  items: LlmRetrievalProfile[];
  pageInfo: LlmPageInfo;
}
