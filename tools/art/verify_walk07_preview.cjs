// This runs the actual preview controller/raster code in Node with a simulated
// DOM. It does not claim browser or game execution or visual acceptance.
const fs=require('fs'),path=require('path'),vm=require('vm'),assert=require('assert/strict');
const rootPath=path.resolve(__dirname,'../..');
const template=fs.readFileSync(path.join(__dirname,'walk07_preview.html'),'utf8');
const source=template.match(/<script>([\s\S]*?)<\/script>/)[1];
const report={review_id:'WALK07',execution:'Node VM, simulated DOM/canvas; renderer fixtures from Rust Image diagnostic',runtime_integration:false,visual_acceptance:false};

// Quantized affine recipes must reproduce the actual Rust-composed poses on
// both dark and light floors, including arms, both grips and rigid weapons.
const rasterCode=source.slice(source.indexOf('function affine07('),source.indexOf('function drawnTile('));
const pixelContext=vm.createContext({Uint8ClampedArray});vm.runInContext(rasterCode,pixelContext);
const fixtures=JSON.parse(fs.readFileSync(path.join(rootPath,'artifacts/player-walk-review-07/raster-fixtures.json')));
let maxDisplayedDifference=0,differentPixels=0;
for(const fixture of fixtures.fixtures){
  const actual=new Uint8ClampedArray(128*128*4);
  for(const group of fixture.commands){
    const layer=new Uint8ClampedArray(actual.length);
    for(const command of group){const src=fixtures.sources[fixture.slug+':'+command[0]];
      const data=Uint8ClampedArray.from(Buffer.from(src.data,'base64'));
      pixelContext.over07(layer,pixelContext.affine07({...src,data},command));}
    pixelContext.over07(actual,layer);
  }
  const expected=Buffer.from(fixture.expected,'base64');
  for(let i=0;i<actual.length;i+=4){let changed=false;
    for(const floor of [41,181])for(let c=0;c<3;c++){
      const a=Math.round((actual[i+c]*actual[i+3]+floor*(255-actual[i+3]))/255);
      const e=Math.round((expected[i+c]*expected[i+3]+floor*(255-expected[i+3]))/255);
      const diff=Math.abs(a-e);maxDisplayedDifference=Math.max(maxDisplayedDifference,diff);changed ||= diff>0;
      assert(diff<=3,`${fixture.key}: visible channel drift ${diff} at ${i/4}`);
    }
    if(changed)differentPixels++;
  }
}
report.pixel_fixtures=fixtures.fixtures.length;report.max_visible_channel_difference=maxDisplayedDifference;report.differing_edge_pixels=differentPixels;

async function boot(data){
  let next;const elements=new Map(),events={};let saves=0;
  function element(tag){
    const result={tag,draws:[],ellipses:[],dataset:{},handlers:{},children:[],width:0,height:0,
      append(o){this.children.push(o);},setAttribute(k,v){this[k]=v;},addEventListener(k,fn){this.handlers[k]=fn;}};
    const context=new Proxy({drawImage(...args){assert([3,5,9].includes(args.length));assert(args.slice(1).every(Number.isFinite));result.draws.push(args);},
      ellipse(...args){result.ellipses.push(args);},
      getImageData(x,y,w,h){return {data:new Uint8ClampedArray(w*h*4)};},putImageData(image){assert.equal(image.data.length,65536);}},
      {get:(o,k)=>o[k]??(()=>{})});result.getContext=()=>context;return result;
  }
  const root={dataset:{},querySelector(selector){
    assert(template.includes(selector.slice(1,-1)),selector);if(!elements.has(selector))elements.set(selector,element('div'));return elements.get(selector);
  },querySelectorAll:()=>[...elements.values()]};
  class MockImage{set src(value){assert(value.startsWith('data:image/'));this.onload();}}
  class ImageData{constructor(data,w,h){this.data=data;this.width=w;this.height=h;}}
  const sandbox={Uint8ClampedArray,Blob,Response,DecompressionStream,atob,ImageData,
    document:{getElementById:()=>root,createElement:element,addEventListener(){}},
    window:{openai:{widgetState:{},setWidgetState:()=>{saves++;return Promise.resolve();}},addEventListener:(k,fn)=>events[k]=fn},
    Image:MockImage,matchMedia:()=>({matches:false}),requestAnimationFrame:fn=>next=fn};
  vm.createContext(sandbox);await vm.runInContext(source.replace('__WALK07_DATA__',JSON.stringify(data)),sandbox);
  assert.equal(root.dataset.ready,'true');assert(next);
  return {root,elements,events,get saves(){return saves;},tick:now=>next(now),
    click(action){elements.get(`[data-action="${action}"]`).handlers.click();},
    change(key,value){const e=elements.get(`[data-control="${key}"]`);e.value=String(value);e.handlers.change({target:e});}};
}

