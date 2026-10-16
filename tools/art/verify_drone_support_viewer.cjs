// Offline controller checks with a mock DOM; no browser/file-page automation.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06'),review=path.join(base,'skill-animation-review-91');
const html=fs.readFileSync(path.join(base,'apercu-animations.html'),'utf8');assert(html.includes('ANIM-S44'));
const script=html.match(/<script>([\s\S]*?)<\/script>/)[1],elements=new Map(),draws=[];
function element(id){if(!elements.has(id))elements.set(id,{disabled:false,textContent:'',dataset:{},checked:true,value:'',classList:{toggle(){}},getContext(){return {clearRect(){},drawImage(...a){draws.push(a);}};}});return elements.get(id);}
let tick;
class ImageStub{constructor(){this.naturalWidth=0;}set src(p){assert(fs.existsSync(path.join(base,p)));this.naturalWidth=2304;this.onload();}}
const context={document:{getElementById:element,fullscreenElement:null,documentElement:{requestFullscreen(){}},exitFullscreen(){}},Image:ImageStub,requestAnimationFrame(fn){tick=fn;},console,window:{},Map,Math,Promise};
vm.runInNewContext(script,context);
async function loaded(){for(let i=0;i<8;i++)await Promise.resolve();assert(context.window.droneSupportReview.isReady());tick(100);}
function change(id,value){elements.get(id).onchange({target:{value,checked:value}});}
async function main(){
 await loaded();const api=context.window.droneSupportReview;assert.deepEqual(Array.from(api.entries(),e=>e.id),['D56','D57']);
 for(const dir of [0,1,2,3]){change('direction',String(dir));await loaded();assert(api.entries().every(e=>e.direction===dir));}
 const scenarios=['DEPLOIEMENT','ATTENTE','MUR','OCCUPEE','ENERGIE','EXISTANT','COLLECTE','CACHE'];
 for(const scenario of scenarios){
  change('case',scenario);await loaded();assert.equal(elements.get('direction').disabled,scenario!=='DEPLOIEMENT');
  elements.get('manual').onclick();tick(200);const before=JSON.stringify(api.states());elements.get('action').onclick();tick(220);
  if(['MUR','OCCUPEE','ENERGIE','EXISTANT'].includes(scenario)){assert.equal(JSON.stringify(api.states()),before);assert(elements.get('action').disabled);}
  else{
   assert(api.states().every(s=>s.deployed&&s.launched&&s.energy===90&&s.cooldown===6&&s.deployments===1));
   if(scenario==='ATTENTE'){for(let i=0;i<100;i++)tick(300+i*100);assert(api.states().every(s=>s.turn===0&&s.cooldown===6&&s.deployed));assert(elements.get('action').disabled);}
   else{for(let t=1;t<=3;t++){elements.get('action').onclick();tick(11000+t*20);assert(api.states().every(s=>s.turn===t&&s.cooldown===6-t&&s.deployed&&s.energy===90));if(scenario==='COLLECTE'&&t===1)assert(api.states()[1].ground_item&&!api.states()[1].cargo);if(scenario==='COLLECTE'&&t>=2){assert(api.states()[1].cargo&&!api.states()[1].ground_item&&api.states()[1].pickup_events===1);assert(api.states()[0].ground_item&&!api.states()[0].cargo);}if(scenario==='CACHE'){assert(api.states().every(s=>s.ground_item&&!s.cargo));assert(api.states()[0].loot_visible&&!api.states()[1].loot_visible);}}assert(elements.get('action').disabled);}
  }
  api.freeze(18);tick(13000);assert.equal(api.frame(),18);
 }
 assert(draws.length&&draws.every(a=>a.slice(1).every(Number.isFinite)&&a[3]===576&&a[4]===576&&a[7]===576&&a[8]===576));
 const receipt={method:'Offline mock-DOM controller checks; native rendering verified separately.',scenarios:scenarios.length,directions:4,atomic_refusals:true,persistent_deployment:true,untimed_wait_keeps_cooldown:true,one_mule_pickup_without_duplication:true,hidden_loot_not_collected:true,scout_does_not_collect:true,scout_shares_vision_but_mule_does_not:true,native_canvas:576,atlas_paths_exist:true,draw_calls:draws.length};
 fs.writeFileSync(path.join(review,'viewer-controller-verification.json'),JSON.stringify(receipt,null,2)+'\n');console.log(receipt);
}
main().catch(e=>{console.error(e);process.exitCode=1;});
