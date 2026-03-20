import os
import sys

SHOT_DIR = os.path.join(os.path.dirname(__file__), '..', 'docs', 'screenshots')
PDF_DIR = os.path.join(os.path.dirname(__file__), '..', 'docs', 'feature-pdfs')
os.makedirs(PDF_DIR, exist_ok=True)

FEATURES = {
    'wallet': {
        'title': 'Wallet & Dashboard',
        'screens': [
            ('01-dashboard.png', 'Merchant Admin — Dashboard', 'Merchant-scoped metrics: customers, credits, earned, redeemed, COD pending'),
            ('02-transactions.png', 'Merchant Admin — Transactions', 'Merchant-wide transaction feed with customer names, movement types, filters'),
            ('03-customers-list.png', 'Merchant Admin — Customer List', 'Browsable customer list with search, click for detail'),
            ('04-customer-detail.png', 'Merchant Admin — Customer Detail', 'Customer profile, wallet balance (displayed vs spendable), recent transactions'),
            ('05-settings-store.png', 'Merchant Admin — Settings', 'My Store tab: name, domain, slug, plan tier, storefront URL'),
            ('06-storefront-landing.png', 'Customer — Loyalty Hub Landing', 'Phone entry to view rewards (mobile-first)'),
            ('07-storefront-balance.png', 'Customer — Wallet Balance', 'Spendable balance, bucket breakdown, tier status'),
            ('08-storefront-transactions.png', 'Customer — Recent Transactions', 'Transaction history with movement types and amounts'),
            ('09-balance-check.png', 'Customer — Quick Balance Check', 'Lightweight phone-based balance lookup'),
        ]
    },
    'loyalty': {
        'title': 'Loyalty Tiers & Rewards',
        'screens': [
            ('01-loyalty-program.png', 'Merchant Admin — Loyalty Program', 'Program config, tier management, tier distribution chart'),
            ('02-reward-rules.png', 'Merchant Admin — Reward Rules', 'Rules table with per-rule performance stats (entries, value, customers)'),
        ]
    },
    'gift-cards': {
        'title': 'Gift Cards',
        'screens': [
            ('01-gift-cards.png', 'Merchant Admin — Gift Cards', 'Issue, bulk upload, list with stats (issued, outstanding, active, claimed)'),
            ('02-gift-card-check.png', 'Customer — Gift Card Balance Check', 'Enter code to check remaining balance and expiry'),
        ]
    },
    'referrals': {
        'title': 'Referral Program',
        'screens': [
            ('01-referrals-program.png', 'Merchant Admin — Referral Program', 'Program config: referrer/referee rewards, max per customer'),
            ('02-referral-codes.png', 'Merchant Admin — Referral Codes', 'All codes with vanity/creator badges, conversion count'),
            ('03-referral-landing.png', 'Customer — Referral Landing Page', '"You\'ve been invited! Get ₹25" with code and Shop Now CTA'),
            ('04-my-referrals.png', 'Customer — My Referrals', 'Phone entry to view own referral code and share'),
        ]
    },
    'campaigns': {
        'title': 'Campaigns & Gamification',
        'screens': [
            ('01-campaigns.png', 'Merchant Admin — Campaigns', 'Festive template grid, campaign builder, calendar'),
        ]
    },
    'analytics': {
        'title': 'Analytics & COD',
        'screens': [
            ('01-analytics.png', 'Merchant Admin — Analytics', 'COD breakdown, prepaid vs COD split, RTO loyalty comparison'),
        ]
    },
    'memberships': {
        'title': 'Memberships & Onboarding',
        'screens': [
            ('01-settings-policies.png', 'Merchant Admin — Wallet Policies', 'Per-bucket redemption constraints, expiry, stackability'),
        ]
    },
    'platform': {
        'title': 'Platform Admin (Super-Admin)',
        'screens': [
            ('01-platform-dashboard.png', 'Platform — System Dashboard', 'System-wide stats: merchants, wallets, credits, recent events'),
            ('02-merchants-list.png', 'Platform — Merchant List', 'All merchants with search, plan tier, status, onboard button'),
            ('03-merchant-detail.png', 'Platform — Merchant Detail', 'Per-merchant stats, plan management, "Open as Merchant" button'),
            ('04-geo-policies.png', 'Platform — Geo Policies', 'India config: COD enabled, WhatsApp default, UPI, currency'),
            ('05-system-health.png', 'Platform — System Health', 'Unprocessed events, failed events, pending COD, expiring credits'),
        ]
    },
}

# Generate HTML for each feature, then PDF via Chrome
for feature, data in FEATURES.items():
    html_path = os.path.join(PDF_DIR, f'{feature}.html')
    pdf_path = os.path.join(PDF_DIR, f'{feature}.pdf')

    slides_html = ''
    for i, (img, title, desc) in enumerate(data['screens']):
        img_path = os.path.abspath(os.path.join(SHOT_DIR, feature, img))
        if not os.path.exists(img_path):
            print(f'  SKIP {feature}/{img} — not found')
            continue
        slides_html += f'''
    <div class="slide">
      <div class="slide-header">
        <span class="slide-feature">{data['title']}</span>
        <span class="slide-num">{i+1}/{len(data['screens'])}</span>
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
    padding: 16px 40px;
    font-size: 14px; color: #86868b;
  }}
  .slide-feature {{ color: #7c6aff; font-weight: 600; text-transform: uppercase; letter-spacing: 0.08em; }}
  .slide-num {{ color: #86868b; }}
  .slide-body {{
    flex: 1; display: flex; align-items: center; justify-content: center;
    padding: 0 40px;
  }}
  .slide-body img {{
    max-width: 1100px; max-height: 520px;
    border-radius: 12px; border: 1px solid rgba(255,255,255,0.1);
    box-shadow: 0 8px 40px rgba(0,0,0,0.4);
    object-fit: contain;
  }}
  .slide-footer {{
    padding: 16px 40px 24px;
    text-align: center;
  }}
  .slide-title {{ font-size: 18px; font-weight: 600; color: #fff; margin-bottom: 4px; }}
  .slide-desc {{ font-size: 14px; color: #86868b; }}
</style>
</head><body>
{slides_html}
</body></html>'''

    with open(html_path, 'w') as f:
        f.write(html)

    # Generate PDF
    os.system(f'"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --headless=new --disable-gpu --print-to-pdf="{pdf_path}" --print-to-pdf-no-header --no-margins "file://{html_path}" 2>/dev/null')

    screen_count = len([s for s in data['screens'] if os.path.exists(os.path.join(SHOT_DIR, feature, s[0]))])
    size = os.path.getsize(pdf_path) // 1024 if os.path.exists(pdf_path) else 0
    print(f'  ✓ {feature}.pdf — {screen_count} screens, {size}KB')

print('\nDone!')
