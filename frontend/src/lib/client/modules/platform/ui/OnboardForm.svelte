<script lang="ts">
  import { Input, Select } from '@juspay/svelte-ui-components';

  let {
    onsubmit,
    loading = false
  }: {
    onsubmit: (data: {
      name: string;
      external_id: string;
      domain: string | null;
      slug: string | null;
      plan_tier: string;
    }) => void;
    loading?: boolean;
  } = $props();

  let name = $state('');
  let externalId = $state('');
  let domain = $state('');
  let slug = $state('');
  let planTier = $state('free');

  const planItems = [
    { id: 'free', label: 'Free' },
    { id: 'grow', label: 'Grow' },
    { id: 'scale', label: 'Scale' },
    { id: 'enterprise', label: 'Enterprise' }
  ];

  function generateSlug(input: string): string {
    return input
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, '-')
      .replace(/^-|-$/g, '');
  }

  function handleNameInput(value: string) {
    name = value;
    if (slug === '' || slug === generateSlug(name.slice(0, -1))) {
      slug = generateSlug(value);
    }
  }

  function handleExternalIdInput(value: string) {
    externalId = value;
  }

  function handleDomainInput(value: string) {
    domain = value;
  }

  function handleSlugInput(value: string) {
    slug = value;
  }

  function handlePlanChange(value: string[]) {
    if (value.length > 0) {
      planTier = value[0];
    }
  }

  function handleSubmit() {
    if (name.trim() === '' || externalId.trim() === '') return;
    onsubmit({
      name: name.trim(),
      external_id: externalId.trim(),
      domain: domain.trim() === '' ? null : domain.trim(),
      slug: slug.trim() === '' ? null : slug.trim(),
      plan_tier: planTier
    });
  }
</script>

<form
  class="onboard-form"
  onsubmit={(e) => {
    e.preventDefault();
    handleSubmit();
  }}
>
  <div class="form-field">
    <Input value={name} label="Merchant Name" placeholder="Acme Store" onInput={handleNameInput} />
  </div>

  <div class="form-field">
    <Input
      value={externalId}
      label="External ID / Shopify Shop ID"
      placeholder="shop_abc123"
      onInput={handleExternalIdInput}
    />
  </div>

  <div class="form-field">
    <Input
      value={domain}
      label="Domain"
      placeholder="acme.myshopify.com"
      onInput={handleDomainInput}
    />
  </div>

  <div class="form-field">
    <Input value={slug} label="Slug" placeholder="acme-store" onInput={handleSlugInput} />
  </div>

  <div class="form-field">
    <span class="field-label">Plan Tier</span>
    <Select
      placeholder="Select plan"
      items={planItems}
      value={[planTier]}
      onchange={handlePlanChange}
    />
  </div>
</form>

<style>
  .onboard-form {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    padding: var(--space-4);
  }

  .form-field {
    --input-width: 100%;
    --input-margin: 0;
    --select-width: 100%;
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .field-label {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-normal);
    color: var(--color-text-muted);
  }
</style>
