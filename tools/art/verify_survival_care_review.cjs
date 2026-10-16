const fs=require('fs'),path=require('path'),assert=require('assert/strict');
const {chromium}=require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const root=path.resolve(__dirname,'../..'),review=path.resolve(process.argv[2]||path.join(root,'artifacts/refonte-agent-2026-10-06/skill-animation-review-77')),file=path.join(root,'artifacts/refonte-agent-2026-10-06/apercu-animations.html');
async function main(){
 const validation=JSON.parse(fs.readFileSync(path.join(review,'validation.json'),'utf8'));
 const isolatedPurge=Boolean(validation.purge_isolated);
 const http=require('http'),base=path.dirname(file),server=http.createServer((req,res)=>{const p=path.resolve(base,'.'+decodeURIComponent(req.url.split('?')[0]));if(!p.startsWith(base+path.sep)){res.writeHead(403);res.end();return;}fs.readFile(p,(err,data)=>{if(err){res.writeHead(404);res.end();return;}res.setHeader('Content-Type',p.endsWith('.webp')?'image/webp':'text/html; charset=utf-8');res.end(data);});});
 await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));const browser=await chromium.launch({channel:'msedge',headless:true});try{
 const page=await browser.newPage({viewport:{width:1400,height:1150}}),errors=[],checks=[];
 page.on('pageerror',e=>errors.push(e.message));page.on('requestfailed',r=>errors.push(r.url()));
 await page.goto(`http://127.0.0.1:${server.address().port}/apercu-animations.html`);
 const ready=()=>page.waitForFunction(()=>survivalCareReview.isReady());await ready();
 async function freeze(n){await page.evaluate(n=>survivalCareReview.freeze(n),n);await page.waitForFunction(n=>[...document.querySelectorAll('canvas')].every((c,i)=>+c.dataset.frame===n&&c.dataset.variant===survivalCareReview.entries()[i].variant),n);}
 for(const scenario of ['COMPLET','ATTENTE','INTERRUPTION'])for(const dir of scenario==='INTERRUPTION'?[0,1,2,3]:[2]){
  await page.selectOption('#case',scenario);await ready();if(scenario==='INTERRUPTION'){await page.selectOption('#direction',String(dir));await ready();}
  await freeze(16);let values=await page.evaluate(()=>({states:survivalCareReview.states(),entries:survivalCareReview.entries()}));
  assert.equal(values.states[0].hp,54);assert.equal(values.states[0].energy,94);assert(values.states[0].channel);assert.equal(values.states[1].hp,40);assert.equal(values.states[1].energy,99);assert(values.states.every(s=>!s.preview&&!s.poison&&!s.burn));
  for(const sample of isolatedPurge?[0,16,110]:[16]){
   await freeze(sample);
   const hashes=await page.evaluate(async()=>Promise.all([...document.querySelectorAll('canvas')].map(async c=>{const data=c.getContext('2d').getImageData(0,0,576,576).data,rgb=new Uint8Array(576*576*3);for(let i=0,j=0;i<data.length;i+=4){rgb[j++]=data[i];rgb[j++]=data[i+1];rgb[j++]=data[i+2];}return Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256',rgb))).map(x=>x.toString(16).padStart(2,'0')).join('');})));
   for(let i=0;i<2;i++){const e=values.entries[i],c=validation.checks.find(c=>c.variant===e.variant);assert.equal(hashes[i],c.frame_rgb_sha256[e.indices[sample]],e.variant);}
  }
  await freeze(110);values=await page.evaluate(()=>survivalCareReview.states());
  if(scenario==='COMPLET'){assert.equal(values[0].hp,82);assert.equal(values[0].doses,3);assert(!values[0].channel);assert.equal(values[1].hp,40);}
  if(scenario==='ATTENTE'){assert.equal(values[0].hp,54);assert.equal(values[0].doses,1);assert(values[0].channel);assert(values.every(s=>s.turn===1));}
  if(scenario==='INTERRUPTION'){assert.equal(values[0].hp,46);assert.equal(values[0].doses,1);assert(!values[0].channel);assert.equal(values[1].poison,!isolatedPurge);assert(!values[1].burn);if(isolatedPurge)assert(!values[1].interrupted);}
  checks.push({scenario,direction:dir,pixel_equality:true});
 }
 await page.goto('file:///'+file.replaceAll('\\','/'));await ready();await freeze(16);await page.screenshot({path:path.join(review,'_browser-proof.png')});
 if(isolatedPurge){await freeze(0);await page.locator('#b').screenshot({path:path.join(review,'_purge-before.png')});await freeze(110);await page.locator('#b').screenshot({path:path.join(review,'_purge-after.png')});}
 await page.click('#manual');await page.waitForFunction(()=>[...document.querySelectorAll('canvas')].every(c=>+c.dataset.frame<12));assert((await page.evaluate(()=>survivalCareReview.states())).every(s=>s.preview));
 await page.click('#cast');await page.waitForTimeout(1800);const before=await page.evaluate(()=>survivalCareReview.states());assert.equal(before[0].hp,54);await page.waitForTimeout(900);assert.deepEqual(await page.evaluate(()=>survivalCareReview.states()),before);
 await page.click('#turn');await page.waitForFunction(()=>survivalCareReview.states()[0].doses===2);await page.waitForTimeout(200);
 await page.click('#turn');await page.waitForFunction(()=>survivalCareReview.states()[0].doses===3);assert.equal((await page.evaluate(()=>survivalCareReview.states()))[0].hp,82);
 await page.selectOption('#case','INTERRUPTION');await ready();await page.click('#manual');await page.click('#cast');await page.waitForTimeout(100);await page.click('#turn');await page.waitForFunction(()=>survivalCareReview.states()[0].interrupted);let states=await page.evaluate(()=>survivalCareReview.states());assert.equal(states[0].hp,46);assert.equal(states[1].poison,!isolatedPurge);await page.waitForTimeout(150);await page.click('#turn');await page.waitForTimeout(100);states=await page.evaluate(()=>survivalCareReview.states());assert.equal(states[0].doses,1);assert.equal(states[1].hp,40);
 await page.uncheck('#native');assert.equal(await page.locator('#pair.native').count(),0);await page.check('#native');assert.equal(await page.locator('#pair.native').count(),1);
 assert.deepEqual(errors,[]);fs.writeFileSync(path.join(review,'browser-verification.json'),JSON.stringify({browser:'Edge Chromium',schemes:['file: controls and display','localhost: pixel equality'],views:12,checks,manual_idle_stable:true,interruption:true,no_purge_healing:true,errors},null,2));console.log('12 native views, manual idle, interrupted channel, cleanse and pixel equality: passed');
 }finally{await browser.close();await new Promise(resolve=>server.close(resolve));}
}
main().catch(e=>{console.error(e);process.exitCode=1;});
