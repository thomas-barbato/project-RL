// Offline controller checks with a mock DOM; no browser/file-page automation.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06'),review=path.join(base,'skill-animation-review-93');
const html=fs.readFileSync(path.join(base,'apercu-animations.html'),'utf8');assert(html.includes('ANIM-S46'));
const script=html.match(/<script>([\s\S]*?)<\/script>/)[1],elements=new Map(),draws=[];
function element(id){if(!elements.has(id))elements.set(id,{disabled:false,textContent:'',dataset:{},checked:true,value:'',classList:{toggle(){}},getContext(){return {clearRect(){},drawImage(...a){draws.push(a);}};}});return elements.get(id);}
let tick;
class ImageStub{constructor(){this.naturalWidth=0;}set src(p){assert(fs.existsSync(path.join(base,p)));this.naturalWidth=3456;this.onload();}}
const context={document:{getElementById:element,fullscreenElement:null,documentElement:{requestFullscreen(){}},exitFullscreen(){}},Image:ImageStub,requestAnimationFrame(fn){tick=fn;},console,window:{},Map,Math,Promise};
vm.runInNewContext(script,context);
async function loaded(){for(let i=0;i<8;i++)await Promise.resolve();assert(context.window.droneGuardianReview.isReady());tick(100);}
function change(id,value){elements.get(id).onchange({target:{value,checked:value}});}
async function main(){
 await loaded();const api=context.window.droneGuardianReview;assert.deepEqual(Array.from(api.entries(),e=>e.id),['D66','O03']);
 for(const dir of [0,1,2,3]){change('direction',String(dir));await loaded();assert(api.entries().every(e=>e.direction===dir));}
 const scenarios=['COMBAT','TRAQUEUR','ARTILLEUR','HORS_PORTEE','CLOISON','MACHINE','SOUTIEN','ATTENTE','ENERGIE','EXISTANT'];
 for(const scenario of scenarios){
  change('case',scenario);await loaded();assert.equal(elements.get('direction').disabled,scenario!=='COMBAT');
  elements.get('manual').onclick();tick(200);const before=JSON.stringify(api.states()[0]);elements.get('action').onclick();tick(220);
  const refused=['ENERGIE','EXISTANT'].includes(scenario);
  let ss=api.states();
  if(refused)assert.equal(JSON.stringify(ss[0]),before);
  else assert(ss[0].deployed&&ss[0].launched&&ss[0].energy===86&&ss[0].cooldown===6&&ss[0].deployments===1);
  assert(ss[1].launched&&ss[1].deployed&&ss[1].deployments===0&&ss[1].cooldown===0&&ss[1].energy===(scenario==='ENERGIE'?13:100));
  if(scenario==='ATTENTE'){
   for(let i=0;i<100;i++)tick(300+i*100);
   ss=api.states();assert(ss.every(s=>s.turn===0&&s.impacts===1));assert(elements.get('action').disabled);
  }else{
   for(let turn=1;turn<=3;turn++){
    elements.get('action').onclick();tick(11000+turn*1000);
    ss=api.states();assert.equal(ss[1].turn,turn);assert.equal(ss[0].turn,refused?0:turn);
    for(let j=1;j<=7;j++)tick(11000+turn*1000+j*100);
   }
   ss=api.states();
   if(!refused){
    assert.equal(ss[0].received_hits,3);
    const protectedAlly=!['HORS_PORTEE','CLOISON'].includes(scenario);
    assert.equal(ss[0].ally_protected,protectedAlly);
    assert.equal(ss[0].ally_hp,120-3*(protectedAlly?19:20));
    assert.equal(ss[0].impacts,scenario==='CLOISON'?0:4);
   }
   assert.equal(ss[1].impacts,['CLOISON','SOUTIEN'].includes(scenario)?0:4);
   if(scenario==='MACHINE')assert(ss.every(s=>s.blood_events===0&&s.machine_hits>0));
   assert(elements.get('action').disabled);
  }
  api.freeze(24);tick(20000);assert.equal(api.frame(),24);
 }
 assert(draws.length&&draws.every(a=>a.slice(1).every(Number.isFinite)&&a[3]===576&&a[4]===576&&a[7]===576&&a[8]===576));
 const receipt={method:'Offline mock-DOM controller checks; native rendering verified separately.',scenarios:scenarios.length,directions:4,atomic_refusals:true,invocation_turn_familiar_phase:true,untimed_wait_gives_no_extra_action:true,wall_blocks_drone_attacks:true,protection_blocked_by_wall_or_distance:true,unarmed_models_gain_no_attack:true,machine_has_no_blood:true,manual_three_turns:true,native_canvas:576,atlas_paths_exist:true,draw_calls:draws.length};
 fs.writeFileSync(path.join(review,'viewer-controller-verification.json'),JSON.stringify(receipt,null,2)+'\n');console.log(receipt);
}
main().catch(e=>{console.error(e);process.exitCode=1;});
