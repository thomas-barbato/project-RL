const fs=require('fs'),path=require('path'),assert=require('assert/strict');
const {chromium}=require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const review=Number(process.argv[2]||70);assert(Number.isInteger(review));
const root=path.resolve(__dirname,'../..'),folder=path.join(root,'artifacts/refonte-agent-2026-10-06/skill-animation-review-'+review);
async function run(){const browser=await chromium.launch({channel:'msedge',headless:true});try{
const page=await browser.newPage({viewport:{width:1320,height:1180}}),errors=[],captures=[];
page.on('pageerror',e=>errors.push(e.message));page.on('requestfailed',r=>errors.push(r.url()));
await page.goto('file:///'+path.join(folder,'cote-a-cote.html').replaceAll('\\','/'));
const done=()=>page.waitForFunction(()=>ready&&!autoPlay&&turn===4);
const state=()=>page.evaluate(()=>entries().map(e=>({profile:e.profile,case:e.case,type:e.type,area:e.area.length,...e.timeline[frame(e)]})));
async function capture(label){await page.screenshot({path:path.join(folder,'browser-'+label+'.png')});await page.check('#native-view');
const samples=await page.evaluate(()=>[0,1].map(i=>{const e=pane(i),f=frame(e);return{profile:e.profile,frame:f,expected_native_png:'frames/'+e.variant+'/'+String(e.image_indices[f]).padStart(3,'0')+'.png'};}));
for(let i=0;i<2;i++){samples[i].browser_png='browser-'+label+'-'+samples[i].profile+'.png';await page.locator('canvas').nth(i).screenshot({path:path.join(folder,samples[i].browser_png)});}captures.push({label,samples});await page.uncheck('#native-view');}
for(const type of ['FEU','POISON','FROID']){
await page.selectOption('#type',type);await done();const initial=await state();assert.deepEqual(initial.map(e=>e.area),[9,81]);assert(initial.every(e=>e.zone_remaining===12&&e.zone_applications===2&&e.player_hp===100));
await page.waitForTimeout(1250);const later=await state();assert.deepEqual(initial.map(e=>[e.hp,e.zone_remaining,e.zone_applications]),later.map(e=>[e.hp,e.zone_remaining,e.zone_applications]));
await page.click('#demo');await page.waitForFunction(()=>turn===3&&elapsed>=170&&elapsed<330);await page.evaluate(()=>{paused=true;last=0;});await page.waitForTimeout(70);await capture(type+'-burst');
await page.click('#demo');await page.waitForFunction(()=>turn===4&&elapsed>=175&&elapsed<330);await page.evaluate(()=>{paused=true;last=0;});await page.waitForTimeout(70);await capture(type+'-residue');
await page.click('#expire');assert((await state()).every(e=>e.zone_remaining===0&&e.zone_applications===13&&!e.zone_gel.some(Boolean)));
await page.selectOption('#scenario','CLOISON');await done();assert.deepEqual((await state()).map(e=>e.area),[6,45]);assert((await state()).every(e=>e.hp[3]===400&&e.player_hp===100));
await page.selectOption('#scenario','HAUT');await done();
}
// Inspect an armed mine: accurate footprint, no turn or resource cost.
await page.click('#reset');await page.click('#cast');await page.waitForFunction(()=>elapsed>=850);await page.click('#next');await page.waitForFunction(()=>elapsed>=850);
const before=await state();const box=await page.locator('canvas').nth(1).boundingBox();await page.mouse.move(box.x+box.width*.5,box.y+box.height*304/576);
await page.waitForFunction(()=>hover[1]&&document.querySelectorAll('canvas')[1].dataset.source==='hover');await page.waitForTimeout(400);
const after=await state();assert.deepEqual(before.map(e=>[e.hp,e.energy,e.step]),after.map(e=>[e.hp,e.energy,e.step]));await page.screenshot({path:path.join(folder,'browser-hover.png')});
await page.mouse.move(2,2);await page.waitForFunction(()=>!hover[1]);
await page.click('#next');await page.waitForFunction(()=>elapsed>=850);await page.click('#next');await page.waitForFunction(()=>turn===3&&elapsed>=850);assert((await state()).every(e=>!e.preview_visible));
await page.click('#expire');await page.waitForTimeout(200);await page.screenshot({path:path.join(folder,'browser-expiry.png')});assert.deepEqual(errors,[]);
fs.writeFileSync(path.join(folder,'browser-visible-verification.json'),JSON.stringify({browser:'Edge Chromium',errors,all_three_types_and_wall:'passed',armed_mine_hover:'passed',idle_never_advances_turns:'passed',preview_removed:'passed',expiry_clear:'passed',captures},null,2)+'\n');console.log(JSON.stringify({browser:'passed',capture_pairs:captures.length,errors}));
}finally{await browser.close();}}
run().catch(e=>{console.error(e);process.exitCode=1;});
