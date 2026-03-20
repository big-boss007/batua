<script lang="ts">
</script>

<svelte:head>
  <title>Defaults - Breeze Platform</title>
</svelte:head>

<div class="defaults-page">
  <div class="page-header">
    <h1>Platform Defaults</h1>
    <p class="subtitle">Platform-wide default configurations inherited by all merchants</p>
  </div>

  <div class="placeholder-card">
    <div class="placeholder-icon">&#x2699;</div>
    <h2>Coming Soon</h2>
    <p class="placeholder-description">
      This section will allow you to configure platform-wide defaults that all merchants inherit.
    </p>
    <div class="planned-features">
      <div class="feature-item">
        <span class="feature-label">Default Wallet Policies</span>
        <span class="feature-desc">Fallback constraints for each bucket type</span>
      </div>
      <div class="feature-item">
        <span class="feature-label">Default Notification Templates</span>
        <span class="feature-desc"
          >System-level templates for earn, redemption, COD, tier upgrades</span
        >
      </div>
      <div class="feature-item">
        <span class="feature-label">Default Connectors</span>
        <span class="feature-desc"
          >Fallback connector configs when merchants have none configured</span
        >
      </div>
    </div>
  </div>
</div>

<style>
  .defaults-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-8);
  }

  .page-header {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .page-header h1 {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    line-height: var(--line-height-tight);
  }

  .subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .placeholder-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-6);
    padding: var(--space-12) var(--space-8);
    background: var(--color-surface);
    border: 1px dashed var(--color-border);
    border-radius: var(--radius-lg);
    text-align: center;
  }

  .placeholder-icon {
    font-size: 48px;
    color: var(--color-text-muted);
    opacity: 0.5;
  }

  .placeholder-card h2 {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .placeholder-description {
    font-size: var(--font-size-base);
    color: var(--color-text-muted);
    max-width: 480px;
  }

  .planned-features {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    margin-top: var(--space-4);
    width: 100%;
    max-width: 480px;
  }

  .feature-item {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    padding: var(--space-4);
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    text-align: left;
  }

  .feature-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .feature-desc {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }
</style>
