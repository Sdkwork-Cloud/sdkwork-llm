import type { LlmCandidate } from './llm-candidate';
import type { LlmPageInfo } from './llm-page-info';

export interface LlmCandidateList {
  items: LlmCandidate[];
  pageInfo: LlmPageInfo;
}
