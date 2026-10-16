// Offline controller and local-frame checks. No browser, network or UI automation.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=__dirname,els=new Map();
for(const id of ['scenario','next','cast','reset','pause','status','loading'])els.set(id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''});
const canvases=['FR04','FR05'].map(skill=>({dataset:{skill},getContext:()=>({drawImage(image){assert(image);}})}));
class LocalFrame{set src(value){assert(fs.existsSync(path.join(root,value)),value);this.onload();}}
const ctx=vm.createContext({Image:LocalFrame,Map,Promise,Math,String,document:{getElementById:id=>els.get(id),querySelectorAll:()=>canvases},requestAnimationFrame(){}});
vm.runInContext(fs.readFileSync(path.join(root,'gallery-script.js'),'utf8'),ctx);
const run=s=>vm.runInContext(s,ctx);
(async()=>{
  await new Promise(resolve=>setImmediate(resolve));assert(run('ready'));assert.strictEqual(run('frame().kind'),'main');assert.strictEqual(run('frame().index'),17);
  els.get('reset').onclick();assert.strictEqual(run('launched'),false);assert(els.get('next').disabled);assert(!els.get('cast').disabled);
  run('elapsed=600;ui()');assert.strictEqual(run('frame().kind'),'main');assert(run('frame().index')<17);
  els.get('cast').onclick();assert.strictEqual(run('frame().index'),17);assert(els.get('next').disabled);
  run('elapsed=10000;ui()');assert.strictEqual(run('turn'),0);assert.strictEqual(run('frame().kind'),'idle');assert.strictEqual(run('frame().stage'),0);assert(run('frame().index')<6);
  els.get('pause').onclick();const before=run('elapsed');run('tick(100);tick(600)');assert.strictEqual(run('elapsed'),before);els.get('pause').onclick();
  const starts=[17,48,65,82,99,116,133,150,167],ends=[47,64,81,98,115,132,149,166,191];
  for(let t=1;t<=8;t++){
    els.get('next').onclick();assert.strictEqual(run('turn'),t);assert.strictEqual(run('elapsed'),0);assert.strictEqual(run('frame().kind'),'main');assert.strictEqual(run('frame().index'),starts[t]);
    run('elapsed=5000;ui();tick(1000)');
    if(t<6){assert.strictEqual(run('frame().kind'),'idle');assert.strictEqual(run('frame().stage'),t);}else{assert.strictEqual(run('frame().kind'),'main');assert.strictEqual(run('frame().index'),ends[t]);}
  }
  assert(els.get('next').disabled);els.get('next').onclick();assert.strictEqual(run('turn'),8);
  els.get('reset').onclick();assert.strictEqual(run('turn'),0);assert.strictEqual(run('launched'),false);
  els.get('scenario').value='CLOISON';els.get('scenario').onchange();await new Promise(resolve=>setImmediate(resolve));assert(run('ready'));assert.strictEqual(run('frame().index'),17);
  const report={syntax:'passed',controller:'passed',all_loaded_native_frames_exist:true,idle_uses_separate_captured_loops:true,idle_never_replays_movement_or_impact:true,pause_freezes_visual_clock:true,reset_returns_to_preview:true,manual_turn_stages:8,browser_check:'Not performed: file protocol is blocked by browser automation policy; offline controller and native captures checked instead.'};
  fs.writeFileSync(path.join(root,'gallery-controller-verification.json'),JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify(report));
})().catch(error=>{console.error(error);process.exitCode=1;});