(async()=>{
  const dataFolder=path.join(rootPath,'artifacts/player-walk-review-07/preview-data');
  const creatureSlugs=fs.readdirSync(dataFolder).filter(n=>/^creatures(?:-lot\d+|-review\d+)?\.json$/.test(n)).map(n=>n.slice(0,-5)).sort();
  // The base slug precedes padded lot numbers in this explicit order.
  creatureSlugs.sort((a,b)=>a==='creatures'?-1:b==='creatures'?1:a.localeCompare(b));
  const visualFolder='C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c';
  const currentCreatureSlug=fs.readFileSync(path.join(visualFolder,'walk07-creatures.html'),'utf8').match(/id="walk07-(creatures(?:-lot\d+)?)"/)[1];
  const slugs=['bastion','technomancien','traqueuse','arcaniste','metamorphe',...creatureSlugs];
  const oldVerifier=fs.readFileSync(path.join(__dirname,'verify_player_walk_matrix_06.cjs'),'utf8');
  const oracleContext=vm.createContext({fs,path,vm,assert});
  vm.runInContext('const rootPath='+JSON.stringify(rootPath)+';'+oldVerifier.slice(oldVerifier.indexOf('function boot('),oldVerifier.indexOf('const entries=')),oracleContext);
  const oracle=oracleContext.boot('tools/art/player_walk_steps_06.html','__WALK_STEP_DATA__',{actors:[{atlas:'walk',idle:'idle'},{atlas:'walk2',idle:'idle2'}]});
  let covered=0,checkedFragments=0,creatureSelections=0;
  for(const slug of slugs){
    const data=JSON.parse(fs.readFileSync(path.join(rootPath,`artifacts/player-walk-review-07/preview-data/${slug}.json`)));
    // Reuse the current creature fragment for the next lot. Lot01's exact
    // accepted figures stay frozen separately; its data still gets checks.
    if(!slug.startsWith('creatures')||slug===currentCreatureSlug||slug.includes('-review')){
      const fragmentSlug=slug.includes('-review')?slug:(slug.startsWith('creatures')?'creatures':slug);
      const html=fs.readFileSync(path.join('C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c',`walk07-${fragmentSlug}.html`),'utf8');
      assert(Buffer.byteLength(html)<1_000_000);assert(!/__WALK07_/.test(html));
      assert(html.includes(`id="walk07-${slug}"`));
      assert(!/<(?:html|head|body|!doctype)\b/i.test(html));assert(!/\b(?:fetch|XMLHttpRequest|WebSocket)\s*\(/.test(html));
      new vm.Script(html.match(/<script>([\s\S]*?)<\/script>/)[1]);checkedFragments++;
    }
    const view=await boot(data);assert.equal(view.saves,0);const phases=new Set(),directions=new Set();
    for(let now=1;now<=12001;now+=20){view.tick(now);
      if(slug==='bastion'){oracle.tick(now);for(const key of ['frame','direction','moving','cell'])assert.equal(view.root.dataset[key],oracle.root.dataset[key],key+' at '+now);}
      if(view.root.dataset.moving==='true'){phases.add(+view.root.dataset.frame);directions.add(view.root.dataset.direction);}
      if(data.dualDirections){
        const left=+view.root.dataset.leftSourceFrame,right=+view.root.dataset.rightSourceFrame;
        assert(left>=0&&left<=6);assert.equal(right,left+7);
        if(view.root.dataset.moving==='false')assert.equal(left,0);
      }}
    assert.deepEqual([...phases].sort(),[0,1,2,3,4,5]);assert.deepEqual([...directions].sort(),['1','7']);
    for(let armor=0;armor<data.armors.length;armor++){view.change('armor',armor);
      for(let option=0;option<data.options.length;option++){
        view.change('option',option);assert.equal(view.root.dataset.armor,String(armor));assert.equal(view.root.dataset.option,String(option));covered++;
        if(slug.startsWith('creatures')){
          creatureSelections++;
          // Native comparison or the same accepted tile before/after its
          // anchor correction; the latter must use identical body pixels.
          if(!data.dualDirections)assert.equal(data.referenceRects.length,data.options.length);
          const draw=view.elements.get('[data-reference-scene]').draws.at(-1);
          const after=view.elements.get('[data-scene]').draws.at(-1),entry=data.options[option];
          if(data.dualDirections){
            assert.equal(draw.length,3);assert.equal(after.length,3);
            assert.notEqual(draw[0],after[0]);
            const left=+view.root.dataset.leftSourceFrame,right=+view.root.dataset.rightSourceFrame;
            const allFrames=JSON.parse(require('zlib').gunzipSync(Buffer.from(data.framesGzip,'base64')));
            const expectedLeft=data.rects[allFrames[0][option][left][0][0]];
            const expectedRight=data.rects[allFrames[0][option][right][0][0]];
            assert.deepEqual(Array.from(draw[0].draws.at(-1).slice(1,5)),expectedLeft);
            assert.deepEqual(Array.from(after[0].draws.at(-1)[0].draws.at(-1).slice(1,5)),expectedRight);
            assert.equal(draw[1]+after[1],256-entry.foot[0]*2);
            assert.equal(draw[2],after[2]);
          }else if(data.placementComparison){
            assert.equal(draw.length,3);assert.equal(draw[0],after[0]);
            assert.equal(after[2]-draw[2],entry.verticalOffset);
            if(entry.shadow){
              const beforeShadow=view.elements.get('[data-reference-scene]').ellipses.at(-1);
              const afterShadow=view.elements.get('[data-scene]').ellipses.at(-1);
              assert.equal(afterShadow[1]-beforeShadow[1],entry.verticalOffset);
            }
          }else{
            assert.deepEqual(Array.from(draw.slice(1,5)),data.referenceRects[option]);
            assert.deepEqual(Array.from(draw.slice(-2)),[128,128]);
          }
          assert.equal(after[2],128-entry.foot[1]+entry.verticalOffset);
          const b=entry.cycleBounds,height=b[3]-b[1];
          if(height<64)assert(Math.abs(32-entry.foot[1]+entry.verticalOffset+(b[1]+b[3])/2)<=.5);
          else assert.equal(entry.verticalOffset,0);
          assert.equal(view.root.dataset.verticalOffset,String(entry.verticalOffset));
          const zoomDraw=view.elements.get('[data-zoom]').draws.at(-1);
          assert.equal(zoomDraw[0],view.elements.get('[data-scene]'));
          assert.deepEqual(Array.from(zoomDraw.slice(3,5)),[128,128]);
          assert.deepEqual(Array.from(zoomDraw.slice(-2)),[256,256]);
          assert.equal(entry.foot[0],64);assert(entry.foot[1]>=112&&entry.foot[1]<=124);
          // Contain every pose's complete envelope and the native64 floor cell.
          const [zx,zy,zw,zh]=zoomDraw.slice(1,5);
          assert(zx<=after[1]+b[0]&&zx+zw>=after[1]+b[2]);
          assert(zy<=after[2]+b[1]&&zy+zh>=after[2]+b[3]);
          const cellX=after[1]+entry.foot[0],cellY=after[2]+entry.foot[1]-32-entry.verticalOffset;
          assert(zx<=cellX-32&&zx+zw>=cellX+32&&zy<=cellY-32&&zy+zh>=cellY+32);
        }
      }}
    view.change('pace',120);view.change('floor','light');assert(view.saves>0);
    const saves=view.saves;view.events['openai:set_globals']({detail:{globals:{widgetState:{modelContent:{armor:0,option:0,pace:400,floor:'textured'}}}}});
    assert.equal(view.saves,saves);assert.equal(view.elements.get('[data-control="pace"]').value,'400');
  }
  const data=JSON.parse(fs.readFileSync(path.join(rootPath,'artifacts/player-walk-review-07/preview-data/bastion.json')));
  const single=await boot(data);single.tick(1);single.click('cell');for(const t of [81,161,241,321,401])single.tick(t);
  assert.equal(single.root.dataset.cell,'1');assert.equal(single.root.dataset.moving,'false');single.tick(1201);assert.equal(single.root.dataset.cell,'1');
  const manual=await boot(data);for(let i=0;i<3;i++){manual.click('step');assert.equal(manual.root.dataset.frame,String(i));}
  manual.click('step');assert.equal(manual.root.dataset.moving,'false');
  Object.assign(report,{selection_pairs_checked:covered,fragments_under_one_megabyte:checkedFragments,all_six_walk_frames:true,both_lateral_directions:true,
    separate_idle_at_arrival:true,single_cell_stops:true,duration_choice_and_manual_step:true,widget_state_restores_without_save_loop:true,
    trace_matches_accepted_walk05_step03_viewer:true,trace_samples:601,creature_native_reference_switches:creatureSelections,
    small_creatures_centered_in_native64_cell:true,whole_cycle_anchor_is_fixed:true,body_and_shadow_share_offset:true,
    large_creature_anchor_unchanged:true,placement_comparison_uses_identical_accepted_body_pixels:true,creature_zoom_preserves_cell_centering:true,
    creature_zoom_contains_whole_cycle_and_cell:true,
    bulk_two_lanes_use_separate_authored_directional_bodies:true});
  fs.writeFileSync(path.join(rootPath,'docs/reviews/player-walk-07/renderer-checks.json'),JSON.stringify(report,null,2)+'\n');
  console.log(JSON.stringify(report,null,2));
})().catch(error=>{console.error(error);process.exitCode=1;});
