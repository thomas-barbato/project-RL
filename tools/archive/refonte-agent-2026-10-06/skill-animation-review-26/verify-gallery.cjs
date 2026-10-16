// Offline gallery controller check; no browser, network or UI automation.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=__dirname,els=new Map();
for(const id of ['scenario','next','cast','reset','pause','status','loading'])els.set(id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''});
els.set('view',{getContext:()=>({drawImage(image){assert(image);}})});
class LocalFrame{set src(value){assert(fs.existsSync(path.join(root,value)));this.onload();}}
const ctx=vm.createContext({Image:LocalFrame,Map,Promise,Math,String,document:{getElementById:id=>els.get(id)},requestAnimationFrame(){}});
vm.runInContext(fs.readFileSync(path.join(root,'gallery-script.js'),'utf8'),ctx);const run=s=>vm.runInContext(s,ctx);
(async()=>{
  await new Promise(resolve=>setImmediate(resolve));assert(run('ready'));assert.strictEqual(run('frame()'),17);
  els.get('reset').onclick();assert(!run('launched'));assert(els.get('next').disabled);assert(!els.get('cast').disabled);
  run('elapsed=600;ui()');assert(run('frame()')<17);els.get('cast').onclick();assert.strictEqual(run('frame()'),17);assert(els.get('next').disabled);
  run('elapsed=10000;ui();tick(100)');assert.strictEqual(run('turn'),0);assert.strictEqual(run('frame()'),47);
  els.get('pause').onclick();const before=run('elapsed');run('tick(600)');assert.strictEqual(run('elapsed'),before);els.get('pause').onclick();
  for(let t=1;t<=3;t++){els.get('next').onclick();assert.strictEqual(run('turn'),t);assert.strictEqual(run('elapsed'),0);assert.strictEqual(run('frame()'),[17,48,65,82][t]);run('elapsed=5000;ui();tick(1000)');assert.strictEqual(run('frame()'),[47,64,81,95][t]);}
  assert(els.get('next').disabled);els.get('next').onclick();assert.strictEqual(run('turn'),3);
  els.get('reset').onclick();assert(!run('launched'));assert.strictEqual(run('turn'),0);
  els.get('scenario').value='UNIQUE';els.get('scenario').onchange();await new Promise(resolve=>setImmediate(resolve));assert(run('ready'));assert.strictEqual(run('frame()'),17);
  const report={syntax:'passed',controller:'passed',native_frames_exist:true,waiting_never_replays_burst:true,waiting_never_advances_turn:true,pause_freezes_visual_clock:true,reset_returns_to_blinking_preview:true,manual_turns:3,browser_check:'Not performed: file protocol is blocked by browser automation policy; native captures and offline controller checked instead.'};
  fs.writeFileSync(path.join(root,'gallery-controller-verification.json'),JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify(report));
})().catch(error=>{console.error(error);process.exitCode=1;});
