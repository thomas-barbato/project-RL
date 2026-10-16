const fs=require('fs'),path=require('path'),assert=require('assert/strict');
const {chromium}=require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06'),review=path.join(base,'skill-animation-review-80');
async function main(){
 const validation=JSON.parse(fs.readFileSync(path.join(review,'validation.json'),'utf8'));
 const http=require('http'),server=http.createServer((req,res)=>{const p=path.resolve(base,'.'+decodeURIComponent(req.url.split('?')[0]));if(!p.startsWith(base+path.sep)){res.writeHead(403);res.end();return;}fs.readFile(p,(err,data)=>{if(err){res.writeHead(404);res.end();return;}res.setHeader('Content-Type',p.endsWith('.webp')?'image/webp':'text/html; charset=utf-8');res.end(data);});});
 await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));const browser=await chromium.launch({channel:'msedge',headless:true});
 try {
  const page=await browser.newPage({viewport:{width:1400,height:1150}}),errors=[],checks=[];
  page.on('pageerror',e=>errors.push(e.message));page.on('requestfailed',r=>errors.push(r.url()));
  await page.goto(`http://127.0.0.1:${server.address().port}/apercu-animations.html`);
  const ready=()=>page.waitForFunction(()=>evasionReview.isReady());await ready();
  async function freeze(n){await page.evaluate(n=>evasionReview.freeze(n),n);await page.waitForFunction(n=>[...document.querySelectorAll('canvas')].every((c,i)=>+c.dataset.frame===n&&c.dataset.variant===evasionReview.entries()[i].variant),n);}
  for(const scenario of ['IMPACT','DUREE','ATTENTE','OBSTACLE'])for(const dir of scenario==='IMPACT'?[0,1,2,3]:[2]){
   await page.selectOption('#case',scenario);await ready();if(scenario==='IMPACT'){await page.selectOption('#direction',String(dir));await ready();}
   for(const frame of [0,15,18,54,95]){
    await freeze(frame);const values=await page.evaluate(()=>({states:evasionReview.states(),entries:evasionReview.entries()}));
    const hashes=await page.evaluate(async()=>Promise.all([...document.querySelectorAll('canvas')].map(async c=>{const data=c.getContext('2d').getImageData(0,0,576,576).data,rgb=new Uint8Array(576*576*3);for(let i=0,j=0;i<data.length;i+=4){rgb[j++]=data[i];rgb[j++]=data[i+1];rgb[j++]=data[i+2];}return Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256',rgb))).map(x=>x.toString(16).padStart(2,'0')).join('');})));
    for(let i=0;i<2;i++){const e=values.entries[i],check=validation.checks.find(c=>c.variant===e.variant);assert.equal(hashes[i],check.frame_rgb_sha256[e.indices[frame]],e.variant);}
    if(frame===0)assert(values.states.every(s=>s.preview&&s.energy===100&&s.turn===0));
    if(frame>=15&&scenario!=='OBSTACLE')assert(values.states.every(s=>s.cast&&!s.preview&&s.damage===0));
    if(scenario==='OBSTACLE')assert(values.states.every(s=>!s.cast&&s.energy===100&&s.turn===0&&s.cooldown===0));
    if(frame===95&&scenario==='ATTENTE'){assert.equal(values.states[0].remaining,3);assert.equal(values.states[1].remaining,2);assert(values.states.every(s=>s.turn===1));}
    if(frame===95&&['IMPACT','DUREE'].includes(scenario))assert(values.states.every(s=>s.remaining===0));
   }
   checks.push({scenario,direction:dir,pixel_equality:true});
  }
  await page.goto('file:///'+path.join(base,'apercu-animations.html').replaceAll('\\','/'));await ready();await freeze(15);await page.screenshot({path:path.join(review,'_browser-proof.png')});
  await freeze(30);await page.locator('#a').screenshot({path:path.join(review,'_lure-proof.png')});
  await freeze(15);await page.locator('#b').screenshot({path:path.join(review,'_phase-proof.png')});
  await page.click('#manual');await page.waitForFunction(()=>evasionReview.states().every(s=>s.preview));
  await page.click('#cast');await page.waitForTimeout(1900);const stable=await page.evaluate(()=>evasionReview.states());await page.waitForTimeout(900);assert.deepEqual(await page.evaluate(()=>evasionReview.states()),stable);
  await page.click('#turn');await page.waitForFunction(()=>evasionReview.states()[0].impact);assert.equal((await page.evaluate(()=>evasionReview.states()))[0].remaining,0);
  await page.click('#turn');await page.waitForFunction(()=>evasionReview.states()[1].remaining===0);
  await page.selectOption('#case','DUREE');await ready();await page.click('#manual');await page.click('#cast');await page.waitForTimeout(300);
  for(let turn=2;turn<=4;turn++){await page.click('#turn');await page.waitForFunction(turn=>evasionReview.states().every(s=>s.turn===turn),turn);}
  assert((await page.evaluate(()=>evasionReview.states())).every(s=>s.remaining===0));
  await page.selectOption('#case','OBSTACLE');await ready();await page.click('#manual');await page.click('#cast');await page.waitForTimeout(150);assert((await page.evaluate(()=>evasionReview.states())).every(s=>s.turn===0&&s.energy===100));assert(await page.locator('#turn').isDisabled());
  await page.uncheck('#native');assert.equal(await page.locator('#pair.native').count(),0);await page.check('#native');assert.equal(await page.locator('#pair.native').count(),1);
  assert.deepEqual(errors,[]);
  fs.writeFileSync(path.join(review,'browser-verification.json'),JSON.stringify({browser:'Edge Chromium',views:14,schemes:['file: playback and controls','localhost: exact native RGB equality'],checks,manual_idle_stable:true,blocked_no_consumption:true,expiry_and_first_impact:true,errors},null,2));
  console.log('14 native views: exact pixels, four directions, manual idle, expiry, first impact and blocked refusal passed.');
 }finally{await browser.close();await new Promise(resolve=>server.close(resolve));}
}
main().catch(e=>{console.error(e);process.exitCode=1;});
