// Offline controller verification. No browser connection or automation.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06');
const html=fs.readFileSync(path.join(base,'apercu-animations.html'),'utf8');
assert(html.includes('O03-A01')&&!html.includes('D66-TAUNT-A01'));
const script=html.match(/<script>([\s\S]*?)<\/script>/)[1],els=new Map(),draws=[];
function element(id){if(!els.has(id))els.set(id,{disabled:false,hidden:false,textContent:'',value:id==='direction'?'2':id==='case1'?'ARTILLEUR':'TRAQUEUR',getContext(){return{clearRect(){},drawImage(...a){draws.push(a);}};}});return els.get(id);}
let tick,now=0;
class ImageStub{set src(p){assert(fs.existsSync(path.join(base,p)),p);this.onload();}}
const sandbox={document:{getElementById:element},Image:ImageStub,requestAnimationFrame(fn){tick=fn;},console,window:{},Map,Math,Promise};
vm.runInNewContext(script,sandbox);
const api=sandbox.window.combatAlgorithmsReview;
async function loaded(){for(let i=0;i<12;i++)await Promise.resolve();assert(api.isReady());tick(now+=100);}
function advance(n=80){for(let i=0;i<n;i++)tick(now+=100);}
async function scenes(left,right){element('case0').value=left;element('case1').value=right;await element('case0').onchange();await loaded();}
function checkPassive(){for(let i=0;i<2;i++){const s=api.state(i);assert.equal(s.energy,100);assert.equal(s.cooldown,0);assert.equal(s.deployments,0);assert.equal(s.passive_bonus_percent,25);assert(!s.preview);}}
async function main(){
 await loaded();assert.equal(api.entries()[0].case,'TRAQUEUR');assert.equal(api.entries()[1].case,'ARTILLEUR');element('mode').onclick();
 for(const model of ['COMBAT','TRAQUEUR','ARTILLEUR']){
  for(const d of [0,1,2,3]){
   element('direction').value=String(d);await scenes(model,model);
   assert(api.entries().every(e=>e.direction===d));checkPassive();
   element('action').onclick();advance();assert.equal(api.state(0).impacts,1);assert.equal(api.state(0).enemy_hp,135);checkPassive();
   const before=JSON.stringify(api.state(0));advance();assert.equal(JSON.stringify(api.state(0)),before);
   for(let n=1;n<=3;n++){element('turn').onclick();advance();assert.equal(api.state(0).turn,n);assert.equal(api.state(0).impacts,n+1);assert.equal(api.state(0).enemy_hp,160-25*(n+1));}
  }
 }
 await scenes('CLOISON','SOUTIEN');element('action').onclick();advance();
 for(let n=0;n<3;n++){element('turn').onclick();advance();}
 assert(!api.state(0).line_of_sight);for(let i=0;i<2;i++){assert.equal(api.state(i).impacts,0);assert.equal(api.state(i).enemy_hp,160);}checkPassive();
 await scenes('MACHINE','ATTENTE');element('action').onclick();advance();
 for(let n=0;n<3;n++){element('turn').onclick();advance();}
 assert.equal(api.state(0).blood_events,0);assert.equal(api.state(0).machine_hits,4);
 assert.equal(api.state(1).turn,0);assert.equal(api.state(1).impacts,1);
 await scenes('ATTENTE','ATTENTE');element('action').onclick();advance();assert(element('turn').disabled);
 const frame=api.frame();element('pause').onclick();advance();assert.equal(api.frame(),frame);element('pause').onclick();
 element('replay').onclick();assert(!api.state(0).launched&&!api.state(1).launched);
 element('mode').onclick();advance();assert(api.frame()>=0&&api.frame()<144);
 assert(draws.length&&draws.every(a=>a.slice(1).every(Number.isFinite)&&a[3]===576&&a[4]===576&&a[7]===576&&a[8]===576));
 const proof={method:'Offline mock DOM; no browser rendering claim.',native_canvas:576,armed_models:3,directions_per_model:4,
  two_scenes_visible:true,passive_no_activation_cost_or_cooldown:true,three_manual_turns:true,
  waiting_never_adds_an_attack:true,support_stays_unarmed:true,wall_blocks_attack:true,
  machines_spark_without_blood:true,pause:true,draw_calls:draws.length};
 fs.writeFileSync(path.join(base,'skill-animation-review-104/viewer-controller-verification.json'),JSON.stringify(proof,null,2)+'\n');console.log(proof);
}
main().catch(e=>{console.error(e);process.exitCode=1;});
