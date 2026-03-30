const inrFormatter = new Intl.NumberFormat('en-IN', {
  style: 'currency',
  currency: 'INR',
  minimumFractionDigits: 2,
  maximumFractionDigits: 2
});

function formatCurrencyINR(amount: number): string {
  return inrFormatter.format(amount);
}

const dateFormatter = new Intl.DateTimeFormat('en-IN', {
  year: 'numeric',
  month: 'short',
  day: 'numeric'
});

const dateTimeFormatter = new Intl.DateTimeFormat('en-IN', {
  year: 'numeric',
  month: 'short',
  day: 'numeric',
  hour: '2-digit',
  minute: '2-digit'
});

function formatDate(iso: string): string {
  const date = new Date(iso);
  return dateFormatter.format(date);
}

function formatDateTime(iso: string): string {
  const date = new Date(iso);
  return dateTimeFormatter.format(date);
}

function normalizePhoneE164(phone: string, countryCode: string = '91'): string {
  const digits = phone.replace(/\D/g, '');

  if (digits.startsWith(countryCode)) {
    return `+${digits}`;
  }

  if (digits.startsWith('0')) {
    return `+${countryCode}${digits.slice(1)}`;
  }

  return `+${countryCode}${digits}`;
}

const POINTS_BUCKETS = new Set([
  'earned_credit',
  'cod_pending',
  'referral_reward',
  'goodwill_credit',
  'membership_benefit',
  'EarnedCredit',
  'CodPending',
  'ReferralReward',
  'GoodwillCredit',
  'MembershipBenefit'
]);

function isPointsBucket(bucketType: string): boolean {
  return POINTS_BUCKETS.has(bucketType);
}

function formatPoints(value: number, icon: string): string {
  return `${value.toLocaleString('en-IN')} ${icon}`;
}

function formatPhone(phone: string): string {
  if (phone.startsWith('+91') && phone.length === 13) {
    const digits = phone.slice(3);
    return `+91 ${digits.slice(0, 5)} ${digits.slice(5)}`;
  }
  return phone;
}

const MODAL_CLOSE_ICON =
  "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='20' height='20' viewBox='0 0 24 24' fill='none' stroke='%23666' stroke-width='2' stroke-linecap='round'%3E%3Cpath d='M18 6L6 18M6 6l12 12'/%3E%3C/svg%3E";

export {
  formatCurrencyINR,
  formatDate,
  formatDateTime,
  normalizePhoneE164,
  formatPhone,
  formatPoints,
  isPointsBucket,
  MODAL_CLOSE_ICON
};
