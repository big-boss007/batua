<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  import { Tabs, Button } from '@juspay/svelte-ui-components';
  import Icon from '$lib/components/Icon.svelte';

  import type { PageData } from './$types';

  type Feature = {
    id: string;
    name: string;
    icon: string;
    accent: string;
    tagline: string;
    lead: string;
    capabilities: Array<string>;
    callout: { label: string; text: string };
  };

  let { data }: { data: PageData } = $props();

  const features: Array<Feature> = [
    {
      id: 'wallet',
      name: 'Wallet',
      icon: 'wallet',
      accent: 'var(--purple-500)',
      tagline: 'Store credit & cashback that brings customers back.',
      lead: 'The Batua wallet holds every form of store value a customer earns — cashback, refunds, gift-card balance, referral rewards, and goodwill credit — in one balance they spend at Shopify checkout.',
      capabilities: [
        'COD-to-prepaid cashback incentives',
        'Multi-bucket balances, each with its own rules',
        'Per-bucket expiry & redemption policies',
        'UPI top-up for customer-funded balance',
        'Append-only ledger for every movement',
        'Spend balance directly at checkout'
      ],
      callout: {
        label: 'COD-aware:',
        text: 'reward prepaid orders with wallet cashback to cut return-to-origin and improve cash flow.'
      }
    },
    {
      id: 'loyalty',
      name: 'Loyalty',
      icon: 'star',
      accent: 'var(--yellow-500)',
      tagline: 'Points and VIP tiers that reward your best customers.',
      lead: 'Set earning rules on the actions you care about and let customers climb VIP tiers that boost how fast they earn — keeping your most valuable buyers engaged.',
      capabilities: [
        'Configurable earn rules — percentage or fixed',
        'VIP tiers with earn-rate multipliers',
        'Milestones for lifetime spend & order count',
        'Purchase streaks',
        'Birthday & profile-completion bonuses',
        'Spin-the-wheel reward mechanic'
      ],
      callout: {
        label: 'Indian shoppers expect tiers',
        text: '— inspired by Tata Neu and Flipkart Plus. Batua brings the same mechanic to D2C brands.'
      }
    },
    {
      id: 'giftcards',
      name: 'Gift Cards',
      icon: 'gift',
      accent: 'var(--magenta-500)',
      tagline: 'Digital gift cards your customers can send and redeem.',
      lead: 'Issue branded digital gift cards, let recipients claim them straight into their wallet, and let anyone check a balance — a gifting channel Shopify’s native feature doesn’t cover.',
      capabilities: [
        'Digital gift card issuance',
        'Claim-to-wallet flow for recipients',
        'Public gift-card balance checker',
        'Bulk issuance for corporate gifting',
        'Full ledger trail per card',
        'Expiry & status management'
      ],
      callout: {
        label: 'Gifting is growing 20–30% a year',
        text: 'in India — personal and corporate. Gift cards open a new acquisition channel.'
      }
    },
    {
      id: 'referrals',
      name: 'Referrals',
      icon: 'share',
      accent: 'var(--p-600)',
      tagline: 'Two-sided referrals that turn customers into a growth channel.',
      lead: 'Give every customer a referral code and reward both sides when a friend buys. Word of mouth is the #1 discovery channel for Indian shoppers — Batua makes it systematic.',
      capabilities: [
        'Auto-generated referral codes',
        'Give-get rewards for referrer & referee',
        'Vanity codes for memorable sharing',
        'Influencer / creator codes with commission',
        'Conversion tracking end-to-end',
        'Trigger codes on registration or first purchase'
      ],
      callout: {
        label: 'No per-referral commission to Batua',
        text: '— unlike referral-only tools that take a cut of referral revenue.'
      }
    },
    {
      id: 'campaigns',
      name: 'Campaigns',
      icon: 'megaphone',
      accent: 'var(--green-500)',
      tagline: 'Time-boxed boosts for the festive season and big moments.',
      lead: 'Layer multiplier campaigns on top of your earn rules to run 2x or 3x earning events during Diwali, sales, or product launches — without rewriting a single rule.',
      capabilities: [
        'Multiplier campaigns on any earn rule',
        'Festive templates (Diwali, and more)',
        'Scheduled start & end dates',
        'Campaign calendar view',
        'Per-campaign performance tracking',
        'Stacks with loyalty & membership boosts'
      ],
      callout: {
        label: 'Festive-native:',
        text: 'India’s biggest sales windows are seasonal — campaigns let you ramp rewards up and down on schedule.'
      }
    },
    {
      id: 'memberships',
      name: 'Memberships',
      icon: 'crown',
      accent: 'var(--purple-500)',
      tagline: 'Paid memberships with premium earning perks.',
      lead: 'Offer paid membership tiers — like Tata Neu or Flipkart Plus — that give members a higher earn rate and exclusive benefits, creating a recurring revenue stream and deeper loyalty.',
      capabilities: [
        'Paid membership tiers',
        'Boosted earn-rate multiplier for members',
        'Upgrade, extend & renew flows',
        'Tied to loyalty tiers',
        'Expiry management & reminders',
        'Member-only benefits'
      ],
      callout: {
        label: 'Recurring revenue:',
        text: 'a paid membership turns retention into predictable income on top of repeat orders.'
      }
    }
  ];

  const tabIds = ['overview', ...features.map((f) => f.id)];
  const tabItems = ['Overview', ...features.map((f) => f.name)];

  // svelte-ignore state_referenced_locally
  let activeTabIndex = $state(Math.max(0, tabIds.indexOf(data.activeTab)));
  let activeTab = $derived(tabIds[activeTabIndex]);
  let activeFeature = $derived(features.find((f) => f.id === activeTab) ?? null);

  function selectTab(index: number) {
    activeTabIndex = index;
    const url = new URL($page.url);
    url.searchParams.set('tab', tabIds[index]);
    goto(url.toString(), { replaceState: true, noScroll: true });
  }

  function goToFeature(id: string) {
    selectTab(tabIds.indexOf(id));
  }

  function openWebsite(path: string) {
    window.open(path, '_blank');
  }

  function iconStyle(accent: string): string {
    return `background: color-mix(in srgb, ${accent} 14%, transparent); color: ${accent};`;
  }
