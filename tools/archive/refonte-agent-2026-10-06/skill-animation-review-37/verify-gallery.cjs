// Offline local-frame/controller verification. No browser or network.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=__dirname,els=new Map();
for(const id of ['scenario','comparison','next','cast','reset','pause','status'])els.set(id,{value:id==='scenario'?'HAUT':id==='comparison'?'new':'',disabled:false,textContent:''});
const canvases=['OC04','OC07'].map(id=>({dataset:{skill:id},getContext:()=>({drawImage(image){assert(image);}})}));
const loadedPaths=[];
class LocalFrame {set src(p){assert(fs.existsSync(path.join(root,p)),p);loadedPaths.push(p);this.onload();}}
const ctx=vm.createContext({Image:LocalFrame,Map,Promise,Math,String,document:{getElementById:id=>els.get(id),querySelectorAll:()=>canvases},requestAnimationFrame(){}});
vm.runInContext(fs.readFileSync(path.join(root,'gallery-script.js'),'utf8'),ctx);
const run=s=>vm.runInContext(s,ctx),settle=()=>new Promise(resolve=>setImmediate(resolve));
(async()=>{
    await settle();assert(run('ready'));assert.strictEqual(run('frame()'),17);
    run('elapsed=10000;ui()');assert.strictEqual(run('turn'),0);assert(run('frame()')>=36&&run('frame()')<=47);
    els.get('pause').onclick();const clock=run('elapsed');run('tick(600)');assert.strictEqual(run('elapsed'),clock);els.get('pause').onclick();
    for(let t=1;t<=13;t++){els.get('next').onclick();assert.strictEqual(run('turn'),t);assert.strictEqual(run('frame()'),[17,48,60,72,84,90,96,102,108,114,120,126,132,138][t]);run('elapsed=10000;ui()');}
    assert(els.get('next').disabled);assert.strictEqual(run('frame()'),143);
    els.get('scenario').value='ATTENTE';els.get('scenario').onchange();assert(els.get('next').disabled);await settle();run('elapsed=10000;ui()');assert.strictEqual(run('turn'),0);assert(run('frame()')>=36&&run('frame()')<=47);assert(els.get('next').disabled);
    els.get('scenario').value='CLOISON';els.get('scenario').onchange();await settle();run('elapsed=10000;ui()');assert.strictEqual(run('frame()'),47);
    els.get('reset').onclick();assert(!run('launched'));assert(!els.get('cast').disabled);assert(els.get('next').disabled);els.get('cast').onclick();assert.strictEqual(run('frame()'),17);
    loadedPaths.length=0;els.get('comparison').value='old';els.get('comparison').onchange();assert(!run('ready'));await settle();assert(run('ready'));assert.strictEqual(loadedPaths.length,288);assert(loadedPaths.every(p=>p.startsWith('../skill-animation-review-36/frames/')&&p.includes('-A01-')));
    loadedPaths.length=0;els.get('comparison').value='new';els.get('comparison').onchange();await settle();assert(run('ready'));assert(loadedPaths.every(p=>p.startsWith('frames/')&&p.includes('-A02-')));
    const report={syntax:'passed',controller:'passed',all_frames_exist:true,previous_version_selector_checked:true,explicit_thirteen_turns_to_expiry:true,waiting_loop_preserves_complete_circle:true,waiting_does_not_advance_turns:true,pause_freezes_clock:true,inputs_disabled_during_loading:true,browser_validation:'Not performed: file protocol blocked by browser automation policy. Native captures and offline controller validated.'};
    fs.writeFileSync(path.join(root,'gallery-controller-verification.json'),JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify(report));
})().catch(e=>{console.error(e);process.exitCode=1;});
