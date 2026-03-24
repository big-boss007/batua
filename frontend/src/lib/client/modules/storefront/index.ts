export type {
  StorefrontMerchant,
  CustomerBalance,
  BucketBalance,
  ExpiringSoon,
  CustomerTierInfo,
  TierProgress,
  TransactionEntry,
  GiftCardInfo,
  ReferralCodeInfo,
  ReferralLandingInfo,
  ReferralProgramInfo,
  CustomerIdentity,
  WalletLookup
} from './types';

export {
  fetchMerchantBySlug,
  lookupCustomer,
  lookupWallet,
  fetchBalance,
  fetchEntries,
  fetchCustomerTier,
  fetchGiftCard,
  claimGiftCardForCustomer,
  fetchReferralCode,
  fetchCustomerReferralCode,
  fetchReferralProgram
} from './remote';

export { customerPhone, merchantContext } from './store';

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
} from './utils';

export type { DateGroup } from './utils';
