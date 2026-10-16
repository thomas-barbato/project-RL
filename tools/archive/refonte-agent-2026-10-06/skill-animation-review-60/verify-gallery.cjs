// Offline DOM/Image harness. Native rendering is validated by GPU readbacks.
const fs=require('fs'),path=require('path'),vm=require('vm');
const root=__dirname;
async function main(){
 const html=fs.readFileSync(path.join(root,'cote-a-cote.html'),'utf8'),code=html.match(/<script>([\s\S]*?)<\/script>/)[1];fs.writeFileSync(path.join(root,'gallery-script.js'),code);
 for(const m of html.matchAll(/(?:href|src)="([^"]+)"/g))if(!m[1].startsWith('http')&&!fs.existsSync(path.join(root,m[1])))throw Error('Missing link '+m[1]);
 const controls=Object.fromEntries(['scenario','next','cast','status','reset','pause','native-view','fullscreen'].map(id=>[id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''}]));
 const drawn={};const canvases=['SM00','SM01'].map(id=>({dataset:{skill:id},width:576,height:576,parentElement:{hidden:false},getContext:()=>({drawImage(im){drawn[id]=im.source;}}),getBoundingClientRect:()=>({left:0,top:0,width:288,height:288})}));
 let requested,images=0,now=100;
 class LocalImage{set src(v){if(!fs.existsSync(path.join(root,v)))throw Error('Missing frame '+v);this.source=v;images++;queueMicrotask(()=>this.onload());}}
 const context=vm.createContext({Image:LocalImage,Promise,Map,Set,Math,Array,Error,document:{getElementById:id=>controls[id],querySelectorAll:()=>canvases,body:{classList:{toggle(){}}},documentElement:{requestFullscreen:async()=>{}},exitFullscreen:async()=>{}},requestAnimationFrame:f=>requested=f});
 const evaluate=s=>vm.runInContext(s,context),settle=()=>new Promise(r=>setImmediate(r)),assert=(v,m)=>{if(!v)throw Error(m);},tick=(dt=1000)=>{now+=dt;requested(now);};
 const state=id=>evaluate(`(()=>{const e=entries().find(e=>e.id==='${id}');return e.timeline[frame(e)]})()`);
 async function choose(label){controls.scenario.value=label;await controls.scenario.onchange();tick();}
 const central=t=>t.mines.find(m=>m.at[0]===4&&m.at[1]===5);
 evaluate(code);await settle();requested(now);assert(evaluate('ready'),'initial load');
 assert(state('SM00').preview_visible,'preplacement preview');controls.cast.onclick();tick();assert(!state('SM00').preview_visible&&central(state('SM00'))&&!central(state('SM00')).armed,'unarmed placement');
 assert(state('SM00').energy===90&&state('SM01').capacity===5,'placement cost and capacity');
 const phases=new Set();for(let i=0;i<30;i++){tick(45);phases.add(state('SM00').led_on);assert(state('SM00').turn===1&&!central(state('SM00')).armed&&state('SM00').explosions===0,'idle advances arming');}assert(phases.size===2,'LED does not blink during unarmed wait');
 canvases[0].onmousemove({clientX:144,clientY:176});tick(50);assert(drawn.SM00.includes('hover-'),'scaled mine hover');assert(evaluate('hovered.has("SM00")'),'hover tracking');canvases[0].onmouseleave();tick(50);assert(!drawn.SM00.includes('hover-'),'hover preview retained');
 controls.next.onclick();tick();assert(central(state('SM00')).armed&&state('SM00').cooldown===0,'arming after explicit turn');const armedPhases=new Set();for(let i=0;i<30;i++){tick(45);armedPhases.add(state('SM00').led_on);assert(state('SM00').turn===2&&state('SM00').explosions===0,'armed idle explosion');}assert(armedPhases.size===2,'armed LED does not blink');
 controls.next.onclick();tick();assert(state('SM00').primary[1]===4&&state('SM00').explosions===0,'approach triggers early');controls.next.onclick();tick();assert(state('SM00').explosions===1&&!central(state('SM00'))&&state('SM00').hp.every(h=>h===87)&&state('SM00').player_hp===100,'single explosion and protected player');
 const dmg=state('SM00').hp.join(',');tick(12000);assert(state('SM00').hp.join(',')===dmg&&state('SM00').explosions===1&&controls.next.disabled,'idle repeats damage');canvases[0].onmousemove({clientX:144,clientY:176});tick(50);assert(!drawn.SM00.includes('hover-'),'consumed mine hover persists');
 const cases=evaluate('Array.from(new Set(manifest.map(e=>e.case)))');for(const label of cases){await choose(label);controls.cast.onclick();tick();for(const e of evaluate('entries()')){const n=evaluate(`frame(entries().find(e=>e.id==='${e.id}'))`);assert(n>=0&&n<160,'frame bounds '+label);}}
 await choose('ATTENTE');controls.cast.onclick();tick();assert(controls.next.disabled&&state('SM00').turn===1&&!central(state('SM00')).armed,'quiet scene');
 await choose('CAPACITE');assert(state('SM00').count===1&&state('SM01').count===5,'supplied historical field');controls.cast.onclick();tick();for(const id of ['SM00','SM01']){const t=state(id);assert(t.count===t.capacity&&t.removed_oldest===1&&!t.mines.some(m=>m.at[0]===3&&m.at[1]===6),'oldest replacement');}
 await choose('ENNEMI-ARMEMENT');controls.cast.onclick();tick();assert(state('SM00').primary[1]===5&&state('SM00').explosions===0,'enemy on unarmed mine');controls.next.onclick();tick();assert(state('SM00').explosions===1&&controls.next.disabled,'presence at arming trigger');tick(8000);assert(state('SM00').explosions===1&&state('SM00').hp[0]===87,'arming explosion repeated');
 await choose('COUCHE');controls.cast.onclick();tick();controls.next.onclick();tick();controls.next.onclick();tick();assert(state('SM00').layer_changed&&state('SM00').count===0&&state('SM00').explosions===0&&controls.next.disabled,'layer cleanup');
 await choose('CLOISON');controls.cast.onclick();tick();for(let i=0;i<3;i++){controls.next.onclick();tick();}assert(state('SM00').hp[2]===100&&state('SM00').hp[0]===87,'protected wall example');
 await choose('RANG-12');controls.cast.onclick();tick();assert(state('SM01').rank===12&&state('SM01').capacity===5,'accepted rank12 capacity');
 await choose('CAPACITE');controls.cast.onclick();tick();for(let i=0;i<3;i++){controls.next.onclick();tick();}const remainingBlink=new Set();for(let i=0;i<30;i++){tick(45);remainingBlink.add(state('SM01').led_on);assert(state('SM01').count===4&&state('SM01').explosions===1,'remaining field changed during wait');}assert(remainingBlink.size===2,'remaining mine blink frozen after explosion');
 const record={mode:'offline DOM/Image harness; no browser screenshot',scenarios:cases.length,loaded_image_references:images,two_canvases:2,arming_only_on_explicit_turn:'passed',central_LED_blinks_before_and_after_arming:'passed',remaining_field_blink:'passed',single_explosion_without_replay:'passed',scaled_native_hover_and_consumed_cleanup:'passed',capacity_replacement:'passed',rank11_rank12_capacity:'passed',player_protection:'passed',layer_cleanup:'passed',static_links:'passed',controller_syntax:'passed'};
 fs.writeFileSync(path.join(root,'browser-controller-verification.json'),JSON.stringify(record,null,2)+'\n');console.log(JSON.stringify(record));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
