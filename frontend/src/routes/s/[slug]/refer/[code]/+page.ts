import type { PageLoad } from './$types';
import { fetchReferralCode, fetchReferralProgram } from '$lib/client/modules/storefront';
import type { ReferralCodeInfo, ReferralProgramInfo, StorefrontMerchant } from '$lib/client/modules/storefront';

export const load: PageLoad = async ({ params, parent }) => {
  const parentData = await parent();
  const merchant = parentData.merchant as StorefrontMerchant | null;

  let referralCode: ReferralCodeInfo | null = null;
  let program: ReferralProgramInfo | null = null;

  const codeResult = await fetchReferralCode(params.code);
  if (codeResult.tag === 'success') {
    referralCode = codeResult.data;
  }

  if (merchant !== null) {
    const programResult = await fetchReferralProgram(merchant.id);
    if (programResult.tag === 'success') {
      program = programResult.data;
    }
  }

  return {
    referralCode,
    program,
    code: params.code
  };
};
