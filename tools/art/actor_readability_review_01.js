const W=320,H=256;
const presets=[
 ['00','Actuel',0],['01','Contour',1],['02','Éclairage',2],['03','Décor',4],
 ['04','Repères',8],['05','Formes',16],['06','Discret',7],['07','Guidage',11],['08','Tout combiner',31],
 ['09','Ta sélection',26],['10','Gris + animations en couleur',26,true]
];
const descriptions={
 0:'Témoin : les dessins et le fond actuels.',
 1:'01 · Contour d’un pixel. Compare surtout les membres et les armes sur le sol.',
 2:'02 · Observe la tête, les épaules et les bras dans les parties sombres.',
 4:'03 · Seul le fond change : les acteurs gardent exactement leurs pixels actuels.',
 8:'04 · Les arcs aident à localiser le joueur et sa cible sans recolorer leur tenue.',
 16:'05 · Seuls les trois monstres changent de dessin. Compare leur forme et leur séparation du sol.',
 7:'06 · Contour + éclairage + décor : combinaison discrète, avec les dessins actuels.',
 11:'07 · Contour + éclairage + repères : davantage de guidage visuel.',
 31:'08 · Les cinq pistes ensemble. Désactive une option pour voir ce qu’elle apporte.',
 26:'09 · Silhouettes éclaircies + repères au sol + nouvelles formes des trois monstres.'
};
const ui={before:document.querySelector('#before'),after:document.querySelector('#after'),scene:document.querySelector('#scene'),cls:document.querySelector('#class'),rank:document.querySelector('#rank'),strength:document.querySelector('#strength'),gray:document.querySelector('#gray'),status:document.querySelector('#status'),effect:document.querySelector('#effect'),time:document.querySelector('#time'),play:document.querySelector('#play')};
let manifest,images={},rgba={},mask=26,target=2,holding=false,ready=false,renderCount=0,playing=!matchMedia('(prefers-reduced-motion: reduce)').matches,animationTime=.576,lastFrame;
const q=new URLSearchParams(location.search);
const requested=presets.find(p=>p[0]===q.get('preset'));if(requested){mask=requested[2];ui.gray.checked=!!requested[3];}
const asset=(src)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>{images[src]=im;const c=document.createElement('canvas');c.width=im.naturalWidth;c.height=im.naturalHeight;c.getContext('2d').drawImage(im,0,0);rgba[src]=c.getContext('2d').getImageData(0,0,c.width,c.height);resolve(im)};im.onerror=()=>reject(Error('Dessin indisponible : '+src));im.src=src});
const lum=(r,g,b)=>.2126*r+.7152*g+.0722*b;
function mix(out,at,r,g,b,a){if(a<=0)return;out[at]=Math.round(r*a+out[at]*(1-a));out[at+1]=Math.round(g*a+out[at+1]*(1-a));out[at+2]=Math.round(b*a+out[at+2]*(1-a));out[at+3]=255;}
function actorList(effects){const p=manifest.players[Number(ui.cls.value)*5+Number(ui.rank.value)];return [
 {key:'player',name:manifest.classes[Number(ui.cls.value)],src:p.path,foot:p.foot,x:64,y:192,player:true},
 ...manifest.monsters.map((m,i)=>({key:m.id,name:m.label,src:(effects&16)?m.candidate:m.original,foot:m.foot,x:[128,256,192][i],y:[96,160,224][i],player:false,selection:i+1}))
 ];}
function quietBackground(input,amount){const src=input.data,out=new Uint8ClampedArray(src);for(let y=1;y<H-1;y++)for(let x=1;x<W-1;x++){
  const n=(y*W+x)*4;let mean=[0,0,0];for(let yy=-1;yy<=1;yy++)for(let xx=-1;xx<=1;xx++){const k=((y+yy)*W+x+xx)*4;for(let c=0;c<3;c++)mean[c]+=src[k+c]/9;}
  // Reduce fine texture, preserving position and high-contrast structural edges.
  const edge=Math.abs(lum(...mean)-lum(src[n],src[n+1],src[n+2]));const a=amount*(edge>28?.18:.62);
  for(let c=0;c<3;c++)out[n+c]=Math.round(src[n+c]*(1-a)+mean[c]*a);
 }return out;}