</script>

<svelte:head>
  <title>About - Batua</title>
</svelte:head>

<div class="about-page">
  <header class="page-header">
    <h1 class="page-title">About</h1>
    <p class="page-subtitle">Learn about Batua and everything the retention suite can do</p>
  </header>

  <Tabs items={tabItems} activeIndex={activeTabIndex} onchange={(idx) => selectTab(idx)} />

  <div class="tab-content">
    {#if activeTab === 'overview'}
      <section class="hero">
        <span class="eyebrow">Breeze retention suite</span>
        <h2 class="hero-headline">Turn one-time buyers into customers who keep coming back.</h2>
        <p class="hero-text">
          Batua is the all-in-one retention platform for Indian Shopify D2C brands. It brings wallet
          &amp; store credit, loyalty, gift cards, referrals, campaigns, and memberships into a
          single platform — built COD-first, WhatsApp-native, and festive-ready for the Indian
          market. Use the tabs above to explore each part of the suite.
        </p>
        <div class="hero-actions">
          <Button
            text="Visit Website →"
            classes="btn-primary"
            onclick={() => openWebsite('/website/index.html')}
          />
          <Button
            text="See pricing"
            classes="btn-secondary"
            onclick={() => openWebsite('/website/pricing.html')}
          />
        </div>
      </section>

      <div>
        <div class="section-label">Explore the suite</div>
        <div class="feature-grid">
          {#each features as feature (feature.id)}
            <button class="feature-card" onclick={() => goToFeature(feature.id)}>
              <span class="feature-icon" style={iconStyle(feature.accent)}
                ><Icon name={feature.icon} size={20} /></span
              >
              <span class="feature-card-name">{feature.name}</span>
              <span class="feature-card-desc">{feature.tagline}</span>
            </button>
          {/each}
        </div>
      </div>
    {:else if activeFeature !== null}
      <section class="feature-detail">
        <div class="feature-head">
          <span class="feature-icon lg" style={iconStyle(activeFeature.accent)}
            ><Icon name={activeFeature.icon} size={26} /></span
          >
          <div>
            <h2 class="feature-title">{activeFeature.name}</h2>
            <p class="feature-tagline">{activeFeature.tagline}</p>
          </div>
        </div>
        <p class="feature-lead">{activeFeature.lead}</p>
        <div class="cap-grid">
          {#each activeFeature.capabilities as capability (capability)}
            <div class="cap">
              <span class="cap-check">✓</span>
              <span>{capability}</span>
            </div>
          {/each}
        </div>
        <div class="callout">
          <span class="callout-icon"><Icon name="lightbulb" size={18} /></span>
          <span><strong>{activeFeature.callout.label}</strong> {activeFeature.callout.text}</span>
        </div>
      </section>
    {/if}
  </div>
</div>

<style>
  .about-page {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    max-width: 1100px;
  }

  .page-header {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .page-title {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .page-subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .tab-content {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

  .hero {
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-10);
    background:
      radial-gradient(
        900px 240px at 0% 0%,
        color-mix(in srgb, var(--color-primary) 10%, transparent),
        transparent 70%
      ),
      var(--color-surface);
  }

  .eyebrow {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    letter-spacing: 1px;
    text-transform: uppercase;
    color: var(--color-primary);
  }

  .hero-headline {
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-bold);
    line-height: var(--line-height-tight);
    color: var(--color-text);
    margin-top: var(--space-3);
    max-width: 620px;
  }

  .hero-text {
    font-size: var(--font-size-base);
    line-height: var(--line-height-loose);
    color: var(--color-text-muted);
    margin-top: var(--space-4);
    max-width: 620px;
  }

  .hero-actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-3);
    margin-top: var(--space-6);
  }

  .section-label {
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-semibold);
    text-transform: uppercase;
    letter-spacing: 0.8px;
    color: var(--color-text-muted);
    margin-bottom: var(--space-4);
  }

  .feature-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-4);
  }

  .feature-card {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: var(--space-5);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    background: var(--color-bg);
    font-family: inherit;
    text-align: left;
    cursor: pointer;
    transition:
      border-color var(--transition-fast),
      transform var(--transition-fast);
  }

  .feature-card:hover {
    border-color: var(--color-primary);
    transform: translateY(-2px);
  }

  .feature-card-name {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
  }

  .feature-card-desc {
    font-size: var(--font-size-xs);
    line-height: var(--line-height-base);
    color: var(--color-text-muted);
    margin-top: var(--space-1);
  }

  .feature-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    border-radius: var(--radius-md);
    font-size: var(--font-size-xl);
    margin-bottom: var(--space-3);
  }

  .feature-icon.lg {
    width: 52px;
    height: 52px;
    font-size: var(--font-size-2xl);
    margin-bottom: 0;
    flex-shrink: 0;
  }

  .feature-detail {
    display: flex;
    flex-direction: column;
  }

  .feature-head {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    margin-bottom: var(--space-5);
  }

  .feature-title {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  .feature-tagline {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin-top: 2px;
  }

  .feature-lead {
    font-size: var(--font-size-base);
    line-height: var(--line-height-loose);
    color: var(--color-text);
    max-width: 680px;
    margin-bottom: var(--space-6);
  }

  .cap-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-3) var(--space-6);
    margin-bottom: var(--space-6);
  }

  .cap {
    display: flex;
    align-items: flex-start;
    gap: var(--space-3);
    font-size: var(--font-size-sm);
    line-height: var(--line-height-base);
    color: var(--color-text);
  }

  .cap-check {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
    border-radius: var(--radius-full);
    background: color-mix(in srgb, var(--color-success) 14%, transparent);
    color: var(--color-success);
    font-size: var(--font-size-xs);
    flex-shrink: 0;
    margin-top: 1px;
  }

  .callout {
    display: flex;
    gap: var(--space-3);
    padding: var(--space-4);
    border-radius: var(--radius-lg);
    background: var(--color-surface);
    border-left: 3px solid var(--color-primary);
    font-size: var(--font-size-sm);
    line-height: var(--line-height-base);
    color: var(--color-text-muted);
    max-width: 680px;
    box-shadow: var(--shadow-card);
  }

  .callout strong {
    color: var(--color-text);
  }

  @media (max-width: 768px) {
    .hero {
      padding: var(--space-6);
    }

    .feature-grid {
      grid-template-columns: 1fr;
    }

    .cap-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
