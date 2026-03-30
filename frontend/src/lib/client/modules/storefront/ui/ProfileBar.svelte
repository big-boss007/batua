<script lang="ts">
  import { Avatar, Button } from '@juspay/svelte-ui-components';

  import { getInitials } from '../utils';

  let {
    name,
    tierName = null,
    isMember = false,
    membershipDaysRemaining = null,
    lifetimeSaved = null,
    onSwitch = null
  }: {
    name: string;
    tierName: string | null;
    isMember?: boolean;
    membershipDaysRemaining?: number | null;
    lifetimeSaved?: number | null;
    onSwitch?: (() => void) | null;
  } = $props();
</script>

<div class="greeting">
  <div class="greeting-left">
    <Avatar alt={name} name={getInitials(name)} classes="profile-avatar" />
    <div>
      <div class="greeting-text">Welcome back,</div>
      <div class="customer-name">{name}</div>
      {#if tierName !== null}
        <div class="greeting-meta">
          <span class="meta-tier">{tierName}</span>
          {#if isMember}
            <span class="meta-sep"> · </span>
            <span class="meta-membership">
              Member{#if membershipDaysRemaining !== null} · {membershipDaysRemaining}d left{/if}
            </span>
          {/if}
        </div>
      {/if}
    </div>
  </div>
  {#if onSwitch !== null}
    <Button text="Switch" classes="switch-btn" onclick={onSwitch} />
  {/if}
</div>

<style>
  .greeting {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .greeting-left {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  :global(.profile-avatar) {
    --avatar-background: linear-gradient(135deg, #6366f1, #818cf8);
    --avatar-color: #ffffff;
    --avatar-size: 48px;
    --avatar-font-size: 16px;
    --avatar-font-weight: 600;
    flex-shrink: 0;
  }

  .greeting-text {
    font-size: 13px;
    color: #9ca3af;
  }

  .customer-name {
    font-size: 20px;
    font-weight: 700;
    color: #ffffff;
    margin-top: 3px;
  }

  .greeting-meta {
    font-size: 12px;
    color: #9ca3af;
    margin-top: 2px;
  }

  .meta-tier {
    color: #818cf8;
    font-weight: 600;
  }

  .meta-sep {
    color: #9ca3af;
  }

  .meta-membership {
    color: #fde047;
    font-weight: 500;
  }

  :global(.switch-btn) {
    --button-font-size: 12px;
    --button-text-color: #818cf8;
    --button-color: transparent;
    --button-border: 1px solid #2a2d3a;
    --button-padding: 5px 14px;
    --button-border-radius: 20px;
    --button-hover-color: rgba(99, 102, 241, 0.1);
    --button-hover-text-color: #818cf8;
    flex-shrink: 0;
  }
</style>