function paintActor(out,actor,effects,strength,gray){const im=rgba[actor.src],p=im.data,w=im.width,h=im.height,dx=Math.round(actor.x-actor.foot[0]),dy=Math.round(actor.y-actor.foot[1]);
 if(effects&1)for(let sy=1;sy<h-1;sy++)for(let sx=1;sx<w-1;sx++){
  const a=p[(sy*w+sx)*4+3];if(a>=32)continue;let near=0;for(let yy=-1;yy<=1;yy++)for(let xx=-1;xx<=1;xx++)near=Math.max(near,p[((sy+yy)*w+sx+xx)*4+3]);if(near<96)continue;
  const x=dx+sx,y=dy+sy;if(x<0||x>=W||y<0||y>=H)continue;const n=(y*W+x)*4,l=lum(out[n],out[n+1],out[n+2]);
  const color=l<88?[151,165,170]:[12,18,21];if(gray)color.fill(lum(...color));mix(out,n,...color,(l<88?.57:.8)*strength);
 }
 for(let sy=0;sy<h;sy++)for(let sx=0;sx<w;sx++){
  const i=(sy*w+sx)*4,a=p[i+3]/255;if(a===0)continue;const x=dx+sx,y=dy+sy;if(x<0||x>=W||y<0||y>=H)continue;
  let r=p[i],g=p[i+1],b=p[i+2];if(effects&2){const l=lum(r,g,b),shadow=12*(1-l/255);let rim=0;
   if(sy>0&&sx>0&&(p[((sy-1)*w+sx)*4+3]<96||p[(sy*w+sx-1)*4+3]<96))rim=12;
   const gain=(shadow+rim)*strength;r=Math.min(255,r+gain);g=Math.min(255,g+gain);b=Math.min(255,b+gain);
  }if(gray)r=g=b=lum(r,g,b);mix(out,(y*W+x)*4,r,g,b,a);
 }
}
const tint=(r,g,b,a,gray)=>gray?`rgba(${Math.round(lum(r,g,b))},${Math.round(lum(r,g,b))},${Math.round(lum(r,g,b))},${a})`:`rgba(${r},${g},${b},${a})`;
function ground(ctx,actor,effects,strength,gray){ctx.fillStyle=tint(8,12,14,.27,gray);ctx.fillRect(actor.x-12,actor.y-2,24,2);ctx.fillStyle=tint(8,12,14,.15,gray);ctx.fillRect(actor.x-10,actor.y-3,20,1);
 if((effects&8)&&(actor.player||actor.selection===target)){ctx.strokeStyle=actor.player?tint(80,205,220,.85*strength,false):tint(218,86,73,.9*strength,false);ctx.lineWidth=1;ctx.beginPath();ctx.ellipse(actor.x,actor.y-1,17,5,0,.12*Math.PI,.88*Math.PI);ctx.stroke();ctx.beginPath();ctx.ellipse(actor.x,actor.y-1,17,5,0,1.12*Math.PI,1.88*Math.PI);ctx.stroke();}}
