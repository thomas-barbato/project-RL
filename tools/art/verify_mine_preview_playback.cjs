// Visible browser-frame regression: placement, automatic contact and short burst.
const fs = require('fs'), path = require('path'), vm = require('vm');
const assert = require('assert/strict');
const root = path.resolve(__dirname, '../..');

async function verify(number) {
  const folder = path.join(root, `artifacts/refonte-agent-2026-10-06/skill-animation-review-${number}`);
  const html = fs.readFileSync(path.join(folder, 'cote-a-cote.html'), 'utf8');
  const code = html.match(/<script>([\s\S]*?)<\/script>/)[1];
  const ids = number === 63 ? ['SM00', 'SM01'] : ['SM02', 'SM03'];
  const controls = Object.fromEntries(['scenario', 'next', 'cast', 'status', 'reset', 'pause', 'demo', 'native-view', 'fullscreen']
    .map(id => [id, {value: id === 'scenario' ? 'HAUT' : '', textContent: '', disabled: false}]));
  const drawn = {}, visibilityHandlers = {};
  const canvases = ids.map(id => ({dataset: {skill: id}, width: 576, height: 576,
    parentElement: {hidden: false}, getContext: () => ({drawImage(im) {drawn[id] = im.src;}}),
    getBoundingClientRect: () => ({left: 0, top: 0, width: 288, height: 288})}));
  let requested, now = 100, imageCount = 0;
  class LocalImage {
    set src(src) {
      assert(fs.existsSync(path.join(folder, src)), `Missing rendered frame: ${src}`);
      this.source = src; imageCount++; queueMicrotask(() => this.onload());
    }
    get src() {return this.source;}
  }
  const document = {hidden: false, getElementById: id => controls[id], querySelectorAll: () => canvases,
    addEventListener: (name, handler) => visibilityHandlers[name] = handler,
    body: {classList: {toggle() {}}}, documentElement: {requestFullscreen: async () => {}}, exitFullscreen: async () => {}};
  const context = vm.createContext({Image: LocalImage, Promise, Map, Set, Math, Array, Error, document,
    requestAnimationFrame: callback => requested = callback});
  const evaluate = expression => vm.runInContext(expression, context);
  const settle = () => new Promise(resolve => setImmediate(resolve));
  const state = id => evaluate(`(()=>{const e=entries().find(e=>e.id==='${id}');return e.timeline[frame(e)];})()`);
  const observed = [];
  async function tick(dt = 40) {
    now += dt; requested(now); await settle();
    const e = evaluate('entries()[0]'), f = evaluate('frame(entries()[0])');
    const expected = `frames/${e.variant}/${String(e.image_indices[f]).padStart(3, '0')}.png`;
    if (evaluate('elapsed') < 1000 || !drawn[e.id].includes('/idle-'))
      assert.equal(drawn[e.id], expected, 'Displayed frame differs from controller frame');
    else assert(drawn[e.id].startsWith(`frames/${e.variant}/idle-`), 'Idle frame belongs to another scenario');
    observed.push({case: controls.scenario.value, frame: f, explosions: state(ids[0]).explosions});
  }
  async function run(ms) {for (let t = 0; t < ms; t += 40) await tick();}
  async function choose(label) {controls.scenario.value = label; await controls.scenario.onchange(); await tick();}
  async function manualPose() {controls.reset.onclick(); controls.cast.onclick(); await run(1200);}
  async function advance() {assert(!controls.next.disabled); controls.next.onclick(); await run(1200);}
  async function explode() {await manualPose(); for (let i = 0; i < 3; i++) await advance();}
  evaluate(code); await settle(); await tick();
  assert(evaluate('autoPlay'), 'Gallery does not start a complete demonstration');
  await run(5000);
  assert.equal(evaluate('turn'), 3); assert.equal(state(ids[0]).explosions, 1);
  assert(!evaluate('autoPlay'), 'Demonstration should stop after one explosion');
  assert(observed.filter(o => o.case === 'HAUT' && o.frame >= 114 && o.frame <= 120 && o.explosions === 1).length >= 5,
    'Short explosion was skipped instead of actually drawn');
  if (number !== 63) {
    assert.deepEqual(Array.from(state('SM02').hp), [339, 339, 339]);
    assert.deepEqual(Array.from(state('SM03').status), [5, 5, 5]);
    await run(3000);
    assert(drawn.SM02.includes('idle-3-')); assert(drawn.SM03.includes('idle-3-'));
    assert.equal(state('SM02').poison_ticks[0], 1, 'Wall time advanced poison turns');
  } else assert.deepEqual(Array.from(state('SM01').hp), [87, 87, 87]);
  controls.demo.onclick(); await tick(10000);
  assert(evaluate('elapsed') <= 1000 / 24, 'A delayed first paint skipped the action');
  document.hidden = true; visibilityHandlers.visibilitychange(); await tick(20000);
  const hiddenTime = evaluate('elapsed'); document.hidden = false; visibilityHandlers.visibilitychange();
  await tick(20000); assert.equal(evaluate('elapsed'), hiddenTime, 'Background time skipped visible frames');
  controls.pause.onclick({target: controls.pause}); const pausedTime = evaluate('elapsed');
  await tick(20000); assert.equal(evaluate('elapsed'), pausedTime);
  controls.pause.onclick({target: controls.pause}); await tick(20000); assert.equal(evaluate('elapsed'), pausedTime);
  await run(5000); assert.equal(state(ids[0]).explosions, 1);
  const cases = evaluate('Array.from(new Set(manifest.map(e=>e.case)))');
  for (const label of cases) {
    await choose(label); await run(5000);
    assert.equal(state(ids[0]).explosions, label === 'ATTENTE' ? 0 : 1, `Automatic trigger in ${label}`);
    assert.equal(state(ids[0]).player_hp, 100);
    if (number === 63) assert.equal(JSON.stringify(state('SM00')), JSON.stringify(state('SM01')), 'Fire comparison mechanics drift');
    if (label === 'CLOISON') assert.equal(state(ids[0]).hp[2], number === 63 ? 100 : 400);
    if (label === 'GEL-RESISTE') {assert.equal(state('SM03').status[0], 0); assert.equal(state('SM03').hp[0], 384);}
    if (label === 'RANG-12') assert.equal(state('SM02').hp[0], 333);
  }
  await choose('HAUT'); await explode();
  if (number !== 63) {
    for (const [timer, ticks, hp] of [[4, 2, 297], [3, 3, 255], [2, 4, 213], [1, 5, 171], [0, 5, 171]]) {
      await advance(); assert.equal(state('SM02').status[0], timer); assert.equal(state('SM03').status[0], timer);
      assert.equal(state('SM02').poison_ticks[0], ticks); assert.equal(state('SM02').hp[0], hp);
    }
    assert(controls.next.disabled);
  }
  const record = {mode: 'DOM/Image regression observing each actually drawn native frame', scenarios: cases.length,
    loaded_image_references: imageCount, automatic_sequence_and_replay: 'passed',
    explosion_frames_actually_drawn: 'passed', action_gap_background_and_pause: 'passed',
    exactly_one_explosion_and_manual_status_turns: 'passed', wall_and_player_protection: 'passed'};
  fs.writeFileSync(path.join(folder, 'browser-playback-controller-verification.json'), JSON.stringify(record, null, 2) + '\n');
  console.log(JSON.stringify({review: number, ...record}));
}
(async () => {for (const number of process.argv.length > 2 ? process.argv.slice(2).map(Number) : [62, 63]) await verify(number);})().catch(error => {console.error(error); process.exitCode = 1;});
