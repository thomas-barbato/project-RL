// SM04 review: paired native captures, independent visual time and turn state.
const manifest=MANIFEST;
let ready=false,elapsed=0,last=0,paused=false,launched=false,turn=0,autoPlay=false,token=0;
let main=new Map(),idle=new Map(),hoverFrames=new Map(),hover=[false,false];
const type=document.getElementById('type'),scenario=document.getElementById('scenario');
const status=document.getElementById('status'),cast=document.getElementById('cast'),next=document.getElementById('next'),expire=document.getElementById('expire');
function entries(){return manifest.filter(e=>e.type===type.value&&e.case===scenario.value);}
function pane(i){return entries().find(e=>e.profile===(i===0?'R1':'R4'));}
function frame(e){const n=Math.floor(elapsed*24/1000);if(!launched)return 3+Math.floor((elapsed%600)*12/600);if(turn===0)return 17+Math.min(n,22);const base=turn===16?176:e.action_frames[turn-1];return base+Math.min(n,turn===16?15:23);}
function ui(){cast.disabled=!ready||launched;next.disabled=!ready||!launched||turn>=4||elapsed<850;expire.disabled=!ready||!launched||turn<3||turn===16;
next.textContent=['Armer la mine','Approcher l’ennemi','Déclencher la mine','Tour suivant','Deux applications montrées'][Math.min(turn,4)];
status.textContent=!ready?'Chargement…':!launched?'Aperçu avant pose : 3×3 à gauche, 9×9 à droite, en terrain ouvert':entries().map(e=>{const t=e.timeline[frame(e)];return e.title+' : '+e.area.length+' cases · '+t.zone_remaining+' tours · '+t.zone_applications+'/13 applications';}).join(' · ');
}
function image(src){return new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=()=>reject(Error(src));im.src=src;});}
function reset(){autoPlay=false;paused=false;launched=false;turn=0;elapsed=0;last=0;hover=[false,false];document.getElementById('pause').textContent='Pause';ui();}
function demo(){if(!ready)return;reset();autoPlay=true;}
async function load(){ready=false;ui();const n=++token,cache=new Map();const read=src=>{if(!cache.has(src))cache.set(src,image(src));return cache.get(src);};const data=await Promise.all(entries().map(async e=>{const prefix='frames/'+e.variant+'/';return {key:e.variant,main:await Promise.all(e.image_indices.map(i=>read(prefix+String(i).padStart(3,'0')+'.png'))),idle:await Promise.all(e.idle_images.map(async s=>[s.step,await Promise.all(s.image_indices.map(i=>read(prefix+'idle-'+s.step+'-'+String(i).padStart(3,'0')+'.png')))])),hover:await Promise.all(e.hover_indices.map(i=>read(prefix+'hover-'+String(i).padStart(3,'0')+'.png')))};}));if(n!==token)return;
main=new Map(data.map(e=>[e.key,e.main]));idle=new Map(data.map(e=>[e.key,new Map(e.idle)]));hoverFrames=new Map(data.map(e=>[e.key,e.hover]));ready=true;demo();}
function advance(){if(turn>=4)return;turn++;elapsed=0;last=0;hover=[false,false];}
function tick(now){if(last&&ready&&!paused&&!document.hidden)elapsed+=Math.min(1000/24,Math.max(0,now-last));last=now;
if(ready){if(autoPlay){if(!launched&&elapsed>=600){launched=true;elapsed=0;}else if(launched&&elapsed>=1000){if(turn>=4)autoPlay=false;else advance();}}
ui();for(const [i,canvas]of [...document.querySelectorAll('canvas')].entries()){const e=pane(i),f=frame(e);let im=main.get(e.variant)[f],source='main';
if(hover[i]&&turn===1){im=hoverFrames.get(e.variant)[Math.floor(elapsed*12/1000)%24];source='hover';}
else if(launched&&elapsed>=1000&&idle.get(e.variant).has(turn)){im=idle.get(e.variant).get(turn)[Math.floor((elapsed-1000)*12/1000)%24];source='idle';}
canvas.getContext('2d').drawImage(im,32,88,576,576,0,0,576,576);canvas.dataset.frame=f;canvas.dataset.source=source;canvas.dataset.variant=e.variant;}}
requestAnimationFrame(tick);}
type.onchange=scenario.onchange=()=>load().catch(e=>status.textContent='Chargement impossible : '+e);
document.getElementById('demo').onclick=demo;document.getElementById('reset').onclick=reset;
cast.onclick=()=>{if(cast.disabled)return;autoPlay=false;launched=true;turn=0;elapsed=0;last=0;};
next.onclick=()=>{if(next.disabled)return;autoPlay=false;advance();};
expire.onclick=()=>{if(expire.disabled)return;autoPlay=false;turn=16;elapsed=0;last=0;hover=[false,false];};
document.getElementById('pause').onclick=e=>{paused=!paused;last=0;e.target.textContent=paused?'Lecture':'Pause';};
document.getElementById('native-view').onchange=e=>document.body.classList.toggle('native-view',e.target.checked);
for(const [i,canvas]of [...document.querySelectorAll('canvas')].entries()){
canvas.onmousemove=e=>{const r=canvas.getBoundingClientRect(),x=(e.clientX-r.left)*576/r.width,y=(e.clientY-r.top)*576/r.height;hover[i]=launched&&turn===1&&Math.abs(x-288)<42&&y>272&&y<344;if(hover[i])autoPlay=false;};
canvas.onmouseleave=()=>hover[i]=false;}
document.addEventListener('visibilitychange',()=>last=0);
load().catch(e=>status.textContent='Chargement impossible : '+e);requestAnimationFrame(tick);
