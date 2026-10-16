// Offline controller checks and local frame inventory; no browser automation.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=__dirname,els=new Map();
for(const id of ['scenario','next','cast','reset','pause','status'])els.set(id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''});
const canvases=['BM01','BM02'].map(id=>({dataset:{skill:id},parentElement:{hidden:false,querySelector:()=>({hidden:true,textContent:''})},getBoundingClientRect:()=>({left:0,top:0}),getContext:()=>({drawImage(im){assert(im);}})}));
class LocalFrame{set src(p){assert(fs.existsSync(path.join(root,p)),p);this.onload();}}
const ctx=vm.createContext({Image:LocalFrame,Map,Promise,Math,String,document:{getElementById:id=>els.get(id),querySelectorAll:()=>canvases},requestAnimationFrame(){}});
const html=fs.readFileSync(path.join(root,'index.html'),'utf8'),script=html.match(/<script>([\s\S]*?)<\/script>/)[1];
for(const m of html.matchAll(/href="([^"]+)"/g))assert(fs.existsSync(path.join(root,m[1])),m[1]);
fs.writeFileSync(path.join(root,'gallery-script.js'),script);
vm.runInContext(script,ctx);const run=s=>vm.runInContext(s,ctx),settle=()=>new Promise(r=>setImmediate(r));
(async()=>{
 await settle();assert(run('ready'));assert.strictEqual(run('entries().length'),2);assert.strictEqual(run('frame(entries()[0])'),17);
 run('elapsed=100000;ui()');assert.strictEqual(run('turn'),0);assert(run('frame(entries()[0])')>=44&&run('frame(entries()[0])')<=51);
 for(let n=1;n<=6;n++){els.get('next').onclick();assert.strictEqual(run('turn'),n);assert.strictEqual(run('frame(entries()[0])'),[52,76,88,100,112,124][n-1]);assert(els.get('status').textContent.includes('buff '+Math.max(0,4-n)+' tours'));}
 assert(els.get('next').disabled);
 for(const e of run('manifest')){const timeline=JSON.parse(fs.readFileSync(path.join(root,'native',e.id+'-'+e.case,'timeline.json'),'utf8'));for(let turn=0;turn<=6;turn++){run('turn='+turn);for(const t of [0,250,800,100000]){run('elapsed='+t);const f=run('frame(manifest.find(e=>e.variant==='+JSON.stringify(e.variant)+'))');assert(f>=0&&f<128);assert(fs.existsSync(path.join(root,'frames',e.variant,String(f).padStart(3,'0')+'.png')));if(e.permitted&&!e.quiet){assert.strictEqual(timeline[f].turn,turn);assert.strictEqual(timeline[f].remaining,Math.max(0,e.duration-turn));}}}}
 els.get('scenario').value='ATTENTE';els.get('scenario').onchange();await settle();run('elapsed=100000;ui()');assert.strictEqual(run('turn'),0);assert(els.get('next').disabled);assert(els.get('status').textContent.includes('buff 4 tours'));
 els.get('pause').onclick();const clock=run('elapsed');run('tick(600)');assert.strictEqual(run('elapsed'),clock);els.get('pause').onclick();
 els.get('reset').onclick();assert(!run('launched'));assert(!els.get('cast').disabled);assert(els.get('next').disabled);run('elapsed=0');const f=run('frame(entries()[0])');run('elapsed=500');assert.notStrictEqual(run('frame(entries()[0])'),f);els.get('cast').onclick();assert.strictEqual(run('frame(entries()[0])'),17);
 for(const key of ['SILENCE','DISTANCE','TRANSFORME']){els.get('scenario').value=key;els.get('scenario').onchange();await settle();assert(run('entries().every(e=>!e.permitted)'));assert(els.get('next').disabled);els.get('reset').onclick();assert(els.get('cast').disabled);}
 for(const key of ['HAUT','DROITE','GAUCHE','BAS','CLOISON','CACHE','SURVOL']){els.get('scenario').value=key;els.get('scenario').onchange();await settle();run('tick(1400)');assert(run('entries().every(e=>e.permitted)'));}
 const report={syntax:'passed',controller:'passed',all_frame_paths_exist:true,native_canvas_count:2,manual_six_turn_expiry:true,waiting_keeps_status_without_replayed_strike:true,pause_freezes_clock:true,preview_blinks_until_launch:true,refusals_and_four_directions_checked:true,browser_validation:'Not performed; native GPU captures and offline controller verified.'};
 fs.writeFileSync(path.join(root,'gallery-controller-verification.json'),JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify(report));
})().catch(e=>{console.error(e);process.exitCode=1;});
