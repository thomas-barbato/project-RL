// Compare the expanded review clock against the accepted viewer.
// This checks JavaScript with a simulated DOM/canvas, not browser or game output.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert/strict');
const rootPath=path.resolve(__dirname,'../..');
function boot(relative,placeholder,data,dynamic=false){
  const html=fs.readFileSync(path.join(rootPath,relative),'utf8');
  const code=html.match(/<script>([\s\S]*?)<\/script>/)[1].replace(placeholder,JSON.stringify(data));
  const elements=new Map(),canvases=[],events={};let next;
  function element(tag){
    const draws=[],handlers={},dataset={};
    const context=new Proxy({drawImage(...args){assert([3,5,9].includes(args.length));assert(args.slice(1).every(Number.isFinite));draws.push(args);},ellipse(...args){assert.equal(args.length,7);}}, {get:(o,k)=>o[k]??(()=>{})});
    const result={tag,draws,dataset,handlers,append(){},setAttribute(){},addEventListener:(key,fn)=>handlers[key]=fn,getContext:()=>context};
    if(tag==='canvas')canvases.push(result);return result;
  }
  const staticScenes=dynamic?[]:Array.from({length:2},()=>element('canvas'));
  const staticZooms=dynamic?[]:Array.from({length:2},()=>element('canvas'));
  const root={dataset:{},querySelector(selector){
    assert(html.includes(selector.startsWith('.')?selector.slice(1):selector.slice(1,-1)),selector);
    if(!elements.has(selector))elements.set(selector,element('div'));return elements.get(selector);
  },querySelectorAll:selector=>selector==='[data-scene]'?staticScenes:staticZooms};
  class MockImage{set src(value){this.label=value;this.onload?.();}}
  const sandbox={document:{getElementById:()=>root,createElement:element,addEventListener(){}},
    window:{openai:{widgetState:{},setWidgetState:()=>Promise.resolve()},addEventListener:(key,fn)=>events[key]=fn},
    Image:MockImage,matchMedia:()=>({matches:false}),requestAnimationFrame:fn=>next=fn};
  vm.createContext(sandbox);vm.runInContext(code,sandbox);
  return {root,elements,canvases,tick:now=>next(now),click:action=>elements.get(`[data-action="${action}"]`).handlers.click(),
    change(key,value){const e=elements.get(`[data-control="${key}"]`);e.value=String(value);e.handlers.change({target:e});}};
}
const entries=Array.from({length:5},(_,i)=>({number:i+1,stage:i,label:'Tenue '+i,status:'proposal'}));
const candidate=()=>boot('tools/art/player_walk_matrix_06.html','__WALK_MATRIX_DATA__',{atlas:'class-atlas',className:'Bastion',entries},true);
const oracle=()=>boot('tools/art/player_walk_steps_06.html','__WALK_STEP_DATA__',{actors:[{atlas:'walk',idle:'idle'},{atlas:'walk2',idle:'idle2'}]});
const after=candidate(),before=oracle(),phases=new Set(),directions=new Set();
function check(view){
  const moving=view.root.dataset.moving==='true',frame=+view.root.dataset.frame,right=view.root.dataset.direction==='1';
  const tiles=view.canvases.filter(c=>c.dataset.scene===undefined&&c.dataset.zoom===undefined);
  assert.equal(tiles.length,5);
  for(let i=0;i<5;i++){
    const d=tiles[i].draws.at(-1);assert.equal(d[0].label,'class-atlas');
    assert.equal(d[1],moving?(frame+1)*64:0,'Idle must select its separate column');
    assert.equal(d[2],i*128+(right?64:0));
    assert.deepEqual(d.slice(3),[64,64,0,0,64,64]);
    assert(d[1]+64<=448&&d[2]+64<=640,'Atlas overflow');
  }
  if(moving){phases.add(frame);directions.add(view.root.dataset.direction);}
}
for(let now=1;now<=12001;now+=20){
  before.tick(now);after.tick(now);
  for(const key of ['frame','direction','moving','cell'])assert.equal(after.root.dataset[key],before.root.dataset[key],key+' at '+now);
  check(after);
}
assert.deepEqual([...phases].sort(),[0,1,2,3,4,5]);assert.deepEqual([...directions].sort(),['1','7']);
const single=candidate();single.tick(1);single.click('cell');
for(const t of [81,161,241,321,401])single.tick(t);
assert.equal(single.root.dataset.cell,'1');assert.equal(single.root.dataset.moving,'false');check(single);
single.tick(1201);assert.equal(single.root.dataset.cell,'1');
single.click('cell');single.tick(1281);assert.equal(single.root.dataset.frame,'3');
single.change('pace',120);single.change('floor','light');single.tick(1361);
assert.equal(single.root.dataset.moving,'false');check(single);
const manual=candidate();for(let i=0;i<3;i++){manual.click('step');assert.equal(manual.root.dataset.frame,String(i));}
manual.click('step');assert.equal(manual.root.dataset.moving,'false');check(manual);
const packaging=JSON.parse(fs.readFileSync(path.join(rootPath,'docs/reviews/player-walk-06/packaging.json')));
assert.equal(packaging.ready_count,25);assert.equal(packaging.errors.length,0);assert.equal(packaging.pending.length,0);assert.equal(packaging.fragments.length,5);
for(const f of packaging.fragments){
  const text=fs.readFileSync(f.path,'utf8');assert(Buffer.byteLength(text)<1_000_000);assert(!text.includes('__WALK_'));assert(!text.includes('__CLASS_'));
  assert(!/<(?:html|head|body|!doctype)\b/i.test(text));assert(!/\b(?:fetch|XMLHttpRequest|WebSocket)\s*\(/.test(text));
  new vm.Script(text.match(/<script>([\s\S]*?)<\/script>/)[1]);
}
const report={review_id:'WALK06-CLASS-ARMOR-MATRIX',class_silhouettes:25,rifle_only:true,
  trace_matches_accepted_viewer:true,trace_samples:601,all_six_frames:true,both_lateral_directions:true,
  separate_idle_at_arrival:true,single_cell_stops:true,compact_atlas_bounds:true,
  duration_choice_and_manual_step:true,fragments_under_one_megabyte:5,
  execution:'Node VM with simulated DOM/canvas; no browser or game execution',visual_acceptance:false,runtime_integration:false};
fs.writeFileSync(path.join(rootPath,'docs/reviews/player-walk-06/renderer-checks.json'),JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify(report,null,2));
