<script lang="ts">
  import { Button } from '@juspay/svelte-ui-components';
  import { formatPhone, formatCurrencyINR } from '$lib/client/modules/foundation';

  import type { BulkIssueInput } from '$lib/client/modules/gift-cards';

  let {
    onBulkIssue
  }: {
    onBulkIssue: (form: BulkIssueInput) => void;
  } = $props();

  type ParsedRow = { amount: number; recipient_phone: string | null };

  let parsedRows = $state<Array<ParsedRow>>([]);
  let parseError = $state<string | null>(null);
  let isSubmitting = $state(false);
  let fileName = $state<string | null>(null);
  let hasRows = $derived(parsedRows.length > 0);
  let totalValue = $derived(parsedRows.reduce((sum, r) => sum + r.amount, 0));

  let fileInput: HTMLInputElement | undefined = $state();

  function handleFileChange(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0] ?? null;
    if (!file) return;

    fileName = file.name;
    const reader = new FileReader();
    reader.onload = () => {
      const text = reader.result as string;
      parseCSV(text);
    };
    reader.readAsText(file);
  }

  function parseCSV(text: string) {
    parseError = null;
    const lines = text
      .split('\n')
      .map((l) => l.trim())
      .filter((l) => l.length > 0);

    if (lines.length === 0) {
      parseError = 'CSV file is empty';
      parsedRows = [];
      return;
    }

    const firstLine = lines[0].toLowerCase();
    const startIndex = firstLine.includes('amount') ? 1 : 0;

    const rows: Array<ParsedRow> = [];

    for (let i = startIndex; i < lines.length; i++) {
      const parts = lines[i].split(',').map((p) => p.trim());
      const amount = Number(parts[0]);

      if (isNaN(amount) || amount <= 0) {
        parseError = `Invalid amount on row ${i + 1}: "${parts[0]}"`;
        parsedRows = [];
        return;
      }

      rows.push({
        amount,
        recipient_phone: parts[1] || null
      });
    }

    parsedRows = rows;
  }

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (!hasRows || isSubmitting) return;
    isSubmitting = true;
    onBulkIssue({ cards: parsedRows });
    isSubmitting = false;
    parsedRows = [];
    fileName = null;
  }

  function downloadSample() {
    const csv = 'amount,recipient_phone\n500,+919876543210\n1000,+918765432109\n750,\n';
    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'gift-cards-sample.csv';
    a.click();
    URL.revokeObjectURL(url);
  }

  function handleChooseFile() {
    fileInput?.click();
  }
</script>

