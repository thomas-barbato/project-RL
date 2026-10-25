(function(){
'use strict';
const $=id=>document.getElementById(id);
const rigNames={biped:'Marche bipède',heavy:'Marche lourde',robe:'Pas sous une robe',quad:'Quadrupède',crawl:'Pattes multiples',serpent:'Ondulation',float:'Flottement',winged:'Appuis et ailes',machine:'Châssis mécanique',roll:'Roulement / chenilles'};
Object.entries(rigNames).forEach(([id,label])=>$('rig').add(new Option(label,id)));
let data,filtered=[],selected,source,poses,currentPack,loadToken=0,page=0,ready=false,situation;
let start=performance.now(),elapsed=0,last=performance.now(),paused=false,automatic=!matchMedia('(prefers-reduced-motion: reduce)').matches,segment=0;
let dx=1,dy=0,origin=[1,1],position=[1,1],moving=true,serial=0;
const requests=new Map(),images=new Map(),cache=new Map();
const before=$('before'),after=$('after'),bc=before.getContext('2d'),ac=after.getContext('2d');
const canvas=(w,h)=>Object.assign(document.createElement('canvas'),{width:w,height:h});
function ask(path){
  if(cache.has(path))return Promise.resolve(cache.get(path));
  if(requests.has(path))return requests.get(path).promise;
  let resolve,reject;const promise=new Promise((a,b)=>{resolve=a;reject=b});requests.set(path,{promise,resolve,reject});
  parent.postMessage({kind:'walk03-asset',path},'*');return promise;
}
async function image(path){
  if(images.has(path)){const v=images.get(path);images.delete(path);images.set(path,v);return v;}
  const url=await ask(path),im=new Image();im.src=url;await im.decode();images.set(path,im);
  while(images.size>38)images.delete(images.keys().next().value);
  return im;
}
function crop(im,index=0,columns=16,side=128){const c=canvas(side,side);c.getContext('2d').drawImage(im,index%columns*side,Math.floor(index/columns)*side,side,side,0,0,side,side);return c;}
function clone(im){const c=canvas(im.width,im.height);c.getContext('2d').drawImage(im,0,0);return c;}
function pixels(im){return im.getContext('2d').getImageData(0,0,im.width,im.height);}
function flip(im){const c=canvas(im.width,im.height),ctx=c.getContext('2d');ctx.translate(im.width,0);ctx.scale(-1,1);ctx.drawImage(im,0,0);return c;}
function overlay(a,b){const c=clone(a);c.getContext('2d').drawImage(b,0,0);return c;}
function combineLeft(body,core,weapon,fingers,bounds){
  const fixed=clone(body),arms=canvas(body.width,body.height),b=pixels(body),c=pixels(core),a=arms.getContext('2d').createImageData(body.width,body.height);
  const low=Math.round(bounds[1]+(bounds[3]-bounds[1])*.14),high=Math.round(bounds[1]+(bounds[3]-bounds[1])*.70);
  for(let y=low;y<high;y++)for(let x=0;x<body.width;x++){
    const k=(y*body.width+x)*4;
    let same=true;for(let j=0;j<4;j++)if(b.data[k+j]!==c.data[k+j]){same=false;break;}
    if(!same){for(let j=0;j<4;j++){a.data[k+j]=b.data[k+j];b.data[k+j]=c.data[k+j];}}
  }
  fixed.getContext('2d').putImageData(b,0,0);arms.getContext('2d').putImageData(a,0,0);
  return overlay(overlay(overlay(fixed,flip(arms)),flip(weapon)),flip(fingers));
}
function bound(im){const a=pixels(im).data;let x0=im.width,y0=im.height,x1=0,y1=0;for(let y=0;y<im.height;y++)for(let x=0;x<im.width;x++)if(a[(y*im.width+x)*4+3]>20){x0=Math.min(x0,x);y0=Math.min(y0,y);x1=Math.max(x1,x+1);y1=Math.max(y1,y+1);}return[x0,y0,x1,y1];}
function retargetHead(body,headBody,mask){
  const out=clone(body),b=pixels(out),head=pixels(headBody),m=pixels(mask);
  // Source-authorized head mask owns the old head. New helmet extents also
  // replace their own pixels; lower body and weapon occlusion stay unchanged.
  const neutral=pixels(crop(currentPack[0],0));
  for(let k=0;k<b.data.length;k+=4){let diff=false;for(let j=0;j<4;j++)if(head.data[k+j]!==neutral.data[k+j]){diff=true;break;}
    if(m.data[k+3]>0||diff){for(let j=0;j<4;j++)b.data[k+j]=head.data[k+j];}}
  out.getContext('2d').putImageData(b,0,0);return out;
}
async function prepare(){
  if(!selected)return;const token=++loadToken;ready=false;$('status').textContent='Préparation des poses…';
  try{
    let base,left;
    if(selected.configs==='class'){
      const paths=selected.pack.concat(selected.core,selected.headmask);const pack=await Promise.all(paths.map(image));
      if(token!==loadToken)return;currentPack=pack;
      const conf=Number($('loadout').value)||0;let body=crop(pack[0],conf),weapon=crop(pack[1],conf),fingers=crop(pack[2],conf);
      const head=Number($('helmet').value);let core=clone(pack[3]);
      if(head){const paintedHead=crop(pack[0],116+head),mask=clone(pack[4]);body=retargetHead(body,paintedHead,mask);core=retargetHead(core,paintedHead,mask);}
      base=overlay(overlay(body,weapon),fingers);
      left=combineLeft(body,core,weapon,fingers,selected.bounds);
    }else if(selected.configs==='legacy'){
      const im=await image(selected.pack[0]);if(token!==loadToken)return;
      base=crop(im,Number($('loadout').value)||0);left=base;
    }else{
      const im=await image(selected.image);if(token!==loadToken)return;
      base=clone(im);left=['quad','crawl','serpent'].includes(selected.rig)?flip(base):base;
    }
    if(token!==loadToken)return;
    source=base;poses={};const rig=$('rig').value,bb=selected.bounds;
    for(const dir of [-1,0,1]){
      const src=pixels(dir===-1?left:base);poses[dir]=[];
      for(let f=0;f<=8;f++){
        const c=canvas(src.width,src.height),ctx=c.getContext('2d');
        ctx.putImageData(new ImageData(WalkReviewRig.warp(src.data,src.width,src.height,rig,dir,(f%8)/8,selected.foot,bb,f<8),src.width,src.height),0,0);
        poses[dir].push(c);
      }
    }
    ready=true;resize();describe();draw();
  }catch(e){if(token!==loadToken)return;$('status').textContent='Impossible de préparer cette source : '+String(e.message||e);$('status').setAttribute('role','alert');}
}
function describe(){
  $('name').textContent=selected.label;$('reviewId').textContent=selected.reviewId;
  $('note').textContent='Essai 2D : les côtés sont déformés à partir de la source, distincts du premier Bastion peint. '+selected.note;
  $('status').textContent=selected.status+' · '+rigNames[$('rig').value]+' · '+$('duration').value+' ms/case · grille 64 px';
}
function select(id){
  const item=data.records.find(e=>e.id===id);if(!item)return;selected=item;$('entry').value=id;$('rig').value=item.rig;
  $('equipment').hidden=!item.configs;$('headField').hidden=item.configs!=='class';
  const prev=$('loadout').options.length?(Number($('loadout').value)||0):3;$('loadout').replaceChildren();
  const configs=item.configs==='class'?data.classConfigs.filter(c=>c.head===0):item.configs==='legacy'?data.legacyConfigs:[];
  configs.forEach((e,i)=>$('loadout').add(new Option(typeof e==='string'?e:e.label,String(i))));
  $('loadout').value=String(Math.min(prev,Math.max(0,configs.length-1)));
  document.querySelectorAll('.tile').forEach(t=>t.setAttribute('aria-pressed',String(t.dataset.id===id)));
  describe();prepare();
}
function match(){
  const category=$('category').value,term=$('search').value.trim().toLocaleLowerCase('fr');
  filtered=data.records.filter(e=>(category==='Tout'||e.category===category)&&[e.id,e.label,e.family,e.rigLabel].join(' ').toLocaleLowerCase('fr').includes(term));
  $('entry').replaceChildren();filtered.forEach(e=>$('entry').add(new Option(e.label+' — '+e.id,e.id)));
  page=0;renderGrid();
  if(filtered.length){select(filtered.some(e=>e.id===selected?.id)?selected.id:!selected&&category==='Classes'?'CLASS-03':filtered[0].id);}
  else{$('entry').add(new Option('Aucun résultat',''));$('status').textContent='Aucun résultat. Le dernier aperçu reste affiché.';}
}
async function renderGrid(){
  const thisPage=page,visible=filtered.slice(page*18,(page+1)*18);$('grid').replaceChildren();
  $('page').textContent=filtered.length?(page+1)+' / '+Math.ceil(filtered.length/18):'0 / 0';
  $('previous').disabled=page===0;$('next').disabled=(page+1)*18>=filtered.length;
  for(const e of visible){
    const b=document.createElement('button');b.type='button';b.className='tile';b.dataset.id=e.id;b.setAttribute('aria-pressed',String(e.id===selected?.id));
    const img=document.createElement('img');img.alt='';const label=document.createElement('span');label.textContent=e.label;const code=document.createElement('span');code.className='code';code.textContent=e.id;
    b.append(img,label,code);b.addEventListener('click',()=>select(e.id));$('grid').append(b);
    ask(e.image).then(url=>{if(thisPage===page)img.src=url;}).catch(()=>{label.textContent=e.label+' (image indisponible)';});
  }
  height();
}
function height(){parent.postMessage({kind:'walk03-height',height:document.documentElement.scrollHeight},'*');}
function resize(){
  const z=Number($('zoom').value),b=selected?.bounds||[32,32,96,96],f=selected?.foot||[64,96];
  const mx=Math.max(1,Math.ceil(Math.max(f[0]-b[0],b[2]-f[0])/64));
  const my=Math.max(1,Math.ceil((f[1]-b[1])/64))+Math.max(0,Math.ceil((b[3]-f[1])/64));
  document.querySelector('.stages').style.gridTemplateColumns=z>1?'1fr':'';
  for(const c of [before,after]){
    const width=Math.max((2*mx+3)*64*z,Math.floor(c.parentElement.clientWidth/64)*64);
    c.width=width;c.height=Math.max(320,(my+3)*64)*z;
  }
  resetMotion();height();
}
function centralOrigin(){const t=64*Number($('zoom').value);return[Math.floor(before.width/t/2),Math.floor(before.height/t/2)];}
function resetMotion(){
  origin=centralOrigin();position=[origin[0]+1,origin[1]];
  elapsed=0;last=performance.now();segment=0;moving=true;dx=1;dy=0;
}
function step(x,y){
  automatic=false;$('auto').setAttribute('aria-pressed','false');
  // Every press starts exactly one visual cell. A new command interrupts the
  // presentation; there is no movement queue and no simulated turn counter.
  origin=centralOrigin();position=[origin[0]+x,origin[1]+y];dx=x;dy=y;elapsed=0;moving=true;serial++;
  if(paused)paused=false;$('pause').textContent='Pause';last=performance.now();draw();
}
function drawFloor(ctx,w,h,tile){
  const floor=$('floor').value,light=floor==='light';ctx.clearRect(0,0,w,h);
  ctx.imageSmoothingEnabled=false;
  for(let y=0;y<h;y+=tile)for(let x=0;x<w;x+=tile){ctx.fillStyle=light?((x/tile+y/tile)%2?'#aaa89a':'#b7b5a5'):((x/tile+y/tile)%2?'#263335':'#2d3b3b');ctx.fillRect(x,y,tile,tile);
    if(floor==='texture'&&situation)ctx.drawImage(situation.floor,x,y,tile,tile);
    ctx.strokeStyle=light?'#8d9085':'#41504f';ctx.lineWidth=1;ctx.strokeRect(x+.5,y+.5,tile,tile);
  }
  ctx.strokeStyle=light?'#645137':'#b08c5d';ctx.lineWidth=2;ctx.strokeRect(position[0]*tile+2,position[1]*tile+2,tile-4,tile-4);
  if(floor==='texture'&&situation){
    const z=tile/64,spots=[[0,0],[w/tile-2,0],[0,h/tile-2]];
    situation.actors.forEach((a,i)=>{const [gx,gy]=spots[i];ctx.drawImage(a.image,Math.round((gx+.5)*tile-a.record.foot[0]*z),Math.round((gy+1)*tile-a.record.foot[1]*z),a.image.width*z,a.image.height*z);});
    ctx.drawImage(situation.item,w-tile*1.5,h-tile*1.5,tile,tile);
  }
}
function drawActor(ctx,im,gx,gy,zoom){
  const tile=64*zoom,foot=selected.foot;
  const x=(gx+.5)*tile-foot[0]*zoom,y=(gy+1)*tile-foot[1]*zoom;
  ctx.fillStyle=$('floor').value==='light'?'#484d4666':'#0c141388';ctx.beginPath();ctx.ellipse((gx+.5)*tile,(gy+1)*tile-2*zoom,Math.min(25,(selected.bounds[2]-selected.bounds[0])*.36)*zoom,3*zoom,0,0,Math.PI*2);ctx.fill();
  ctx.imageSmoothingEnabled=false;ctx.drawImage(im,Math.round(x),Math.round(y),im.width*zoom,im.height*zoom);
}
function draw(){
  if(!ready||!source)return;const zoom=Number($('zoom').value),duration=Number($('duration').value),t=WalkReviewRig.travel(elapsed,duration);
  const dir=dx===0?0:dx<0?-1:1,frame=moving&&t<1?Math.min(7,Math.floor(t*8)):8;
  $('facing').textContent=dx===0?'Face · déplacement vertical':dx<0?'Vers la gauche':'Vers la droite';
  drawFloor(bc,before.width,before.height,64*zoom);drawFloor(ac,after.width,after.height,64*zoom);
  drawActor(bc,source,position[0],position[1],zoom);
  drawActor(ac,poses[dir][frame],origin[0]+dx*t,origin[1]+dy*t,zoom);
  after.dataset.frame=String(frame);after.dataset.direction=String(dir);after.dataset.progress=t.toFixed(3);after.dataset.duration=String(duration);after.dataset.distance=String(Math.max(Math.abs(dx),Math.abs(dy)));
}
function tick(now){
  const delta=Math.min(100,now-last);last=now;
  if(ready&&!paused){elapsed+=delta;const d=Number($('duration').value);
    if(elapsed>=d)moving=false;
    if(automatic&&elapsed>=d+650){const seq=[[1,0],[0,1],[-1,0],[0,-1]];[dx,dy]=seq[segment++%4];
      origin=centralOrigin();position=[origin[0]+dx,origin[1]+dy];elapsed=0;moving=true;}
    draw();}
  requestAnimationFrame(tick);
}
window.addEventListener('message',e=>{
  if(e.source!==parent)return;const m=e.data;if(!m||typeof m!=='object')return;
  if(m.kind==='walk03-data'&&!data){data=m.data;$('coverage').textContent='25 tenues de classe · 113 modèles d’armes · 6 casques · 129 modèles de campagne et leurs 471 variantes · 14 PNJ, drones et formes · 149 entrées historiques.';
    $('auto').setAttribute('aria-pressed',String(automatic));if(!automatic){paused=true;$('pause').textContent='Reprendre';}
    match();resize();requestAnimationFrame(tick);
    (async()=>{const actors=await Promise.all(data.scene.actors.map(async id=>{const record=data.records.find(e=>e.id===id);return{record,image:await image(record.image)};}));
      situation={floor:await image(data.scene.floor),item:await image(data.scene.item),actors};draw();})().catch(e=>{$('status').textContent='Scène indisponible : '+e.message;});
  }
  if(m.kind==='walk03-asset-result'){
    const r=requests.get(m.path);if(!r)return;requests.delete(m.path);
    if(m.error){r.reject(new Error(m.error));return;}cache.set(m.path,m.url);r.resolve(m.url);
  }
  if(m.kind==='walk03-error'){$('status').textContent=m.error;$('status').setAttribute('role','alert');}
});
$('category').addEventListener('change',match);$('search').addEventListener('input',match);$('entry').addEventListener('change',()=>select($('entry').value));
$('previous').addEventListener('click',()=>{page--;renderGrid();});$('next').addEventListener('click',()=>{page++;renderGrid();});
for(const id of ['loadout','helmet','rig'])$(id).addEventListener('change',prepare);
$('duration').addEventListener('change',()=>{resetMotion();describe();draw();});$('floor').addEventListener('change',draw);$('zoom').addEventListener('change',()=>{resize();draw();});
$('auto').addEventListener('click',()=>{automatic=!automatic;$('auto').setAttribute('aria-pressed',String(automatic));if(automatic){paused=false;$('pause').textContent='Pause';resetMotion();}});
$('pause').addEventListener('click',()=>{paused=!paused;$('pause').textContent=paused?'Reprendre':'Pause';last=performance.now();});
$('frame').addEventListener('click',()=>{paused=true;$('pause').textContent='Reprendre';elapsed=Math.min(Number($('duration').value),elapsed+Number($('duration').value)/8);moving=elapsed<Number($('duration').value);draw();});
$('reset').addEventListener('click',()=>{automatic=false;$('auto').setAttribute('aria-pressed','false');paused=true;$('pause').textContent='Reprendre';dx=dy=0;position=origin.slice();elapsed=Number($('duration').value);moving=false;draw();});
document.querySelectorAll('[data-step]').forEach(b=>b.addEventListener('click',()=>step(...b.dataset.step.split(',').map(Number))));
document.addEventListener('keydown',e=>{if(['INPUT','SELECT','TEXTAREA'].includes(document.activeElement.tagName))return;const keys={ArrowLeft:[-1,0],ArrowRight:[1,0],ArrowUp:[0,-1],ArrowDown:[0,1]};if(keys[e.key]){e.preventDefault();step(...keys[e.key]);}});
new ResizeObserver(()=>{resize();draw();}).observe(before.parentElement);
new ResizeObserver(height).observe(document.querySelector('main'));
parent.postMessage({kind:'walk03-ready'},'*');
})();
