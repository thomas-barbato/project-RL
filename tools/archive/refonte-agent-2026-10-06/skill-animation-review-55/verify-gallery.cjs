// Offline controller/path checks. Native render captures provide visual evidence.
const fs=require('fs'),path=require('path'),vm=require('vm');
const root=__dirname,html=fs.readFileSync(path.join(root,'cote-a-cote.html'),'utf8');
const code=html.match(/<script>([\s\S]*?)<\/script>/)[1];
fs.writeFileSync(path.join(root,'gallery-script.js'),code);
for(const match of html.matchAll(/(?:href|src)="([^"]+)"/g)) {
  if(!match[1].startsWith('http')&&!fs.existsSync(path.join(root,match[1])))throw Error('Missing link '+match[1]);
}
const controls=Object.fromEntries(['scenario','next','cast','status','reset','pause','native-view','fullscreen'].map(id=>[id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''}]));
const canvases=['HL01','HL02'].map(id=>({dataset:{skill:id},parentElement:{hidden:false},getContext(){return {drawImage(image){this.current=image;}};}}));
let requested;
let imageCount=0;
class LocalImage {set src(value){if(!fs.existsSync(path.join(root,value)))throw Error('Missing frame '+value);this.path=value;imageCount++;queueMicrotask(()=>this.onload());}}
const context=vm.createContext({Image:LocalImage,Promise,Map,Math,Array,document:{getElementById(id){if(!controls[id])throw Error('Missing control '+id);return controls[id];},querySelectorAll(){return canvases;},body:{classList:{toggle(){}}},documentElement:{requestFullscreen:async()=>{}},exitFullscreen:async()=>{}},requestAnimationFrame(callback){requested=callback;}});
const evaluate=code=>vm.runInContext(code,context);
const settle=()=>new Promise(resolve=>setImmediate(resolve));
function assert(value,message){if(!value)throw Error(message);}
async function main(){
  evaluate(code);await settle();assert(evaluate('ready'),'Initial images not loaded');
  requested(100);assert(!controls.cast.disabled&&controls.next.disabled,'Initial controls');
  controls.cast.onclick();requested(1000);assert(!controls.next.disabled,'Next action disabled after impact');
  assert(evaluate("entries().find(e=>e.id==='HL02').timeline[frame(entries()[1])].stun[1]")===2,'Initial stun');
  requested(5000);assert(evaluate("entries()[1].timeline[frame(entries()[1])].stun[1]")===2,'Idle time advanced status');
  controls.next.onclick();requested(6000);assert(evaluate("entries()[1].timeline[frame(entries()[1])].stun[1]")===1,'First action did not decrement status');
  controls.next.onclick();requested(7000);assert(evaluate("entries()[1].timeline[frame(entries()[1])].stun[1]")===0,'Second action did not expire status');
  controls.reset.onclick();assert(evaluate('launched')===false,'Reset');
  const cases=evaluate('Array.from(new Set(manifest.map(e=>e.case)))');
  for(const label of cases){controls.scenario.value=label;await controls.scenario.onchange();requested(8000);assert(evaluate('ready'),'Scenario failed '+label);for(const e of evaluate('entries()')){const index=evaluate(`frame(entries().find(e=>e.id==='${e.id}'))`);assert(index>=0&&index<128,'Frame bounds '+label);}}
  const record={mode:'offline DOM/Image harness; no browser screenshot',html_static_links:'passed',all_scenarios:cases.length,loaded_frame_references:imageCount,two_canvases:canvases.length,manual_turn_status_expiry:'2 -> 1 -> 0',idle_time_preserves_stun:true,controller_syntax:'passed'};
  fs.writeFileSync(path.join(root,'browser-controller-verification.json'),JSON.stringify(record,null,2));console.log(JSON.stringify(record));
}
main().catch(error=>{console.error(error);process.exitCode=1;});
