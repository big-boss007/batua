<script lang="ts">
  import { Input } from '@juspay/svelte-ui-components';
  import type { Customer } from '$lib/client/modules/customers';
  import { formatDate } from '$lib/client/modules/foundation';

  let {
    onSelect
  }: {
    onSelect: (customer: Customer) => void;
  } = $props();

  let query = $state('');
  let results = $state<Array<Customer>>([]);
  let loading = $state(false);
  let searched = $state(false);
  let debounceTimer: ReturnType<typeof setTimeout> | null = $state(null);

  function handleInput(value: string) {
    query = value;
    searched = false;

    if (debounceTimer !== null) {
      clearTimeout(debounceTimer);
    }

    if (query.trim().length < 3) {
      results = [];
      return;
    }

    debounceTimer = setTimeout(() => {
      performSearch();
    }, 400);
  }

  async function performSearch() {
    if (query.trim().length < 3) return;

    loading = true;
    const { searchCustomers } = await import('$lib/client/modules/customers');
    const result = await searchCustomers('default', query.trim());

    if (result.tag === 'success') {
      results = result.data;
    } else {
      results = [];
    }

    loading = false;
    searched = true;
  }

  function handleSelect(customer: Customer) {
    onSelect(customer);
  }
</script>

<div class="customer-search">
  <div class="search-input-wrapper">
    <Input
      value={query}
      placeholder="Search by phone or external ID..."
      onInput={handleInput}
    />
    {#if loading}
      <span class="search-spinner"></span>
    {/if}
  </div>

  {#if results.length > 0}
    <ul class="search-results">
      {#each results as customer (customer.id)}
        <li>
          <button class="result-item" onclick={() => handleSelect(customer)}>
            <div class="result-main">
              <span class="result-name">{customer.name ?? 'Unnamed'}</span>
              <span class="result-phone">{customer.phone}</span>
            </div>
            <div class="result-meta">
              {#if customer.email}
                <span class="result-email">{customer.email}</span>
              {/if}
              <span class="result-date">{formatDate(customer.created_at)}</span>
            </div>
          </button>
        </li>
      {/each}
    </ul>
  {:else if searched && !loading}
    <p class="no-results">No customers found for "{query}"</p>
  {/if}
</div>

<style>
  .customer-search {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .search-input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .search-spinner {
    position: absolute;
    right: var(--space-3);
    width: 16px;
    height: 16px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-primary);
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .search-results {
    list-style: none;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-surface);
    overflow: hidden;
  }

  .result-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    padding: var(--space-3) var(--space-4);
    border: none;
    background: transparent;
    color: var(--color-text);
    text-align: left;
    transition: background var(--transition-fast);
  }

  .result-item:hover {
    background: var(--color-surface-2);
  }

  .search-results li + li {
    border-top: 1px solid var(--color-border);
  }

  .result-main {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .result-name {
    font-weight: var(--font-weight-medium);
    font-size: var(--font-size-base);
  }

  .result-phone {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-family: var(--font-mono);
  }

  .result-meta {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: var(--space-1);
  }

  .result-email {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .result-date {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .no-results {
    padding: var(--space-4);
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }
</style>
