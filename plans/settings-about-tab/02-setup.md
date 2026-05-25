# Phase 2: Setup

## Objective
Bundle the marketing site into the app's static assets.

## Tasks
- Create directory `frontend/static/website/`.
- Copy from `site/` into it: `index.html`, `pricing.html`, `styles.css`, `script.js`.
  Do not copy `node_modules/`, `tests/`, `test-results/`, `package*.json`,
  `playwright.config.js`, or `.DS_Store`.
- In the copied `frontend/static/website/index.html`, change the nav logo link
  `href="/"` to `href="index.html"`.

## Outputs
- `frontend/static/website/{index.html,pricing.html,styles.css,script.js}`

## Validation
- `/website/index.html`, `/website/pricing.html`, `/website/styles.css`,
  `/website/script.js` all return 200 from the dev server.
- The bundled homepage renders styled and navigates to the pricing page.
