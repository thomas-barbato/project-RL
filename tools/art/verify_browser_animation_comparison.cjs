// Offline browser-comparison controls, paths, scaled hover and layout checks.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=path.resolve(process.argv[2]),html=fs.readFileSync(path.join(root,'cote-a-cote.html'),'utf8');
assert(!html.includes('href="Ouvrir_laboratoire.ps1"'));assert(html.includes('grid-template-columns:repeat(2,minmax(0,1fr))'));
for(const m of html.matchAll(/href="([^"]+)"/g))assert(fs.existsSync(path.join(root,m[1])),m[1]);
const els=new Map();for(const id of ['scenario','next','cast','reset','pause','status','native-view','fullscreen'])els.set(id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''});
const ids=[...html.matchAll(/data-skill="([^"]+)"/g)].map(m=>m[1]);assert.strictEqual(ids.length,2);
const canvases=ids.map(id=>{const tip={hidden:true,textContent:''};return {dataset:{skill:id},width:576,height:576,_tip:tip,parentElement:{hidden:false,querySelector:()=>tip},getBoundingClientRect:()=>({left:0,top:0,width:288,height:288}),getContext:()=>({drawImage(im){assert(im);}})};});
let native=false,full=0;
const document={getElementById:id=>els.get(id),querySelectorAll:()=>canvases,body:{classList:{toggle(name,on){assert.strictEqual(name,'native-view');native=on;}}},documentElement:{async requestFullscreen(){full++;}},async exitFullscreen(){full--;}};
class Frame{set src(p){assert(fs.existsSync(path.join(root,p)),p);this.onload();}}
const ctx=vm.createContext({document,Image:Frame,Map,Promise,Math,String,requestAnimationFrame(){}}),run=s=>vm.runInContext(s,ctx);
vm.runInContext(html.match(/<script>([\s\S]*?)<\/script>/)[1],ctx);
(async()=>{await new Promise(r=>setImmediate(r));assert(run('ready'));assert.strictEqual(run('entries().length'),2);run('tick(100)');
run('elapsed=400;ui()');
for(const c of canvases){c.onmousemove({clientX:251,clientY:42});assert(!c._tip.hidden);assert(c._tip.textContent.includes(run('entries().find(e=>e.id==='+JSON.stringify(c.dataset.skill)+').title')));c.onmouseleave();assert(c._tip.hidden);}
els.get('native-view').onchange({target:{checked:true}});assert(native);els.get('native-view').onchange({target:{checked:false}});assert(!native);
await els.get('fullscreen').onclick();assert.strictEqual(full,1);document.fullscreenElement={};await els.get('fullscreen').onclick();assert.strictEqual(full,0);
els.get('next').onclick();assert.strictEqual(run('turn'),1);els.get('reset').onclick();assert(!run('launched'));els.get('cast').onclick();assert(run('launched'));
console.log(JSON.stringify({browser_comparison:'offline checks passed',two_canvases:true,local_links_and_frames:true,scaled_hover:true,native_size_toggle:true,fullscreen_control:true,actual_browser_runtime:'not automated'}));
})().catch(e=>{console.error(e);process.exitCode=1;});