<form class="bulk-card" onsubmit={handleSubmit}>
  <div class="bulk-header">
    <h4 class="bulk-title">Bulk Issue Gift Cards</h4>
    <p class="bulk-desc">Upload a CSV to issue multiple gift cards at once. Each row creates one gift card.</p>
  </div>

  <div class="bulk-body">
    <input
      bind:this={fileInput}
      class="file-input-hidden"
      type="file"
      accept=".csv"
      onchange={handleFileChange}
    />

    <button type="button" class="upload-row" onclick={handleChooseFile}>
      <div class="upload-icon">{hasRows ? '✅' : '📄'}</div>
      <div class="upload-text">
        {#if hasRows && fileName}
          <span class="upload-text-main">{fileName}</span>
          <span class="upload-text-sub">{parsedRows.length} cards parsed successfully</span>
        {:else}
          <span class="upload-text-main">Drop a CSV file or click to browse</span>
          <span class="upload-text-sub">Columns: amount (required), recipient_phone (optional)</span>
        {/if}
      </div>
      <span class="upload-btn" class:secondary={hasRows}>{hasRows ? 'Change File' : 'Choose File'}</span>
    </button>

    <div class="sample-row">
      <button type="button" class="sample-link" onclick={downloadSample}>
        <span class="sample-icon">↓</span> Download sample CSV
      </button>
      <span class="sample-divider"></span>
      <span class="format-hint">amount, recipient_phone</span>
    </div>

    {#if parseError}
      <p class="error-text">{parseError}</p>
    {/if}

    {#if hasRows}
      <div class="preview-section">
        <div class="preview-bar">
          <span class="preview-count">{parsedRows.length} cards to issue</span>
          <span class="preview-total">Total: {formatCurrencyINR(totalValue)}</span>
        </div>
        <div class="table-wrap">
          <table class="table">
            <thead>
              <tr>
                <th>#</th>
                <th>Amount (₹)</th>
                <th>Recipient Phone</th>
              </tr>
            </thead>
            <tbody>
              {#each parsedRows as row, idx (idx)}
                <tr>
                  <td>{idx + 1}</td>
                  <td>{row.amount.toLocaleString('en-IN')}</td>
                  <td>{row.recipient_phone ? formatPhone(row.recipient_phone) : '--'}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {/if}
  </div>

  {#if hasRows}
    <div class="bulk-footer">
      <Button
        text="Issue {parsedRows.length} Gift Cards"
        classes="btn-primary"
        disabled={isSubmitting}
        type="submit"
      />
    </div>
  {/if}
</form>

<style>
  .bulk-card {
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .bulk-header {
    padding: var(--space-5) var(--space-6);
    border-bottom: 1px solid var(--color-border-light, #f3f4f6);
  }

  .bulk-title {
    font-size: var(--font-size-md);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    margin-bottom: 4px;
  }

  .bulk-desc {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    line-height: 1.5;
  }

  .bulk-body {
    padding: var(--space-5) var(--space-6);
  }

  .file-input-hidden {
    display: none;
  }

  .upload-row {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-4) var(--space-5);
    border: 2px dashed var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-surface-2, #fafafa);
    cursor: pointer;
    width: 100%;
    text-align: left;
  }

  .upload-row:hover {
    border-color: var(--color-primary);
    background: color-mix(in srgb, var(--color-primary) 3%, transparent);
  }

  .upload-icon {
    width: 40px;
    height: 40px;
    background: color-mix(in srgb, var(--color-primary) 10%, transparent);
    border-radius: var(--radius-md);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 18px;
    flex-shrink: 0;
  }

  .upload-text {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .upload-text-main {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .upload-text-sub {
    font-size: 11px;
    color: var(--color-text-muted);
  }

  .upload-btn {
    height: 34px;
    padding: 0 16px;
    background: var(--color-primary);
    color: #fff;
    border: none;
    border-radius: var(--radius-md);
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    display: flex;
    align-items: center;
    white-space: nowrap;
  }

  .upload-btn.secondary {
    background: var(--color-surface);
    color: var(--color-text-muted);
    border: 1px solid var(--color-border);
  }

  .sample-row {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin-top: var(--space-3);
  }

  .sample-link {
    font-size: var(--font-size-xs);
    color: var(--color-primary);
    font-weight: var(--font-weight-medium);
    cursor: pointer;
    background: none;
    border: none;
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 0;
  }

  .sample-link:hover {
    text-decoration: underline;
  }

  .sample-icon {
    font-size: 14px;
  }

  .sample-divider {
    width: 1px;
    height: 14px;
    background: var(--color-border);
  }

  .format-hint {
    font-size: 11px;
    color: var(--color-text-muted);
  }

  .error-text {
    color: var(--color-error);
    font-size: var(--font-size-sm);
    margin-top: var(--space-3);
  }

  .preview-section {
    margin-top: var(--space-5);
  }

  .preview-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-2);
  }

  .preview-count {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .preview-total {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .table-wrap {
    overflow-x: auto;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    max-height: 240px;
    overflow-y: auto;
  }

  .table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--font-size-xs);
  }

  .table th {
    text-align: left;
    padding: var(--space-2) var(--space-3);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-muted);
    background: var(--color-surface);
    border-bottom: 1px solid var(--color-border);
    position: sticky;
    top: 0;
  }

  .table td {
    padding: var(--space-2) var(--space-3);
    border-bottom: 1px solid var(--color-border);
  }

  .table tr:last-child td {
    border-bottom: none;
  }

  .bulk-footer {
    padding: var(--space-4) var(--space-6);
    border-top: 1px solid var(--color-border-light, #f3f4f6);
    display: flex;
    justify-content: flex-end;
  }
</style>
