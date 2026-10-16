// Local controller QA only; no browser or network automation.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=path.resolve(process.argv[2]);
const els=new Map();for(const id of ['scenario','next','cast','reset','pause','status','native-view','fullscreen'])els.set(id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''});
const canvases=['BD04','BD06'].map(id=>{const tip={hidden:true,textContent:''};return{width:576,height:576,dataset:{skill:id},_tip:tip,parentElement:{hidden:false,querySelector:()=>tip},getBoundingClientRect:()=>({left:0,top:0,width:288,height:288}),getContext:()=>({drawImage(im){assert(im);}})};});
class Frame{set src(p){assert(fs.existsSync(path.join(root,p)),p);this.onload();}}
let native=false,full=0;
const document={getElementById:id=>els.get(id),querySelectorAll:()=>canvases,body:{classList:{toggle(name,on){assert.strictEqual(name,'native-view');native=on;}}},documentElement:{async requestFullscreen(){full++;}},async exitFullscreen(){full--;}};
const html=fs.readFileSync(path.join(root,'cote-a-cote.html'),'utf8');assert(html.includes('grid-template-columns:repeat(2,minmax(0,1fr))'));
assert(!html.includes('href="Ouvrir_laboratoire.ps1"'));
for(const m of html.matchAll(/href="([^"]+)"/g))assert(fs.existsSync(path.join(root,m[1])),m[1]);
const ctx=vm.createContext({Image:Frame,document,Map,Promise,Math,String,requestAnimationFrame(){}}),run=s=>vm.runInContext(s,ctx),settle=()=>new Promise(r=>setImmediate(r));
const script=html.match(/<script>([\s\S]*?)<\/script>/)[1];fs.writeFileSync(path.join(root,'gallery-script.js'),script);vm.runInContext(script,ctx);
(async()=>{
 await settle();assert(run('ready'));run('elapsed=100000;ui()');assert.strictEqual(run('turn'),0);assert(els.get('status').textContent.includes('1 cumuls'));assert(els.get('status').textContent.includes('62/100'));
 for(let n=1;n<=3;n++){els.get('next').onclick();run('elapsed=100000;ui()');assert.strictEqual(run('turn'),n);assert(els.get('status').textContent.includes([0,2,3,3][n]+' cumuls'));assert(els.get('status').textContent.includes(n===3?'84/100':'62/100'));}assert(els.get('next').disabled);
 for(const e of run('manifest'))for(let n=0;n<=3;n++){
  run('turn='+n);for(const elapsed of [0,400,600,900,100000]){run('elapsed='+elapsed);const f=run('frame(manifest.find(e=>e.variant==='+JSON.stringify(e.variant)+'))');assert(f>=0&&f<128);assert(fs.existsSync(path.join(root,'frames',e.variant,String(f).padStart(3,'0')+'.png')));if(!e.quiet)assert.strictEqual(e.timeline[f].step,n);}
 }
 els.get('scenario').value='ATTENTE';els.get('scenario').onchange();await settle();run('elapsed=100000;ui()');assert.strictEqual(run('turn'),0);assert(els.get('next').disabled);assert(els.get('status').textContent.includes('62/100'));
 els.get('pause').onclick();const time=run('elapsed');run('tick(1200)');assert.strictEqual(run('elapsed'),time);els.get('pause').onclick();els.get('reset').onclick();assert(!run('launched'));run('elapsed=0');const f=run('frame(entries()[0])');run('elapsed=500');assert.notStrictEqual(run('frame(entries()[0])'),f);els.get('cast').onclick();assert.strictEqual(run('frame(entries()[0])'),17);
 for(const key of ['DEPLACEMENT','FORCE','MAGIQUE','SANS-TOUR']){els.get('scenario').value=key;els.get('scenario').onchange();await settle();for(let n=0;n<3;n++){els.get('next').onclick();run('elapsed=100000;ui()');}assert(els.get('status').textContent.includes(key==='SANS-TOUR'?'3 cumuls':'0 cumuls'));}
 for(const key of ['PLEINE','LIMITE','FAMILIER','RENVOI','RIPOSTE','LEURRE']){els.get('scenario').value=key;els.get('scenario').onchange();await settle();run('elapsed=100000;tick(1600)');assert.strictEqual(run('entries().length'),1);assert.strictEqual(canvases.filter(c=>!c.parentElement.hidden).length,1);assert(els.get('status').textContent.includes(['PLEINE','LIMITE'].includes(key)?'100/100':'40/100'));}
 els.get('scenario').value='HAUT';els.get('scenario').onchange();await settle();run('elapsed=600;ui()');canvases[0].onmousemove({clientX:251,clientY:42});assert(!canvases[0]._tip.hidden);assert(canvases[0]._tip.textContent.includes('précision +15 %'));canvases[0].onmouseleave();assert(canvases[0]._tip.hidden);canvases[1].onmousemove({clientX:251,clientY:42});assert(canvases[1]._tip.hidden);
 els.get('native-view').onchange({target:{checked:true}});assert(native);els.get('native-view').onchange({target:{checked:false}});assert(!native);await els.get('fullscreen').onclick();assert.strictEqual(full,1);document.fullscreenElement={};await els.get('fullscreen').onclick();assert.strictEqual(full,0);
 const report={controller:'passed',all_local_frame_paths:true,manual_stages_do_not_show_future_actions:true,quiet_pause_preview:true,passive_charges_and_resource_limits:true,exclusions:true,responsive_comparison_scaled_hover_native_and_fullscreen:true,browser_runtime:'Not automated; offline controller and native captures checked.'};fs.writeFileSync(path.join(root,'gallery-controller-verification.json'),JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify(report));
})().catch(e=>{console.error(e);process.exitCode=1;});
