# Marketing Site — Test Plan

## Viewports

| Name | Width | Height | Device |
|------|-------|--------|--------|
| Mobile S | 320 | 568 | iPhone SE |
| Mobile M | 375 | 812 | iPhone 14 |
| Mobile L | 428 | 926 | iPhone 14 Pro Max |
| Tablet | 768 | 1024 | iPad |
| Laptop | 1280 | 800 | MacBook Air |
| Desktop | 1440 | 900 | Standard desktop |
| Wide | 1920 | 1080 | Full HD |

## Test Categories

### 1. Layout — No Overflow
- Body scrollWidth must equal window innerWidth (no horizontal scroll)
- No element extends beyond viewport right edge (except inside scroll wrappers)

### 2. Navigation
- **Desktop (>768px):** All 7 nav links + CTA visible in horizontal row
- **Mobile (<=768px):** Hamburger icon visible, nav links hidden
- **Hamburger open:** Full-screen white overlay, all links visible, X button visible
- **Hamburger close:** Overlay disappears, hero visible, hamburger icon back
- **Link click closes menu:** Clicking a nav link closes the overlay and scrolls to section
- **Sticky nav:** Nav stays at top on scroll with blur background

### 3. Hero Section
- Eyebrow "BREEZE RETENTION SUITE" visible
- Headline readable (not clipped, not overflowing)
- 6 pill badges wrap properly on small screens
- CTA buttons stack vertically on mobile

### 4. Tables (India-native + Competitors)
- **Desktop:** Full table visible, all columns including Breeze
- **Mobile:** Table scrollable horizontally, "← swipe →" hint visible
- All table content accessible via scroll

### 5. Pricing Cards
- **Desktop:** 4 cards in a row
- **Tablet:** 2 cards per row
- **Mobile:** 1 card per row, full width
- "Most Popular" badge visible on Scale plan

### 6. Feature Sections (Wallet, Loyalty, Gift Cards, Referrals, Campaigns, Memberships)
- Headlines readable at all sizes
- Mockups (WhatsApp bubble, checkout card, gift card, referral code, spin wheel, membership cards) don't overflow
- Tier cards (Bronze→Platinum) stack vertically on mobile
- Membership cards stack vertically on mobile
- Campaign grid wraps properly

### 7. Dashboard Mockup
- Tabs wrap on small screens
- Metrics grid adapts (3-col desktop → 2-col tablet → 1-col mobile)
- No overflow from mockup container

### 8. Footer
- **Desktop:** 4 columns (brand + 3 link groups)
- **Mobile:** Single column, stacked
- Copyright and tagline visible

### 9. Scroll Animations
- Elements fade up on scroll (IntersectionObserver)
- Animations trigger correctly at all viewports

### 10. Accessibility
- All interactive elements keyboard-focusable
- aria-expanded toggles on hamburger
- Images have alt text
- Color contrast sufficient (text on backgrounds)