function sheet(ctx,id,frame,side,x,y,size,alpha=1){ctx.save();ctx.imageSmoothingEnabled=true;ctx.globalAlpha=alpha;ctx.drawImage(images['fx/'+id+'.png'],frame*side,0,side,side,x,y,size,size);ctx.restore();}
function attachedFx(ctx,actor,background){if(actor.selection!==target)return;
 if(ui.effect.value==='burn'){
  const clock=animationTime/1.3*(2/3),plumes=background?[[-9,19,.33,.90,0],[10,19,.27,.90,3]]:[[-7,20,.29,.85,1],[7,19,.24,.70,4]];
  for(const [x,y,s,a,phase] of plumes){const frame=(Math.floor(clock*12)+7+phase)%8;sheet(ctx,'flame',frame,128,actor.x+x-64*s,actor.y-32+y-120*s,128*s,a);}
 }else if(ui.effect.value==='gel'){
  const phase=animationTime/1.3*3+7,frame=Math.floor(phase)%6,blend=phase%1,size=72,y=background?192:64,h=background?64:192,alpha=background?.26:.70;
  ctx.save();ctx.imageSmoothingEnabled=true;
  for(const [f,weight] of [[frame,1-blend],[(frame+1)%6,blend]]){ctx.globalAlpha=weight*alpha;ctx.drawImage(images['fx/gel-'+(background?'rear':'front')+'.png'],f*256,y,256,h,actor.x-size/2,actor.y-32-size/2+y*size/256,size,size*h/256);}ctx.restore();
 }else if(ui.effect.value==='fireball'&&background){const age=animationTime/1.3;if(age>=.35&&age<.90){const frame=Math.min(5,Math.floor((age-.35)/.55*6)),size=320*3/7;sheet(ctx,'burst',frame,256,actor.x-132*size/256,actor.y-32-146*size/256,size,.95*Math.max(0,Math.min(1,(.90-age)/.16)));}}
}
function flyingFx(ctx,actors){if(ui.effect.value!=='fireball')return;const age=animationTime/1.3;if(age>=.35)return;
 const from=actors.find(a=>a.player),to=actors.find(a=>a.selection===target),ox=from.x,oy=from.y-40,dx=to.x-ox,dy=to.y-40-oy,progress=age/.35,x=ox+dx*progress,y=oy+dy*progress;
 ctx.save();ctx.translate(x,y);ctx.rotate(Math.atan2(dy,dx));sheet(ctx,'projectile',[0,1,2,3,2,1,0,1][Math.floor(age*8)%8],256,-190*96/256,-132*96/256,96,.9);ctx.restore();
}
function scene(canvas,effects,gray){const ctx=canvas.getContext('2d');ctx.imageSmoothingEnabled=false;const input=rgba[ui.scene.value+'.png'],strength=Number(ui.strength.value)/100;
 const background=(effects&4)?quietBackground(input,strength):new Uint8ClampedArray(input.data);ctx.putImageData(new ImageData(background,W,H),0,0);
 if(gray){const data=ctx.getImageData(0,0,W,H);for(let i=0;i<data.data.length;i+=4){const l=lum(data.data[i],data.data[i+1],data.data[i+2]);data.data[i]=data.data[i+1]=data.data[i+2]=l;}ctx.putImageData(data,0,0);}
 const actors=actorList(effects).sort((a,b)=>a.y-b.y);
 for(const actor of actors){ground(ctx,actor,effects,strength,gray);attachedFx(ctx,actor,true);const pixels=ctx.getImageData(0,0,W,H);paintActor(pixels.data,actor,effects,strength,gray);ctx.putImageData(pixels,0,0);attachedFx(ctx,actor,false);}flyingFx(ctx,actors);
}
function exports(){document.querySelector('#download-before').href=ui.before.toDataURL('image/png');document.querySelector('#download-after').href=ui.after.toDataURL('image/png');}
function render(metadata=true){if(!ready)return;renderCount++;scene(ui.before,0,false);scene(ui.after,holding?0:mask,!holding&&ui.gray.checked);if(!metadata)return;const preset=presets.find(p=>p[2]===mask&&!!p[3]===ui.gray.checked);
 document.querySelector('#afterTitle').textContent=holding?'00 · Actuel (comparaison)':preset?`${preset[0]} · ${preset[1]}`:'Combinaison personnalisée';
 ui.status.textContent=`${ui.scene.options[ui.scene.selectedIndex].text} · ${manifest.classes[Number(ui.cls.value)]} · armure 0${Number(ui.rank.value)+1} · cible : ${manifest.monsters[target-1].label}`;
 document.querySelector('#description').textContent=ui.gray.checked?'10 · Décor et personnages en nuances de gris ; repères, sorts et animations en couleur.':descriptions[mask]||'Combinaison personnalisée : compare avec le témoin à gauche.';
 document.querySelector('#strengthValue').textContent=ui.strength.value+' %';
 document.querySelectorAll('[data-bit]').forEach(el=>el.checked=!!(mask&Number(el.dataset.bit)));
 document.querySelectorAll('[data-preset]').forEach(el=>el.setAttribute('aria-pressed',String(Number(el.dataset.preset)===mask&&(el.dataset.gray==='true')===ui.gray.checked)));
 ui.play.textContent=playing?'Mettre en pause':'Reprendre l’animation';ui.play.setAttribute('aria-pressed',String(playing));exports();
 document.body.dataset.ready='true';document.body.dataset.mask=String(mask);document.body.dataset.renderCount=String(renderCount);
}
function studies(){const black=document.querySelector('#silhouettes').checked;document.querySelectorAll('.study canvas').forEach((canvas,i)=>{
 const ctx=canvas.getContext('2d');ctx.fillStyle='#8b9692';ctx.fillRect(0,0,256,128);ctx.imageSmoothingEnabled=false;
 for(let j=0;j<2;j++){const src=rgba[j?manifest.monsters[i].candidate:manifest.monsters[i].original];const tmp=document.createElement('canvas');tmp.width=128;tmp.height=128;const data=new ImageData(new Uint8ClampedArray(src.data),128,128);
  if(black)for(let p=0;p<data.data.length;p+=4)data.data[p]=data.data[p+1]=data.data[p+2]=9;
  tmp.getContext('2d').putImageData(data,0,0);ctx.drawImage(tmp,j*128,0);
 }});}
