const fs=require('fs'),path=require('path'),assert=require('assert/strict');
const {chromium}=require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const root=path.resolve(__dirname,'../..'),folder=path.join(root,'artifacts/refonte-agent-2026-10-06/skill-animation-review-65');
async function main(){const browser=await chromium.launch({channel:'msedge',headless:true});try{
 const page=await browser.newPage({viewport:{width:1320,height:1180}}),errors=[],checks=[];
 page.on('pageerror',e=>errors.push(e.message));page.on('requestfailed',r=>errors.push(r.url()));
 await page.goto('file:///'+path.join(folder,'cote-a-cote.html').replaceAll('\\','/'));
 const finished=()=>page.waitForFunction(()=>ready&&launched&&!autoPlay&&turn===4);
 const state=()=>page.evaluate(()=>entries().map(e=>({type:e.type,...e.timeline[frame(e)]})));
 async function capture(label){await page.click('#demo');await page.waitForFunction(()=>turn===4&&elapsed>=165&&elapsed<310);await page.evaluate(()=>{paused=true;last=0;});await page.waitForTimeout(60);await page.screenshot({path:path.join(folder,'browser-'+label+'.png')});await page.check('#native-view');
  const samples=await page.evaluate(()=>[...document.querySelectorAll('canvas')].map((canvas,i)=>{const e=pane(i),f=frame(e);return{type:e.type,frame:f,expected_native_png:'frames/'+e.variant+'/'+String(e.image_indices[f]).padStart(3,'0')+'.png'};}));
  for(let i=0;i<samples.length;i++){samples[i].browser_png='browser-'+label+'-'+samples[i].type+'.png';await page.locator('canvas').nth(i).screenshot({path:path.join(folder,samples[i].browser_png)});}checks.push({capture:label,samples});await page.uncheck('#native-view');
 }
 await finished();assert((await page.locator('#left-title').textContent())==='Sol enflammé');const first=await state();assert(first.every(t=>t.zone_remaining===12&&t.zone_applications===2&&t.entrant_inside));
 await page.waitForTimeout(1800);const later=await state();assert.deepEqual(first.map(t=>[t.hp,t.zone_remaining,t.zone_applications]),later.map(t=>[t.hp,t.zone_remaining,t.zone_applications]));
 assert(first.find(t=>t.type==='FEU').hp[2]===397);assert(first.find(t=>t.type==='POISON').hp[2]===396);
 await capture('feu-poison');
 await page.click('#expire');assert((await state()).every(t=>t.zone_remaining===0&&t.zone_applications===13));
 await page.waitForTimeout(80);await page.screenshot({path:path.join(folder,'browser-expiry.png')});
 await page.selectOption('#right-type','FROID');await finished();assert((await state()).find(t=>t.type==='FROID').zone_gel[2]);
 await capture('feu-froid');await page.click('#expire');assert((await state()).every(t=>!t.zone_gel.some(Boolean)));
 await page.selectOption('#scenario','CLOISON');await finished();assert((await state()).every(t=>t.hp[3]===400&&t.player_hp===100));await capture('cloison');
 // Manual advances consume exactly the prepared applications, never a wall-clock tick.
 await page.selectOption('#left-type','POISON');await finished();
 for(let i=0;i<12;i++){await page.click('#next');await page.waitForFunction(()=>elapsed>=880);}
 const expired=await state();assert(expired.every(t=>t.zone_remaining===0&&t.zone_applications===13));assert(expired.find(t=>t.type==='POISON').hp[0]===119);assert(expired.find(t=>t.type==='POISON').poison_ticks[0]===5);
 await page.click('#reset');await page.click('#cast');await page.waitForFunction(()=>elapsed>=880);assert((await state()).every(t=>t.zone_remaining===0&&t.explosions===0&&!t.preview_visible));
 assert.deepEqual(errors,[]);
 const record={browser:'Edge Chromium headless',scheme:'file',errors,automatic_explosion_then_entry:'passed',all_three_types:'passed',manual_13_application_expiry:'passed',idle_does_not_advance_turns:'passed',player_and_wall_protection:'passed',replay_and_expiry_shortcut:'passed',captures:checks};
 fs.writeFileSync(path.join(folder,'browser-visible-verification.json'),JSON.stringify(record,null,2)+'\n');console.log(JSON.stringify({browser:'passed',captures:checks.length,errors}));
 }finally{await browser.close();}}
main().catch(e=>{console.error(e);process.exitCode=1;});
