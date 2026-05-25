import { execSync } from 'child_process';
import { resolve } from 'path';

const dir = '/Users/chirag/Developer/batua/plans/storefront-redesign';
const files = [
  'concept-1-glassmorphism',
  'concept-2-gamified',
  'concept-3-minimal'
];

const chrome = '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';

for (const f of files) {
  const html = resolve(dir, `${f}.html`);
  const pdf = resolve(dir, `${f}.pdf`);
  console.log(`Converting ${f}...`);
  execSync(`"${chrome}" --headless --disable-gpu --print-to-pdf="${pdf}" --no-margins "file://${html}"`, {
    stdio: 'inherit'
  });
  console.log(`  -> ${pdf}`);
}

console.log('Done!');
