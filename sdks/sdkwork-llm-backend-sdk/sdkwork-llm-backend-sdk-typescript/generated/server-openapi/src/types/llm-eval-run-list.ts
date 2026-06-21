import type { LlmEvalRun } from './llm-eval-run';
import type { LlmPageInfo } from './llm-page-info';

export interface LlmEvalRunList {
  items: LlmEvalRun[];
  pageInfo: LlmPageInfo;
}
