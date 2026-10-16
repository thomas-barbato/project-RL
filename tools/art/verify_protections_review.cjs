const fs=require('fs'),path=require('path'),assert=require('assert/strict');
const {chromium}=require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const root=path.resolve(__dirname,'../..'),review=path.join(root,'artifacts/refonte-agent-2026-10-06/skill-animation-review-76'),file=path.join(root,'artifacts/refonte-agent-2026-10-06/apercu-animations.html');
const validation=JSON.parse(fs.readFileSync(path.join(review,'validation.json'),'utf8'));
async function main(){
 const http=require('http'),base=path.dirname(file),server=http.createServer((req,res)=>{const p=path.resolve(base,'.'+decodeURIComponent(req.url.split('?')[0]));if(!p.startsWith(base+path.sep)){res.writeHead(403);res.end();return;}fs.readFile(p,(err,data)=>{if(err){res.writeHead(404);res.end();return;}res.setHeader('Content-Type',p.endsWith('.webp')?'image/webp':'text/html; charset=utf-8');res.end(data);});});
 await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
 const browser=await chromium.launch({channel:'msedge',headless:true});try{
 const page=await browser.newPage({viewport:{width:1400,height:1100}}),errors=[],checks=[];
 page.on('pageerror',e=>errors.push(e.message));page.on('requestfailed',r=>errors.push(r.url()));
 await page.goto(`http://127.0.0.1:${server.address().port}/apercu-animations.html`);
 const ready=()=>page.waitForFunction(()=>protectionReview.isReady());await ready();
 async function freeze(n){await page.evaluate(n=>protectionReview.freeze(n),n);await page.waitForFunction(n=>[...document.querySelectorAll('canvas')].every((c,i)=>+c.dataset.frame===n&&c.dataset.variant===protectionReview.entries()[i].variant),n);}
 for(const scenario of ['MELEE','DISTANCE','PERSISTANT','ATTENTE','SILENCE'])for(const dir of scenario==='MELEE'?[0,1,2,3]:[2]){
  await page.selectOption('#case',scenario);await ready();if(scenario==='MELEE'){await page.selectOption('#direction',String(dir));await ready();}
  await freeze(36);const values=await page.evaluate(()=>({states:protectionReview.states(),entries:protectionReview.entries()}));
  if(scenario==='SILENCE')assert(values.states.every(s=>s.energy===100&&s.hp===100&&!s.icon_visible));
  else if(scenario==='PERSISTANT')assert(values.states.every(s=>s.damage===10&&s.reflected===0));
  else{assert.equal(values.states[0].damage,9);assert.equal(values.states[1].damage,20);assert.equal(values.states[1].reflected,scenario==='DISTANCE'?0:8);}
  const hashes=await page.evaluate(async()=>Promise.all([...document.querySelectorAll('canvas')].map(async c=>{const data=c.getContext('2d').getImageData(0,0,576,576).data,rgb=new Uint8Array(576*576*3);for(let i=0,j=0;i<data.length;i+=4){rgb[j++]=data[i];rgb[j++]=data[i+1];rgb[j++]=data[i+2];}return Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256',rgb))).map(x=>x.toString(16).padStart(2,'0')).join('');})));
  for(let i=0;i<2;i++){const e=values.entries[i],check=validation.checks.find(c=>c.variant===e.variant);assert.equal(hashes[i],check.frame_rgb_sha256[e.indices[36]],e.variant);}
  await freeze(95);if(!['ATTENTE','SILENCE'].includes(scenario))assert((await page.evaluate(()=>protectionReview.states())).every(s=>!s.icon_visible&&s.turn===5));
  checks.push({scenario,direction:dir,pixel_equality:true});
 }
 await page.goto('file:///'+file.replaceAll('\\','/'));await ready();await freeze(36);await page.screenshot({path:path.join(review,'_browser-proof.png')});
 await page.click('#manual');await page.waitForFunction(()=>[...document.querySelectorAll('canvas')].every(c=>+c.dataset.frame<18));assert((await page.evaluate(()=>protectionReview.states())).every(s=>s.preview));
 await page.click('#cast');await page.waitForTimeout(450);assert((await page.evaluate(()=>protectionReview.states())).every(s=>!s.preview&&s.icon_visible));
 await page.click('#hit');await page.waitForTimeout(1100);const before=await page.evaluate(()=>protectionReview.states());await page.waitForTimeout(800);assert.deepEqual(await page.evaluate(()=>protectionReview.states()),before);
 await page.click('#turn');await page.waitForFunction(()=>protectionReview.states().every(s=>s.turn===1));
 for(let i=0;i<4;i++){await page.click('#turn');await page.waitForTimeout(50);}
 assert((await page.evaluate(()=>protectionReview.states())).every(s=>!s.icon_visible&&s.turn===5));
 await page.click('#hit');await page.waitForTimeout(100);assert((await page.evaluate(()=>protectionReview.states())).every(s=>s.damage===20&&s.reflected===0));
 await page.uncheck('#native');assert.equal(await page.locator('#pair.native').count(),0);await page.check('#native');assert.equal(await page.locator('#pair.native').count(),1);
 assert.deepEqual(errors,[]);fs.writeFileSync(path.join(review,'browser-verification.json'),JSON.stringify({browser:'Edge Chromium',schemes:['file: manual controls and display','localhost: pixel equality'],checks,views:16,manual_idle_stable:true,expiry:true,pixel_equality:true,errors},null,2));console.log('16 native views, manual controls, timers and browser pixel equality: passed');
}finally{await browser.close();await new Promise(resolve=>server.close(resolve));}}
main().catch(e=>{console.error(e);process.exitCode=1;});
