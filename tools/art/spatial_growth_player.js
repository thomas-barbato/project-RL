/* The native laboratory exports parity fixtures for this browser mirror. */
const Spatial = (() => {
  const walk=(map,x,y)=>map[y]?.[x]==='.';
  function los(map,from,to) {
    let [x,y]=from; if(x===to[0]&&y===to[1])return true;
    const dx=Math.abs(to[0]-x),sx=Math.sign(to[0]-x),dy=-Math.abs(to[1]-y),sy=Math.sign(to[1]-y);
    let err=dx+dy;
    while(true){const px=x,py=y,e=err*2;if(e>=dy){err+=dy;x+=sx;}if(e<=dx){err+=dx;y+=sy;}
      if(x!==px&&y!==py&&!walk(map,x,py)&&!walk(map,px,y))return false;
      if(x===to[0]&&y===to[1])return true;if(!walk(map,x,y))return false;
    }
  }
  const within=(a,b,r)=>(a[0]-b[0])**2+(a[1]-b[1])**2<=r*r;
  function visible(map,player,radius){const out=[];for(let x=0;x<map[0].length;x++)for(let y=0;y<map.length;y++)if(within(player,[x,y],radius)&&los(map,player,[x,y]))out.push([x,y]);return out;}
  function cells(map,origin,shape,dir){
    if(!walk(map,...origin))return [];
    const out=[];
    if(shape.kind==='line'){
      for(let side=-Math.floor(shape.width/2);side<=Math.floor(shape.width/2);side++)for(let f=0;f<shape.length;f++){
        const p=[origin[0]+f*dir[0]-side*dir[1],origin[1]+f*dir[1]+side*dir[0]];
        if(!walk(map,...p))break;out.push(p);
      }return out.sort((a,b)=>a[0]-b[0]||a[1]-b[1]);
    }
    for(let x=0;x<map[0].length;x++)for(let y=0;y<map.length;y++){
      const dx=x-origin[0],dy=y-origin[1],f=dx*dir[0]+dy*dir[1],side=-dx*dir[1]+dy*dir[0];
      let inside=false;
      if(shape.kind==='point')inside=dx===0&&dy===0;
      if(shape.kind==='square')inside=Math.max(Math.abs(dx),Math.abs(dy))<=shape.radius;
      if(shape.kind==='disc')inside=dx*dx+dy*dy<=shape.radius*shape.radius;
      if(shape.kind==='diamond')inside=Math.abs(dx)+Math.abs(dy)<=shape.radius;
      if(shape.kind==='ring')inside=Math.max(Math.abs(dx),Math.abs(dy))===shape.radius;
      if(shape.kind==='wall')inside=f===0&&Math.abs(side)<=Math.floor(shape.length/2);
      if(shape.kind==='cone')inside=f>=1&&f<=shape.length&&Math.abs(side)<=f;
      if(inside&&walk(map,x,y)&&los(map,origin,[x,y]))out.push([x,y]);
    }return out;
  }
  return {walk,los,within,visible,cells};
})();
if(typeof module!=='undefined')module.exports=Spatial;
if(typeof document!=='undefined'){
  const $=id=>document.getElementById(id), canvas=$('world'),ctx=canvas.getContext('2d'),mini=$('mini'),mc=mini.getContext('2d');
  const state={id:'A02',learned:10,bonus:3,vision:12,case:0,dir:0,player:[12,12],target:[20,12],confirmed:false,tests:0};
  const maps=DATA.maps.map(m=>m.split('\n')), dirs=[[1,0],[-1,0],[0,-1],[0,1]],images={};
  const key=p=>p.join(','),set=c=>new Set(c.map(key));
  function curve(p,rank){const n=Math.max(0,rank-1);let v=p.kind==='constant'?p.value:p.kind==='linear'?p.base+n*p.gain_per_rank:p.base+Math.floor((n+(p.rank_offset||0))/p.every_ranks)*p.gain_per_step;return Math.floor(Math.min(p.maximum??v,Math.max(p.minimum??v,v)));}
  function skill(){return DATA.skills.find(s=>s.id===state.id);}
  function spatial(rank){const s=skill(),p=s.parameters,v=k=>p[k]?curve(p[k],rank):0;
    let range=0;for(const k of ['range_cells','base_throw_range_cells','placement_range_cells','center_range_cells','command_range_cells','maximum_drone_distance_cells','maximum_target_distance_cells','cone_range_cells','maximum_wave_travel_cells','bolt_range_cells','movement_cells','leap_cells'])if(p[k]){range=v(k);break;}
    if(s.id==='TR64')range=0;
    if(!range&&p.weapon_range_bonus_cells){range=6+v('weapon_range_bonus_cells');if(p.range_skill_limit_cells)range=Math.min(range,v('range_skill_limit_cells'));}
    let shape={kind:'point'};
    if(['FR02','FR05','FE05'].includes(s.id))shape={kind:'disc',radius:v('radius_cells')};
    else if(s.id==='HL03')shape={kind:'disc',radius:v('aim_zone_radius_cells')};
    else if(['FR06','HY07'].includes(s.id))shape={kind:'diamond',radius:v('radius_cells')};
    else if(s.id==='HL02')shape={kind:'diamond',radius:v('stun_radius_cells')};
    else if(s.id==='OC04')shape={kind:'ring',radius:v('ritual_radius_cells')};
    else {
      for(const k of ['radius_cells','aim_zone_radius_cells','stun_radius_cells','pulse_radius_cells'])if(p[k]){shape={kind:'square',radius:v(k)};break;}
      if(p.zone_width_cells)shape={kind:'square',radius:Math.floor(v('zone_width_cells')/2)};
      if(p.maximum_wall_length_cells)shape={kind:'wall',length:v('maximum_wall_length_cells')};
      if(p.cone_range_cells||p.cone_angle_degrees)shape={kind:'cone',length:range};
      if(p.wave_width_cells)shape={kind:'line',length:v('maximum_wave_travel_cells'),width:v('wave_width_cells')};
    }return {range,shape};
  }
  function current(){const g=spatial(state.learned+state.bonus),dir=dirs[state.dir],map=maps[state.case];
    const origin=g.shape.kind==='line'?[state.player[0]+dir[0],state.player[1]+dir[1]]:
      (g.shape.kind==='cone'||!g.range)?state.player:state.target;
    const visible=Spatial.visible(map,state.player,state.vision),vs=set(visible);
    const valid=Spatial.walk(map,...origin)&&Spatial.within(state.player,origin,g.range)&&vs.has(key(origin))&&Spatial.los(map,state.player,origin);
    return {...g,map,origin,visible,vs,valid,cells:Spatial.cells(map,origin,g.shape,dir)};
  }
  function shapeText(s){if(s.kind==='square')return `Carré de ${s.radius*2+1} × ${s.radius*2+1} cases`;
    if(s.kind==='disc')return `Cercle de rayon ${s.radius} cases`;
    if(s.kind==='diamond')return `Zone de rayon ${s.radius} cases`;
    if(s.kind==='ring')return `Cercle rituel · rayon ${s.radius} cases`;
    if(s.kind==='wall')return `Mur de ${s.length} cases`;
    if(s.kind==='line')return `Bande de ${s.width} cases · trajet ${s.length} cases`;
    if(s.kind==='cone')return `Cône de ${s.length} cases`;
    return 'Impact sur la cible';
  }
  function damageRetention(shape,origin,p){
    if(!['square','disc','diamond'].includes(shape.kind))return null;
    const dx=Math.abs(p[0]-origin[0]),dy=Math.abs(p[1]-origin[1]),r=shape.radius;
    if(!r)return dx===0&&dy===0?100:null;
    if(shape.kind==='disc'){
      const squared=dx*dx+dy*dy;if(squared>r*r)return null;
      return 100-Math.ceil(Math.ceil(Math.sqrt(10000*squared))/(2*r));
    }
    const d=shape.kind==='diamond'?dx+dy:Math.max(dx,dy);
    return d>r?null:100-Math.ceil(100*d/(2*r));
  }
  window.centredDamageRetention=damageRetention;
  function describe(){const s=skill(),g=spatial(state.learned+state.bonus);$('title').textContent=s.title;$('rp').textContent=s.roleplay;
    $('effective').textContent=`Rang effectif ${state.learned+state.bonus}`;
    $('present').replaceChildren();
    for(const text of [`Portée : ${g.range} cases`,shapeText(g.shape)]){const p=document.createElement('p');p.textContent=text;$('present').append(p);}
    if(s.parameters.weapon_range_bonus_cells){const p=document.createElement('p');p.textContent=`Arme d'exemple : portée 6 cases + ${curve(s.parameters.weapon_range_bonus_cells,state.learned+state.bonus)} cases de compétence.`;$('present').append(p);}
    if(s.centred_damage){const p=document.createElement('p');p.textContent='Dégâts directs : 100 % au centre, puis baisse progressive jusqu’à 50 % sur les cases les plus éloignées.';$('present').append(p);}
    $('next').replaceChildren();
    $('learned').max=s.learned_cap;
    if(state.learned>=s.learned_cap){$('nexttitle').textContent='Plafond appris atteint';$('next').textContent='Les bonus d’équipement continuent à augmenter le rang effectif.';}
    else {$('nexttitle').textContent='Prochain rang achetable';const next=spatial(state.learned+state.bonus+1);let any=false;
      for(const [changed,text] of [[next.range!==g.range,`Portée : ${next.range} cases`],[JSON.stringify(next.shape)!==JSON.stringify(g.shape),shapeText(next.shape)]])if(changed){const p=document.createElement('p');p.textContent=text;$('next').append(p);any=true;}
      if(!any)$('next').textContent='Aucun changement de portée ou de surface à ce rang.';
    }
    $('learned-value').textContent=state.learned;$('bonus-value').textContent=state.bonus;
  }
  function draw(time){ctx.fillStyle='#080d12';ctx.fillRect(0,0,canvas.width,canvas.height);const c=current(),area=set(c.cells),pulse=.16+.13*Math.abs(Math.sin(time/1000*6));
    for(let y=0;y<c.map.length;y++)for(let x=0;x<c.map[0].length;x++){
      if(!c.vs.has(`${x},${y}`))continue;
      const wall=c.map[y][x]!=='.';
      ctx.drawImage(images[wall?'wall':`floor-${x%4}-${y%4}`],x*64,y*64,64,64);
      ctx.strokeStyle='rgba(38,46,49,.75)';ctx.lineWidth=.7;ctx.strokeRect(x*64+.35,y*64+.35,63.3,63.3);
      if(!state.confirmed&&area.has(`${x},${y}`)&&c.vs.has(key(c.origin))){ctx.fillStyle=c.valid?`rgba(10,189,240,${pulse})`:`rgba(245,66,31,${pulse})`;ctx.fillRect(x*64+2,y*64+2,60,60);
        ctx.lineWidth=2;ctx.strokeStyle=c.valid?'rgba(10,189,240,.85)':'rgba(245,66,31,.85)';ctx.strokeRect(x*64+2,y*64+2,60,60);}
      if(!state.confirmed&&skill().centred_damage&&area.has(`${x},${y}`)&&c.vs.has(key(c.origin))){
        const value=damageRetention(c.shape,c.origin,[x,y]);if(value!==null){ctx.font='bold 14px system-ui';ctx.textAlign='center';ctx.lineWidth=3;ctx.strokeStyle='#10181e';ctx.fillStyle='#e3edf2';ctx.strokeText(value+'%',x*64+32,y*64+57);ctx.fillText(value+'%',x*64+32,y*64+57);ctx.textAlign='start';}
      }
    }
    for(const [p,name] of [[state.player,'player'],[[17,12],'enemy'],[[20,10],'enemy'],[[23,12],'enemy']])if(c.vs.has(key(p))&&Spatial.walk(c.map,...p))ctx.drawImage(images[name],p[0]*64,p[1]*64,64,64);
    ctx.lineWidth=3;ctx.strokeStyle=c.valid?'#ffd844':'#ff3a32';ctx.strokeRect(c.origin[0]*64+5,c.origin[1]*64+5,54,54);
    mc.fillStyle='#080d12';mc.fillRect(0,0,mini.width,mini.height);
    for(const p of c.visible){mc.fillStyle=Spatial.walk(c.map,...p)?'#66747b':'#adbac1';mc.fillRect(p[0]*8,p[1]*8,8,8);}
    mc.fillStyle='#0bbbea';for(const p of c.cells)if(c.vs.has(key(p)))mc.fillRect(p[0]*8+2,p[1]*8+2,4,4);
    mc.fillStyle='#ffffff';mc.fillRect(state.player[0]*8,state.player[1]*8,8,8);mc.fillStyle='#ffd844';mc.fillRect(c.origin[0]*8,c.origin[1]*8,8,8);
    $('valid').textContent=c.valid?`${c.cells.filter(p=>c.vs.has(key(p))).length} cases visibles concernées · cible accessible`:'Cible hors portée, masquée ou bloquée';$('valid').className=c.valid?'ok':'bad';$('test').disabled=!c.valid;
    $('player-zone').textContent=area.has(key(state.player))?'Le joueur est dans la zone et reste protégé de son propre sort.':'Le joueur est hors de la zone.';
    canvas.dataset.valid=String(c.valid);canvas.dataset.visible=String(c.visible.length);canvas.dataset.cells=String(c.cells.length);
    requestAnimationFrame(draw);
  }
  function change(fn){fn();state.confirmed=false;describe();}
  $('skill').innerHTML=DATA.skills.map(s=>`<option value="${s.id}">${s.id} · ${s.title}</option>`).join('');$('skill').value=state.id;
  $('skill').onchange=e=>change(()=>{state.id=e.target.value;state.learned=Math.min(state.learned,skill().learned_cap);$('learned').value=state.learned;});
  $('learned').oninput=e=>change(()=>state.learned=Number(e.target.value));
  $('bonus').oninput=e=>change(()=>state.bonus=Number(e.target.value));
  $('vision').onchange=e=>change(()=>state.vision=Number(e.target.value));
  $('scene').onchange=e=>change(()=>{state.case=Number(e.target.value);state.player=[12,12];state.target=state.case===0?[20,12]:[16,12];});
  $('direction').onchange=e=>change(()=>state.dir=Number(e.target.value));
  canvas.onclick=e=>{const r=canvas.getBoundingClientRect();change(()=>state.target=[Math.floor((e.clientX-r.left)*canvas.width/r.width/64),Math.floor((e.clientY-r.top)*canvas.height/r.height/64)]);};
  $('test').onclick=()=>{if(current().valid){state.confirmed=true;state.tests++;$('test-count').textContent=`${state.tests} placements testés`;}};
  $('reset').onclick=()=>change(()=>{state.confirmed=false;state.tests=0;state.player=[12,12];state.target=state.case===0?[20,12]:[16,12];$('test-count').textContent='';});
  $('centre').onclick=()=>{$('viewport').scrollLeft=state.player[0]*64-220;$('viewport').scrollTop=state.player[1]*64-320;};
  mini.onclick=e=>{const r=mini.getBoundingClientRect();$('viewport').scrollLeft=(e.clientX-r.left)*mini.width/r.width/8*64-$('viewport').clientWidth/2;$('viewport').scrollTop=(e.clientY-r.top)*mini.height/r.height/8*64-$('viewport').clientHeight/2;};
  window.addEventListener('keydown',e=>{if(['INPUT','SELECT','TEXTAREA'].includes(document.activeElement.tagName))return;const d={ArrowRight:[1,0],ArrowLeft:[-1,0],ArrowDown:[0,1],ArrowUp:[0,-1],d:[1,0],q:[-1,0],s:[0,1],z:[0,-1]}[e.key];if(d){e.preventDefault();const p=[state.player[0]+d[0],state.player[1]+d[1]];if(Spatial.walk(maps[state.case],...p))change(()=>state.player=p);}});
  window.spatialReview={state,current,spatial,curve,change};
  Promise.all(DATA.sprites.map(s=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>{images[s.name]=im;resolve();};im.onerror=reject;im.src=`native/${s.name}.png`;}))).then(()=>{window.ready=true;describe();$('centre').click();requestAnimationFrame(draw);}).catch(e=>{$('valid').textContent='Échec du chargement des images.';console.error(e);});
}
