import type { LlmHabit } from './llm-habit';
import type { LlmPageInfo } from './llm-page-info';

export interface LlmHabitList {
  items: LlmHabit[];
  pageInfo: LlmPageInfo;
}
