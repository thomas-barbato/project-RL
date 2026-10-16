// Offline DOM/Image checks. Native GPU readbacks provide the visual captures.
const fs=require('fs'),path=require('path'),vm=require('vm');
const root=__dirname;
async function main(){
 const html=fs.readFileSync(path.join(root,'cote-a-cote.html'),'utf8'),code=html.match(/<script>([\s\S]*?)<\/script>/)[1];fs.writeFileSync(path.join(root,'gallery-script.js'),code);
 for(const m of html.matchAll(/(?:href|src)="([^"]+)"/g))if(!m[1].startsWith('http')&&!fs.existsSync(path.join(root,m[1])))throw Error('Missing link '+m[1]);
 const controls=Object.fromEntries(['scenario','next','cast','status','reset','pause','native-view','fullscreen'].map(id=>[id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''}]));
 const canvases=['SF00','SF01'].map(id=>{const tip={hidden:true,textContent:''};return{dataset:{skill:id},width:576,height:576,parentElement:{hidden:false,querySelector:()=>tip},getContext:()=>({drawImage(){}}),getBoundingClientRect:()=>({left:0,top:0,width:288,height:288})};});
 let requested,images=0,now=100;
 class LocalImage{set src(v){if(!fs.existsSync(path.join(root,v)))throw Error('Missing frame '+v);images++;queueMicrotask(()=>this.onload());}}
 const context=vm.createContext({Image:LocalImage,Promise,Map,Math,Array,document:{getElementById:id=>controls[id],querySelectorAll:()=>canvases,body:{classList:{toggle(){}}},documentElement:{requestFullscreen:async()=>{}},exitFullscreen:async()=>{}},requestAnimationFrame:f=>requested=f});
 const evaluate=s=>vm.runInContext(s,context),settle=()=>new Promise(r=>setImmediate(r)),assert=(v,m)=>{if(!v)throw Error(m);},tick=(dt=1000)=>{now+=dt;requested(now);};
 const state=id=>evaluate(`(()=>{const e=entries().find(e=>e.id==='${id}');return e.timeline[frame(e)]})()`);
 async function choose(label){controls.scenario.value=label;await controls.scenario.onchange();tick();}
 evaluate(code);await settle();requested(now);assert(evaluate('ready'),'load');controls.cast.onclick();tick();
 assert(state('SF00').active&&state('SF01').active,'activation');assert(state('SF00').energy===100&&state('SF01').energy===100,'activation cost');
 tick(20000);assert(state('SF00').turn===1&&state('SF00').energy===100,'idle drain');
 canvases[0].onmousemove({clientX:250,clientY:18});assert(!canvases[0].parentElement.querySelector().hidden,'scaled hover');assert(canvases[0].parentElement.querySelector().textContent.includes('11 énergie'),'base tooltip');
 canvases[1].onmousemove({clientX:250,clientY:18});assert(canvases[1].parentElement.querySelector().textContent.includes('1 énergie'),'passive tooltip');
 for(let n=1;n<=4;n++){controls.next.onclick();tick();assert(state('SF00').walks===n&&state('SF00').spent===11*n,'base cost '+n);assert(state('SF01').walks===n&&state('SF01').spent===n,'economy cost '+n);}
 assert(controls.next.disabled,'step limit');controls.reset.onclick();assert(!evaluate('launched'),'reset');
 const cases=evaluate('Array.from(new Set(manifest.map(e=>e.case)))');for(const label of cases){await choose(label);assert(evaluate('ready'),'load '+label);for(const e of evaluate('entries()')){const n=evaluate(`frame(entries().find(e=>e.id==='${e.id}'))`);assert(n>=0&&n<160,'bounds '+label);}}
 await choose('ZERO-ENERGIE');controls.cast.onclick();tick();canvases[0].onmousemove({clientX:250,clientY:18});assert(!canvases[0].parentElement.querySelector().hidden,'zero before walk');controls.next.onclick();tick();assert(!state('SF00').active&&state('SF00').walks===1&&state('SF00').player[1]===5,'zero step at normal visibility');assert(canvases[0].parentElement.querySelector().hidden,'tooltip clears without mouse motion');
 await choose('MENU');controls.cast.onclick();tick();controls.next.onclick();tick();assert(state('SF00').menu&&state('SF00').active&&state('SF00').turn===1&&state('SF00').energy===100,'menu preservation');
 await choose('EPUISEMENT');controls.cast.onclick();tick();for(let n=0;n<3;n++){controls.next.onclick();tick();}assert(state('SF00').active&&state('SF00').energy===0,'last paid step');controls.next.onclick();tick();assert(!state('SF00').active&&state('SF00').walks===4&&state('SF00').spent===33,'exhaustion next step');assert(!state('SF01').active&&state('SF01').spent===3,'economy exhaustion');
 await choose('RANG-9');assert(canvases[0].parentElement.hidden,'one panel');canvases[0].onmousemove({clientX:250,clientY:18});controls.cast.onclick();tick();controls.next.onclick();tick();assert(state('SF01').cost===2&&state('SF01').spent===2,'rank9');
 await choose('BONUS-EQUIPEMENT');controls.cast.onclick();tick();assert(state('SF01').rank===12&&state('SF01').cost===1,'gear minimum');
 const record={mode:'offline DOM/Image harness; no browser screenshot',scenarios:cases.length,loaded_image_references:images,two_canvases:2,manual_steps_and_energy:'passed',idle_and_menu:'passed',zero_energy_step_and_expired_hover:'passed',scaled_tooltip:'passed',rank9_and_equipment_floor:'passed',static_links:'passed',controller_syntax:'passed'};
 fs.writeFileSync(path.join(root,'browser-controller-verification.json'),JSON.stringify(record,null,2));console.log(JSON.stringify(record));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
