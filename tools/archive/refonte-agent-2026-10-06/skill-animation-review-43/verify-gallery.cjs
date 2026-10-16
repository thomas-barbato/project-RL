// Offline controller verification. No browser/network workaround.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=__dirname,els=new Map();
for(const id of ['scenario','next','cast','reset','pause','status'])els.set(id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''});
const ids=['TR63-HEURT','TR63-DECHARGE','TR63-FERMETURE','TR64-TRAIT','TR64-PULSATION','TR64-DRAIN'];
ids.push("RX01-DMG-M","RX01-DMG-J","RX01-STATUS-M","RX01-STATUS-J");
const canvases=ids.map(id=>({dataset:{skill:id},getContext:()=>({drawImage(image){assert(image);}})}));
class LocalFrame {set src(p){assert(fs.existsSync(path.join(root,p)),p);this.onload();}}
const ctx=vm.createContext({Image:LocalFrame,Map,Promise,Math,String,document:{getElementById:id=>els.get(id),querySelectorAll:()=>canvases},requestAnimationFrame(){}});
vm.runInContext(fs.readFileSync(path.join(root,'gallery-script.js'),'utf8'),ctx);
const run=s=>vm.runInContext(s,ctx),settle=()=>new Promise(resolve=>setImmediate(resolve));
(async()=>{
  await settle();assert(run('ready'));assert.strictEqual(run('frame()'),17);
  run('elapsed=10000;ui()');assert.strictEqual(run('turn'),0);assert.strictEqual(run('frame()'),47);
  els.get('pause').onclick();const clock=run('elapsed');run('tick(600)');assert.strictEqual(run('elapsed'),clock);els.get('pause').onclick();
  for(let t=1;t<=3;t++){els.get('next').onclick();assert.strictEqual(run('turn'),t);assert.strictEqual(run('frame()'),run('entries()[0].turn_frames[turn-1]'));run('elapsed=10000;ui()');}
  assert(els.get('next').disabled);assert.strictEqual(run('frame()'),95);
  els.get('scenario').value='ATTENTE';els.get('scenario').onchange();assert(els.get('next').disabled);await settle();run('elapsed=100000;ui()');assert.strictEqual(run('turn'),0);assert.strictEqual(run('frame()'),47);assert(els.get('next').disabled);
  els.get('reset').onclick();assert(!run('launched'));assert(!els.get('cast').disabled);assert(els.get('next').disabled);els.get('cast').onclick();assert.strictEqual(run('frame()'),17);
  for(const s of ['DROITE','GAUCHE','BAS','OBSTACLE','BLOQUE','RESERVE','AGIR','SILENCE']){els.get('scenario').value=s;els.get('scenario').onchange();assert(els.get('next').disabled);await settle();assert(run('ready'));run('tick(700)');}
  const report={syntax:'passed',controller:'passed',six_native_views_loaded:true,common_reaction_views_loaded:true,native_canvas_count:ids.length,explicit_three_turn_sequence:true,waiting_does_not_advance_turns:true,pause_freezes_clock:true,inputs_disabled_during_loading:true,browser_validation:'Not performed: file protocol blocked. Native captures and offline controller verified.'};
  fs.writeFileSync(path.join(root,'gallery-controller-verification.json'),JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify(report));
})().catch(e=>{console.error(e);process.exitCode=1;});
