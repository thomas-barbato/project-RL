// Offline DOM/Image harness; native GPU renders inspected separately.
const fs=require('fs'),path=require('path'),vm=require('vm'),root=__dirname;
async function main(){
 const html=fs.readFileSync(path.join(root,'cote-a-cote.html'),'utf8'),code=html.match(/<script>([\s\S]*?)<\/script>/)[1];fs.writeFileSync(path.join(root,'gallery-script.js'),code);
 for(const m of html.matchAll(/(?:href|src)="([^"]+)"/g))if(!m[1].startsWith('http')&&!fs.existsSync(path.join(root,m[1])))throw Error('Missing link '+m[1]);
 const controls=Object.fromEntries(['scenario','next','cast','status','reset','pause','native-view','fullscreen'].map(id=>[id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''}])),drawn={};
 const canvases=['SM02','SM03'].map(id=>({dataset:{skill:id},width:576,height:576,parentElement:{hidden:false},getContext:()=>({drawImage(im){drawn[id]=im.source;}}),getBoundingClientRect:()=>({left:0,top:0,width:288,height:288})}));
 let requested,images=0,now=100;
 class LocalImage{set src(v){if(!fs.existsSync(path.join(root,v)))throw Error('Missing frame '+v);this.source=v;images++;queueMicrotask(()=>this.onload());}}
 const context=vm.createContext({Image:LocalImage,Promise,Map,Set,Math,Array,Error,document:{getElementById:id=>controls[id],querySelectorAll:()=>canvases,body:{classList:{toggle(){}}},documentElement:{requestFullscreen:async()=>{}},exitFullscreen:async()=>{}},requestAnimationFrame:f=>requested=f});
 const evaluate=s=>vm.runInContext(s,context),settle=()=>new Promise(r=>setImmediate(r)),assert=(v,m)=>{if(!v)throw Error(m);},tick=(dt=1000)=>{now+=dt;requested(now);};
 const state=id=>evaluate(`(()=>{const e=entries().find(e=>e.id==='${id}');return e.timeline[frame(e)]})()`),central=t=>t.mines.find(m=>m.at[0]===4&&m.at[1]===5);
 async function choose(label){controls.scenario.value=label;await controls.scenario.onchange();tick();}
 async function advance(){controls.next.onclick();await settle();tick();}
 async function explode(){controls.cast.onclick();tick();await advance();await advance();await advance();}
 evaluate(code);await settle();requested(now);assert(evaluate('ready'),'initial load');controls.cast.onclick();tick();assert(!central(state('SM02')).armed&&!state('SM02').preview_visible,'unarmed placement');
 const phases=new Set();for(let i=0;i<30;i++){tick(45);phases.add(state('SM02').led_on);assert(state('SM02').turn===1&&!central(state('SM02')).armed,'idle arming');}assert(phases.size===2,'central blink');
 canvases[0].onmousemove({clientX:144,clientY:176});tick(50);assert(drawn.SM02.includes('hover-'),'scaled hover');canvases[0].onmouseleave();
 await advance();assert(central(state('SM02')).armed&&state('SM02').explosions===0,'arming');await advance();assert(state('SM02').primary[1]===4&&state('SM02').explosions===0,'approach');await advance();
 assert(state('SM02').hp.every(h=>h===339)&&state('SM02').poison_ticks[0]===1&&state('SM02').last_damage[0]===61,'immediate poison counts once');assert(state('SM03').hp[0]===384&&state('SM03').status[0]===5,'cold and Gel');
 tick(150);assert(drawn.SM02.includes('idle-3-')&&drawn.SM03.includes('idle-3-'),'animated idle sequence');const stable=JSON.stringify(state('SM02').hp);tick(20000);assert(JSON.stringify(state('SM02').hp)===stable&&state('SM02').status[0]===5,'idle repeats periodic damage');
 for(const [step,timer,ticks,hp]of [[4,4,2,297],[5,3,3,255],[6,2,4,213],[7,1,5,171],[8,0,5,171]]){await advance();assert(state('SM02').step===step&&state('SM02').status[0]===timer&&state('SM02').poison_ticks[0]===ticks&&state('SM02').hp[0]===hp,'five-application sequence');assert(state('SM03').status[0]===timer,'Gel timer');}
 assert(controls.next.disabled&&state('SM02').player_hp===100&&state('SM03').player_hp===100,'expiry and protected player');tick(15000);assert(state('SM02').status[0]===0&&!drawn.SM02.includes('idle-'),'expired status not removed');
 const cases=evaluate('Array.from(new Set(manifest.map(e=>e.case)))');for(const label of cases){await choose(label);controls.cast.onclick();tick();for(const e of evaluate('entries()'))assert(evaluate(`frame(entries().find(e=>e.id==='${e.id}'))`)>=0&&evaluate(`frame(entries().find(e=>e.id==='${e.id}'))`)<256,'frame bounds');}
 await choose('ATTENTE');controls.cast.onclick();tick();assert(controls.next.disabled&&state('SM02').turn===1&&state('SM02').status[0]===0,'quiet scene');
 await choose('ELOIGNEMENT');await explode();await advance();assert(state('SM02').primary.join(',')==='3,4'&&state('SM02').status[0]===4&&state('SM02').hp[0]===297,'poison follows departing victim');assert(state('SM03').primary.join(',')==='4,5'&&state('SM03').blocked_walk&&state('SM03').status[0]===4,'Gel movement block');
 await choose('GEL-RESISTE');await explode();assert(state('SM03').status[0]===0&&state('SM03').status[1]===5&&state('SM03').hp[0]===384,'failed Gel preserves cold damage');
 await choose('CLOISON');await explode();assert(state('SM02').hp[2]===400&&state('SM02').status[2]===0&&state('SM03').hp[2]===400,'wall protection');
 await choose('ENNEMI-ARMEMENT');controls.cast.onclick();tick();assert(state('SM02').primary[1]===5&&state('SM02').explosions===0,'unarmed presence');await advance();assert(state('SM02').hp[0]===339&&state('SM02').poison_ticks[0]===1,'arming contact');
 await choose('RANG-12');await explode();assert(state('SM02').hp[0]===333&&state('SM02').last_damage[0]===67&&state('SM03').hp[0]===383,'accepted rank12 samples');for(let i=0;i<5;i++)await advance();assert(state('SM02').hp[0]===145&&state('SM02').poison_ticks[0]===5,'rank12 total');
 const record={mode:'offline DOM/Image harness; no browser screenshot',scenarios:cases.length,loaded_image_references:images,two_canvases:2,blink_before_arming:'passed',scaled_hover:'passed',immediate_poison_first_of_five:'passed',animated_idle_without_turn_damage_or_timer_change:'passed',bounded_idle_stage_loading:'passed',poison_follows_victim:'passed',Gel_blocks_movement:'passed',failed_Gel_preserves_damage:'passed',wall_and_player_protection:'passed',rank11_rank12_and_expiry:'passed',static_links:'passed',controller_syntax:'passed'};
 fs.writeFileSync(path.join(root,'browser-controller-verification.json'),JSON.stringify(record,null,2)+'\n');console.log(JSON.stringify(record));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
