function generateWhatsAppShareUrl(text: string): string {
  return `https://wa.me/?text=${encodeURIComponent(text)}`;
}

async function copyToClipboard(text: string): Promise<boolean> {
  try {
    await navigator.clipboard.writeText(text);
    return true;
  } catch {
    return false;
  }
}

const bucketLabels: Record<string, string> = {
  earned_credit: 'Earned Credit',
  cod_pending: 'COD Pending',
  gift_card: 'Gift Card',
  customer_funded: 'Customer Funded',
  referral_reward: 'Referral Reward',
  goodwill_credit: 'Goodwill Credit',
  membership_benefit: 'Membership Benefit',
  refund_credit: 'Refund Credit',
  EarnedCredit: 'Earned Credit',
  CodPending: 'COD Pending',
  GiftCard: 'Gift Card',
  CustomerFunded: 'Customer Funded',
  ReferralReward: 'Referral Reward',
  GoodwillCredit: 'Goodwill Credit',
  MembershipBenefit: 'Membership Benefit',
  RefundCredit: 'Refund Credit'
};

function formatBucketLabel(type: string): string {
  return bucketLabels[type] ?? type.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
}

const movementLabels: Record<string, { label: string; prefix: string }> = {
  In: { label: 'Earned', prefix: '+' },
  Out: { label: 'Redeemed', prefix: '-' },
  Held: { label: 'Held', prefix: '' },
  Released: { label: 'Released', prefix: '+' },
  Expired: { label: 'Expired', prefix: '-' },
  Across: { label: 'Transferred', prefix: '' }
};

function formatMovementLabel(type: string): string {
  const entry = movementLabels[type];
  if (entry !== undefined) return entry.label;
  return type.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());
}

function getMovementPrefix(type: string): string {
  const entry = movementLabels[type];
  if (entry !== undefined) return entry.prefix;
  return '';
}

function maskPhone(phone: string): string {
  const digits = phone.replace(/\D/g, '');
  if (digits.length < 4) return phone;
  const last4 = digits.slice(-4);
  const masked = '*'.repeat(Math.max(0, digits.length - 4));
  if (digits.length > 10) {
    return `+${digits.slice(0, digits.length - 10)}${masked.slice(0, masked.length)}${last4}`;
  }
  return `${masked}${last4}`;
}

type DateGroup = {
  date: string;
  label: string;
  entries: Array<{ id: string; created_at: string; [key: string]: unknown }>;
};

function formatDateLabel(date: Date): string {
  const now = new Date();
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const target = new Date(date.getFullYear(), date.getMonth(), date.getDate());
  const diffMs = today.getTime() - target.getTime();
  const diffDays = Math.round(diffMs / (1000 * 60 * 60 * 24));

  if (diffDays === 0) return 'Today';
  if (diffDays === 1) return 'Yesterday';

  const sameYear = date.getFullYear() === now.getFullYear();
  const monthDay = date.toLocaleDateString('en-IN', { month: 'short', day: 'numeric' });
  if (sameYear) return monthDay;
  return `${monthDay}, ${date.getFullYear()}`;
}

function groupEntriesByDate<T extends { id: string; created_at: string }>(
  entries: Array<T>
): Array<{ date: string; label: string; entries: Array<T> }> {
  const groups: Array<{ date: string; label: string; entries: Array<T> }> = [];
  let currentDate = '';

  for (const entry of entries) {
    const d = new Date(entry.created_at);
    const dateKey = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;

    if (dateKey !== currentDate) {
      currentDate = dateKey;
      groups.push({ date: dateKey, label: formatDateLabel(d), entries: [] });
    }
    groups[groups.length - 1].entries.push(entry);
  }

  return groups;
}

function computeRunningBalances(
  entries: Array<{ id: string; movement_type: string; currency_equivalent: number }>,
  currentBalance: number
): Map<string, number> {
  const balances = new Map<string, number>();
  let running = currentBalance;

  for (const entry of entries) {
    balances.set(entry.id, running);
    const amount = Math.abs(entry.currency_equivalent);
    switch (entry.movement_type) {
      case 'In':
      case 'Released':
        running -= amount;
        break;
      case 'Out':
      case 'Expired':
        running += amount;
        break;
    }
  }

  return balances;
}

function getInitials(name: string | null): string {
  if (name === null || name.trim() === '') return '?';
  return name
    .trim()
    .split(/\s+/)
    .map((w) => w.charAt(0).toUpperCase())
    .slice(0, 2)
    .join('');
}

export {
  generateWhatsAppShareUrl,
  copyToClipboard,
  formatBucketLabel,
  formatMovementLabel,
  getMovementPrefix,
  maskPhone,
  formatDateLabel,
  groupEntriesByDate,
  computeRunningBalances,
  getInitials
};

export type { DateGroup };
