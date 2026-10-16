// Real browser QA using the bundled Playwright runtime and installed Edge.
const fs = require('fs'), path = require('path'), assert = require('assert/strict');
const {chromium} = require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const root = path.resolve(__dirname, '../..');
async function main() {
  const browser = await chromium.launch({channel: 'msedge', headless: true});
  try {
    for (const number of process.argv.length > 2 ? process.argv.slice(2).map(Number) : [62, 63]) {
      const folder = path.join(root, `artifacts/refonte-agent-2026-10-06/skill-animation-review-${number}`);
      const page = await browser.newPage({viewport: {width: 1320, height: 1180}}), errors = [];
      page.on('pageerror', error => errors.push(error.message));
      page.on('requestfailed', request => errors.push(request.url() + ': ' + request.failure()?.errorText));
      await page.goto('file:///' + path.join(folder, 'cote-a-cote.html').replaceAll('\\', '/'));
      await page.waitForFunction(() => typeof ready !== 'undefined' && ready);
      await page.waitForFunction(() => launched && turn === 3 && elapsed >= 165 && elapsed < 310);
      await page.evaluate(() => {paused = true; last = 0;});
      await page.waitForTimeout(70);
      await page.screenshot({path: path.join(folder, 'browser-visible-explosion.png'), fullPage: false});
      const snapshot = await page.evaluate(() => ({turn, elapsed, status: document.getElementById('status').textContent,
        canvases: [...document.querySelectorAll('canvas')].map(canvas => {
          const e = entries().find(e => e.id === canvas.dataset.skill), f = frame(e);
          return {id: e.id, frame: f, explosions: e.timeline[f].explosions,
            expected_native_png: `frames/${e.variant}/${String(e.image_indices[f]).padStart(3, '0')}.png`};
        })}));
      await page.check('#native-view');
      for (const canvas of snapshot.canvases) {
        assert(canvas.frame >= 115 && canvas.frame <= 120); assert.equal(canvas.explosions, 1);
        canvas.browser_png = `browser-canvas-${canvas.id}.png`;
        await page.locator(`canvas[data-skill="${canvas.id}"]`).screenshot({path: path.join(folder, canvas.browser_png)});
      }
      await page.click('#demo');
      const finished = () => page.waitForFunction(() => ready && launched && !autoPlay && entries().every(e => e.timeline[frame(e)].explosions === 1));
      await finished();
      const state = () => page.evaluate(() => entries().map(e => e.timeline[frame(e)]));
      const stopped = await state(); await page.waitForTimeout(1300); const later = await state();
      assert.deepEqual(stopped.map(t => [t.hp, t.status, t.explosions]), later.map(t => [t.hp, t.status, t.explosions]));
      const cases = await page.locator('#scenario option').evaluateAll(options => options.map(option => option.value));
      if (cases.includes('ATTENTE')) {
        await page.selectOption('#scenario', 'ATTENTE');
        await page.waitForFunction(() => ready && launched && !autoPlay);
        assert((await state()).every(t => t.explosions === 0 && !t.mines[0].armed));
      }
      if (cases.includes('ENNEMI-ARMEMENT')) {await page.selectOption('#scenario', 'ENNEMI-ARMEMENT'); await finished();}
      await page.selectOption('#scenario', 'CLOISON'); await finished();
      assert((await state()).every(t => t.hp[2] === (number === 63 ? 100 : 400)));
      await page.goto('file:///' + path.join(folder, 'index.html').replaceAll('\\', '/')); await finished();
      assert.deepEqual(errors, []);
      fs.writeFileSync(path.join(folder, 'browser-visible-playback-verification.json'), JSON.stringify({
        browser: 'Edge Chromium headless', url_scheme: 'file', page_errors: errors,
        automatic_placement_to_contact: 'passed', replay: 'passed',
        paused_native_explosion_capture: snapshot, idle_preserves_damage_and_status_turns: 'passed',
        quiet_scene_and_arming_contact: cases.includes('ATTENTE') ? 'passed' : 'Not included in this targeted readability review', wall_protection: 'passed',
        index_and_comparison_pages: 'passed', screenshot: 'browser-visible-explosion.png'
      }, null, 2) + '\n');
      console.log(JSON.stringify({review: number, real_browser: 'passed', explosion_frames: snapshot.canvases.map(c => c.frame), errors}));
      await page.close();
    }
  } finally {await browser.close();}
}
main().catch(error => {console.error(error); process.exitCode = 1;});
