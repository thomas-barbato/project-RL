// Offline path/controller checks; visual inspection uses native GPU readbacks.
const fs=require('fs'),path=require('path'),vm=require('vm');
const root=__dirname,html=fs.readFileSync(path.join(root,'cote-a-cote.html'),'utf8');
const code=html.match(/<script>([\s\S]*?)<\/script>/)[1];
fs.writeFileSync(path.join(root,'gallery-script.js'),code);
for(const match of html.matchAll(/(?:href|src)="([^"]+)"/g))if(!match[1].startsWith('http')&&!fs.existsSync(path.join(root,match[1])))throw Error('Missing link '+match[1]);
const controls=Object.fromEntries(['scenario','next','cast','status','reset','pause','native-view','fullscreen'].map(id=>[id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''}]));
const canvases=['HL03','HL04'].map(id=>{const tip={hidden:true,textContent:''};return {dataset:{skill:id},width:576,height:576,parentElement:{hidden:false,querySelector(){return tip;}},getContext(){return {drawImage(){}};},getBoundingClientRect(){return {left:0,top:0,width:288,height:288};}};});
let requested,imageCount=0;
class LocalImage{set src(value){if(!fs.existsSync(path.join(root,value)))throw Error('Missing frame '+value);this.path=value;imageCount++;queueMicrotask(()=>this.onload());}}
const context=vm.createContext({Image:LocalImage,Promise,Map,Math,Array,document:{getElementById(id){if(!controls[id])throw Error('Missing control '+id);return controls[id];},querySelectorAll(){return canvases;},body:{classList:{toggle(){}}},documentElement:{requestFullscreen:async()=>{}},exitFullscreen:async()=>{}},requestAnimationFrame(callback){requested=callback;}});
const evaluate=code=>vm.runInContext(code,context),settle=()=>new Promise(resolve=>setImmediate(resolve));
function assert(value,message){if(!value)throw Error(message);}
const channel=()=>evaluate("entries().find(e=>e.id==='HL03').timeline[frame(entries().find(e=>e.id==='HL03'))]");
async function main(){
 evaluate(code);await settle();assert(evaluate('ready'),'Initial loading');requested(100);assert(!controls.cast.disabled&&controls.next.disabled,'Initial buttons');
 controls.cast.onclick();requested(1100);assert(channel().shots===1&&channel().remaining===4,'Initial channel');
 requested(5100);assert(channel().shots===1&&channel().remaining===4,'Real time advanced channel');
 canvases[0].onmousemove({clientX:250,clientY:18});assert(!canvases[0].parentElement.querySelector().hidden,'Scaled icon hover');
 for(let step=1;step<=3;step++){controls.next.onclick();requested(5100+step*1000);assert(channel().shots===step+1,'Per-turn shot '+step);}
 assert(!channel().active&&channel().cooldown===8,'Final shot did not end channel');
 canvases[0].onmousemove({clientX:250,clientY:18});assert(canvases[0].parentElement.querySelector().hidden,'Expired icon hover');
 controls.next.onclick();requested(9100);assert(channel().shots===4&&channel().cooldown===7,'Cooldown action');assert(controls.next.disabled,'Step cap');
 controls.reset.onclick();assert(!evaluate('launched'),'Reset');
 const cases=evaluate('Array.from(new Set(manifest.map(e=>e.case)))');
 for(const label of cases){controls.scenario.value=label;await controls.scenario.onchange();requested(10000);assert(evaluate('ready'),'Load '+label);for(const e of evaluate('entries()')){const n=evaluate(`frame(entries().find(e=>e.id==='${e.id}'))`);assert(n>=0&&n<160,'Frame bounds '+label);}}
 controls.scenario.value='LEGERE';await controls.scenario.onchange();controls.cast.onclick();requested(11000);canvases[0].onmousemove({clientX:250,clientY:18});assert(canvases[0].parentElement.querySelector().hidden,'Absent channel hover');
 const record={mode:'offline DOM/Image harness; no browser screenshot',static_links:'passed',scenarios:cases.length,loaded_frame_references:imageCount,manual_shots:'1 -> 2 -> 3 -> 4',idle_time_preserves_channel:true,final_cooldown_starts_at8:true,scaled_hover_and_expiry:'passed',hidden_panel_hover:'passed',two_canvases:2,controller_syntax:'passed'};
 fs.writeFileSync(path.join(root,'browser-controller-verification.json'),JSON.stringify(record,null,2));console.log(JSON.stringify(record));
}
main().catch(error=>{console.error(error);process.exitCode=1;});