document.querySelectorAll('[data-bit]').forEach(el=>el.addEventListener('change',()=>{const bit=Number(el.dataset.bit);mask=el.checked?mask|bit:mask&~bit;render()}));
for(const el of [ui.scene,ui.cls,ui.rank,ui.gray])el.addEventListener('change',render);ui.strength.addEventListener('input',render);
ui.effect.addEventListener('change',()=>{animationTime=.576;ui.time.value='32';render()});
ui.play.onclick=()=>{playing=!playing;lastFrame=undefined;render()};
ui.time.addEventListener('input',()=>{playing=false;animationTime=Number(ui.time.value)/100*1.8;render()});
for(const id of ['download-before','download-after'])document.querySelector('#'+id).addEventListener('click',exports);
function animate(now){if(lastFrame===undefined)lastFrame=now;if(ready&&playing&&ui.effect.value!=='none'&&!document.hidden&&now-lastFrame>=1000/30){animationTime=(animationTime+(now-lastFrame)/1000)%1.8;lastFrame=now;ui.time.value=String(Math.round(animationTime/1.8*100));render(false);}else if(document.hidden||!playing||ui.effect.value==='none')lastFrame=now;requestAnimationFrame(animate);}requestAnimationFrame(animate);
document.querySelector('#zoom').onclick=(e)=>{const yes=document.querySelector('#viewport').classList.toggle('big');e.currentTarget.setAttribute('aria-pressed',String(yes));e.currentTarget.textContent=yes?'Revenir au natif':'Zoom ×2';};
const hold=document.querySelector('#hold');hold.onpointerdown=()=>{holding=true;render()};for(const event of ['pointerup','pointercancel','pointerleave'])hold.addEventListener(event,()=>{if(holding){holding=false;render()}});
addEventListener('keydown',e=>{if(e.code==='Space'&&!['SELECT','INPUT'].includes(e.target.tagName)){e.preventDefault();holding=true;render()}});addEventListener('keyup',e=>{if(e.code==='Space'){holding=false;render()}});addEventListener('blur',()=>{holding=false;render()});
ui.after.addEventListener('click',e=>{if(!ready)return;const rect=ui.after.getBoundingClientRect(),x=(e.clientX-rect.left)*W/rect.width,y=(e.clientY-rect.top)*H/rect.height;for(const actor of actorList(mask).filter(a=>!a.player).sort((a,b)=>b.y-a.y)){
 const src=rgba[actor.src],sx=Math.floor(x-actor.x+actor.foot[0]),sy=Math.floor(y-actor.y+actor.foot[1]);if(sx>=0&&sy>=0&&sx<src.width&&sy<src.height&&src.data[(sy*src.width+sx)*4+3]>32){target=actor.selection;render();break;}
}});
document.querySelector('#silhouettes').onchange=studies;
(async()=>{try{
 manifest=await(await fetch('manifest.json')).json();manifest.classes.forEach((c,i)=>ui.cls.add(new Option(c,i)));ui.cls.value='2';
 const scope=manifest.bestiary_scope;document.querySelector('#bestiary-scope').textContent=`Périmètre : les ${scope.campaign_models} modèles du bestiaire de campagne et leurs ${scope.campaign_appearances_including_material_palettes} apparences, variantes comprises. Les trois nouvelles formes montrées ici sont retenues ; les ${scope.remaining_campaign_models_for_shape_review} autres modèles restent à travailler et à présenter.`;
 for(const [id,name,bits,gray] of [...presets.slice(9),...presets.slice(0,9)]){const b=document.createElement('button');b.type='button';b.textContent=`${id} · ${name}`;b.dataset.preset=String(bits);b.dataset.gray=String(!!gray);b.setAttribute('aria-pressed','false');b.onclick=()=>{mask=bits;ui.gray.checked=!!gray;render()};document.querySelector(Number(id)>=9?'#presets':'#legacy-presets').append(b);}
 await Promise.all([...manifest.backgrounds.map(b=>b.path),...manifest.players.map(p=>p.path),...manifest.monsters.flatMap(m=>[m.original,m.candidate]),...manifest.effects.map(e=>e.path)].map(asset));
 manifest.monsters.forEach(m=>{const card=document.createElement('div');card.className='study';const h=document.createElement('h3');h.textContent=m.id+' · '+m.label;const canvas=document.createElement('canvas');canvas.width=256;canvas.height=128;const s=document.createElement('small');s.textContent='Actuel                  Candidat';card.append(h,canvas,s);document.querySelector('#studies').append(card)});
 ready=true;studies();render();
 }catch(error){ui.status.textContent='Impossible de charger la comparaison : '+error.message;document.body.dataset.error=String(error);}})();
