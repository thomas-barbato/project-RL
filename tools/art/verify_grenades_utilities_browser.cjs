const fs=require('fs'),path=require('path'),assert=require('assert/strict');
const {chromium}=require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const REVIEW=Number(process.argv[2]||74);assert([74,75].includes(REVIEW));
const root=path.resolve(__dirname,'../..'),folder=path.join(root,`artifacts/refonte-agent-2026-10-06/skill-animation-review-${REVIEW}`);
async function main(){const browser=await chromium.launch({channel:'msedge',headless:true});try{
 const page=await browser.newPage({viewport:{width:1400,height:1300}}),errors=[],captures=[],checks=[];
 page.on('pageerror',e=>errors.push(e.message));page.on('requestfailed',r=>errors.push(r.url()));
 await page.goto('file:///'+path.join(folder,'cote-a-cote.html').replaceAll('\\','/'));
 async function selection(p,c,d){await page.selectOption('#pair-mode',String(p));await page.selectOption('#scene',String(c));await page.selectOption('#direction',String(d));await page.waitForFunction(([p,c,d])=>ready&&pairMode===p&&scene===c&&direction===d&&[...document.querySelectorAll('canvas')].every((v,i)=>v.dataset.variant===entries()[i].variant),[p,c,d]);}
 const state=()=>page.evaluate(()=>grenadeUtilityReview.getState());
 async function freeze(n){await page.evaluate(n=>{paused=true;demo=true;elapsed=(n+.1)/24*1000;last=0;},n);await page.waitForFunction(n=>[...document.querySelectorAll('canvas')].every(c=>Number(c.dataset.frame)===n),n);}
 async function capture(label,n){await freeze(n);await page.screenshot({path:path.join(folder,`browser-${label}.png`)});
  const samples=await page.evaluate(()=>[...document.querySelectorAll('canvas')].map((c,i)=>{const e=entries()[i],n=Number(c.dataset.frame);return{variant:e.variant,frame:n,expected_native_png:`frames/${e.variant}/${String(e.image_indices[n]).padStart(3,'0')}.png`};}));
  for(let i=0;i<samples.length;i++){samples[i].browser_png=`browser-${label}-${i}.png`;await page.locator('canvas').nth(i).screenshot({path:path.join(folder,samples[i].browser_png)});}captures.push({label,samples});}
 for(let p=0;p<2;p++)for(let c=0;c<3;c++)for(let d=0;d<4;d++){
  await selection(p,c,d);await freeze(27);const s=await state();
  assert(s.every(x=>c===2?x.turn===0&&x.energy===100&&x.preview:x.turn===1&&x.energy===92&&!x.preview));
  if(c!==2){if(p===0){assert.deepEqual(s[0].blindness,[12,12,0]);assert(s.every(x=>x.hp.every(h=>h===100)));assert.equal(s[1].noise_turns,2);}else{assert.deepEqual(s[0].hp,[76,88,100]);assert.deepEqual(s[1].hp,[50,75,100]);}}
  for(let i=0;i<2;i++){const href=await page.locator('#gif'+i).getAttribute('href');assert(fs.existsSync(path.join(folder,href)),href);}
  checks.push({pair:p,case:c,direction:d});
 }
 await selection(0,0,2);await capture('projectiles',20);assert((await state()).every(s=>s.projectile));
 await capture('utility-impact',24);await capture('status-and-sound',45);await capture('two-turns',92);assert.equal((await state())[1].noise_turns,0);
 await capture('blindness-expired',118);assert.deepEqual((await state())[0].blindness,[0,0,0]);
 await selection(0,1,0);await capture('utility-wall-right',27);
 await selection(1,0,2);await capture('power-impact',27);
 await selection(1,1,1);await capture('power-wall-left',27);
 await selection(0,2,2);await capture('blocked',27);assert(await page.locator('#cast').isDisabled());
 await selection(0,0,2);await page.click('#reset');await page.waitForFunction(()=>!demo&&manual===0);
 assert((await state()).every(s=>s.preview&&s.turn===0));
 await page.click('#cast');await page.waitForFunction(()=>manual===1&&elapsed>1250);
 const before=await state();assert.deepEqual(before[0].blindness,[12,12,0]);assert.equal(before[1].noise_turns,2);
 await page.waitForTimeout(1000);const after=await state();assert.deepEqual(before.map(s=>[s.turn,s.energy,s.cooldown,s.blindness,s.noise_turns,s.positions]),after.map(s=>[s.turn,s.energy,s.cooldown,s.blindness,s.noise_turns,s.positions]));
 await page.click('#next');await page.waitForFunction(()=>manual===2&&elapsed>80);assert.deepEqual((await state())[0].blindness,[11,11,0]);assert.equal((await state())[1].noise_turns,1);
 await page.click('#next');await page.waitForFunction(()=>manual===3&&elapsed>80);assert.deepEqual((await state())[0].blindness,[10,10,0]);assert.equal((await state())[1].noise_turns,0);
 await page.click('#expiry');await page.waitForFunction(()=>manual===4&&elapsed>80);assert.deepEqual((await state())[0].blindness,[0,0,0]);assert.deepEqual(errors,[]);
 fs.writeFileSync(path.join(folder,'browser-verification.json'),JSON.stringify({browser:'Edge Chromium headless',scheme:'file',native_tile:64,views_checked:48,errors,manual_idle_stability:'passed',manual_expiry:'passed',checks,captures},null,2)+'\n');
 console.log(JSON.stringify({browser:'passed',views:48,captures:captures.length,errors}));
 }finally{await browser.close();}}
main().catch(e=>{console.error(e);process.exitCode=1;});
