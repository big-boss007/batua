<script lang="ts">
  import { Pill } from '@juspay/svelte-ui-components';
  import { formatMultiplier } from '$lib/client/modules/customers';

  let {
    tierName,
    rank,
    multiplier
  }: {
    tierName: string;
    rank: number;
    multiplier: number;
  } = $props();

  let label = $derived(formatMultiplier(multiplier));
</script>

<span class="tier-badge-wrapper">
  <Pill text="{tierName} {label}" />
</span>
