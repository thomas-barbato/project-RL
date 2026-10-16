// Offline verification of local frame paths and controller, no browser/network.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=__dirname,els=new Map();
for(const id of ['scenario','next','cast','reset','pause','status'])els.set(id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''});
const canvases=['A01','A02'].map(id=>({dataset:{skill:id},getContext:()=>({drawImage(image){assert(image);}})}));
class LocalFrame{set src(p){assert(fs.existsSync(path.join(root,p)),p);this.onload();}}
const ctx=vm.createContext({Image:LocalFrame,Map,Promise,Math,String,document:{getElementById:id=>els.get(id),querySelectorAll:()=>canvases},requestAnimationFrame(){}});
vm.runInContext(fs.readFileSync(path.join(root,'gallery-script.js'),'utf8'),ctx);
const run=s=>vm.runInContext(s,ctx),settle=()=>new Promise(resolve=>setImmediate(resolve));
(async()=>{
  await settle();assert(run('ready'));assert.strictEqual(run('frame()'),17);
  run('elapsed=10000;ui();tick(100)');assert.strictEqual(run('turn'),0);assert.strictEqual(run('frame()'),47);
  els.get('pause').onclick();const clock=run('elapsed');run('tick(600)');assert.strictEqual(run('elapsed'),clock);els.get('pause').onclick();
  for(let t=1;t<=4;t++){els.get('next').onclick();assert.strictEqual(run('turn'),t);assert.strictEqual(run('frame()'),[17,48,65,82,99][t]);run('elapsed=5000;ui()');}
  assert(els.get('next').disabled);assert.strictEqual(run('frame()'),111);
  els.get('reset').onclick();assert(!run('launched'));assert(els.get('next').disabled);assert(!els.get('cast').disabled);run('elapsed=10000;ui()');assert(run('frame()')<17);
  els.get('cast').onclick();assert.strictEqual(run('frame()'),17);assert(els.get('next').disabled);
  els.get('scenario').value='CLOISON';els.get('scenario').onchange();assert(els.get('next').disabled);await settle();assert(run('ready'));assert(!run('entry().views[0].valid'));assert(run('entry().views[1].valid'));
  run('elapsed=10000;ui();tick(1000)');assert.strictEqual(run('turn'),0);assert(els.get('status').textContent.includes('Trait 0'));
  const report={syntax:'passed',controller:'passed',local_frames_exist:true,instant_effects_never_replay_while_waiting:true,waiting_does_not_advance_turn_or_cooldown:true,manual_turns:4,reset_returns_to_preview:true,inputs_disabled_during_loading:true,pause_freezes_clock:true,browser_validation:'Not performed: file protocol blocked by browser automation policy. Native captures and offline controller validated.'};
  fs.writeFileSync(path.join(root,'gallery-controller-verification.json'),JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify(report));
})().catch(e=>{console.error(e);process.exitCode=1;});
