(function(){
'use strict';
const $=id=>document.getElementById(id),before=$('before'),after=$('after'),bc=before.getContext('2d'),ac=after.getContext('2d');
const requests=new Map(),images=new Map();let data,selected,atlas,source,filtered=[],page=0,ready=false,token=0,situation;
let paused=matchMedia('(prefers-reduced-motion: reduce)').matches,automatic=!paused,last=performance.now(),elapsed=0,segment=0,dx=1,dy=0,origin=[2,2],position=[3,2],moving=true;
function ask(path){if(requests.has(path))return requests.get(path).promise;let resolve,reject;const promise=new Promise((a,b)=>{resolve=a;reject=b});requests.set(path,{promise,resolve,reject});parent.postMessage({kind:'walk04-asset',path},'*');return promise;}
async function image(path){if(images.has(path))return images.get(path);const im=new Image();im.src=await ask(path);await im.decode();images.set(path,im);while(images.size>10)images.delete(images.keys().next().value);return im;}
function canvas(w,h){return Object.assign(document.createElement('canvas'),{width:w,height:h});}
function frame(im,column,row,side){const c=canvas(side,side);c.getContext('2d').drawImage(im,column*side,row*side,side,side,0,0,side,side);return c;}
function config(){return Number($('loadout').value)||0;}
function describe(){if(!selected)return;$('name').textContent=selected.label;$('reviewId').textContent='WALK04-'+selected.id;$('rigLabel').textContent=selected.rigLabel;
 $('status').textContent=(selected.painted?'Dessins de trois quarts disponibles':'Dessins en préparation')+' · '+$('duration').value+' ms/case · grille 64 px';
 $('note').textContent=selected.painted?'Deux orientations peintes, bras et arme conservés entiers. La vue verticale reprend la source actuelle de face. '+(selected.painted.derived_from?'Palette dérivée du même dessin de base. ':'')+selected.note:'Cette silhouette attend ses dessins de trois quarts. La source actuelle reste affichée ; aucune déformation latérale ne la remplace.';
}
async function prepare(){const mine=++token;ready=false;describe();try{
 if(selected.painted){atlas=await image(selected.painted.path);if(mine!==token)return;source=frame(atlas,8,config()*3+1,selected.size[0]);}
 else{atlas=null;source=await image(selected.image);if(mine!==token)return;}
 ready=true;resize();draw();}catch(e){if(mine!==token)return;$('status').textContent='Source indisponible : '+e.message;}}
function select(id){const item=data.records.find(e=>e.id===id);if(!item)return;selected=item;$('entry').value=id;const prev=$('loadout').options.length?config():3;$('loadout').replaceChildren();
 const labels=item.painted?.labels||[];labels.forEach((s,i)=>$('loadout').add(new Option(s,String(i))));$('equipment').hidden=labels.length<2;$('loadout').value=String(Math.min(prev,Math.max(0,labels.length-1)));
 document.querySelectorAll('.tile').forEach(b=>b.setAttribute('aria-pressed',String(b.dataset.id===id)));prepare();}
function match(){const cat=$('category').value,q=$('search').value.trim().toLocaleLowerCase('fr');filtered=data.records.filter(e=>(cat==='Tout'||e.category===cat)&&[e.id,e.label,e.family,e.rigLabel].join(' ').toLocaleLowerCase('fr').includes(q));
 $('entry').replaceChildren();filtered.forEach(e=>$('entry').add(new Option((e.painted?'':'[en préparation] ')+e.label+' — '+e.id,e.id)));page=0;renderGrid();
 if(filtered.length)select(filtered.some(e=>e.id===selected?.id)?selected.id:!selected&&cat==='Classes'?'CLASS-03':filtered[0].id);else{$('entry').add(new Option('Aucun résultat',''));$('status').textContent='Aucun résultat. Le dernier aperçu reste affiché.';}}
function renderGrid(){const batch=page,items=filtered.slice(page*18,(page+1)*18);$('grid').replaceChildren();$('page').textContent=filtered.length?(page+1)+' / '+Math.ceil(filtered.length/18):'0 / 0';$('previous').disabled=page===0;$('next').disabled=(page+1)*18>=filtered.length;
 for(const e of items){const b=document.createElement('button');b.className='tile';b.dataset.id=e.id;b.setAttribute('aria-pressed',String(e.id===selected?.id));const c=canvas(128,128);c.setAttribute('aria-hidden','true');const title=document.createElement('span');title.textContent=e.label;const code=document.createElement('span');code.className='code';code.textContent=e.id+(e.painted?' · 3/4':' · en préparation');b.append(c,title,code);b.addEventListener('click',()=>select(e.id));$('grid').append(b);
 image(e.image).then(im=>{if(batch!==page)return;const ctx=c.getContext('2d');ctx.imageSmoothingEnabled=false;const scale=Math.min(1,128/im.width,128/im.height);ctx.drawImage(im,(128-im.width*scale)/2,(128-im.height*scale)/2,im.width*scale,im.height*scale);}).catch(()=>{code.textContent=e.id+' · source indisponible';});}height();}
function height(){parent.postMessage({kind:'walk04-height',height:document.documentElement.scrollHeight},'*');}
function center(){const t=64*Number($('zoom').value);return[Math.floor(before.width/t/2),Math.floor(before.height/t/2)];}
function resetMotion(){origin=center();position=[origin[0]+1,origin[1]];dx=1;dy=0;elapsed=0;segment=0;moving=true;last=performance.now();}
function resize(){const z=Number($('zoom').value),side=selected?.size[0]||128;document.querySelector('.stages').style.gridTemplateColumns=z>1?'1fr':'';
 for(const c of [before,after]){c.width=Math.max((Math.ceil(side/64)+3)*64*z,Math.floor(c.parentElement.clientWidth/64)*64);c.height=Math.max(256,(Math.ceil(side/64)+2)*64)*z;c.parentElement.scrollLeft=Math.max(0,(c.width-c.parentElement.clientWidth)/2);}resetMotion();height();}
function step(x,y){automatic=false;$('auto').setAttribute('aria-pressed','false');origin=center();position=[origin[0]+x,origin[1]+y];dx=x;dy=y;elapsed=0;moving=true;paused=false;$('pause').textContent='Pause';last=performance.now();draw();}
function floor(ctx,c,t){const type=$('floor').value,light=type==='light';ctx.imageSmoothingEnabled=false;ctx.clearRect(0,0,c.width,c.height);
 for(let y=0;y<c.height;y+=t)for(let x=0;x<c.width;x+=t){ctx.fillStyle=light?((x/t+y/t)%2?'#aaa89a':'#b7b5a5'):((x/t+y/t)%2?'#263335':'#2d3b3b');ctx.fillRect(x,y,t,t);if(type==='texture'&&situation)ctx.drawImage(situation.floor,x,y,t,t);ctx.strokeStyle=light?'#8d9085':'#41504f';ctx.lineWidth=1;ctx.strokeRect(x+.5,y+.5,t,t);}
 ctx.strokeStyle=light?'#645137':'#b08c5d';ctx.lineWidth=2;ctx.strokeRect(position[0]*t+2,position[1]*t+2,t-4,t-4);
 if(type==='texture'&&situation){const z=t/64,spots=[[0,0],[c.width/t-2,0],[0,c.height/t-2]];situation.actors.forEach((a,i)=>{const [gx,gy]=spots[i];ctx.drawImage(a.image,Math.round((gx+.5)*t-a.record.foot[0]*z),Math.round((gy+1)*t-a.record.foot[1]*z),a.image.width*z,a.image.height*z);});ctx.drawImage(situation.item,c.width-t*1.5,c.height-t*1.5,t,t);}}
function actor(ctx,im,gx,gy,z,col,row){const t=64*z,f=selected.foot,side=selected.size[0],x=Math.round((gx+.5)*t-f[0]*z),y=Math.round((gy+1)*t-f[1]*z);ctx.imageSmoothingEnabled=false;
 ctx.fillStyle=$('floor').value==='light'?'#484d4666':'#0c141388';ctx.beginPath();ctx.ellipse((gx+.5)*t,(gy+1)*t-2*z,Math.min(25,(selected.bounds[2]-selected.bounds[0])*.36)*z,3*z,0,0,Math.PI*2);ctx.fill();
 if(col===undefined)ctx.drawImage(im,x,y,im.width*z,im.height*z);else ctx.drawImage(im,col*side,row*side,side,side,x,y,side*z,side*z);}
function draw(){if(!ready||!source)return;const z=Number($('zoom').value),d=Number($('duration').value),t=Math.max(0,Math.min(1,elapsed/d)),direction=dx===0?1:dx<0?0:2,f=moving&&t<1?Math.min(7,Math.floor(t*8)):8;
 $('facing').textContent=dx===0?'Face · haut / bas':dx<0?'Trois quarts gauche':'Trois quarts droit';floor(bc,before,64*z);floor(ac,after,64*z);actor(bc,source,position[0],position[1],z);
 if(atlas)actor(ac,atlas,origin[0]+dx*t,origin[1]+dy*t,z,f,config()*3+direction);else actor(ac,source,position[0],position[1],z);
 after.dataset.frame=String(f);after.dataset.direction=String(direction-1);after.dataset.progress=t.toFixed(3);after.dataset.duration=String(d);after.dataset.distance=String(Math.max(Math.abs(dx),Math.abs(dy)));after.dataset.id=selected.id;after.dataset.ready=String(!!selected.painted);}
function tick(now){const dt=Math.min(100,now-last);last=now;if(ready&&!paused){elapsed+=dt;const d=Number($('duration').value);if(elapsed>=d)moving=false;if(automatic&&elapsed>=d+650){const seq=[[1,0],[0,1],[-1,0],[0,-1]];[dx,dy]=seq[segment++%4];origin=center();position=[origin[0]+dx,origin[1]+dy];elapsed=0;moving=true;}draw();}requestAnimationFrame(tick);}
window.addEventListener('message',e=>{if(e.source!==parent)return;const m=e.data;if(!m||typeof m!=='object')return;
 if(m.kind==='walk04-data'&&!data){data=m.data;$('coverage').textContent=data.readyCount+' / '+data.records.length+' entrées avec dessins de trois quarts · 25 tenues de classe · 129 modèles de campagne et leurs 471 variantes · 14 PNJ, drones et formes · archives historiques. Six configurations d’équipement de revue par tenue.';$('auto').setAttribute('aria-pressed',String(automatic));$('pause').textContent=paused?'Reprendre':'Pause';match();requestAnimationFrame(tick);
 (async()=>{situation={floor:await image(data.scene.floor),item:await image(data.scene.item),actors:await Promise.all(data.scene.actors.map(async id=>{const record=data.records.find(r=>r.id===id);return{record,image:await image(record.image)};}))};draw();})().catch(()=>{});}
 if(m.kind==='walk04-asset-result'){const r=requests.get(m.path);if(!r)return;requests.delete(m.path);m.error?r.reject(Error(m.error)):r.resolve(m.url);}
 if(m.kind==='walk04-error')$('status').textContent=m.error;});
$('category').addEventListener('change',match);$('search').addEventListener('input',match);$('entry').addEventListener('change',()=>select($('entry').value));$('loadout').addEventListener('change',prepare);
$('previous').addEventListener('click',()=>{page--;renderGrid();});$('next').addEventListener('click',()=>{page++;renderGrid();});
$('duration').addEventListener('change',()=>{resetMotion();describe();draw();});$('floor').addEventListener('change',draw);$('zoom').addEventListener('change',()=>{resize();draw();});
$('auto').addEventListener('click',()=>{automatic=!automatic;$('auto').setAttribute('aria-pressed',String(automatic));if(automatic){paused=false;$('pause').textContent='Pause';resetMotion();}});
$('pause').addEventListener('click',()=>{paused=!paused;$('pause').textContent=paused?'Reprendre':'Pause';last=performance.now();});
$('frame').addEventListener('click',()=>{automatic=false;$('auto').setAttribute('aria-pressed','false');paused=true;$('pause').textContent='Reprendre';elapsed=Math.min(Number($('duration').value),elapsed+Number($('duration').value)/8);moving=elapsed<Number($('duration').value);draw();});
$('reset').addEventListener('click',()=>{automatic=false;$('auto').setAttribute('aria-pressed','false');paused=true;$('pause').textContent='Reprendre';dx=dy=0;position=origin.slice();elapsed=Number($('duration').value);moving=false;draw();});
document.querySelectorAll('[data-step]').forEach(b=>b.addEventListener('click',()=>step(...b.dataset.step.split(',').map(Number))));document.addEventListener('keydown',e=>{if(['INPUT','SELECT','TEXTAREA'].includes(document.activeElement.tagName))return;const keys={ArrowLeft:[-1,0],ArrowRight:[1,0],ArrowUp:[0,-1],ArrowDown:[0,1]};if(keys[e.key]){e.preventDefault();step(...keys[e.key]);}});
new ResizeObserver(()=>{resize();draw();}).observe(before.parentElement);new ResizeObserver(height).observe(document.querySelector('main'));parent.postMessage({kind:'walk04-ready'},'*');
})();
