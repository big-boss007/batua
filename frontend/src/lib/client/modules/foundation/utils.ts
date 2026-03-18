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

export { formatCurrencyINR, formatDate, formatDateTime, normalizePhoneE164 };
