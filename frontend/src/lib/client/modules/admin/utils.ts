type MetricType = 'number' | 'currency' | 'percentage' | 'points';

const numberFormatter = new Intl.NumberFormat('en-IN');

const currencyFormatter = new Intl.NumberFormat('en-IN', {
  style: 'currency',
  currency: 'INR',
  minimumFractionDigits: 2,
  maximumFractionDigits: 2
});

function formatMetricValue(value: number, metricType: MetricType): string {
  if (metricType === 'currency') {
    return currencyFormatter.format(value);
  }
  if (metricType === 'percentage') {
    return `${value.toFixed(1)}%`;
  }
  return numberFormatter.format(value);
}

const MERCHANT_ID_KEY = 'batua_merchant_id';

function getCurrentMerchantId(): string | null {
  if (typeof window === 'undefined') return null;
  try {
    return localStorage.getItem(MERCHANT_ID_KEY);
  } catch {
    return null;
  }
}

function setCurrentMerchantId(id: string): void {
  if (typeof window === 'undefined') return;
  try {
    localStorage.setItem(MERCHANT_ID_KEY, id);
  } catch {
    // localStorage unavailable
  }
}

export { formatMetricValue, getCurrentMerchantId, setCurrentMerchantId, MERCHANT_ID_KEY };
export type { MetricType };
