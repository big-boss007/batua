import os, sys

SHOT_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'docs', 'screenshots-v2')
PDF_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'docs', 'feature-pdfs-v2')
os.makedirs(PDF_DIR, exist_ok=True)

FEATURES = {
    'wallet': {
        'title': '1. Wallet & Dashboard',
        'screens': [
            ('01-dashboard.png', 'Merchant Admin — Dashboard', 'Merchant-scoped metrics: active customers, credits, earned, redeemed, COD pending, redemptions'),
            ('02-dashboard-empty.png', 'Merchant Admin — Dashboard (Low Activity)', 'BookWorm merchant with minimal data — shows empty/low state'),
            ('03-transactions-feed.png', 'Merchant Admin — Transactions Feed', 'Merchant-wide transaction feed showing all customers, movement types, amounts'),
            ('04-transactions-filtered.png', 'Merchant Admin — Transactions (Filtered)', 'Filtered view with bucket type and movement type dropdowns applied'),
            ('05-customers-list.png', 'Merchant Admin — Customer List', 'Browsable list of all customers with name, phone, email, joined date'),
            ('06-customer-detail.png', 'Merchant Admin — Customer Detail', 'Customer profile with wallet balance (displayed vs spendable), recent transactions'),
            ('07-customer-search.png', 'Merchant Admin — Customer Search', 'Search by phone number with debounced results'),
            ('08-settings-store.png', 'Merchant Admin — My Store Settings', 'Editable merchant profile: name, domain, slug, plan tier, storefront URL'),
            ('09-settings-policies.png', 'Merchant Admin — Wallet Policies', 'Per-bucket redemption constraints: min, max %, stackability, expiry'),
            ('10-storefront-landing.png', 'Customer — Loyalty Hub Landing', 'Mobile-first phone entry screen at /s/desi-threads'),
            ('11-storefront-balance.png', 'Customer — Wallet Balance', 'Spendable balance card with bucket breakdown after phone entry'),
            ('12-storefront-transactions.png', 'Customer — Recent Transactions', 'Transaction history with color-coded movement types'),
            ('13-balance-check.png', 'Customer — Quick Balance Check', 'Lightweight balance lookup at /s/desi-threads/balance'),
        ]
    },
    'loyalty': {
        'title': '2. Loyalty Tiers & Rewards',
        'screens': [
            ('01-loyalty-program.png', 'Merchant Admin — Loyalty Program', 'Program configuration with tiers, tier distribution chart'),
            ('02-reward-rules.png', 'Merchant Admin — Reward Rules', 'Rules table with inline performance stats: entries, value, customers per rule'),
            ('03-reward-rules-detail.png', 'Merchant Admin — Rule Editor', 'Create/edit modal for reward rule with conditions builder'),
            ('04-onboarding-wizard.png', 'Merchant Admin — Onboarding Wizard', '6-step setup for new merchants: rewards, policy, tiers, referrals'),
            ('05-tier-progress.png', 'Customer — Tier Progress', 'Customer\'s loyalty tier shown in the storefront loyalty hub'),
        ]
    },
    'gift-cards': {
        'title': '3. Gift Cards',
        'screens': [
            ('01-gift-cards-list.png', 'Merchant Admin — Gift Cards List', 'Gift card inventory with stats: issued, outstanding, active, claimed, expired'),
            ('02-gift-cards-issue.png', 'Merchant Admin — Issue Gift Card', 'Single issue form with amount and optional expiry'),
            ('03-gift-card-check.png', 'Customer — Gift Card Balance Check', 'Code entry page at /s/desi-threads/gift-cards/check'),
        ]
    },
    'referrals': {
        'title': '4. Referral Program',
        'screens': [
            ('01-program-config.png', 'Merchant Admin — Referral Program Config', 'Referrer/referee reward amounts, max per customer, active toggle'),
            ('02-codes-list.png', 'Merchant Admin — Referral Codes', 'All codes with vanity/creator badges, conversion count, status'),
            ('03-analytics.png', 'Merchant Admin — Referral Analytics', 'Total codes, referrals, conversions, conversion rate, suspicious count'),
            ('04-conversions.png', 'Merchant Admin — Referral Conversions', 'Conversion history with fraud signal badges'),
            ('05-referral-landing.png', 'Customer — Referral Landing Page', '"You\'ve been invited! Get ₹25" with code display and Shop Now CTA'),
            ('06-my-referrals.png', 'Customer — My Referrals', 'Phone entry to view own referral code and share buttons'),
        ]
    },
    'campaigns': {
        'title': '5. Campaigns & Gamification',
        'screens': [
            ('01-campaigns-list.png', 'Merchant Admin — Campaigns', 'Active and upcoming campaigns with festive template grid'),
            ('02-festive-templates.png', 'Merchant Admin — Festive Templates', 'Pre-built templates: Diwali, Navratri, Holi, Rakhi, etc.'),
        ]
    },
    'analytics': {
        'title': '6. Analytics & COD',
        'screens': [
            ('01-analytics-overview.png', 'Merchant Admin — Analytics Overview', 'Total earned/redeemed/active/expired with COD breakdown'),
            ('02-analytics-scroll.png', 'Merchant Admin — COD & RTO Analysis', 'COD pending/delivered/RTO, prepaid vs COD split, RTO loyalty comparison'),
        ]
    },
    'memberships': {
        'title': '7. Notifications & Connectors',
        'screens': [
            ('01-settings-connectors.png', 'Merchant Admin — Connectors', 'WhatsApp BSP, SMS connector configuration'),
            ('02-notifications-templates.png', 'Merchant Admin — Notification Templates', 'Template editor with variable injection ({{customer_name}}, {{amount}})'),
            ('03-notifications-logs.png', 'Merchant Admin — Notification Logs', 'Send history: channel, status, timestamp'),
        ]
    },
    'platform': {
        'title': '8. Platform Admin (Super-Admin)',
        'screens': [
            ('01-dashboard.png', 'Platform — System Dashboard', 'System-wide stats: 10 merchants, 1300+ wallets, 12K+ entries, ₹5.9L credits'),
            ('02-merchants-list.png', 'Platform — All Merchants', '10 merchants with plan tier, status, search, onboard button'),
            ('03-merchant-detail.png', 'Platform — Desi Threads Detail', 'Per-merchant stats, plan management, "Open as Merchant" button'),
            ('04-geo-policies.png', 'Platform — Geo Policies', 'India config: COD enabled, WhatsApp default, UPI, INR currency'),
            ('05-system-health.png', 'Platform — System Health', 'Queue depths: unprocessed events, failed events, pending COD, expiring credits'),
            ('06-merchant-detail-2.png', 'Platform — TechBazaar Detail', 'Different merchant for variety — scale plan, different stats'),
        ]
    },
}

