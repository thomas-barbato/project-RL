// Offline viewer checks. Does not launch, connect to or automate a browser.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06');
const html=fs.readFileSync(path.join(base,'apercu-animations.html'),'utf8');
assert(html.includes('D66-TAUNT-A01')&&html.includes('D66-TAUNT-I01'));
const script=html.match(/<script>([\s\S]*?)<\/script>/)[1],els=new Map(),draws=[];
function element(id){if(!els.has(id))els.set(id,{disabled:false,hidden:false,textContent:'',value:id==='direction'?'2':id==='case1'?'DEUX_GARDIENS':'LIBRE',getContext(){return{clearRect(){},drawImage(...a){draws.push(a);}};}});return els.get(id);}
let tick,now=0;
class ImageStub{set src(p){assert(fs.existsSync(path.join(base,p)),p);this.onload();}}
const sandbox={document:{getElementById:element},Image:ImageStub,requestAnimationFrame(fn){tick=fn;},console,window:{},Map,Math,Promise};
vm.runInNewContext(script,sandbox);
const api=sandbox.window.guardianFinalReview;
async function loaded(){for(let i=0;i<12;i++)await Promise.resolve();assert(api.isReady());tick(now+=100);}
function advance(n=75){for(let i=0;i<n;i++)tick(now+=100);}
async function scene(left,right){element('case0').value=left;element('case1').value=right;await element('case0').onchange();await loaded();}
async function main(){
 await loaded();assert.equal(api.entries()[0].case,'LIBRE');assert.equal(api.entries()[1].case,'DEUX_GARDIENS');
 element('mode').onclick();
 for(const d of [0,1,2,3]){
  element('direction').value=String(d);await element('direction').onchange();await loaded();
  assert.equal(api.entries()[0].direction,d);assert.equal(api.entries()[1].direction,2);
  assert(api.state(0).preview&&api.state(1).preview);
  element('action').onclick();advance();
  assert(!api.state(0).preview&&!api.state(1).preview);
  assert.deepEqual(Array.from(api.state(1).sources),[0,1,0]);
  assert.equal(api.state(0).icons,3);assert.equal(api.state(1).icons,3);
  element('turn').onclick();advance();assert.equal(api.state(0).turn,1);
  element('turn').onclick();advance();assert.equal(api.state(0).icons,0);assert.equal(api.state(1).icons,0);
 }
 await scene('CLOISON','RAPPEL');element('action').onclick();advance();
 assert.deepEqual(Array.from(api.state(0).sources),[0,null,0]);assert.equal(api.state(0).icons,2);
 assert.equal(api.state(1).icons,3);element('turn').onclick();advance();
 assert.equal(api.state(0).icons,2);assert(api.state(1).recalled);assert.equal(api.state(1).icons,0);
 await scene('ATTENTE','LIBRE');element('action').onclick();advance();
 element('turn').onclick();advance();element('turn').onclick();advance();
 assert.equal(api.state(0).turn,0);assert.equal(api.state(0).icons,3);assert.equal(api.state(1).icons,0);
 await scene('ATTENTE','ATTENTE');element('action').onclick();advance();assert(element('turn').disabled);
 const frame=api.frame();element('pause').onclick();advance();assert.equal(api.frame(),frame);element('pause').onclick();
 element('replay').onclick();assert(api.state(0).preview&&api.state(1).preview);
 element('mode').onclick();advance();assert(api.frame()>=0&&api.frame()<120);
 assert(draws.length&&draws.every(a=>a.slice(1).every(Number.isFinite)&&a[3]===576&&a[4]===576&&a[7]===576&&a[8]===576));
 const proof={method:'Offline mock DOM, no browser rendering claim.',native_canvas:576,directions:4,situations:5,
  two_scenes_visible:true,preview_removed:true,single_source_per_target:true,wall_excludes_target:true,
  recall_removes_only_its_sources:true,independent_wait_scene_preserves_status:true,manual_turns:true,pause:true,
  same_native_atlases:true,draw_calls:draws.length};
 fs.writeFileSync(path.join(base,'skill-animation-review-103/viewer-controller-verification.json'),JSON.stringify(proof,null,2)+'\n');console.log(proof);
}
main().catch(e=>{console.error(e);process.exitCode=1;});
