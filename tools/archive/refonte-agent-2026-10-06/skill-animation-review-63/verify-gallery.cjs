const fs=require('fs'),path=require('path'),vm=require('vm'),root=__dirname;
async function main(){
 const html=fs.readFileSync(path.join(root,'cote-a-cote.html'),'utf8'),code=html.match(/<script>([\s\S]*?)<\/script>/)[1];fs.writeFileSync(path.join(root,'gallery-script.js'),code);
 for(const m of html.matchAll(/(?:href|src)="([^"]+)"/g))if(!m[1].startsWith('http')&&!fs.existsSync(path.join(root,m[1])))throw Error('Missing link '+m[1]);
 const controls=Object.fromEntries(['scenario','next','cast','status','reset','pause','native-view','fullscreen'].map(id=>[id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''}])),drawn={};
 const canvases=['SM00','SM01'].map(id=>({dataset:{skill:id},width:576,height:576,parentElement:{hidden:false},getContext:()=>({drawImage(im){drawn[id]=im.source;}}),getBoundingClientRect:()=>({left:0,top:0,width:288,height:288})}));
 let requested,images=0,now=100;class LocalImage{set src(v){if(!fs.existsSync(path.join(root,v)))throw Error('Missing frame '+v);this.source=v;images++;queueMicrotask(()=>this.onload());}}
 const context=vm.createContext({Image:LocalImage,Promise,Map,Set,Math,Array,Error,document:{getElementById:id=>controls[id],querySelectorAll:()=>canvases,body:{classList:{toggle(){}}},documentElement:{requestFullscreen:async()=>{}},exitFullscreen:async()=>{}},requestAnimationFrame:f=>requested=f});
 const evaluate=s=>vm.runInContext(s,context),settle=()=>new Promise(r=>setImmediate(r)),assert=(v,m)=>{if(!v)throw Error(m);},tick=(dt=1000)=>{now+=dt;requested(now);},state=id=>evaluate(`(()=>{const e=entries().find(e=>e.id==='${id}');return e.timeline[frame(e)]})()`);
 async function choose(label){controls.scenario.value=label;await controls.scenario.onchange();tick();}
 function equal(){assert(JSON.stringify(state('SM00'))===JSON.stringify(state('SM01')),'mechanics changed between old/new');}
 evaluate(code);await settle();requested(now);assert(evaluate('ready'),'load');controls.cast.onclick();tick();equal();assert(state('SM01').energy===90&&!state('SM01').mines[0].armed,'placement');
 canvases[1].onmousemove({clientX:144,clientY:176});tick(50);assert(drawn.SM01.includes('hover-'),'scaled hover');canvases[1].onmouseleave();
 for(let i=0;i<3;i++){controls.next.onclick();tick();equal();}assert(state('SM01').hp.join(',')==='87,87,87'&&state('SM01').explosions===1,'single impact');tick(12000);equal();assert(state('SM01').explosions===1&&state('SM01').player_hp===100&&controls.next.disabled,'idle repeats impact');
 const cases=evaluate('Array.from(new Set(manifest.map(e=>e.case)))');for(const label of cases){await choose(label);controls.cast.onclick();tick();equal();}
 await choose('CLOISON');controls.cast.onclick();tick();for(let i=0;i<3;i++){controls.next.onclick();tick();equal();}assert(state('SM01').hp[2]===100,'wall protection');
 await choose('ATTENTE');controls.cast.onclick();tick();tick(20000);equal();assert(state('SM01').turn===1&&state('SM01').explosions===0&&controls.next.disabled,'idle arming');
 await choose('ENNEMI-ARMEMENT');controls.cast.onclick();tick();controls.next.onclick();tick();equal();assert(state('SM01').explosions===1&&state('SM01').count===0,'arming trigger');
 const record={mode:'offline DOM/Image harness; no browser screenshot',scenarios:cases.length,loaded_image_references:images,two_canvases:2,original_and_revised_mechanics_identical:'passed',single_impact_without_idle_replay:'passed',scaled_hover:'passed',wall_and_player_protection:'passed',arming_and_presence:'passed',static_links:'passed',controller_syntax:'passed'};
 fs.writeFileSync(path.join(root,'browser-controller-verification.json'),JSON.stringify(record,null,2)+'\n');console.log(JSON.stringify(record));
}main().catch(e=>{console.error(e);process.exitCode=1;});
