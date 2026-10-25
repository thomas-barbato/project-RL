// Replay the accepted viewer as an independent oracle for timing/idle selection.
// This is a simulated DOM/canvas trace, not a browser or game capture.
const fs=require('fs');
const vm=require('vm');
const path=require('path');
const assert=require('assert/strict');
const rootPath=path.resolve(__dirname,'../..');

function boot(relative,placeholder,data){
  const html=fs.readFileSync(path.join(rootPath,relative),'utf8');
  const text=html.match(/<script>([\s\S]*?)<\/script>/)[1].replace(placeholder,JSON.stringify(data));
  const elements=new Map(),tiles=[],listeners={};
  let tick;
  function canvas(label){
    const draws=[];
    const ctx=new Proxy({drawImage(...args){
      assert([3,5,9].includes(args.length),'Invalid drawImage arguments');
      assert(args.slice(1).every(Number.isFinite),'Non-finite drawing coordinate');
      draws.push(args);
    },ellipse(...args){assert.equal(args.length,7,'Invalid ellipse arguments');}},
    {get(target,key){return target[key]??(()=>{});}});
    return {label,width:288,height:256,draws,getContext(){return ctx;}};
  }
  const count=[...html.matchAll(/<canvas data-scene=/g)].length;
  const scenes=Array.from({length:count},(_,i)=>canvas('scene'+i));
  const zooms=Array.from({length:count},(_,i)=>canvas('zoom'+i));
  const domRoot={dataset:{},querySelector(selector){
    const attribute=selector.slice(1,-1);
    assert(html.includes(attribute),'Missing HTML element '+selector);
    if(!elements.has(selector))elements.set(selector,{value:'',events:{},addEventListener(type,cb){this.events[type]=cb;},setAttribute(){}});
    return elements.get(selector);
  },querySelectorAll(selector){return selector==='[data-scene]'?scenes:zooms;}};
  class MockImage{set src(value){this.label=value;if(this.onload)this.onload();}}
  const sandbox={
    document:{getElementById(id){assert(html.includes(`id="${id}"`));return domRoot;},createElement(tag){assert.equal(tag,'canvas');const value=canvas('tile'+tiles.length);tiles.push(value);return value;},addEventListener(){}},
    window:{openai:{widgetState:{modelContent:{path:'lateral',pace:400,floor:'dark'}},setWidgetState(){return Promise.resolve();}},addEventListener(type,cb){listeners[type]=cb;}},
    Image:MockImage,matchMedia(){return {matches:false};},requestAnimationFrame(cb){tick=cb;},console
  };
  vm.createContext(sandbox);vm.runInContext(text,sandbox);
  return {root:domRoot,tiles,scenes,zooms,elements,tick:now=>tick(now),
    click:action=>elements.get(`[data-action="${action}"]`).events.click(),
    change(key,value){const element=elements.get(`[data-control="${key}"]`);element.value=String(value);element.events.change({target:element});}};
}

const original=()=>boot('tools/art/player_walk_review_01.html','__WALK_DATA__',
  {atlas:'bastion-walk',front:'bastion-front',idle:'bastion-idle'});
const candidate=()=>boot('tools/art/player_walk_steps_06.html','__WALK_STEP_DATA__',
  {actors:[{atlas:'bastion-walk',idle:'bastion-idle'},{atlas:'traqueuse-walk',idle:'traqueuse-idle'}]});
function snapshot(view,tile,scene){
  const d=view.tiles[tile].draws.at(-1);
  return {frame:view.root.dataset.frame,moving:view.root.dataset.moving,cell:view.root.dataset.cell,
    sprite:[d[0].label,...d.slice(1)],placement:view.scenes[scene].draws.at(-1).slice(1)};
}
const before=original(),after=candidate(),phases=new Set(),rows=new Set();
for(let now=1;now<=12001;now+=20){
  before.tick(now);after.tick(now);
  assert.deepEqual(snapshot(after,0,0),snapshot(before,1,1),'Mismatch with accepted option02 at '+now+'ms');
  const moving=after.root.dataset.moving==='true';
  const draw=after.tiles[1].draws.at(-1);
  assert.equal(draw[0].label,moving?'traqueuse-walk':'traqueuse-idle','Arrival freezes a walking pose');
  if(moving){phases.add(+after.root.dataset.frame);rows.add(draw[2]);}
}
assert.deepEqual([...phases].sort(),[0,1,2,3,4,5]);
assert.deepEqual([...rows].sort((a,b)=>a-b),[64,448]);
const single=candidate();single.tick(1);single.click('cell');
for(const now of [81,161,241,321,401])single.tick(now);
assert.equal(single.root.dataset.cell,'1');
assert.equal(single.root.dataset.moving,'false');
assert.equal(single.tiles[1].draws.at(-1)[0].label,'traqueuse-idle');
assert.equal(single.elements.get('[data-action="play"]').textContent,'Lecture auto');
single.tick(1201);assert.equal(single.root.dataset.cell,'1','Single-cell action continued automatically');
single.click('cell');single.tick(1281);
assert.equal(single.root.dataset.frame,'3','Second cell must use the opposite half-cycle');
single.change('pace',200);single.change('floor','light');
single.tick(1361);single.tick(1441);
assert.equal(single.root.dataset.moving,'false');
single.click('cell');single.tick(1521);
assert.equal(single.tiles[1].draws.at(-1)[2],448,'Leftward movement must use SW row');
const manual=candidate();manual.click('step');
assert.equal(manual.root.dataset.frame,'0');
manual.click('step');assert.equal(manual.root.dataset.frame,'1');
manual.click('step');assert.equal(manual.root.dataset.frame,'2');
manual.click('step');assert.equal(manual.root.dataset.moving,'false');
assert.equal(manual.tiles[1].draws.at(-1)[0].label,'traqueuse-idle');
const report={review_id:'WALK05-STEP03',renderer_trace_matches_original_option02:true,
  trace_duration_ms:12000,trace_samples:601,both_directions:true,all_six_phases:true,
  idle_used_at_rest:true,single_cell_stops:true,second_cell_uses_opposite_half:true,
  manual_frames_and_duration_choice:true,canvas_api_argument_checks:true,
  execution:'Node VM with simulated DOM and canvas; no real browser or game execution',
  visual_acceptance:false};
fs.writeFileSync(path.join(rootPath,'docs/reviews/player-walk-05/step03-renderer-checks.json'),JSON.stringify(report,null,2)+'\n');
console.log(JSON.stringify(report,null,2));
