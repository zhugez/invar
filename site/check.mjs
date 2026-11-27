import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';

const session = 'invar-check';
const base = process.argv[2] || 'http://127.0.0.1:8787';
const browser = (...args) => execFileSync('agent-browser', ['--session', session, ...args], { encoding: 'utf8', stdio: ['pipe', 'pipe', 'pipe'] });
const evaluate = code => JSON.parse(browser('eval', code));

try {
  browser('open', base);
  for (const width of [1440, 768, 390, 320]) {
    browser('set', 'viewport', String(width), '1000');
    assert.equal(evaluate('document.documentElement.scrollWidth <= innerWidth'), true, `Horizontal overflow at ${width}px`);
  }
  browser('click', '#copy-command');
  const result = evaluate(`(async () => {
    await document.fonts.ready;
    const assert = (value, message) => { if (!value) throw Error(message); };
    assert(document.querySelectorAll('h1').length === 1, 'Expected a single main heading');
    assert(document.fonts.check('16px Barlow'), 'Brand font did not load');
    for (const link of document.querySelectorAll('a[href^="#"]')) {
      assert(document.getElementById(link.hash.slice(1)), 'Broken anchor: ' + link.hash);
    }
    for (const scenario of ['change', 'match']) {
      document.querySelector('[data-scenario="' + scenario + '"]').click();
      const changed = scenario === 'change';
      assert(document.querySelector('.terminal').classList.contains('changed') === changed, 'Wrong result styling');
      assert(document.getElementById('exit-code').textContent === (changed ? '1' : '0'), 'Wrong exit code');
      assert(document.getElementById('current-output').textContent === (changed ? 'carol\\nbob\\nalice' : 'alice\\nbob\\ncarol'), 'Wrong output');
      assert(document.querySelectorAll('[aria-pressed="true"]').length === 1, 'Ambiguous demo selection');
    }
    for (let i = 0; i < 50 && !document.getElementById('copy-feedback').textContent; i++) {
      await new Promise(resolve => setTimeout(resolve, 20));
    }
    assert(document.getElementById('copy-feedback').textContent === 'Install command copied.', 'Clipboard copy failed');
    const assets = ['/style.css', '/main.js', '/assets/Barlow-Regular.ttf', '/assets/Barlow-Bold.ttf', '/assets/favicon.svg', '/assets/logo-reversed.svg', '/assets/social-card.png', '/robots.txt', '/sitemap.xml'];
    for (const asset of assets) assert((await fetch(asset)).ok, 'Missing asset: ' + asset);
    return { responsiveWidths: [1440, 768, 390, 320], scenarios: ['changed', 'matching'], clipboard: 'passed', assets: assets.length };
  })()`);
  console.log(JSON.stringify(result, null, 2));
} finally {
  browser('close');
}
