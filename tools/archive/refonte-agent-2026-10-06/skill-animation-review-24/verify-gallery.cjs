// Offline controller check. No browser, networking, page execution or UI automation.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=__dirname,els=new Map();
for(const id of ['scenario','next','cast','reset','pause','status','loading'])els.set(id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''});
const canvases=['before','after'].map(version=>({dataset:{version},getContext:()=>({drawImage(){}})}));
class LocalFrame{set src(value){assert(fs.existsSync(path.join(root,value)));this.onload();}}
const ctx=vm.createContext({Image:LocalFrame,Map,Promise,Math,String,document:{getElementById:id=>els.get(id),querySelectorAll:()=>canvases},requestAnimationFrame(){}});
vm.runInContext(fs.readFileSync(path.join(root,'gallery-script.js'),'utf8'),ctx);
const run=s=>vm.runInContext(s,ctx);
(async()=>{
  await new Promise(resolve=>setImmediate(resolve));
  assert(run('ready'));assert.strictEqual(run('frame()'),17);
  els.get('reset').onclick();assert.strictEqual(run('launched'),false);assert(els.get('next').disabled);assert(!els.get('cast').disabled);
  run('elapsed=600;ui()');assert(run('frame()')<17);
  els.get('cast').onclick();assert.strictEqual(run('frame()'),17);assert(els.get('next').disabled);
  run('elapsed=5000;ui()');assert(run('frame()')>=48&&run('frame()')<=71);assert.strictEqual(run('turn'),0);
  els.get('pause').onclick();const before=run('elapsed');run('tick(100);tick(600)');assert.strictEqual(run('elapsed'),before);els.get('pause').onclick();
  for(let t=1;t<=5;t++){
    els.get('next').onclick();assert.strictEqual(run('turn'),t);assert.strictEqual(run('elapsed'),0);
    run('elapsed=2000;ui()');assert.strictEqual(run('frame()'),[71,88,105,122,139,167][t]);
  }
  assert(els.get('next').disabled);els.get('next').onclick();assert.strictEqual(run('turn'),5);
  els.get('reset').onclick();assert.strictEqual(run('turn'),0);assert.strictEqual(run('launched'),false);
  els.get('scenario').value='CLOISON';els.get('scenario').onchange();await new Promise(resolve=>setImmediate(resolve));
  assert(run('ready'));assert.strictEqual(run('turn'),0);assert.strictEqual(run('frame()'),17);
  const report={syntax:'passed',controller:'passed',native_frames_exist:true,waiting_preserves_turn:true,waiting_does_not_replay_nova:true,pause_freezes_visual_clock:true,reset_returns_to_preview:true,manual_turn_stages:true,final_stage_blocks_further_fixture_turns:true,browser_check:'Unavailable: browser URL policy blocks file protocol. Offline controller and native captures checked instead.'};
  fs.writeFileSync(path.join(root,'gallery-controller-verification.json'),JSON.stringify(report,null,2)+'\n');
  console.log(JSON.stringify(report));
})().catch(error=>{console.error(error);process.exitCode=1;});
