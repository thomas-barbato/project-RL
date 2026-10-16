// Offline controller tests with a mock DOM. No browser or file-page observation.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06');
const reviewNumber=+(process.argv[2]||90),reviewDir=path.join(base,'skill-animation-review-'+reviewNumber);
const metadata=JSON.parse(fs.readFileSync(path.join(reviewDir,'review.json'),'utf8')),ids=metadata.candidates.map(c=>c.id);
const catalogue=JSON.parse(fs.readFileSync(path.join(root,'assets/skills/agent/catalogue.json'),'utf8'));
function parameter(id,key){if(id==='AF06')return metadata.prepared_values[id][{duration_rank_turns:'duration',cooldown_turns:'cooldown',energy:'energy'}[key]];const p=catalogue.skills.find(s=>s.id===id).parameters[key];return p.kind==='constant'?p.value:p.base;}
function count(id,scenario){if(scenario==='JOUEUR')return 1;if(scenario==='MUR')return null;if(scenario==='RESISTE')return id==='AF06'?3:2;return id==='AF06'&&!['DEPLACEMENT','CHAINE'].includes(scenario)?4:3;}
const html=fs.readFileSync(path.join(base,'apercu-animations.html'),'utf8');
assert(html.includes(metadata.batch));
const script=html.match(/<script>([\s\S]*?)<\/script>/)[1],elements=new Map(),draws=[];
function element(id){if(!elements.has(id))elements.set(id,{disabled:false,textContent:'',dataset:{},checked:true,value:'',classList:{toggle(){}},getContext(){return {clearRect(){},drawImage(...args){draws.push(args);}};}});return elements.get(id);}
let tick;
class ImageStub{constructor(){this.naturalWidth=0;}set src(value){assert(fs.existsSync(path.join(base,value)));this.naturalWidth=2304;this.onload();}}
const context={document:{getElementById:element,fullscreenElement:null,documentElement:{requestFullscreen(){}},exitFullscreen(){}},Image:ImageStub,requestAnimationFrame(fn){tick=fn;},console,window:{},Map,Math,Promise};
vm.runInNewContext(script,context);
async function loaded(){for(let i=0;i<8;i++)await Promise.resolve();assert(context.window.afflictionsAreaReview.isReady());tick(100);}
function event(id,value){elements.get(id).onchange({target:{value,checked:value}});}
async function main(){
 await loaded();const api=context.window.afflictionsAreaReview;
 assert.deepEqual(Array.from(api.entries(),e=>e.id),ids);
 for(const dir of [0,1,2,3]){event('direction',String(dir));await loaded();assert(api.entries().every(e=>e.direction===dir));}
 let checked=0;
 for(const scenario of metadata.scenario_ids||['GROUPE','JOUEUR','ATTENTE','MUR','RESISTE','DEPLACEMENT']){
  event('case',scenario);await loaded();assert.equal(elements.get('direction').disabled,scenario!=='GROUPE');
  elements.get('manual').onclick();tick(200);assert(api.states().every((s,i)=>s.energy===100&&s.preview===(ids[i]!=='AF06')&&s.applied_count===0));
  elements.get('action').onclick();tick(220);assert(api.states().every((s,i)=>!s.preview&&s.cast&&s.cooldown===parameter(api.entries()[i].id,'cooldown_turns')));
  assert(api.states().every((s,i)=>count(ids[i],scenario)===null||s.applied_count===count(ids[i],scenario)));
  assert(api.states().every(s=>s.victims.every(v=>v.hostile||!v.applied)));
  if(scenario==='ATTENTE'){
   assert(elements.get('action').disabled);for(let i=0;i<100;i++)tick(300+i*100);
   assert(api.states().every((s,i)=>s.turn===0&&s.active_count===count(ids[i],scenario)));
  }else{
   for(let t=1;t<=3;t++){
    elements.get('action').onclick();tick(11000+t*20);assert(api.states().every((s,i)=>s.turn===t&&s.cooldown===Math.max(0,parameter(api.entries()[i].id,'cooldown_turns')-t)));
    assert(api.states().every((s,i)=>s.victims.every(v=>{const interrupted=v.index===0&&(scenario==='CHAINE'||(scenario==='DEGATS'&&ids[i]==='AF05'));return v.remaining===(v.applied&&!interrupted?Math.max(0,parameter(ids[i],'duration_rank_turns')-t):0);} )));
    if(['AF03','AF05'].includes(ids[0])&&t===2){assert.equal(api.states()[0].active_count,0);assert(api.states()[1].active_count>0);assert(!elements.get('phase').textContent.includes('ont disparu'));}
    if(scenario==='DEPLACEMENT'&&t===1)assert(api.states().every((s,i)=>!s.victims[0].inside_area_now&&s.victims[0].remaining===parameter(api.entries()[i].id,'duration_rank_turns')-1));
    if(scenario==='DEPLACEMENT'&&t===2)assert(api.states().every(s=>s.victims[3].inside_area_now&&!s.victims[3].icon_visible));
    if(ids[1]==='AF06'){const passive=api.states()[1];assert.equal(passive.energy,100);assert.equal(passive.cooldown,0);assert.equal(passive.death_procs,1);if(scenario==='CHAINE'){assert(passive.victims[0].dead);assert(!passive.victims[3].applied);}}
   }
   assert(api.states().every(s=>s.active_count===0));assert(elements.get('action').disabled);
  }
  elements.get('replay').onclick();tick(12000);assert(elements.get('action').disabled);
  api.freeze(18);tick(12020);assert.equal(api.frame(),18);elements.get('pause').onclick();tick(12040);assert.equal(elements.get('pause').textContent,'Pause');checked++;
 }
 assert(draws.length>0);assert(draws.every(a=>a.slice(1).every(Number.isFinite)&&a[3]===576&&a[4]===576&&a[7]===576&&a[8]===576));
 const receipt={method:'Offline JavaScript controller checks with mock DOM and image loader; no browser rendering claimed.',scenarios:checked,directions:4,manual_turns:true,real_time_wait_preserves_status:true,individual_resistance:true,ally_exclusion:true,status_follows_departing_actor:true,no_application_to_newcomer:true,expiry:true,damage_interruption:ids.includes('AF05'),no_retransmission_from_inherited_effect:ids.includes('AF06'),passive_has_no_energy_cost_or_cooldown:ids.includes('AF06'),native_canvas_dimensions:576,source_atlas_files_exist:true,draw_calls:draws.length};
 fs.writeFileSync(path.join(reviewDir,'viewer-controller-verification.json'),JSON.stringify(receipt,null,2)+'\n');console.log(receipt);
}
main().catch(e=>{console.error(e);process.exitCode=1;});
