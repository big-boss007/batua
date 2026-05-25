# Phase 9: Integration — Library Component API Patterns

**Status:** COMPLETED

This phase documents the `@juspay/svelte-ui-components` API patterns established during migration and the global CSS theming approach.

## Library Component APIs

### Button

```svelte
<Button
  variant="primary" | "secondary" | "outline" | "ghost"
  size="sm" | "md" | "lg"
  disabled={boolean}
  onclick={handler}
>
  Label
</Button>
```

Themed via `app.css` classes: `btn-primary`, `btn-secondary`.

### Input

```svelte
<Input
  label="Field Name"
  value={bindableValue}
  placeholder="..."
  type="text" | "number" | "email" | "password"
  required={boolean}
  disabled={boolean}
  oninput={handler}
/>
```

For textarea usage:

```svelte
<Input
  useTextArea
  label="Description"
  value={bindableValue}
  rows={4}
/>
```

Used in: TierForm, ConnectorForm, NotificationTemplateEditor.

### Select

```svelte
<script>
  import type { SelectItem } from '@juspay/svelte-ui-components';

  let items: SelectItem[] = [
    { label: 'Option A', value: 'a' },
    { label: 'Option B', value: 'b' }
  ];
  let value: string[] = $state([]);
</script>

<Select
  {items}
  bind:value
  onchange={handler}
/>
```

**Important:** The actual API uses `items` / `value` / `onchange`. MCP docs reference `allItems` / `selectedItem` / `onselect` which are outdated. Always verify against the `.d.ts` file.

Used in: LoyaltyProgramForm, ConnectorForm.

### Toggle

```svelte
<Toggle
  label="Enable feature"
  checked={bindableBoolean}
  onchange={handler}
/>
```

Replaces custom `<input type="checkbox">` with associated `<label>` and styling.

Used in: CreateCodeForm, ReferralProgramForm, WalletPolicyForm, NotificationTemplateEditor.

### Pill

```svelte
<Pill
  variant="success" | "warning" | "error" | "info" | "neutral"
  size="sm" | "md"
>
  Status Text
</Pill>
```

Themed via `app.css` classes: `pill-success`, `pill-warning`, `pill-error`, etc.

Replaces custom `.badge` spans with inline background-color styling.

Used in: GiftCardDetail, ConversionsList, NotificationTemplateEditor.

### Progress

```svelte
<Progress
  value={currentValue}
  max={maxValue}
/>
```

Replaces custom `<div>` containers with inner width-percentage bars.

Used in: GiftCardDetail, TierDistributionChart.

## Global CSS Theming (`app.css`)

Library components are themed through CSS custom properties on `:root` and `[data-theme="dark"]`. The following global classes were already defined and applied automatically to library components:

- `btn-primary`, `btn-secondary` -- button variants
- `pill-success`, `pill-warning`, `pill-error`, `pill-info`, `pill-neutral` -- status pill colors

No additional CSS was needed for library components beyond what `app.css` already provided. The ~400 lines eliminated were per-component duplications of button, input, select, checkbox, badge, and progress-bar styles.
