const fs=require('fs'),path=require('path'),assert=require('assert/strict');
const {chromium}=require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const REVIEW=Number(process.argv[2]||72);assert([72,73].includes(REVIEW));
const root=path.resolve(__dirname,'../..'),folder=path.join(root,`artifacts/refonte-agent-2026-10-06/skill-animation-review-${REVIEW}`);
async function main(){const browser=await chromium.launch({channel:'msedge',headless:true});try{
 const page=await browser.newPage({viewport:{width:1400,height:1280}}),errors=[],captures=[],checks=[];
 page.on('pageerror',e=>errors.push(e.message));page.on('requestfailed',r=>errors.push(r.url()));
 await page.goto('file:///'+path.join(folder,'cote-a-cote.html').replaceAll('\\','/'));
 const warmed=()=>page.waitForFunction(()=>ready&&[...document.querySelectorAll('canvas')].every(c=>c.dataset.variant));
 await warmed();
 const state=()=>page.evaluate(()=>grenadeReview.getState());
 async function selection(c,d){await page.selectOption('#scene',String(c));await page.selectOption('#direction',String(d));await warmed();
  await page.waitForFunction(([c,d])=>ready&&scene===c&&direction===d&&[...document.querySelectorAll('canvas')].every((v,i)=>v.dataset.variant===entries()[i].variant),[c,d]);
 }
 async function freeze(n){await page.evaluate(n=>{paused=true;demo=true;elapsed=(n+.1)/24*1000;last=0;},n);await page.waitForFunction(n=>[...document.querySelectorAll('canvas')].every(c=>Number(c.dataset.frame)===n),n);}
 async function capture(label,n){await freeze(n);await page.screenshot({path:path.join(folder,'browser-'+label+'.png')});
  const samples=await page.evaluate(()=>[...document.querySelectorAll('canvas')].map((c,i)=>{const e=entries()[i],n=Number(c.dataset.frame);return{variant:e.variant,frame:n,expected_native_png:`frames/${e.variant}/${String(e.image_indices[n]).padStart(3,'0')}.png`};}));
  for(let i=0;i<samples.length;i++){samples[i].browser_png=`browser-${label}-${i}.png`;await page.locator('canvas').nth(i).screenshot({path:path.join(folder,samples[i].browser_png)});}captures.push({label,samples});
 }
 // All 32 variants are available at native size, including links for each direction.
 for(let c=0;c<4;c++)for(let d=0;d<4;d++){
  await selection(c,d);await freeze(27);const s=await state();
  assert(s.every(x=>x.player_hp===100));
  assert(s.every(x=>c===2?x.turn===0&&x.energy===100&&!x.explosion:x.turn===1&&x.energy===92&&x.explosion));
  for(let i=0;i<2;i++){const href=await page.locator('#gif'+i).getAttribute('href');assert(fs.existsSync(path.join(folder,href)),href);}
  if(REVIEW===73&&c===0){assert(s.every(x=>x.falloff));assert(s.every(x=>x.damage.filter(d=>d).sort((a,b)=>a-b).join(',')==='12,24'));}
  checks.push({case:c,direction:d,states:s});
 }
 await selection(0,2);await capture('projectile',20);assert((await state()).every(s=>s.projectile&&!s.preview));
 await capture('impact',27);assert((await state()).every(s=>s.contacts.reduce((a,b)=>a+b)===2));
 await capture('finished',40);assert((await state()).every(s=>!s.projectile&&!s.explosion));
 await selection(1,2);await capture('wall',27);
 await selection(2,2);await capture('blocked',27);assert(await page.locator('#cast').isDisabled());
 await selection(3,2);await capture('self',27);assert((await state()).every(s=>s.player_hp===100&&s.contacts.reduce((a,b)=>a+b)===2));
 // Manual play shows the active preview, then ends the transient effect without an idle cooldown tick.
 await selection(0,2);await page.click('#reset');await page.waitForFunction(()=>!demo&&manual===0&&frame(entries()[0])<18);
 assert((await state()).every(s=>s.preview&&s.energy===100&&s.turn===0));
 await page.click('#cast');await page.waitForFunction(()=>!demo&&manual===1&&elapsed>=1100);
 const before=await state();assert(before.every(s=>!s.preview&&!s.projectile&&!s.explosion&&s.turn===1&&s.cooldown===2&&s.energy===92));
 await page.waitForTimeout(1300);const after=await state();assert.deepEqual(before.map(s=>[s.turn,s.energy,s.cooldown,s.hp]),after.map(s=>[s.turn,s.energy,s.cooldown,s.hp]));
 await page.click('#next');await page.waitForFunction(()=>manual===2&&elapsed>=80);assert((await state()).every(s=>s.turn===2&&s.cooldown===1));
 await page.click('#next');await page.waitForFunction(()=>manual===3&&elapsed>=80);assert((await state()).every(s=>s.turn===3&&s.cooldown===0));
 if(REVIEW===73){
  assert.equal(await page.locator('#falloff-grid > div').count(),9);
  await page.selectOption('#falloff-radius','2');assert.equal(await page.locator('#falloff-grid > div').count(),25);
  assert.equal((await page.locator('#falloff-grid > div').allTextContents()).join(','),Array.from({length:25},(_,i)=>String(Math.floor(100*(4-Math.max(Math.abs(i%5-2),Math.abs(Math.floor(i/5)-2)))/4))).join(','));
  await page.locator('#falloff-grid').scrollIntoViewIfNeeded();await page.screenshot({path:path.join(folder,'browser-rule-5x5.png')});
  await page.selectOption('#falloff-radius','5');assert.equal(await page.locator('#falloff-grid > div').count(),121);
  assert.equal((await page.locator('#falloff-grid > div').allTextContents()).map(Number).reduce((a,b)=>a+b,0),7700);
  await page.locator('#falloff-base').fill('25');await page.selectOption('#falloff-radius','1');
  assert.deepEqual([...new Set((await page.locator('#falloff-grid > div').allTextContents()).map(Number))].sort((a,b)=>a-b),[12,25]);
 }
 assert(fs.existsSync(path.join(folder,REVIEW===73?'proposal.json':'generation.json')));assert.deepEqual(errors,[]);
 const record={browser:'Edge Chromium headless',scheme:'file',native_tile:64,views_checked:32,errors,
  projectile_then_ephemeral_burst:'passed',preview_removed_after_launch:'passed',idle_counters_stable:'passed',
  manual_turns_and_cooldown:'passed',blocked_attack_no_cost:'passed',self_damage_immunity:'passed',
  directional_gif_links_exist:'passed',captures,checks};
 fs.writeFileSync(path.join(folder,'browser-visible-verification.json'),JSON.stringify(record,null,2)+'\n');
 console.log(JSON.stringify({browser:'passed',views:32,captures:captures.length,errors}));
 }finally{await browser.close();}}
main().catch(e=>{console.error(e);process.exitCode=1;});
