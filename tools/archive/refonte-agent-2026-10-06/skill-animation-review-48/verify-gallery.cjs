// Verify the local gallery controller and frame paths without browser/network access.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=__dirname,els=new Map();
for(const id of ['scenario','next','cast','reset','pause','status'])els.set(id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''});
const canvases=['M02','HY07'].map(id=>({dataset:{skill:id},parentElement:{hidden:false},getContext:()=>({drawImage(im){assert(im);}})}));
class LocalFrame{set src(p){assert(fs.existsSync(path.join(root,p)),p);this.onload();}}
const ctx=vm.createContext({Image:LocalFrame,Map,Promise,Math,String,document:{getElementById:id=>els.get(id),querySelectorAll:()=>canvases},requestAnimationFrame(){}});
vm.runInContext(fs.readFileSync(path.join(root,'gallery-script.js'),'utf8'),ctx);
const run=s=>vm.runInContext(s,ctx),settle=()=>new Promise(resolve=>setImmediate(resolve));
(async()=>{
  await settle();assert(run('ready'));assert.strictEqual(run('entries().length'),2);assert.strictEqual(run('frame(entries()[0])'),17);
  run('elapsed=100000;ui()');assert.strictEqual(run('turn'),0);assert.strictEqual(run('frame(entries()[0])'),41);
  for(let turn=1;turn<=3;turn++){els.get('next').onclick();assert.strictEqual(run('turn'),turn);assert.strictEqual(run('frame(entries()[0])'),[48,66,84][turn-1]);}
  assert(els.get('next').disabled);
  els.get('scenario').value='ATTENTE';els.get('scenario').onchange();assert(els.get('next').disabled);await settle();run('elapsed=100000;ui()');
  assert.strictEqual(run('turn'),0);assert.strictEqual(run('frame(entries()[0])'),41);assert(els.get('next').disabled);
  els.get('pause').onclick();const clock=run('elapsed');run('tick(600)');assert.strictEqual(run('elapsed'),clock);els.get('pause').onclick();
  els.get('reset').onclick();assert(!run('launched'));assert(!els.get('cast').disabled);assert(els.get('next').disabled);
  run('elapsed=0');const p1=run('frame(entries()[0])');run('elapsed=500');const p2=run('frame(entries()[0])');assert.notStrictEqual(p1,p2);
  els.get('cast').onclick();assert.strictEqual(run('frame(entries()[0])'),17);
  for(const key of ['SILENCE','ECLAIREUR','MULE','CACHE','TRANSFORME']){
    els.get('scenario').value=key;els.get('scenario').onchange();await settle();run('tick(1200)');
    assert(run('entries().every(e=>!e.permitted)'));assert(els.get('next').disabled);
    els.get('reset').onclick();assert(els.get('cast').disabled);
  }
  for(const key of ['PREDATEUR','COLOSSE','CUIRASSE','POSSEDE','ABYSSE','SANS-PV','PLAFOND']){
    els.get('scenario').value=key;els.get('scenario').onchange();await settle();run('tick(1400)');
    assert.strictEqual(run('entries().length'),1);assert(run('entries()[0].permitted'));assert.strictEqual(run('entries()[0].id'),'M02');
    assert.strictEqual(canvases.filter(c=>!c.parentElement.hidden).length,1);
  }
  for(const key of ['HAUT','DROITE','GAUCHE','BAS','CLOISON','ARTILLEUR','GARDIEN']){
    els.get('scenario').value=key;els.get('scenario').onchange();await settle();run('tick(1500)');
    assert(run('entries().some(e=>e.id==="HY07"&&e.permitted)'));
  }
  const report={syntax:'passed',controller:'passed',all_frame_paths_exist:true,native_canvas_count:2,manual_three_turn_sequence:true,
    waiting_does_not_advance_turns:true,pause_freezes_clock:true,preview_changes_frames_before_launch:true,
    all_five_forms_and_three_combat_models_checked:true,refusals_and_wall_case_checked:true,
    browser_validation:'Not performed; native GPU readbacks and offline controller verified.'};
  fs.writeFileSync(path.join(root,'gallery-controller-verification.json'),JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify(report));
})().catch(e=>{console.error(e);process.exitCode=1;});
