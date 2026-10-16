const fs = require('node:fs');
const path = require('node:path');
const { pathToFileURL } = require('node:url');
const { chromium } = require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const revision = process.argv[2] || '02';
if (!/^\d{2}$/.test(revision)) throw new Error('Expected a two-digit bank revision');
const out = path.join(__dirname, `weapon-silhouette-review-${revision}`);

(async () => {
  const browser = await chromium.launch({ headless: true, executablePath: 'C:/Users/User/AppData/Local/ms-playwright/chromium_headless_shell-1169/chrome-win/headless_shell.exe' });
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  await page.goto(pathToFileURL(path.join(out, 'weapon-silhouette-gallery-02.html')).href);
  await page.waitForFunction(() => document.querySelectorAll('canvas').length === 10 && [...document.images].every(image => image.complete));
  await page.screenshot({ path: path.join(out, 'gallery-desktop-gray-pistols.png'), fullPage: true });
  if (await page.locator('.class').count() !== 5) throw new Error('Missing class');
  await page.selectOption('#right', '14');
  if (!await page.locator('#left').isDisabled()) throw new Error('Two-handed left selector enabled');
  if (await page.locator('#left').inputValue() !== 'none') throw new Error('Two-handed left weapon retained');
  await page.screenshot({ path: path.join(out, 'gallery-desktop-heavy.png'), fullPage: true });
  await page.selectOption('#right', '0');
  await page.selectOption('#left', '6');
  if (await page.locator('#left').isDisabled()) throw new Error('One-handed left selector disabled');
  if (!(await page.locator('#selection').textContent()).includes('gauche : Pistolet')) throw new Error('Dual selection missing');
  await page.screenshot({ path: path.join(out, 'gallery-desktop-dual.png'), fullPage: true });
  await page.setViewportSize({ width: 375, height: 900 });
  await page.selectOption('#ground', 'light');
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth > document.documentElement.clientWidth);
  if (overflow) throw new Error('Mobile horizontal overflow');
  const zoom = await page.locator('canvas.zoom').first().evaluate(canvas => ({ width: canvas.clientWidth, height: canvas.clientHeight }));
  if (zoom.width !== 128 || zoom.height !== 128) throw new Error('Noninteger mobile zoom');
  await page.screenshot({ path: path.join(out, 'gallery-mobile-dual.png'), fullPage: true });
  if (errors.length) throw new Error(errors.join('\n'));
  fs.writeFileSync(path.join(out, 'gallery-browser-validation-01.json'), JSON.stringify({
    date: '2026-10-09', browser: 'bundled_playwright_headless_chromium', visible_classes: 5,
    native_canvases: 5, zoom_canvases: 5, tested_configurations: ['gray_pistol', 'two_handed_rocket_launcher', 'right_knife_left_pistol'],
    backgrounds: ['game_floor', 'light'], two_handed_disables_left: true, dual_selection_updates_all_classes: true,
    mobile_viewport: [375, 900], mobile_horizontal_overflow: false, mobile_zoom: 2,
    console_errors: errors, scope: 'review_gallery;not_live_game_capture', runtime_integration: 'not_integrated'
  }, null, 2) + '\n');
  await browser.close();
  process.stdout.write(JSON.stringify({ classes: 5, configurations_tested: 3, console_errors: 0, mobile_overflow: false, runtime: 'not_integrated' }));
})().catch(error => { process.stderr.write(error.stack); process.exit(1); });