for feature, data in FEATURES.items():
    html_path = os.path.join(PDF_DIR, f'{feature}.html')
    pdf_path = os.path.join(PDF_DIR, f'{feature}.pdf')

    slides_html = ''
    actual_count = 0
    for i, (img, title, desc) in enumerate(data['screens']):
        img_path = os.path.abspath(os.path.join(SHOT_DIR, feature, img))
        if not os.path.exists(img_path):
            continue
        actual_count += 1
        slides_html += f'''
    <div class="slide">
      <div class="slide-header">
        <span class="slide-feature">{data['title']}</span>
        <span class="slide-num">{actual_count}/{len(data['screens'])}</span>
      </div>
      <div class="slide-body">
        <img src="file://{img_path}" alt="{title}">
      </div>
      <div class="slide-footer">
        <p class="slide-title">{title}</p>
        <p class="slide-desc">{desc}</p>
      </div>
    </div>
'''

    html = f'''<!DOCTYPE html>
<html><head>
<meta charset="UTF-8">
<title>{data['title']} — Breeze Product Demo</title>
<style>
  @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');
  @page {{ size: landscape; margin: 0; }}
  * {{ margin: 0; padding: 0; box-sizing: border-box; }}
  body {{ font-family: Inter, -apple-system, sans-serif; background: #1d1d1f; color: #fff; }}
  .slide {{
    width: 1280px; height: 720px;
    display: flex; flex-direction: column;
    background: #1d1d1f;
    page-break-after: always; break-after: page;
    position: relative;
  }}
  .slide-header {{
    display: flex; justify-content: space-between; align-items: center;
    padding: 14px 40px;
    font-size: 13px;
  }}
  .slide-feature {{ color: #7c6aff; font-weight: 600; text-transform: uppercase; letter-spacing: 0.08em; font-size: 12px; }}
  .slide-num {{ color: #555; }}
  .slide-body {{
    flex: 1; display: flex; align-items: center; justify-content: center;
    padding: 0 40px;
  }}
  .slide-body img {{
    max-width: 1100px; max-height: 530px;
    border-radius: 10px; border: 1px solid rgba(255,255,255,0.08);
    box-shadow: 0 8px 40px rgba(0,0,0,0.5);
    object-fit: contain;
  }}
  .slide-footer {{
    padding: 12px 40px 20px;
    text-align: center;
  }}
  .slide-title {{ font-size: 16px; font-weight: 600; color: #fff; margin-bottom: 3px; }}
  .slide-desc {{ font-size: 13px; color: #86868b; }}
</style>
</head><body>
{slides_html}
</body></html>'''

    with open(html_path, 'w') as f:
        f.write(html)

    os.system(f'"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --headless=new --disable-gpu --print-to-pdf="{pdf_path}" --print-to-pdf-no-header --no-margins "file://{html_path}" 2>/dev/null')

    size = os.path.getsize(pdf_path) // 1024 if os.path.exists(pdf_path) else 0
    print(f'  {feature}.pdf — {actual_count} slides, {size}KB')

print('\nAll PDFs generated at docs/feature-pdfs-v2/')
