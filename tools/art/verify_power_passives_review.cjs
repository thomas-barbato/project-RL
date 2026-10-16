const fs=require('fs'),path=require('path'),assert=require('assert/strict');
const {chromium}=require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06'),review=path.join(base,'skill-animation-review-85');
async function main(){
 const validation=JSON.parse(fs.readFileSync(path.join(review,'validation.json'),'utf8'));
 const server=require('http').createServer((req,res)=>{const p=path.resolve(base,'.'+decodeURIComponent(req.url.split('?')[0]));if(!p.startsWith(base+path.sep)){res.writeHead(403);res.end();return;}fs.readFile(p,(err,data)=>{if(err){res.writeHead(404);res.end();return;}res.setHeader('Content-Type',p.endsWith('.webp')?'image/webp':'text/html; charset=utf-8');res.end(data);});});
 await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));const browser=await chromium.launch({channel:'msedge',headless:true});
 try{
  const page=await browser.newPage({viewport:{width:1400,height:1250}}),errors=[],checks=[];
  page.on('pageerror',e=>errors.push(e.message));page.on('requestfailed',r=>errors.push(r.url()));
  await page.goto(`http://127.0.0.1:${server.address().port}/apercu-animations.html`);
  const ready=()=>page.waitForFunction(()=>powerPassivesReview.isReady());await ready();
  async function freeze(n){await page.evaluate(n=>powerPassivesReview.freeze(n),n);await page.waitForFunction(n=>[...document.querySelectorAll('canvas')].every((c,i)=>+c.dataset.frame===n&&c.dataset.variant===powerPassivesReview.entries()[i].variant),n);}
  for(const scenario of ['BONUS','SANS','HORS','ATTENTE'])for(const dir of scenario==='BONUS'?[0,1,2,3]:[2]){
   await page.selectOption('#case',scenario);await ready();if(scenario==='BONUS'){await page.selectOption('#direction',String(dir));await ready();}
   for(const frame of [0,18,27,54,90,111]){
    await freeze(frame);const value=await page.evaluate(()=>({states:powerPassivesReview.states(),entries:powerPassivesReview.entries()}));
    const hashes=await page.evaluate(async()=>Promise.all([...document.querySelectorAll('canvas')].map(async c=>{const data=c.getContext('2d').getImageData(0,0,576,576).data,rgb=new Uint8Array(576*576*3);for(let i=0,j=0;i<data.length;i+=4){rgb[j++]=data[i];rgb[j++]=data[i+1];rgb[j++]=data[i+2];}return [...new Uint8Array(await crypto.subtle.digest('SHA-256',rgb))].map(x=>x.toString(16).padStart(2,'0')).join('');})));
    for(let i=0;i<2;i++){const e=value.entries[i],s=value.states[i],check=validation.checks.find(c=>c.variant===e.variant);assert.equal(hashes[i],check.frame_rgb_sha256[e.indices[frame]],e.variant);assert.equal(s.healed,0);assert(!s.temporary_buff);}
    if(frame===0)assert(value.states.every(s=>s.energy===100&&s.target_hp===100&&s.actions===0));
    if(frame===111){const [a,m]=value.states,count=scenario==='ATTENTE'?1:3,cost=scenario==='HORS'?0:scenario==='SANS'?9:5,damage=['BONUS','ATTENTE'].includes(scenario)?32:20;assert.equal(a.energy,100-count*cost);assert.equal(a.target_hp,100-count*20);assert.equal(m.energy,100);assert.equal(m.target_hp,100-count*damage);assert.equal(a.actions,count);assert.equal(m.actions,count);}
   }
   checks.push({scenario,direction:dir,views:2,pixel_equality:true});
  }
  await page.goto('file:///'+path.join(base,'apercu-animations.html').replaceAll('\\','/'));await ready();await freeze(18);await page.screenshot({path:path.join(review,'_browser-proof.png')});await page.locator('#b').screenshot({path:path.join(review,'_power-proof.png')});
  await page.click('#manual');assert.equal((await page.evaluate(()=>powerPassivesReview.states()))[0].actions,0);await page.click('#action');await page.waitForTimeout(1100);const stable=await page.evaluate(()=>powerPassivesReview.states());assert.equal(stable[0].energy,95);assert.equal(stable[1].target_hp,68);await page.waitForTimeout(900);assert.deepEqual(await page.evaluate(()=>powerPassivesReview.states()),stable);
  await page.click('#action');await page.waitForTimeout(700);assert.equal((await page.evaluate(()=>powerPassivesReview.states()))[0].actions,2);await page.click('#action');await page.waitForTimeout(700);assert.equal((await page.evaluate(()=>powerPassivesReview.states()))[1].target_hp,4);assert(await page.locator('#action').isDisabled());
  await page.selectOption('#case','ATTENTE');await ready();await page.click('#manual');await page.click('#action');await page.waitForTimeout(1000);assert(await page.locator('#action').isDisabled());
  await page.uncheck('#native');assert.equal(await page.locator('#pair.native').count(),0);await page.check('#native');assert.equal(await page.locator('#pair.native').count(),1);
  assert.deepEqual(errors,[]);fs.writeFileSync(path.join(review,'browser-verification.json'),JSON.stringify({browser:'Edge Chromium',views:14,schemes:['file: controls','localhost: native pixel equality'],checks,manual_idle_stable:true,permanent_passives:true,comparisons_and_exclusions:true,errors},null,2));console.log('14 views passed: exact native pixels, four directions, cost/damage bonuses, exclusions and stable manual playback.');
 }finally{await browser.close();await new Promise(resolve=>server.close(resolve));}
}
main().catch(e=>{console.error(e);process.exitCode=1;});
