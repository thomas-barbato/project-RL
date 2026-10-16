// Offline DOM/Image harness; native GPU frames are inspected separately.
const fs=require('fs'),path=require('path'),vm=require('vm');
const root=__dirname;
async function main(){
 const html=fs.readFileSync(path.join(root,'cote-a-cote.html'),'utf8'),code=html.match(/<script>([\s\S]*?)<\/script>/)[1];fs.writeFileSync(path.join(root,'gallery-script.js'),code);
 for(const m of html.matchAll(/(?:href|src)="([^"]+)"/g))if(!m[1].startsWith('http')&&!fs.existsSync(path.join(root,m[1])))throw Error('Missing link '+m[1]);
 const controls=Object.fromEntries(['scenario','next','cast','status','reset','pause','native-view','fullscreen'].map(id=>[id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''}]));
 const canvases=['SF00','SF02'].map(id=>{const tip={hidden:true,textContent:''};return{dataset:{skill:id},width:576,height:576,parentElement:{hidden:false,querySelector:()=>tip},getContext:()=>({drawImage(){}}),getBoundingClientRect:()=>({left:0,top:0,width:288,height:288})};});
 let requested,images=0,now=100;
 class LocalImage{set src(v){if(!fs.existsSync(path.join(root,v)))throw Error('Missing frame '+v);images++;queueMicrotask(()=>this.onload());}}
 const context=vm.createContext({Image:LocalImage,Promise,Map,Math,Array,document:{getElementById:id=>controls[id],querySelectorAll:()=>canvases,body:{classList:{toggle(){}}},documentElement:{requestFullscreen:async()=>{}},exitFullscreen:async()=>{}},requestAnimationFrame:f=>requested=f});
 const evaluate=s=>vm.runInContext(s,context),settle=()=>new Promise(r=>setImmediate(r)),assert=(v,m)=>{if(!v)throw Error(m);},tick=(dt=1000)=>{now+=dt;requested(now);};
 const state=id=>evaluate(`(()=>{const e=entries().find(e=>e.id==='${id}');return e.timeline[frame(e)]})()`);
 async function choose(label){controls.scenario.value=label;await controls.scenario.onchange();tick();}
 evaluate(code);await settle();requested(now);assert(evaluate('ready'),'initial load');controls.cast.onclick();tick();assert(state('SF00').active&&state('SF02').active,'activation');
 for(const [i,d]of [[0,2],[1,1]]){canvases[i].onmousemove({clientX:250,clientY:18});const tip=canvases[i].parentElement.querySelector();assert(!tip.hidden,'scaled hover');assert(tip.textContent.includes('+'+50+' %')&&tip.textContent.includes('repérage : '+d),'conditional detection and bonus tooltip');}
 controls.next.onclick();tick();assert(state('SF00').last_damage===30&&state('SF00').last_bonus===10&&!state('SF00').active,'first reinforced attack');assert(canvases.every(c=>c.parentElement.querySelector().hidden),'icon tooltip removed after attack');
 const held=evaluate("frame(entries().find(e=>e.id==='SF00'))");tick(20000);assert(evaluate("frame(entries().find(e=>e.id==='SF00'))")===held,'finished effect replayed while idle');assert(state('SF00').shots===1&&state('SF00').total_damage===30,'idle generated another hit');
 controls.next.onclick();tick();assert(state('SF00').last_damage===20&&state('SF00').last_bonus===0&&state('SF00').total_damage===50,'ordinary followup');
 controls.next.onclick();tick();assert(state('SF00').active&&state('SF00').shots===2,'reactivation without attack');
 controls.next.onclick();tick();assert(state('SF00').last_damage===30&&state('SF00').total_damage===80&&!state('SF00').active,'second sneak hit');assert(controls.next.disabled,'step cap');
 const cases=evaluate('Array.from(new Set(manifest.map(e=>e.case)))');for(const label of cases){await choose(label);controls.cast.onclick();tick();for(const e of evaluate('entries()')){const n=evaluate(`frame(entries().find(e=>e.id==='${e.id}'))`);assert(n>=0&&n<160,'bounds '+label);}}
 await choose('SORT');controls.cast.onclick();tick();controls.next.onclick();tick();assert(state('SF00').last_damage===30&&state('SF02').last_damage===30,'magic direct bonus');const magicFrame=evaluate("frame(entries().find(e=>e.id==='SF00'))");assert(magicFrame===87,'magic hold frame');tick(10000);assert(evaluate("frame(entries().find(e=>e.id==='SF00'))")===87,'magic effect replay');
 for(const [label,rank,value]of [['RANG-11',11,31],['RANG-12',12,32]]){await choose(label);controls.cast.onclick();tick();controls.next.onclick();tick();assert(state('SF00').rank===rank&&state('SF00').last_damage===value,'rank scaling '+rank);}
 await choose('DECOUVERT');controls.cast.onclick();tick();controls.next.onclick();tick();assert(!state('SF00').active&&state('SF00').last_damage===20&&state('SF00').last_bonus===0,'ordinary scenario');
 await choose('CLOISON');controls.cast.onclick();tick();assert(controls.next.disabled&&state('SF00').shots===0,'wall fixture action unavailable');
 await choose('PAS');controls.cast.onclick();tick();controls.next.onclick();tick();assert(state('SF00').active&&state('SF00').energy===99&&state('SF00').player[1]===5,'stealth step');controls.next.onclick();tick();assert(state('SF00').last_damage===30,'bonus retained after move');
 const record={mode:'offline DOM/Image harness; no browser screenshot',scenarios:cases.length,loaded_image_references:images,two_canvases:2,first_bonus_then_normal:'passed',reactivation:'passed',idle_does_not_replay_impacts:'passed',scaled_bonus_and_detection_hover:'passed',icon_expiry:'passed',weapon_melee_and_spell_prepared_cases:'passed',rank11_rank12_and_movement:'passed',wall_fixture_unavailable:'passed',static_links:'passed',controller_syntax:'passed'};
 fs.writeFileSync(path.join(root,'browser-controller-verification.json'),JSON.stringify(record,null,2));console.log(JSON.stringify(record));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
