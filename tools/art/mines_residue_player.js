// Review player only: native captured frames and manually advanced game turns.
const manifest = MANIFEST;
let ready=false,last=0,elapsed=0,paused=false,launched=false,turn=0,autoPlay=false,token=0,idleToken=0;
let sequences=new Map(),idleSequences=new Map(),idleStep=-1;
const scenario=document.getElementById('scenario'),left=document.getElementById('left-type'),right=document.getElementById('right-type');
const next=document.getElementById('next'),cast=document.getElementById('cast'),demo=document.getElementById('demo'),expire=document.getElementById('expire'),status=document.getElementById('status');
function entries(){return manifest.filter(e=>e.case===scenario.value&&(e.type===left.value||e.type===right.value));}
function pane(index){return manifest.find(e=>e.case===scenario.value&&e.type===(index===0?left.value:right.value));}
function frame(e){const step=Math.floor(elapsed*24/1000);if(!launched)return 3+Math.floor((elapsed%600)*12/600);if(turn===0)return step<27?17+step:44+(step-27)%18;const base=e.action_frames[turn-1];if(turn<3)return step<24?base+step:base+6+(step-24)%18;return base+Math.min(step,23);}
function ui(){demo.disabled=!ready;cast.disabled=!ready||launched;next.disabled=!ready||!launched||turn>=16||elapsed<850;expire.disabled=!ready||!launched||turn<3||turn>=16;
status.textContent=!ready?'Chargement…':!launched?'Prévisualisation de la zone · pose à confirmer':entries().map(e=>{const t=e.timeline[frame(e)];return e.title+' : '+t.zone_remaining+' tours · '+t.zone_applications+'/13 applications · explosions '+t.explosions;}).join(' · ');
if(autoPlay)status.textContent='Démonstration : explosion puis entrée dans la zone · '+status.textContent;
next.textContent=turn===0?'Armer la mine':turn===1?'Faire approcher l’ennemi':turn===2?'Déclencher la mine':turn===3?'Faire entrer un ennemi':'Tour suivant';
}
function image(src){return new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=()=>reject(Error(src));im.src=src;});}
async function loadIdle(){const n=++idleToken,id=turn;idleStep=-1;idleSequences=new Map();const cache=new Map();const data=await Promise.all(entries().filter(e=>e.idle_images.some(s=>s.step===id)).map(async e=>{const s=e.idle_images.find(s=>s.step===id);return[e.id,await Promise.all(s.image_indices.map(i=>{const src='frames/'+e.variant+'/idle-'+id+'-'+String(i).padStart(3,'0')+'.png';if(!cache.has(src))cache.set(src,image(src));return cache.get(src);} ))];}));if(n!==idleToken||id!==turn)return;idleSequences=new Map(data);idleStep=id;}
function reset(){autoPlay=false;paused=false;document.getElementById('pause').textContent='Pause';launched=false;turn=0;elapsed=0;last=0;idleToken++;idleStep=-1;idleSequences=new Map();ui();}
function beginDemo(){if(!ready)return;reset();autoPlay=true;ui();}
function advance(){turn++;elapsed=0;last=0;loadIdle().catch(e=>status.textContent='Chargement impossible : '+e);}
function updateDemo(){if(!autoPlay||paused||document.hidden)return;if(!launched){if(elapsed>=600){launched=true;elapsed=0;last=0;}return;}if(elapsed<1000)return;if(turn>=4){autoPlay=false;return;}advance();}
async function load(){ready=false;idleToken++;idleSequences=new Map();idleStep=-1;ui();const n=++token,cache=new Map();const data=await Promise.all(entries().map(async e=>[e.id,await Promise.all(e.image_indices.map(i=>{const src='frames/'+e.variant+'/'+String(i).padStart(3,'0')+'.png';if(!cache.has(src))cache.set(src,image(src));return cache.get(src);} ))]));if(n!==token)return;sequences=new Map(data);ready=true;beginDemo();}
function tick(now){if(last&&ready&&!paused&&!document.hidden)elapsed+=Math.min(1000/24,Math.max(0,now-last));last=now;if(ready){updateDemo();ui();for(const [index,canvas]of [...document.querySelectorAll('canvas')].entries()){const e=pane(index),f=frame(e);let im=sequences.get(e.id)[f];if(launched&&elapsed>=1000&&idleStep===turn&&idleSequences.has(e.id))im=idleSequences.get(e.id)[Math.floor((elapsed-1000)*12/1000)%24];canvas.getContext('2d').drawImage(im,0,0);canvas.dataset.skill=e.id;canvas.dataset.frame=f;document.getElementById(index===0?'left-title':'right-title').textContent=e.title;}}requestAnimationFrame(tick);}
scenario.onchange=left.onchange=right.onchange=()=>load().catch(e=>status.textContent='Chargement impossible : '+e);
demo.onclick=beginDemo;cast.onclick=()=>{if(cast.disabled)return;autoPlay=false;launched=true;turn=0;elapsed=0;last=0;ui();};
next.onclick=()=>{if(next.disabled)return;autoPlay=false;advance();ui();};
expire.onclick=()=>{if(expire.disabled)return;autoPlay=false;turn=16;elapsed=0;last=0;idleToken++;idleSequences=new Map();idleStep=-1;ui();};
document.getElementById('reset').onclick=reset;document.getElementById('pause').onclick=ev=>{paused=!paused;last=0;ev.target.textContent=paused?'Lecture':'Pause';};
document.getElementById('native-view').onchange=ev=>document.body.classList.toggle('native-view',ev.target.checked);
document.addEventListener('visibilitychange',()=>{last=0;});
load().catch(e=>status.textContent='Chargement impossible : '+e);requestAnimationFrame(tick);
