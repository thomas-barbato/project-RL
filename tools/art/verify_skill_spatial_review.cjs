const fs=require('fs'),path=require('path'),assert=require('assert/strict');
const Spatial=require('./spatial_growth_player.js');
const {chromium}=require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const root=path.resolve(__dirname,'../..'),out=path.join(root,'artifacts/refonte-agent-2026-10-06',process.argv[2]||'skill-spatial-review-01');
const data=JSON.parse(fs.readFileSync(path.join(out,'native/capture.json'),'utf8'));
const order=a=>a.map(p=>p.join(',')).sort();
for(const f of data.fixtures){const map=data.maps[f.case].split('\n');
  assert.deepEqual(order(Spatial.cells(map,f.origin,f.shape,f.direction)),order(f.affected),`${f.id} rank${f.rank} case${f.case}: area`);
  assert.deepEqual(order(Spatial.visible(map,f.player,f.vision)),order(f.visible),`${f.id}: FOV`);
  const valid=Spatial.walk(map,...f.origin)&&Spatial.within(f.player,f.origin,f.range)&&f.visible.some(p=>p[0]===f.origin[0]&&p[1]===f.origin[1])&&Spatial.los(map,f.player,f.origin);
  assert.equal(valid,f.valid);
}
async function run(){const browser=await chromium.launch({channel:'msedge',headless:true});try{
  const page=await browser.newPage({viewport:{width:1440,height:1120}}),errors=[];
  page.on('pageerror',e=>errors.push(String(e)));page.on('requestfailed',r=>errors.push(r.url()));
  await page.goto('file:///'+path.join(out,'index.html').replaceAll('\\','/'));await page.waitForFunction(()=>window.ready===true);
  const weighted=data.fixtures.filter(f=>f.direct_damage_retention?.length).map(f=>({shape:f.shape,origin:f.origin,values:f.direct_damage_retention}));
  const retained=await page.evaluate(rows=>{let count=0;for(const f of rows)for(const v of f.values){if(window.centredDamageRetention(f.shape,f.origin,v.cell)!==v.percent)throw new Error('Damage retention differs from native');count++;}return count;},weighted);
  const state=()=>page.evaluate(()=>{const r=spatialReview,c=r.current();return {id:r.state.id,rank:r.state.learned+r.state.bonus,range:c.range,shape:c.shape,valid:c.valid,visible:c.visible.length,cells:c.cells,origin:c.origin,player:r.state.player,tests:r.state.tests,confirmed:r.state.confirmed};});
  let s=await state();assert.equal(s.id,'A02');assert.equal(s.rank,13);assert.equal(s.range,8);assert.equal(s.cells.length,121);assert.equal(s.valid,true);assert(!s.cells.some(p=>p[0]===s.player[0]&&p[1]===s.player[1]));
  await page.screenshot({path:path.join(out,'browser-vision12.png')});
  await page.selectOption('#vision','8');await page.waitForTimeout(80);const old=await state();assert(old.visible<s.visible);assert(old.valid);await page.screenshot({path:path.join(out,'browser-vision8.png')});
  await page.selectOption('#vision','12');
  for(const scene of ['1','2','3']){await page.selectOption('#scene',scene);s=await state();assert(s.valid);if(scene==='3'){assert(s.cells.some(p=>p[0]===19&&p[1]===12));assert(!s.cells.some(p=>p[0]===15&&p[1]===10));}else assert(!s.cells.some(p=>p[0]===19&&p[1]===12));await page.screenshot({path:path.join(out,'browser-scene'+scene+'.png')});}
  await page.selectOption('#scene','0');await page.selectOption('#skill','ME05');s=await state();assert.equal(s.cells.length,1);assert.equal(s.range,7);assert(!s.valid);
  await page.selectOption('#skill','HY04');assert.equal((await state()).range,8);
  await page.selectOption('#skill','FE02');for(const dir of ['0','1','2','3']){await page.selectOption('#direction',dir);s=await state();assert(s.cells.length>1);assert(s.valid);}
  await page.selectOption('#skill','FR04');s=await state();assert.equal(s.shape.kind,'line');assert.equal(s.shape.width,11);assert.equal(s.shape.length,9);
  await page.selectOption('#skill','A02');await page.evaluate(()=>spatialReview.change(()=>{spatialReview.state.player=[12,12];spatialReview.state.target=[12,12];}));
  s=await state();assert(s.valid&&s.cells.some(p=>p[0]===12&&p[1]===12));await page.click('#test');assert((await state()).confirmed);const tests=(await state()).tests;
  await page.evaluate(()=>spatialReview.change(()=>spatialReview.state.target=[29,12]));await page.waitForTimeout(70);assert(!(await state()).valid);assert(await page.locator('#test').isDisabled());assert.equal((await state()).tests,tests);
  await page.evaluate(()=>spatialReview.change(()=>{spatialReview.state.case=1;spatialReview.state.target=[18,12];}));assert(!(await state()).valid);
  await page.selectOption('#scene','0');await page.selectOption('#skill','DI04');assert.equal((await state()).range,10);
  await page.evaluate(()=>{spatialReview.change(()=>{spatialReview.state.learned=9;spatialReview.state.bonus=0;});document.querySelector('#learned').value=9;});
  assert.equal((await state()).range,8);assert((await page.locator('#next').innerText()).includes('9 cases'));
  // Each proposed skill's current geometry agrees with the Rust fixture at rank 10.
  for(const id of [...new Set(data.fixtures.map(f=>f.id))]){
    await page.selectOption('#skill',id);await page.evaluate(()=>spatialReview.change(()=>{spatialReview.state.learned=10;spatialReview.state.bonus=0;spatialReview.state.player=[12,12];spatialReview.state.target=[20,12];spatialReview.state.case=0;spatialReview.state.dir=0;}));
    s=await state();const f=data.fixtures.find(f=>f.id===id&&f.rank===10&&f.case===0);assert.equal(s.range,f.range);assert.deepEqual(s.shape,f.shape);assert.deepEqual(order(s.cells),order(f.affected));
  }
  assert.deepEqual(errors,[]);
  const result={native_geometry_fixture_parity:data.fixtures.length,browser:'Edge Chromium',errors,skill_geometry_parity:46,free_targeting:true,isolated_obstacle_shadow:true,full_wall:true,long_corridor:true,four_directions:true,melee_impact_stays_one_cell:true,self_target_has_no_empty_square:true,hidden_target_refused:true,invalid_target_changes_no_test_count:true,next_rank_retains_equipment:true,tile:64};
  result.skill_geometry_parity=new Set(data.fixtures.map(f=>f.id)).size;
  result.centred_damage_retention_parity=retained;
  fs.writeFileSync(path.join(out,'verification.json'),JSON.stringify(result,null,2)+'\n');console.log(JSON.stringify(result));
}finally{await browser.close();}}
run().catch(e=>{console.error(e);process.exitCode=1;});
