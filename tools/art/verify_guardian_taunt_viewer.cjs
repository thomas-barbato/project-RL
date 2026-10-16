// Offline mock-DOM checks only; this does not open or automate a browser.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06'),review=path.join(base,'skill-animation-review-94');
const html=fs.readFileSync(path.join(base,'apercu-animations.html'),'utf8');assert(html.includes('D66-TAUNT-A01'));
const script=html.match(/<script>([\s\S]*?)<\/script>/)[1],elements=new Map(),draws=[];
function element(id){if(!elements.has(id))elements.set(id,{disabled:false,hidden:false,textContent:'',value:id==='case'?'LIBRE':id==='direction'?'2':'',getContext(){return {clearRect(){},drawImage(...a){draws.push(a);}};}});return elements.get(id);}
let tick,now=0;
class ImageStub{set src(p){assert(fs.existsSync(path.join(base,p)));this.onload();}}
const context={document:{getElementById:element},Image:ImageStub,requestAnimationFrame(fn){tick=fn;},console,window:{},Map,Math,Promise};
vm.runInNewContext(script,context);
const api=context.window.guardianTauntReview;
async function loaded(){for(let i=0;i<8;i++)await Promise.resolve();assert(api.isReady());tick(now+=100);}
function advance(ticks=70){for(let i=0;i<ticks;i++)tick(now+=100);}
async function main(){
 await loaded();
 elements.get('mode').onclick();assert.equal(api.state().launched,false);
 for(const direction of [0,1,2,3]){
  elements.get('direction').value=String(direction);await elements.get('direction').onchange();await loaded();
  assert.equal(api.entry().direction,direction);assert.equal(api.state().preview,true);
  elements.get('action').onclick();advance();assert.equal(api.state().icons,3);assert.equal(api.state().turn,0);assert.equal(api.state().preview,false);
  elements.get('turn').onclick();advance();assert.equal(api.state().turn,1);assert(api.state().remaining.every(r=>r===1));
  elements.get('turn').onclick();advance();assert.equal(api.state().turn,2);assert.equal(api.state().icons,0);
 }
 for(const scenario of ['CLOISON','DEUX_GARDIENS','RAPPEL','ATTENTE']){
  elements.get('case').value=scenario;await elements.get('case').onchange();await loaded();assert(elements.get('direction').disabled);
  elements.get('action').onclick();advance();
  assert.deepEqual(Array.from(api.state().sources),scenario==='CLOISON'?[0,null,0]:scenario==='DEUX_GARDIENS'?[0,1,0]:[0,0,0]);
  assert.equal(api.state().turn,0);
  const frame=api.frame();elements.get('pause').onclick();advance();assert.equal(api.frame(),frame);elements.get('pause').onclick();
  if(scenario==='ATTENTE'){assert(elements.get('turn').disabled);advance();assert.equal(api.state().turn,0);assert.equal(api.state().icons,3);}
  else {elements.get('turn').onclick();advance();assert.equal(api.state().turn,1);if(scenario==='RAPPEL'){assert(api.state().recalled);assert.equal(api.state().icons,0);}}
 }
 elements.get('mode').onclick();advance();assert(api.frame()>=0&&api.frame()<120);
 assert(draws.length>0&&draws.every(a=>a.slice(1).every(Number.isFinite)&&a[3]===576&&a[4]===576&&a[7]===576&&a[8]===576));
 const receipt={method:'Offline mock-DOM checks, no browser rendering or automation.',four_directions:true,preview_removed:true,manual_wait_preserves_status:true,manual_turns_expire_status:true,wall_excludes_target:true,multiple_guardians_single_marker:true,recall_removes_marker:true,pause_freezes_visual_clock:true,native_canvas:576,draw_calls:draws.length};
 fs.writeFileSync(path.join(review,'viewer-controller-verification.json'),JSON.stringify(receipt,null,2)+'\n');console.log(receipt);
}
main().catch(e=>{console.error(e);process.exitCode=1;});
