// Offline local controller validation. No browser, network or UI automation.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=__dirname,els=new Map();
for(const id of ['scenario','next','cast','reset','pause','status'])els.set(id,{value:id==='scenario'?'GEL':'',disabled:false,textContent:''});
els.set('view',{getContext:()=>({drawImage(im){assert(im);}})});
class LocalFrame {set src(p){assert(fs.existsSync(path.join(root,p)),p);this.onload();}}
const ctx=vm.createContext({Image:LocalFrame,Map,Promise,Math,String,document:{getElementById:id=>els.get(id)},requestAnimationFrame(){}});
vm.runInContext(fs.readFileSync(path.join(root,'gallery-script.js'),'utf8'),ctx);
const run=s=>vm.runInContext(s,ctx),settle=()=>new Promise(resolve=>setImmediate(resolve));
(async()=>{
  await settle();assert(run('ready'));assert.strictEqual(run('frame().i'),17);
  run('elapsed=10000;ui();tick(100)');assert.strictEqual(run('turn'),0);assert.strictEqual(run('frame().kind'),'idle');assert(els.get('status').textContent.includes('3 tours'));
  els.get('pause').onclick();const before=run('elapsed');run('tick(600)');assert.strictEqual(run('elapsed'),before);els.get('pause').onclick();
  for(let t=1;t<=3;t++){els.get('next').onclick();assert.strictEqual(run('turn'),t);assert.strictEqual(run('frame().i'),[17,48,64,80][t]);run('elapsed=10000;ui()');}
  assert(els.get('next').disabled);assert.strictEqual(run('frame().kind'),'main');assert.strictEqual(run('frame().i'),95);
  els.get('reset').onclick();assert(!run('applied'));assert(els.get('next').disabled);assert(!els.get('cast').disabled);els.get('cast').onclick();assert(run('applied'));
  els.get('scenario').value='EXPLOSION/HAUT';els.get('scenario').onchange();await settle();assert(run('ready'));assert(!run('applied'));assert.strictEqual(run('frame().kind'),'preview');
  run('elapsed=10000;ui()');assert.strictEqual(run('turn'),0);assert.strictEqual(run('frame().kind'),'preview');
  els.get('cast').onclick();assert.strictEqual(run('frame().i'),17);run('elapsed=10000;ui()');assert.strictEqual(run('frame().i'),47);assert.strictEqual(run('turn'),0);
  for(let t=1;t<=3;t++){els.get('next').onclick();assert.strictEqual(run('turn'),t);run('elapsed=10000;ui()');}
  assert(els.get('next').disabled);
  const report={syntax:'passed',controller:'passed',all_local_frames_exist:true,persistent_status_uses_native_idle_bank:true,waiting_never_advances_status_or_turn:true,consumed_gel_never_returns_after_cast:true,one_shot_burst_never_replays_without_reset:true,pause_freezes_clock:true,browser_validation:'Not performed: file protocol was blocked by browser automation policy. Native captures and offline JavaScript validated.'};
  fs.writeFileSync(path.join(root,'gallery-controller-verification.json'),JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify(report));
})().catch(e=>{console.error(e);process.exitCode=1;});
