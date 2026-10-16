// Offline controller checks only; never launches a browser.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06'),review=path.join(base,'skill-animation-review-99');
const html=fs.readFileSync(path.join(base,'apercu-animations.html'),'utf8');assert(html.includes('ANIM-S50'));
const script=html.match(/<script>([\s\S]*?)<\/script>/)[1],elements=new Map(),draws=[];
function element(id){if(!elements.has(id))elements.set(id,{disabled:false,textContent:'',value:'',getContext(){return {clearRect(){},drawImage(...args){draws.push(args);}};}});return elements.get(id);}
let tick,now=0;
class ImageStub{set src(p){assert(fs.existsSync(path.join(base,p)));this.onload();}}
const context={document:{getElementById:element},Image:ImageStub,requestAnimationFrame(f){tick=f;},console,window:{},Map,Math,Promise};vm.runInNewContext(script,context);
const api=context.window.rangedFanPrecisionReview;
async function loaded(){for(let i=0;i<10;i++)await Promise.resolve();assert(api.isReady());tick(now+=100);}
function wait(n=35){for(let i=0;i<n;i++)tick(now+=100);}
async function main(){
 await loaded();elements.get('mode').onclick();
 for(const direction of [0,1,2,3]){
  await elements.get('direction').onchange({target:{value:String(direction)}});await loaded();assert(api.entries().every(e=>e.direction===direction));
  assert(api.states().every(s=>s.preview));elements.get('action').onclick();wait();
  assert(api.states().every(s=>s.started&&!s.preview));assert.equal(api.states()[0].hits,5);assert.equal(api.states()[1].hits,1);
  const damages=JSON.stringify(api.states().map(s=>s.damage)),ammunition=api.states().map(s=>s.ammo);
  for(let i=0;i<6;i++){elements.get('action').onclick();wait();}
  assert(api.states().every(s=>s.cooldown===0&&s.turn===7));assert.equal(JSON.stringify(api.states().map(s=>s.damage)),damages);
  assert.deepEqual(api.states().map(s=>s.ammo),ammunition);assert(elements.get('action').disabled);
 }

 for(const scenario of ['CLOISON','MACHINE','MANQUE','MAXIMUM','VIDE','MUNITIONS','ENERGIE','MELEE','ATTENTE']){
  await elements.get('case').onchange({target:{value:scenario}});await loaded();assert(elements.get('direction').disabled);
  if(['VIDE','MUNITIONS','ENERGIE','MELEE'].includes(scenario)){
   const before=JSON.stringify(api.states());assert(elements.get('action').disabled);elements.get('action').onclick();wait();assert.equal(JSON.stringify(api.states()),before);continue;
  }
  elements.get('action').onclick();wait();
  const frame=api.frame();elements.get('pause').onclick();wait();assert.equal(api.frame(),frame);elements.get('pause').onclick();
  const [volley,precision]=api.states();
  if(scenario==='CLOISON'){assert.deepEqual(Array.from(volley.damage),[29,29,0,29,29,0,0]);assert(!precision.started);assert.equal(precision.energy,100);}
  if(scenario==='MACHINE'){assert.equal(volley.blood_events,0);assert.equal(precision.blood_events,0);assert(volley.reaction_events>0);}
  if(scenario==='MANQUE'){assert.equal(volley.damage[2],0);assert.equal(volley.misses,1);assert.equal(precision.hits,0);assert.equal(precision.misses,1);}
  if(scenario==='MAXIMUM'){assert.equal(volley.projectiles,5);assert.equal(volley.damage[5],0);assert.equal(volley.damage[6],0);assert.equal(precision.projectiles,1);}
  if(scenario==='ATTENTE'){assert(elements.get('action').disabled);const before=JSON.stringify(api.states());wait();assert.equal(JSON.stringify(api.states()),before);assert.equal(volley.cooldown,6);assert.equal(precision.cooldown,4);}
 }
 assert(draws.length&&draws.every(a=>a.slice(1).every(Number.isFinite)&&a[3]===576&&a[4]===576&&a[7]===576&&a[8]===576));
 const proof={method:'Offline mock-DOM checks, no browser automation or rendered-page verification.',directions:4,situations:10,native_canvas:576,manual_launch_and_six_turns:true,wait_never_repeats_attacks:true,preview_removed_on_confirmation:true,one_shot_per_selected_target:true,single_precision_shot:true,wall_stop:true,target_cap:true,no_target_is_revealed_through_walls:true,machines_without_blood:true,atomic_fixture_refusals:true,pause_freezes_visual_clock:true,draw_calls:draws.length};
 fs.writeFileSync(path.join(review,'viewer-controller-verification.json'),JSON.stringify(proof,null,2)+'\n');console.log(proof);
}
main().catch(error=>{console.error(error);process.exitCode=1;});


