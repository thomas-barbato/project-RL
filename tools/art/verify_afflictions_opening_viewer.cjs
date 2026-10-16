// Offline unit checks of the viewer controller. No browser or local-page access.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06');
const html=fs.readFileSync(path.join(base,'apercu-animations.html'),'utf8');
assert(html.includes('ANIM-S41'));
const script=html.match(/<script>([\s\S]*?)<\/script>/)[1];
const elements=new Map(),draws=[];
function element(id){if(!elements.has(id))elements.set(id,{disabled:false,textContent:'',dataset:{},checked:true,value:'',classList:{toggle(){}},getContext(){return {clearRect(){},drawImage(...args){draws.push(args);}};}});return elements.get(id);}
let tick;
class ImageStub{constructor(){this.naturalWidth=0;}set src(value){assert(fs.existsSync(path.join(base,value)));this.naturalWidth=2304;this.onload();}}
const context={document:{getElementById:element,fullscreenElement:null,documentElement:{requestFullscreen(){}},exitFullscreen(){}},Image:ImageStub,requestAnimationFrame(fn){tick=fn;},console,window:{},Map,Math,Promise};
vm.runInNewContext(script,context);
async function loaded(){for(let i=0;i<8;i++)await Promise.resolve();assert(context.window.afflictionsReview.isReady());tick(100);}
function event(id,value){elements.get(id).onchange({target:{value,checked:value}});}
async function main(){
 await loaded();const api=context.window.afflictionsReview;
 assert.deepEqual(Array.from(api.entries(),e=>e.id),['O02','AF02']);
 for(const dir of [0,1,2,3]){event('direction',String(dir));await loaded();assert(api.entries().every(e=>e.direction===dir));}
 let checked=0;
 for(const scenario of ['CIBLE','JOUEUR','ATTENTE','MUR','RESISTE']){
  event('case',scenario);await loaded();
  assert.equal(elements.get('direction').disabled,scenario!=='CIBLE');
  elements.get('manual').onclick();tick(200);
  assert(api.states().every(s=>s.energy===100&&s.remaining===0));
  elements.get('action').onclick();tick(220);
  assert(api.states().every(s=>s.preview===false));
  if(scenario==='MUR'){assert(api.states().every(s=>s.energy===100&&!s.cast));assert(elements.get('action').disabled);}
  else{
   assert(api.states().every(s=>s.cast&&s.cooldown===5));
   assert(api.states().every(s=>s.remaining===(scenario==='RESISTE'?0:3)));
   if(scenario==='ATTENTE'){
    assert(elements.get('action').disabled);
    for(let i=0;i<100;i++)tick(300+i*100);
    assert(api.states().every(s=>s.remaining===3&&s.turn===0));
   }else{
    for(let t=1;t<=3;t++){elements.get('action').onclick();tick(11000+t*20);assert(api.states().every(s=>s.turn===t&&s.remaining===(scenario==='RESISTE'?0:3-t)));}
    assert(api.states().every(s=>!s.icon_visible&&s.cooldown===2));assert(elements.get('action').disabled);
   }
  }
  elements.get('replay').onclick();tick(12000);assert.equal(elements.get('action').disabled,true);
  api.freeze(18);tick(12020);assert.equal(api.frame(),18);
  elements.get('pause').onclick();tick(12040);assert.equal(elements.get('pause').textContent,'Pause');checked++;
 }
 assert(draws.length>0);assert(draws.every(a=>a.slice(1).every(Number.isFinite)&&a[3]===576&&a[4]===576&&a[7]===576&&a[8]===576));
 const receipt={method:'Offline JavaScript controller unit checks with a mock DOM and image loader; no browser rendering claimed.',scenarios:checked,directions:4,manual_turns:true,real_time_wait_preserves_status:true,refused_cast:true,resisted_result:true,expiry:true,native_canvas_dimensions:576,source_atlas_files_exist:true,draw_calls:draws.length};
 fs.writeFileSync(path.join(base,'skill-animation-review-86/viewer-controller-verification.json'),JSON.stringify(receipt,null,2)+'\n');console.log(receipt);
}
main().catch(e=>{console.error(e);process.exitCode=1;});
