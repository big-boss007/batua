<script lang="ts">
  type QuickAction = {
    label: string;
    href: string;
    icon: string;
    description: string;
  };

  let {
    actions = []
  }: {
    actions?: QuickAction[];
  } = $props();

  let displayActions: QuickAction[] = $derived(
    actions.length > 0
      ? actions
      : [
          {
            label: 'Create Rule',
            href: '/admin/rules',
            icon: '+',
            description: 'Set up earn or burn rules'
          },
          {
            label: 'Issue Gift Card',
            href: '/admin/gift-cards',
            icon: '$',
            description: 'Create and send gift cards'
          },
          {
            label: 'View Transactions',
            href: '/admin/transactions',
            icon: '#',
            description: 'Browse recent transactions'
          }
        ]
  );
</script>

<section class="quick-actions">
  <h3 class="section-title">Quick Actions</h3>
  <div class="actions-grid">
    {#each displayActions as action}
      <a href={action.href} class="action-card">
        <span class="action-icon">{action.icon}</span>
        <div class="action-content">
          <span class="action-label">{action.label}</span>
          <span class="action-description">{action.description}</span>
        </div>
      </a>
    {/each}
  </div>
</section>

<style>
  .quick-actions {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .section-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .actions-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: var(--space-4);
  }

  .action-card {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-4);
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    text-decoration: none;
    transition:
      border-color var(--transition-fast),
      box-shadow var(--transition-fast);
  }

  .action-card:hover {
    border-color: var(--color-primary);
    box-shadow: var(--shadow-sm);
  }

  .action-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    background: var(--color-primary);
    color: #ffffff;
    border-radius: var(--radius-md);
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-bold);
    flex-shrink: 0;
  }

  .action-content {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    min-width: 0;
  }

  .action-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .action-description {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
</style>
