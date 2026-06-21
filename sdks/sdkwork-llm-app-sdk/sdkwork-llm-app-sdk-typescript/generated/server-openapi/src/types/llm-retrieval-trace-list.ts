import type { LlmPageInfo } from './llm-page-info';
import type { LlmRetrievalTrace } from './llm-retrieval-trace';

export interface LlmRetrievalTraceList {
  items: LlmRetrievalTrace[];
  pageInfo: LlmPageInfo;
}
