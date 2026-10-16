// Browser checks for the accepted SU06-C02 / SU06-A03 rendition; no campaign mutation.
const fs=require('fs'),path=require('path'),assert=require('assert/strict');
const {chromium}=require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const revision=84,presentations=['rear_halo'];
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06'),review=path.join(base,'skill-animation-review-'+revision);
async function main(){
 const validation=JSON.parse(fs.readFileSync(path.join(review,'validation.json'),'utf8'));
 const http=require('http'),server=http.createServer((req,res)=>{const p=path.resolve(base,'.'+decodeURIComponent(req.url.split('?')[0]));if(!p.startsWith(base+path.sep)){res.writeHead(403);res.end();return;}fs.readFile(p,(err,data)=>{if(err){res.writeHead(404);res.end();return;}res.setHeader('Content-Type',p.endsWith('.webp')?'image/webp':p.endsWith('.png')?'image/png':'text/html; charset=utf-8');res.end(data);});});
 await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));const browser=await chromium.launch({channel:'msedge',headless:true});
 try{
  const page=await browser.newPage({viewport:{width:1200,height:1200}}),errors=[],checks=[];
  page.on('pageerror',e=>errors.push(e.message));page.on('requestfailed',r=>errors.push(r.url()));
  await page.goto(`http://127.0.0.1:${server.address().port}/apercu-animations.html`);
  const ready=()=>page.waitForFunction(()=>survivalPassivesReview.isReady());await ready();
  async function freeze(n){await page.evaluate(n=>survivalPassivesReview.freeze(n),n);await page.waitForFunction(n=>{const c=document.querySelector('canvas');return +c.dataset.frame===n&&c.dataset.variant===survivalPassivesReview.entries()[0].variant;},n);}
  for(const presentation of presentations){
  assert.equal(await page.locator('#version').count(),0);await ready();
  for(const scenario of ['DIRECT','ATTENTE','PERSISTANT','RECHARGE','SECOND'])for(const dir of scenario==='DIRECT'?[0,1,2,3]:[2]){
   await page.selectOption('#case',scenario);await ready();if(scenario==='DIRECT'){await page.selectOption('#direction',String(dir));await ready();}
   for(const frame of [0,18,27,54,111]){
    await freeze(frame);const value=await page.evaluate(()=>({state:survivalPassivesReview.states()[0],entry:survivalPassivesReview.entries()[0]}));
    const hash=await page.evaluate(async()=>{const data=document.querySelector('canvas').getContext('2d').getImageData(0,0,576,576).data,rgb=new Uint8Array(576*576*3);for(let i=0,j=0;i<data.length;i+=4){rgb[j++]=data[i];rgb[j++]=data[i+1];rgb[j++]=data[i+2];}return Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256',rgb))).map(x=>x.toString(16).padStart(2,'0')).join('');});
    const check=validation.checks.find(c=>c.variant===value.entry.variant);assert.equal(hash,check.frame_rgb_sha256[value.entry.indices[frame]],value.entry.variant);
    const s=value.state;assert.equal(s.energy,100);assert.equal(s.maximum,100);assert.equal(s.healed,0);assert(!s.aura&&!s.invulnerable);
    if(frame===18)assert.equal(s.hp,20);
    if(frame===27){if(['PERSISTANT','RECHARGE'].includes(scenario)){assert.equal(s.hp,0);assert.equal(s.saves,0);}else{assert.equal(s.hp,1);assert.equal(s.saves,1);assert.equal(s.cooldown,60);assert.equal(s.damage,19);}}
    if(frame===111){if(scenario==='SECOND'){assert.equal(s.hp,0);assert.equal(s.saves,1);assert.equal(s.cooldown,59);}if(scenario==='ATTENTE'){assert.equal(s.hp,1);assert.equal(s.cooldown,60);assert.equal(s.turn,1);}if(scenario==='DIRECT'){assert.equal(s.hp,1);assert.equal(s.cooldown,57);}}
   }
   checks.push({presentation,scenario,direction:dir,pixel_equality:true});
  }
  }
  const icons=await page.locator('.icons img').evaluateAll(imgs=>imgs.map(im=>({width:im.width,height:im.height,naturalWidth:im.naturalWidth,naturalHeight:im.naturalHeight})));
  assert.deepEqual(icons,[{width:32,height:32,naturalWidth:32,naturalHeight:32},{width:64,height:64,naturalWidth:64,naturalHeight:64}]);
  await page.goto('file:///'+path.join(base,'apercu-animations.html').replaceAll('\\','/'));await ready();await freeze(27);
  await page.screenshot({path:path.join(review,'_browser-proof.png')});await page.locator('#b').screenshot({path:path.join(review,'_survival-proof.png')});
  await page.click('#manual');await page.click('#start');await page.waitForTimeout(750);assert.equal((await page.evaluate(()=>survivalPassivesReview.states()))[0].hp,20);
  await page.click('#turn');await page.waitForTimeout(1100);const stable=await page.evaluate(()=>survivalPassivesReview.states());await page.waitForTimeout(900);assert.deepEqual(await page.evaluate(()=>survivalPassivesReview.states()),stable);assert.equal(stable[0].cooldown,60);
  await page.click('#turn');await page.waitForFunction(()=>survivalPassivesReview.states()[0].cooldown===59);
  await page.selectOption('#case','SECOND');await ready();await page.click('#manual');await page.click('#start');await page.click('#turn');await page.waitForTimeout(200);assert.equal((await page.evaluate(()=>survivalPassivesReview.states()))[0].hp,1);await page.click('#turn');await page.waitForFunction(()=>survivalPassivesReview.states()[0].hp===0);
  await page.selectOption('#case','ATTENTE');await ready();await page.click('#manual');await page.click('#start');await page.click('#turn');await page.waitForTimeout(1200);assert.equal((await page.evaluate(()=>survivalPassivesReview.states()))[0].cooldown,60);assert(await page.locator('#turn').isDisabled());
  await page.uncheck('#native');assert.equal(await page.locator('canvas.native').count(),0);await page.check('#native');assert.equal(await page.locator('canvas.native').count(),1);
  await page.setViewportSize({width:800,height:1000});await page.waitForTimeout(50);assert.equal(await page.locator('canvas').evaluate(c=>c.getBoundingClientRect().width),576);
  assert.deepEqual(errors,[]);
  fs.writeFileSync(path.join(review,'browser-verification.json'),JSON.stringify({browser:'Edge Chromium',views:checks.length,presentations,obsolete_variant_absent:true,schemes:['file: playback and controls','localhost: exact native RGB equality'],checks,icons_native_32_and_64:true,manual_idle_stable:true,prepared_mechanics_unchanged:true,no_persistent_save:true,no_second_save:true,no_energy_cost:true,errors},null,2));
  console.log(checks.length+' views passed: native pixels, 4 directions, icons, manual turns, unchanged results and accepted-only playback.');
 }finally{await browser.close();await new Promise(resolve=>server.close(resolve));}
}
main().catch(e=>{console.error(e);process.exitCode=1;});
