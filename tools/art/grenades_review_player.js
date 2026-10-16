const $=id=>document.getElementById(id),canvases=[...document.querySelectorAll('canvas')],contexts=canvases.map(c=>c.getContext('2d'));
let scene=0,direction=2,elapsed=0,manual=0,demo=true,paused=false,last=0,speed=1,ready=false;
let loadingSerial=0;
const images=new Map();
function entries(){return ['SG00','SG03'].map(id=>ENTRIES.find(e=>e.id===id&&e.case===scene&&e.direction===direction));}
function frame(e){if(demo)return Math.min(103,Math.floor(elapsed/1000*24));const start=[0,18,64,88][manual],end=[17,63,87,103][manual];return Math.min(end,start+Math.floor(elapsed/1000*24));}
function img(e,n){const p=`frames/${e.variant}/${String(e.image_indices[n]).padStart(3,'0')}.png`;if(!images.has(p)){const im=new Image();im.src=p;images.set(p,im);}return images.get(p);}
function updateLinks(){const e=entries();for(let i=0;i<2;i++)$('gif'+i).href=`${e[i].variant}-64.gif`;}
function reset(asDemo){elapsed=0;manual=0;demo=asDemo;paused=false;last=0;updateLinks();}
function render(time){if(!last)last=time;const dt=Math.min(100,time-last);last=time;if(!paused)elapsed+=dt*speed;if(demo&&elapsed>4750&&$('auto').checked)elapsed=0;
  if(!ready){last=0;requestAnimationFrame(render);return;}
  const selected=entries();for(let i=0;i<2;i++){const e=selected[i],n=frame(e),im=img(e,n);if(im.complete&&im.naturalWidth){contexts[i].drawImage(im,0,0);canvases[i].dataset.frame=n;canvases[i].dataset.variant=e.variant;}
    const s=e.timeline[n];$('state'+i).textContent=`Tour ${s.turn} · énergie ${s.energy} · recharge ${s.cooldown} · PV joueur ${s.player_hp} · cibles ${s.hp.join(' / ')}`;
  }
  const n=frame(selected[0]),s=selected[0].timeline[n];$('phase').textContent=scene===2?'Trajet bloqué : aucun lancement, coût ou tour consommé.':n<18?'Prévisualisation clignotante':s.projectile?'Grenade en vol':s.explosion?'Impact, explosion et blessures':'Animation terminée · les compteurs attendent une action';
  $('cast').disabled=demo||manual!==0||scene===2;$('next').disabled=demo||manual===0||manual>=3||scene===2;$('pause').textContent=paused?'Reprendre':'Pause';requestAnimationFrame(render);
}
$('scene').onchange=e=>{scene=Number(e.target.value);reset(true);warmSelected();};$('direction').onchange=e=>{direction=Number(e.target.value);reset(true);warmSelected();};
$('native').onchange=e=>$('pair').classList.toggle('native',e.target.checked);
$('replay').onclick=()=>reset(true);$('reset').onclick=()=>reset(false);
$('cast').onclick=()=>{if(!demo&&manual===0&&scene!==2){manual=1;elapsed=0;paused=false;last=0;}};
$('next').onclick=()=>{if(!demo&&manual>0&&manual<3){manual++;elapsed=0;paused=false;last=0;}};
$('pause').onclick=()=>{paused=!paused;last=0;};$('speed').onchange=e=>speed=Number(e.target.value);
window.grenadeReview={entries,frame,getState:()=>entries().map(e=>e.timeline[frame(e)])};
async function warmSelected(){const serial=++loadingSerial;ready=false;const selected=entries();try{await Promise.all(selected.flatMap(e=>[...new Set(e.image_indices)].map(n=>new Promise((resolve,reject)=>{const p=`frames/${e.variant}/${String(n).padStart(3,'0')}.png`;let im=images.get(p);if(im?.complete&&im.naturalWidth){resolve();return;}if(!im){im=new Image();images.set(p,im);im.src=p;}im.addEventListener('load',resolve,{once:true});im.addEventListener('error',reject,{once:true});}))));if(serial!==loadingSerial)return;for(const p of images.keys())if(!selected.some(e=>p.startsWith(`frames/${e.variant}/`)))images.delete(p);ready=true;elapsed=0;last=0;updateLinks();}catch(e){$('phase').textContent='Les images ne se sont pas chargées.';console.error(e);}}
warmSelected();requestAnimationFrame(render);
