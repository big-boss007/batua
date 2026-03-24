export type { ReferralProgram, ReferralCode, ReferralAnalytics, ReferralConversion } from './types';

export { referralProgram, referralCodes } from './store';
export {
  fetchProgram,
  createProgram,
  updateProgram,
  createCode,
  fetchCodeByCode,
  processConversion,
  fetchAnalytics,
  fetchConversions,
  fetchMerchantCodes,
  resolveCustomerByPhone
} from './remote';
