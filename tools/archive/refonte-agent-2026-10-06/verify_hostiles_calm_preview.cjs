const { chromium } = require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');
const directory = __dirname;

(async () => {
  const browser = await chromium.launch({ channel:'msedge', headless:true });
  const page = await browser.newPage({ viewport:{width:736,height:800} });
  const errors = [];
  page.on('pageerror',error => errors.push(error.message));
  await page.goto('file:///' + path.join(directory,'hostiles-et-calme-preview.html').replaceAll('\\','/'));
  const frame = page.frameLocator('iframe');
  const root = frame.locator('#hostiles-calme-review');
  await root.waitFor();
  const scenario = root.locator('[data-situation]');
  const next = root.locator('[data-next]');
  const phase = root.locator('[data-phase]');
  const status = root.locator('[data-hostiles]');
  const hp = root.locator('[data-hp]');
  async function checkPhase(expected) { assert.equal(await phase.innerText(),expected); }

  await checkPhase('Combat');
  assert.equal(await root.locator('.hostile').count(),2);
  assert.equal(await root.locator('.targeted').count(),0);
  await status.hover();
  assert.equal(await root.locator('.targeted').count(),2);
  assert.equal(await root.locator('.targeted:not(.hostile)').count(),0);
  assert.equal(await root.locator('[data-turn]').innerText(),'Tour 0');
  await page.screenshot({path:path.join(directory,'hostiles-calm-combat-hover.png'),fullPage:true});
  await next.hover();
  assert.equal(await root.locator('.targeted').count(),0);
  await status.click();
  await next.hover();
  assert.equal(await root.locator('.targeted').count(),0,'mouse click must not pin the hover highlight');

  await scenario.selectOption('lost');
  await checkPhase('Accalmie — 3 tours');
  assert.equal(await root.locator('.hostile').count(),0,'lost targets must not remain in the diagram');
  await status.hover();
  assert.equal(await root.locator('.targeted').count(),0);
  await next.click(); await checkPhase('Accalmie — 2 tours');
  await page.screenshot({path:path.join(directory,'hostiles-calm-accalmie.png'),fullPage:true});
  await next.click(); await checkPhase('Accalmie — 1 tour');
  await next.click(); await checkPhase('Calme');
  assert.equal(await hp.innerText(),'Drone : 42 / 70 PV','the first three quiet turns must not repair');
  await next.click();
  assert.equal(await hp.innerText(),'Drone : 47 / 70 PV');
  await next.click(); await next.click(); await next.click();
  assert.equal(await hp.innerText(),'Drone : 63 / 70 PV','four eligible turns must repair 5,5,5,6');
  await page.screenshot({path:path.join(directory,'hostiles-calm-calme.png'),fullPage:true});
  await next.click(); await next.click();
  assert.equal(await hp.innerText(),'Drone : 70 / 70 PV');
  await next.click(); assert.equal(await hp.innerText(),'Drone : 70 / 70 PV');

  await scenario.selectOption('attack');
  await checkPhase('Combat');
  assert.equal(await root.locator('[data-hostile-label]').innerText(),'Aucun hostile repéré');
  assert.equal(await hp.innerText(),'Drone : 42 / 70 PV','a missed incoming attack does not fabricate damage');
  await next.click(); await checkPhase('Accalmie — 2 tours');
  await scenario.selectOption('damage');
  for (let index=0; index<5; index++) await next.click();
  await checkPhase('Combat');
  assert.equal(await hp.innerText(),'Drone : 42 / 70 PV','ongoing team damage must suspend drone repair');
  assert.equal(await root.locator('[data-hostile-label]').innerText(),'Aucun hostile repéré');

  await scenario.selectOption('visible');
  await next.focus();
  await page.keyboard.press('Tab');
  assert.equal(await root.locator('.targeted').count(),2);
  await next.focus(); assert.equal(await root.locator('.targeted').count(),0);
  await scenario.selectOption('lost');
  await page.setViewportSize({width:320,height:800});
  const overflow = await root.evaluate(element => ({width:element.getBoundingClientRect().width,scroll:element.scrollWidth}));
  assert(overflow.scroll<=overflow.width+1,JSON.stringify(overflow));
  await page.screenshot({path:path.join(directory,'hostiles-calm-narrow.png'),fullPage:true});
  assert.deepEqual(errors,[]);
  fs.writeFileSync(path.join(directory,'hostiles-calm-preview-checks.json'),JSON.stringify({
    status:'passed_in_schematic_preview_not_game_runtime',viewport_widths:[736,320],
    verified:['hover highlights only visible hostiles','hover does not advance a turn','mouse leave clears highlight','hidden targets leave count and highlight','three quiet turns before repair','four eligible repairs total 21 HP','maximum HP respected','combat possible with zero visible hostiles','ongoing damage suspends repair','keyboard focus highlight','no narrow layout overflow','no JavaScript runtime errors'],
    user_approval:'pending_text_mechanics_and_visual_review',engine_integration:false
  },null,2)+'\n');
  await browser.close();
  console.log('Preview interactions, fourth-turn repair, visibility, focus, and 320px layout verified.');
})().catch(error => { console.error(error); process.exit(1); });
