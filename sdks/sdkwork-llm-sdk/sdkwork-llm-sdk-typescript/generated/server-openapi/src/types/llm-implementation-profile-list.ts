import type { LlmImplementationProfile } from './llm-implementation-profile';
import type { LlmPageInfo } from './llm-page-info';

export interface LlmImplementationProfileList {
  items: LlmImplementationProfile[];
  pageInfo: LlmPageInfo;
}
