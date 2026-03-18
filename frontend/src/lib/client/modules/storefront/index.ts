export type {
  StorefrontMerchant,
  CustomerBalance,
  BucketBalance,
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
  maskPhone
} from './utils';
