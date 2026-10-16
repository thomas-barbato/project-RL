// Offline DOM/Image verification; native GPU readbacks are inspected separately.
const fs=require('fs'),path=require('path'),vm=require('vm');
const root=__dirname,html=fs.readFileSync(path.join(root,'cote-a-cote.html'),'utf8'),code=html.match(/<script>([\s\S]*?)<\/script>/)[1];
fs.writeFileSync(path.join(root,'gallery-script.js'),code);
for(const m of html.matchAll(/(?:href|src)="([^"]+)"/g))if(!m[1].startsWith('http')&&!fs.existsSync(path.join(root,m[1])))throw Error('Missing link '+m[1]);
const controls=Object.fromEntries(['scenario','next','cast','status','reset','pause','native-view','fullscreen'].map(id=>[id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''}]));
const canvases=['HL05','HL06'].map(id=>({dataset:{skill:id},width:576,height:576,parentElement:{hidden:false},getContext(){return {drawImage(){}};}}));
let requested,imageCount=0,now=100;
class LocalImage{set src(value){if(!fs.existsSync(path.join(root,value)))throw Error('Missing frame '+value);imageCount++;queueMicrotask(()=>this.onload());}}
const context=vm.createContext({Image:LocalImage,Promise,Map,Math,Array,document:{getElementById:id=>controls[id],querySelectorAll:()=>canvases,body:{classList:{toggle(){}}},documentElement:{requestFullscreen:async()=>{}},exitFullscreen:async()=>{}},requestAnimationFrame:f=>requested=f});
const evaluate=s=>vm.runInContext(s,context),settle=()=>new Promise(resolve=>setImmediate(resolve)),assert=(v,m)=>{if(!v)throw Error(m);};
const tick=(dt=1000)=>{now+=dt;requested(now);},state=id=>evaluate(`(()=>{const e=entries().find(e=>e.id==='${id}');return e.timeline[frame(e)]})()`);
async function choose(label){controls.scenario.value=label;await controls.scenario.onchange();tick();}
async function main(){
 evaluate(code);await settle();requested(now);assert(evaluate('ready'),'initial load');assert(!controls.cast.disabled&&controls.next.disabled,'initial buttons');
 controls.cast.onclick();tick();assert(state('HL06').hp===65&&state('HL06').healed===15&&state('HL06').cooldown===10,'initial impact/heal');
 tick(20000);assert(state('HL06').hp===65&&state('HL06').cooldown===10&&state('HL06').turn===1,'real time advanced turns');
 for(let step=1;step<=3;step++){controls.next.onclick();tick();assert(state('HL06').cooldown===10-step,'manual cooldown '+step);}
 assert(controls.next.disabled,'step cap');controls.reset.onclick();assert(!evaluate('launched'),'reset');
 const cases=evaluate('Array.from(new Set(manifest.map(e=>e.case)))');
 for(const label of cases){await choose(label);assert(evaluate('ready'),'load '+label);for(const e of evaluate('entries()'))for(const span of [0,30,500,9000]){evaluate('elapsed='+span);const n=evaluate(`frame(entries().find(e=>e.id==='${e.id}'))`);assert(n>=0&&n<128,'preview bounds '+label);}controls.cast.onclick();tick();for(const e of evaluate('entries()')){const s=state(e.id);assert(s.turn===(e.permitted?1:0),'refusal '+label+'/'+e.id);}}
 await choose('RETARDEMENT');assert(canvases[0].parentElement.hidden&&!canvases[1].parentElement.hidden,'single-skill layout');controls.cast.onclick();tick();assert(state('HL06').pending&&state('HL06').healed===0,'early delayed heal');tick(10000);assert(state('HL06').pending&&state('HL06').hp===50,'idle delayed explosion');controls.next.onclick();tick();assert(!state('HL06').pending&&state('HL06').hp===65&&state('HL06').cooldown===9,'delayed manual action');
 for(const [label,heal,hp] of [['PV-PLEINS',0,100],['PV-MANQUANTS',3,100],['CIBLE-10-PV',6,56],['PROTECTIONS',12,62]]){await choose(label);controls.cast.onclick();tick();assert(state('HL06').healed===heal&&state('HL06').hp===hp,'heal limit '+label);}
 await choose('ATTENTE');controls.cast.onclick();tick();assert(controls.next.disabled,'idle scenario next disabled');
 const record={mode:'offline DOM/Image harness; no browser screenshot',static_links:'passed',scenarios:cases.length,loaded_frame_references:imageCount,two_canvases:2,manual_cooldowns_and_idle:'passed',delayed_impact_and_heal:'passed',missing_and_full_HP_limits:'passed',atomic_prepared_refusals:'passed',single_skill_layout:'passed',controller_syntax:'passed'};
 fs.writeFileSync(path.join(root,'browser-controller-verification.json'),JSON.stringify(record,null,2));console.log(JSON.stringify(record));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
