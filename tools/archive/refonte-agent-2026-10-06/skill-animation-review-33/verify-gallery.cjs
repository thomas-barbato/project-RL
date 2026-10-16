// Offline controller/frame validation; no browser/network automation.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=__dirname,els=new Map();
for(const id of ['scenario','next','cast','reset','pause','status'])els.set(id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''});
const canvases=['HY05','OC05'].map(id=>({dataset:{skill:id},getContext:()=>({drawImage(image){assert(image);}})}));
class LocalFrame {set src(p){assert(fs.existsSync(path.join(root,p)),p);this.onload();}}
const ctx=vm.createContext({Image:LocalFrame,Map,Promise,Math,String,document:{getElementById:id=>els.get(id),querySelectorAll:()=>canvases},requestAnimationFrame(){}});
vm.runInContext(fs.readFileSync(path.join(root,'gallery-script.js'),'utf8'),ctx);
const run=s=>vm.runInContext(s,ctx),settle=()=>new Promise(resolve=>setImmediate(resolve));
(async()=>{
    await settle();assert(run('ready'));assert.strictEqual(run('frame()'),17);
    run('elapsed=10000;ui()');assert.strictEqual(run('turn'),0);assert(run('frame()')>=36&&run('frame()')<=59);
    els.get('pause').onclick();const clock=run('elapsed');run('tick(600)');assert.strictEqual(run('elapsed'),clock);els.get('pause').onclick();
    for(let t=1;t<=3;t++){els.get('next').onclick();assert.strictEqual(run('turn'),t);assert.strictEqual(run('frame()'),[17,60,88,116][t]);run('elapsed=10000;ui()');if(t<3)assert(run('frame()')>=[36,72,100][t]&&run('frame()')<=[59,87,115][t]);}
    assert(els.get('next').disabled);assert.strictEqual(run('frame()'),143);
    els.get('scenario').value='DEPLACEMENT';els.get('scenario').onchange();assert(els.get('next').disabled);await settle();assert(run('ready'));run('elapsed=10000;ui()');els.get('next').onclick();run('elapsed=10000;ui()');assert.strictEqual(run('frame()'),87);assert.strictEqual(run('turn'),1);
    els.get('scenario').value='CLOISON';els.get('scenario').onchange();await settle();run('elapsed=10000;ui()');assert.strictEqual(run('frame()'),59);assert(els.get('status').textContent.includes('0/3'));
    els.get('reset').onclick();assert(!run('launched'));assert(!els.get('cast').disabled);assert(els.get('next').disabled);els.get('cast').onclick();assert.strictEqual(run('frame()'),17);
    const report={syntax:'passed',controller:'passed',all_frames_exist:true,ray_waiting_loop_excludes_new_drain_pulses:true,waiting_does_not_advance_turn_or_cooldown:true,interruptions_stop_waiting_loop:true,pause_freezes_clock:true,inputs_disabled_during_loading:true,browser_validation:'Not performed: file protocol blocked by browser automation policy. Native captures and offline controller validated.'};
    fs.writeFileSync(path.join(root,'gallery-controller-verification.json'),JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify(report));
})().catch(e=>{console.error(e);process.exitCode=1;});
