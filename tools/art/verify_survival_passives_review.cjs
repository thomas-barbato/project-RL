const fs=require('fs'),path=require('path'),assert=require('assert/strict');
const {chromium}=require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06'),review=path.join(base,'skill-animation-review-82');
async function main(){
 const validation=JSON.parse(fs.readFileSync(path.join(review,'validation.json'),'utf8'));
 const http=require('http'),server=http.createServer((req,res)=>{const p=path.resolve(base,'.'+decodeURIComponent(req.url.split('?')[0]));if(!p.startsWith(base+path.sep)){res.writeHead(403);res.end();return;}fs.readFile(p,(err,data)=>{if(err){res.writeHead(404);res.end();return;}res.setHeader('Content-Type',p.endsWith('.webp')?'image/webp':'text/html; charset=utf-8');res.end(data);});});
 await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));const browser=await chromium.launch({channel:'msedge',headless:true});
 try {
  const page=await browser.newPage({viewport:{width:1400,height:1200}}),errors=[],checks=[];
  page.on('pageerror',e=>errors.push(e.message));page.on('requestfailed',r=>errors.push(r.url()));
  await page.goto(`http://127.0.0.1:${server.address().port}/apercu-animations.html`);
  const ready=()=>page.waitForFunction(()=>survivalPassivesReview.isReady());await ready();
  async function freeze(n){await page.evaluate(n=>survivalPassivesReview.freeze(n),n);await page.waitForFunction(n=>[...document.querySelectorAll('canvas')].every((c,i)=>+c.dataset.frame===n&&c.dataset.variant===survivalPassivesReview.entries()[i].variant),n);}
  for(const scenario of ['DIRECT','ATTENTE','PERSISTANT','RECHARGE','SECOND'])for(const dir of scenario==='DIRECT'?[0,1,2,3]:[2]){
   await page.selectOption('#case',scenario);await ready();if(scenario==='DIRECT'){await page.selectOption('#direction',String(dir));await ready();}
   for(const frame of [0,18,27,54,111]){
    await freeze(frame);const values=await page.evaluate(()=>({states:survivalPassivesReview.states(),entries:survivalPassivesReview.entries()}));
    const hashes=await page.evaluate(async()=>Promise.all([...document.querySelectorAll('canvas')].map(async c=>{const data=c.getContext('2d').getImageData(0,0,576,576).data,rgb=new Uint8Array(576*576*3);for(let i=0,j=0;i<data.length;i+=4){rgb[j++]=data[i];rgb[j++]=data[i+1];rgb[j++]=data[i+2];}return Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256',rgb))).map(x=>x.toString(16).padStart(2,'0')).join('');})));
    for(let i=0;i<2;i++){const e=values.entries[i],check=validation.checks.find(c=>c.variant===e.variant);assert.equal(hashes[i],check.frame_rgb_sha256[e.indices[frame]],e.variant);}
    assert(values.states.every(s=>s.energy===100&&s.healed===0&&!s.aura&&!s.invulnerable));
    const [vital,last]=values.states;
    if(frame===18){assert.equal(vital.hp,60);assert.equal(vital.maximum,150);assert.equal(vital.turn,0);assert.equal(last.hp,20);}
    if(frame===27){
     if(['PERSISTANT','RECHARGE'].includes(scenario)){assert.equal(last.hp,0);assert.equal(last.saves,0);}
     else {assert.equal(last.hp,1);assert.equal(last.saves,1);assert.equal(last.cooldown,60);assert.equal(last.damage,19);}
    }
    if(frame===111){
     assert.equal(vital.hp,scenario==='ATTENTE'?60:scenario==='SECOND'?25:30);
     if(scenario==='ATTENTE'){assert.equal(last.hp,1);assert.equal(last.cooldown,60);assert.equal(last.turn,1);}
     if(scenario==='SECOND'){assert.equal(last.hp,0);assert.equal(last.saves,1);assert.equal(last.cooldown,59);}
     if(scenario==='DIRECT'){assert.equal(last.hp,1);assert.equal(last.cooldown,57);}
    }
   }
   checks.push({scenario,direction:dir,pixel_equality:true});
  }
  await page.goto('file:///'+path.join(base,'apercu-animations.html').replaceAll('\\','/'));await ready();await freeze(27);await page.screenshot({path:path.join(review,'_browser-proof.png')});
  await page.locator('#b').screenshot({path:path.join(review,'_survival-proof.png')});
  await page.click('#manual');await page.click('#start');await page.waitForTimeout(750);let values=await page.evaluate(()=>survivalPassivesReview.states());assert.equal(values[0].hp,60);assert.equal(values[0].maximum,150);assert.equal(values[1].hp,20);
  await page.click('#turn');await page.waitForTimeout(1100);const stable=await page.evaluate(()=>survivalPassivesReview.states());await page.waitForTimeout(900);assert.deepEqual(await page.evaluate(()=>survivalPassivesReview.states()),stable);assert.equal(stable[1].cooldown,60);
  await page.click('#turn');await page.waitForFunction(()=>survivalPassivesReview.states()[1].cooldown===59);
  await page.selectOption('#case','SECOND');await ready();await page.click('#manual');await page.click('#start');await page.click('#turn');await page.waitForTimeout(200);assert.equal((await page.evaluate(()=>survivalPassivesReview.states()))[1].hp,1);await page.click('#turn');await page.waitForFunction(()=>survivalPassivesReview.states()[1].hp===0);
  await page.selectOption('#case','ATTENTE');await ready();await page.click('#manual');await page.click('#start');await page.click('#turn');await page.waitForTimeout(1200);assert.equal((await page.evaluate(()=>survivalPassivesReview.states()))[1].cooldown,60);assert(await page.locator('#turn').isDisabled());
  await page.uncheck('#native');assert.equal(await page.locator('#pair.native').count(),0);await page.check('#native');assert.equal(await page.locator('#pair.native').count(),1);
  assert.deepEqual(errors,[]);
  fs.writeFileSync(path.join(review,'browser-verification.json'),JSON.stringify({browser:'Edge Chromium',views:16,schemes:['file: playback and controls','localhost: exact native RGB equality'],checks,manual_idle_stable:true,no_maximum_hp_healing:true,no_persistent_save:true,no_second_save:true,no_energy_cost:true,errors},null,2));
  console.log('16 native views: exact pixels, 4 directions, max HP without healing, last survival, exclusions and manual turns passed.');
 }finally{await browser.close();await new Promise(resolve=>server.close(resolve));}
}
main().catch(e=>{console.error(e);process.exitCode=1;});
