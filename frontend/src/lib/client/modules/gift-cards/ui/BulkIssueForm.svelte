<script lang="ts">
  import type { BulkIssueForm } from '$lib/client/modules/gift-cards';

  let {
    onBulkIssue
  }: {
    onBulkIssue: (form: BulkIssueForm) => void;
  } = $props();

  type ParsedRow = { amount: number; recipient_phone: string | null };

  let parsedRows = $state<Array<ParsedRow>>([]);
  let parseError = $state<string | null>(null);
  let isSubmitting = $state(false);
  let hasRows = $derived(parsedRows.length > 0);

  function handleFileChange(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0] ?? null;
    if (!file) return;

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
  }
</script>

<form class="form" onsubmit={handleSubmit}>
  <div class="upload-area">
    <label class="upload-label" for="csv-upload">
      <span class="upload-title">Upload CSV</span>
      <span class="upload-hint">Format: amount, recipient_phone (optional)</span>
    </label>
    <input
      id="csv-upload"
      class="file-input"
      type="file"
      accept=".csv"
      onchange={handleFileChange}
    />
  </div>

  {#if parseError}
    <p class="error-text">{parseError}</p>
  {/if}

  {#if hasRows}
    <div class="preview">
      <p class="preview-heading">{parsedRows.length} cards to issue</p>
      <div class="table-wrapper">
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
                <td>{row.amount}</td>
                <td>{row.recipient_phone ?? '--'}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <button class="btn btn-primary" type="submit" disabled={isSubmitting}>
      Issue {parsedRows.length} Gift Cards
    </button>
  {/if}
</form>

<style>
  .form {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .upload-area {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-6);
    border: 2px dashed var(--color-border);
    border-radius: var(--radius-lg);
    text-align: center;
  }

  .upload-label {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    cursor: pointer;
  }

  .upload-title {
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .upload-hint {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .file-input {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    align-self: center;
  }

  .error-text {
    color: var(--color-error);
    font-size: var(--font-size-sm);
  }

  .preview {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .preview-heading {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .table-wrapper {
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

  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-2) var(--space-4);
    border: none;
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    transition: background-color var(--transition-fast);
    align-self: flex-start;
  }

  .btn-primary {
    background: var(--color-primary);
    color: #ffffff;
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--color-primary-hover);
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
