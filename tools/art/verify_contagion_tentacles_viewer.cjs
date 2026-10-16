// Offline controller checks; no browser or file-page observation.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06'),review=path.join(base,'skill-animation-review-102');
const html=fs.readFileSync(path.join(base,'apercu-animations.html'),'utf8');assert(html.includes('AF06-A02'));
const script=html.match(/<script>([\s\S]*?)<\/script>/)[1],elements=new Map(),draws=[];
function element(id){if(!elements.has(id))elements.set(id,{disabled:false,textContent:'',value:'',getContext(){return{clearRect(){},drawImage(...args){draws.push(args);}};}});return elements.get(id);}
let tick,now=0;
class ImageStub{set src(p){assert(fs.existsSync(path.join(base,p)));this.onload();}}
const context={document:{getElementById:element},Image:ImageStub,requestAnimationFrame(f){tick=f;},console,window:{},Map,Math,Promise};vm.runInNewContext(script,context);
const api=context.window.contagionTentaclesReview;
async function loaded(){for(let i=0;i<10;i++)await Promise.resolve();assert(api.isReady());tick(now+=100);}
function wait(n=20){for(let i=0;i<n;i++){tick(now+=100);if(api.isReady())assert.equal(JSON.stringify(api.states()[0]),JSON.stringify(api.states()[1]));}}
async function main(){
 await loaded();elements.get('mode').onclick();
 for(const dir of [0,1,2,3]){
  await elements.get('direction').onchange({target:{value:String(dir)}});await loaded();assert(api.entries().every(e=>e.direction===dir));
  assert(api.entries()[0].comparison_variant==='reference'&&api.entries()[1].comparison_variant==='tentacles');
  assert(api.states().every(s=>s.source_alive&&!s.cast));elements.get('action').onclick();wait();
  assert(api.states().every(s=>s.cast&&s.applied_count===4&&s.energy===100&&s.cooldown===0));
  for(let turn=1;turn<=3;turn++){elements.get('action').onclick();wait();assert(api.states().every(s=>s.turn===turn));}
  assert(api.states().every(s=>s.active_count===0&&s.death_procs===1));assert(elements.get('action').disabled);
 }
 for(const scenario of ['GROUPE','RESISTE','MUR','DEPLACEMENT','JOUEUR','ATTENTE','DEGATS','CHAINE']){
  await elements.get('case').onchange({target:{value:scenario}});await loaded();
  assert(elements.get('direction').disabled===(scenario!=='GROUPE'));elements.get('action').onclick();wait();
  assert(api.states().every(s=>s.energy===100&&s.cooldown===0&&s.death_procs===1&&s.victims.every(v=>v.hostile||!v.applied)));
  if(scenario==='RESISTE')assert(api.states().every(s=>!s.victims[1].applied&&s.active_count===3));
  if(scenario==='MUR')assert(api.states().every(s=>s.active_count>0&&!s.victims[1].applied));
  if(scenario==='JOUEUR')assert(api.states().every(s=>s.applied_count===1&&s.victims[2].player));
  const frame=api.frame();elements.get('pause').onclick();wait();assert.equal(api.frame(),frame);elements.get('pause').onclick();
  if(scenario==='ATTENTE'){const before=JSON.stringify(api.states());wait();assert.equal(JSON.stringify(api.states()),before);assert(elements.get('action').disabled);continue;}
  for(let turn=1;turn<=3;turn++){
   elements.get('action').onclick();wait();assert(api.states().every(s=>s.turn===turn));
   if(scenario==='DEPLACEMENT'&&turn===1)assert(api.states().every(s=>!s.victims[0].inside_area_now&&s.victims[0].icon_visible));
   if(scenario==='DEPLACEMENT'&&turn===2)assert(api.states().every(s=>s.victims[3].inside_area_now&&!s.victims[3].applied));
   if(scenario==='DEGATS'&&turn===1)assert(api.states().every(s=>s.victims[0].damage===10&&s.victims[0].remaining===2));
   if(scenario==='CHAINE'&&turn===1)assert(api.states().every(s=>s.victims[0].dead&&s.death_procs===1&&!s.victims[3].applied));
  }
 }
 assert(draws.length&&draws.every(a=>a.slice(1).every(Number.isFinite)&&a[3]===576&&a[4]===576&&a[7]===576&&a[8]===576));
 const proof={method:'Offline mock-DOM controller checks; no browser rendering claimed.',directions:4,situations:8,mechanical_states_identical_in_both_columns:true,manual_death_and_three_turns:true,wait_never_repeats_transmission:true,passive_cost_and_cooldown_zero:true,allies_excluded:true,individual_resistance:true,status_follows_actor:true,newcomer_unaffected:true,damage_does_not_end_inherited_rupture:true,no_chain_propagation:true,pause:true,native_canvas:576,draw_calls:draws.length};
 fs.writeFileSync(path.join(review,'viewer-controller-verification.json'),JSON.stringify(proof,null,2)+'\n');console.log(proof);
}
main().catch(error=>{console.error(error);process.exitCode=1;});
