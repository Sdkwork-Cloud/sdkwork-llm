import type { LlmProviderBinding } from './llm-provider-binding';

export interface LlmProviderHealth {
  status: 'healthy' | 'degraded' | 'unhealthy' | 'unknown';
  checkedAt: string;
  providers: LlmProviderBinding[];
}
