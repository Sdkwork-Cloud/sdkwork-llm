import type { LlmPageInfo } from './llm-page-info';
import type { LlmProviderBinding } from './llm-provider-binding';

export interface LlmProviderBindingList {
  items: LlmProviderBinding[];
  pageInfo: LlmPageInfo;
}
