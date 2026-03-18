<script lang="ts">
  import { Progress } from '@juspay/svelte-ui-components';
  import type { TierProgress as TierProgressType } from '$lib/client/modules/customers';

  let { progress }: { progress: TierProgressType } = $props();
</script>

<div class="tier-progress">
  <div class="progress-header">
    <span class="progress-label">Progress to {progress.next_tier_name}</span>
  </div>
  <Progress value={progress.percentage} showLabel />
  <div class="progress-footer">
    <span class="progress-current">{progress.current_value.toLocaleString('en-IN')}</span>
    <span class="progress-threshold">{progress.threshold.toLocaleString('en-IN')}</span>
  </div>
</div>

<style>
  .tier-progress {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .progress-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .progress-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .progress-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .progress-current,
  .progress-threshold {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
</style>
