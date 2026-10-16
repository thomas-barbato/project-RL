// Offline controller checks with a mock DOM; no browser/file-page automation.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06'),review=path.join(base,'skill-animation-review-92');
const html=fs.readFileSync(path.join(base,'apercu-animations.html'),'utf8');assert(html.includes('ANIM-S45'));
const script=html.match(/<script>([\s\S]*?)<\/script>/)[1],elements=new Map(),draws=[];
function element(id){if(!elements.has(id))elements.set(id,{disabled:false,textContent:'',dataset:{},checked:true,value:'',classList:{toggle(){}},getContext(){return {clearRect(){},drawImage(...a){draws.push(a);}};}});return elements.get(id);}
let tick;
class ImageStub{constructor(){this.naturalWidth=0;}set src(p){assert(fs.existsSync(path.join(base,p)));this.naturalWidth=3456;this.onload();}}
const context={document:{getElementById:element,fullscreenElement:null,documentElement:{requestFullscreen(){}},exitFullscreen(){}},Image:ImageStub,requestAnimationFrame(fn){tick=fn;},console,window:{},Map,Math,Promise};
vm.runInNewContext(script,context);
async function loaded(){for(let i=0;i<8;i++)await Promise.resolve();assert(context.window.droneCombatReview.isReady());tick(100);}
function change(id,value){elements.get(id).onchange({target:{value,checked:value}});}
async function main(){
 await loaded();const api=context.window.droneCombatReview;assert.deepEqual(Array.from(api.entries(),e=>e.id),['D58','D59']);
 for(const dir of [0,1,2,3]){change('direction',String(dir));await loaded();assert(api.entries().every(e=>e.direction===dir));}
 const scenarios=['COMBAT','CONTACT','DEPLACEMENT','CLOISON','MACHINE','ATTENTE','ENERGIE','EXISTANT'];
 for(const scenario of scenarios){
  change('case',scenario);await loaded();assert.equal(elements.get('direction').disabled,scenario!=='COMBAT');
  elements.get('manual').onclick();tick(200);const before=JSON.stringify(api.states());elements.get('action').onclick();tick(220);
  if(['ENERGIE','EXISTANT'].includes(scenario)){assert.equal(JSON.stringify(api.states()),before);assert(elements.get('action').disabled);}
  else{
   assert(api.states().every(s=>s.deployed&&s.launched&&s.energy===88&&s.cooldown===6&&s.deployments===1));
   if(scenario==='ATTENTE'){
    for(let i=0;i<100;i++)tick(300+i*100);
    assert(api.states().every(s=>s.turn===0&&s.cooldown===6&&s.deployed));
    assert.equal(api.states()[0].moves,1);assert.equal(api.states()[1].impacts,1);assert(elements.get('action').disabled);
   }else{
    for(let turn=1;turn<=3;turn++){
     elements.get('action').onclick();tick(11000+turn*1000);
     assert(api.states().every(s=>s.turn===turn&&s.cooldown===6-turn&&s.deployed&&s.energy===88));
     for(let j=1;j<=6;j++)tick(11000+turn*1000+j*100);
    }
    const ss=api.states();
    if(scenario==='CLOISON')assert(ss.every(s=>s.impacts===0&&s.enemy_hp===100&&s.moves===0));
    else{assert.equal(ss[0].impacts,scenario==='CONTACT'?4:scenario==='DEPLACEMENT'?1:2);assert.equal(ss[1].impacts,scenario==='DEPLACEMENT'?3:4);}
    if(scenario==='DEPLACEMENT')assert.equal(ss[1].moves,1);
    if(scenario==='MACHINE')assert(ss.every(s=>s.blood_events===0&&s.impacts>0));
    assert(elements.get('action').disabled);
   }
  }
  api.freeze(24);tick(20000);assert.equal(api.frame(),24);
 }
 assert(draws.length&&draws.every(a=>a.slice(1).every(Number.isFinite)&&a[3]===576&&a[4]===576&&a[7]===576&&a[8]===576));
 const receipt={method:'Offline mock-DOM controller checks; native rendering verified separately.',scenarios:scenarios.length,directions:4,atomic_refusals:true,invocation_turn_familiar_phase:true,untimed_wait_gives_no_extra_action:true,wall_blocks_both_drone_attacks:true,machine_has_no_blood:true,manual_three_turns:true,native_canvas:576,atlas_paths_exist:true,draw_calls:draws.length};
 fs.writeFileSync(path.join(review,'viewer-controller-verification.json'),JSON.stringify(receipt,null,2)+'\n');console.log(receipt);
}
main().catch(e=>{console.error(e);process.exitCode=1;});
