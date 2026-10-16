// Offline controller verification only. Never opens or automates a browser.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06'),review=path.join(base,'skill-animation-review-95');
const html=fs.readFileSync(path.join(base,'apercu-animations.html'),'utf8');assert(html.includes('ANIM-S48'));
const script=html.match(/<script>([\s\S]*?)<\/script>/)[1],elements=new Map(),draws=[];
function element(id){if(!elements.has(id))elements.set(id,{disabled:false,textContent:'',value:'',getContext(){return {clearRect(){},drawImage(...a){draws.push(a);}};}});return elements.get(id);}
let tick,now=0;
class ImageStub{set src(p){assert(fs.existsSync(path.join(base,p)));this.onload();}}
const context={document:{getElementById:element},Image:ImageStub,requestAnimationFrame(f){tick=f;},console,window:{},Map,Math,Promise};vm.runInNewContext(script,context);
const api=context.window.droneRepairsReview;
async function loaded(){for(let i=0;i<10;i++)await Promise.resolve();assert(api.isReady());tick(now+=100);}
function wait(n=45){for(let i=0;i<n;i++)tick(now+=100);}
async function main(){
 await loaded();elements.get('mode').onclick();
 for(const direction of [0,1,2,3]){
  await elements.get('direction').onchange({target:{value:String(direction)}});await loaded();assert(api.entries().every(e=>e.direction===direction));
  assert.deepEqual(Array.from(api.states()[0].hp),[70,110,40]);elements.get('action').onclick();wait();
  assert.equal(api.states()[1].energy,88);assert.equal(api.states()[1].hp[0],116);assert.equal(api.states()[0].turn,0);
  for(let t=1;t<=6;t++){elements.get('action').onclick();wait();assert.equal(api.states()[0].turn,t);if(t<4)assert.equal(api.states()[0].hp[0],70);}
  assert.deepEqual(Array.from(api.states()[0].hp),[112,152,82]);assert.equal(api.states()[1].repaired_total,46);assert(elements.get('action').disabled);
 }
 for(const scenario of ['HOSTILE','PLAFOND','CLOISON','RAPPELE','DETRUIT','ENERGIE','ATTENTE']){
  await elements.get('case').onchange({target:{value:scenario}});await loaded();assert(elements.get('direction').disabled);
  elements.get('action').onclick();wait();
  const f=api.frame();elements.get('pause').onclick();wait();assert.equal(api.frame(),f);elements.get('pause').onclick();
  if(scenario==='ATTENTE'){assert(elements.get('action').disabled);wait();assert.equal(api.states()[0].turn,0);assert.equal(api.states()[0].events,0);assert.equal(api.states()[1].events,1);continue;}
  for(let i=0;i<6;i++){elements.get('action').onclick();wait();}
  const [passive,active]=api.states();
  if(scenario==='HOSTILE'){assert.equal(passive.quiet_turns,0);assert.equal(passive.repaired_total,42);assert.equal(active.events,1);}
  if(scenario==='PLAFOND'){assert.deepEqual(Array.from(passive.hp),[200,200,200]);assert.equal(active.repaired_total,1);}
  if(['CLOISON','RAPPELE','DETRUIT','ENERGIE'].includes(scenario)){assert.equal(active.events,0);assert.equal(active.energy,scenario==='ENERGIE'?11:100);assert.equal(active.turn,0);}
  if(['RAPPELE','DETRUIT'].includes(scenario))assert.equal(passive.events,0);
 }
 assert(draws.length&&draws.every(a=>a.slice(1).every(Number.isFinite)&&a[3]===576&&a[4]===576&&a[7]===576&&a[8]===576));
 const proof={method:'Offline mock-DOM checks, no browser automation or rendered-page verification.',directions:4,situations:8,native_canvases:576,manual_six_turns:true,wait_gives_no_extra_heal:true,active_immediate_hp_once:true,maintenance_calm_delay_and_interrupt:true,unavailable_targets_unchanged:true,health_cap:true,pause_freezes_visual_clock:true,draw_calls:draws.length};
 fs.writeFileSync(path.join(review,'viewer-controller-verification.json'),JSON.stringify(proof,null,2)+'\n');console.log(proof);
}
main().catch(e=>{console.error(e);process.exitCode=1;});
