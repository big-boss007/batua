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
  cashback: 'Cashback',
  loyalty_points: 'Loyalty Points',
  referral_bonus: 'Referral Bonus',
  gift_card: 'Gift Card',
  promotional: 'Promotional',
  store_credit: 'Store Credit',
  signup_bonus: 'Signup Bonus'
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

export {
  generateWhatsAppShareUrl,
  copyToClipboard,
  formatBucketLabel,
  formatMovementLabel,
  getMovementPrefix,
  maskPhone
};
