<script lang="ts">
  import { Button } from '@juspay/svelte-ui-components';
  import { generateWhatsAppShareUrl, copyToClipboard } from '../utils';

  let { text, url, onCopy }: { text: string; url: string; onCopy: () => void } = $props();

  let shareText = $derived(`${text}\n${url}`);

  function handleWhatsApp() {
    const waUrl = generateWhatsAppShareUrl(shareText);
    window.open(waUrl, '_blank', 'noopener');
  }

  async function handleCopy() {
    const success = await copyToClipboard(url);
    if (success) {
      onCopy();
    }
  }
</script>

<div class="share-buttons">
  <div class="share-btn-wa">
    <Button text="Share on WhatsApp" onclick={handleWhatsApp} classes="btn-whatsapp" />
  </div>
  <div class="share-btn-copy">
    <Button text="Copy Link" onclick={handleCopy} classes="btn-secondary btn-copy-link" />
  </div>
</div>

<style>
  .share-buttons {
    display: flex;
    gap: var(--space-3);
  }

  .share-btn-wa {
    flex: 1;
    --button-width: 100%;
    --button-color: #25d366;
    --button-hover-color: #1da851;
    --button-text-color: #ffffff;
    --button-height: 48px;
    --button-font-weight: var(--font-weight-semibold);
    --button-border-radius: var(--radius-md);
  }

  .share-btn-copy {
    flex: 1;
    --button-width: 100%;
    --button-height: 48px;
    --button-border-radius: var(--radius-md);
  }
</style>
