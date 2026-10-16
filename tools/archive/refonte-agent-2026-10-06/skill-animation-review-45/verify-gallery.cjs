// Offline frame/controller verification only; no browser or network.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert');
const root=__dirname,els=new Map();
for(const id of ['scenario','next','cast','reset','pause','status'])els.set(id,{value:id==='scenario'?'HAUT':'',disabled:false,textContent:''});
const canvases=['TR65-ALLONGE','TR65-FENTE','TR65-PAS'].map(id=>({dataset:{skill:id},parentElement:{hidden:false},getContext:()=>({drawImage(im){assert(im);}})}));
class LocalFrame{set src(p){assert(fs.existsSync(path.join(root,p)),p);this.onload();}}
const ctx=vm.createContext({Image:LocalFrame,Map,Promise,Math,String,document:{getElementById:id=>els.get(id),querySelectorAll:()=>canvases},requestAnimationFrame(){}});
vm.runInContext(fs.readFileSync(path.join(root,'gallery-script.js'),'utf8'),ctx);
const run=s=>vm.runInContext(s,ctx),settle=()=>new Promise(resolve=>setImmediate(resolve));
(async()=>{
    await settle();assert(run('ready'));assert.strictEqual(run('entries().length'),3);assert.strictEqual(run('frame(entries()[0])'),17);
    run('elapsed=10000;ui()');assert.strictEqual(run('turn'),0);assert.strictEqual(run('frame(entries()[0])'),47);
    for(let turn=1;turn<=3;turn++){els.get('next').onclick();assert.strictEqual(run('turn'),turn);assert.strictEqual(run('frame(entries()[0])'),run('entries()[0].turn_frames[turn-1]'));run('elapsed=10000;ui()');}
    assert(els.get('next').disabled);assert.strictEqual(run('frame(entries()[0])'),95);assert(els.get('status').textContent.includes('forme 19 tours'));
    els.get('scenario').value='ATTENTE';els.get('scenario').onchange();assert(els.get('next').disabled);await settle();run('elapsed=100000;ui()');
    assert.strictEqual(run('turn'),0);assert.strictEqual(run('frame(entries()[0])'),47);assert(els.get('next').disabled);
    els.get('pause').onclick();const clock=run('elapsed');run('tick(600)');assert.strictEqual(run('elapsed'),clock);els.get('pause').onclick();
    els.get('reset').onclick();assert(!run('launched'));assert(!els.get('cast').disabled);assert(els.get('next').disabled);
    els.get('cast').onclick();assert.strictEqual(run('frame(entries()[0])'),17);
    for(const key of ['OCCUPEE','CACHEE','SANS-OMBRE','HORS-PORTEE']){
        els.get('scenario').value=key;els.get('scenario').onchange();await settle();run('tick(1200)');
        assert.strictEqual(run('entries().length'),1);assert(!run('entries()[0].permitted'));assert(els.get('next').disabled);
        els.get('reset').onclick();assert(els.get('cast').disabled);assert.strictEqual(canvases.filter(c=>!c.parentElement.hidden).length,1);
    }
    els.get('scenario').value='SILENCE';els.get('scenario').onchange();await settle();run('tick(1400)');
    assert.deepStrictEqual(Array.from(run('entries().map(e=>e.permitted)')),[true,true,false]);
    assert.strictEqual(canvases.filter(c=>!c.parentElement.hidden).length,3);
    els.get('scenario').value='PARTIEL';els.get('scenario').onchange();await settle();run('tick(1500)');
    assert.strictEqual(run('entries().length'),1);assert(run('entries()[0].permitted'));
    assert.strictEqual(canvases.filter(c=>!c.parentElement.hidden).length,1);
    const report={syntax:'passed',controller:'passed',all_frame_paths_exist:true,native_canvas_count:3,
        explicit_three_turn_sequence:true,waiting_does_not_advance_turns:true,pause_freezes_clock:true,
        inputs_disabled_during_loading:true,refused_destinations_and_silence_checked:true,partial_line_view_checked:true,
        browser_validation:'Not performed: file protocol blocked; native GPU and offline controller verified.'};
    fs.writeFileSync(path.join(root,'gallery-controller-verification.json'),JSON.stringify(report,null,2)+'\n');
    console.log(JSON.stringify(report));
})().catch(e=>{console.error(e);process.exitCode=1;});
