<script lang="ts">
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
    <div class="avatar">{getInitials(name)}</div>
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
    <button class="switch-btn" onclick={onSwitch}>Switch</button>
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

  .avatar {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    background: linear-gradient(135deg, #6366f1, #818cf8);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 16px;
    color: #ffffff;
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

  .switch-btn {
    font-size: 12px;
    color: #818cf8;
    background: none;
    border: 1px solid #2a2d3a;
    padding: 5px 14px;
    border-radius: 20px;
    cursor: pointer;
    transition: all 0.2s;
    flex-shrink: 0;
  }

  .switch-btn:hover {
    border-color: #6366f1;
    background: rgba(99, 102, 241, 0.1);
  }
</style>
