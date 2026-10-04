// Authored geometry on the final 32-pixel grid. The inventory atlases are
// references only: no image is sampled, reduced or filtered to make these.
(function (root) {
  'use strict';
  function drawing() {
    const grid=Array.from({length:32},()=>Array(32).fill(null));
    const dot=(x,y,c)=>{if(x>=0&&y>=0&&x<32&&y<32)grid[y][x]=c;};
    const rect=(x,y,w,h,c)=>{for(let j=y;j<y+h;j++)for(let i=x;i<x+w;i++)dot(i,j,c);};
    function poly(points,c) {
      for(let y=0;y<32;y++)for(let x=0;x<32;x++) {
        let inside=false;
        for(let i=0,j=points.length-1;i<points.length;j=i++) {
          const [ax,ay]=points[i],[bx,by]=points[j];
          if((ay>y+.5)!==(by>y+.5)&&x+.5<(bx-ax)*(y+.5-ay)/(by-ay)+ax)inside=!inside;
        }
        if(inside)dot(x,y,c);
      }
    }
    function finish(model) {
      const colors=[null];grid.forEach(row=>row.forEach(c=>{if(!colors.includes(c))colors.push(c);}));
      return {name:model.name,kind:'item',width:32,height:32,colors,pixels:grid.map(row=>row.map(c=>colors.indexOf(c)))};
    }
    return {dot,rect,poly,finish};
  }
  const ink='#292824', dark='#41423b', metal='#787971', pale='#d4c9af', highlight='#eee0c4';
  const olive='#777455', oliveLight='#aba17a', brown='#806044', cyan='#36c9c8', orange='#d17a28';
  const materials={ivory:[pale,highlight],olive:[olive,oliveLight],dark:[dark,metal],brown:[brown,'#bd9067'],steel:[metal,'#b1b0a1']};
  // Every row is a deliberate model profile, in the order of the reference
  // atlas: finish, stock, barrel, body height, magazine, sight, accent.
  const guns1=[
    ['ivory','olive',4,4,'curve','small','cyan'], // Brenek: ivory receiver, olive furniture, black grip.
    ['dark','frame',3,5,'curve','small','orange'],
    ['ivory','olive',4,4,'curve','small','orange'],
    ['dark','frame',5,4,'curve','rail','orange'],
    ['ivory','ivory',5,4,'curve','small','cyan'],
    ['dark','frame',4,4,'curve','small','cyan'],
    ['ivory','frame',2,5,'drum','small','orange'],
    ['ivory','frame',2,6,'grip','small','cells'],
    ['ivory','frame',3,5,'grip','small','window'],
    ['ivory','frame',2,6,'grip','small','window'],
    ['ivory','frame',4,6,'grip','small','ring'],
    ['ivory','frame',3,5,'grip','small','cells'],
    ['ivory','frame',4,5,'grip','small','window'],
    ['ivory','tank',3,5,'tank','small','flame'],
    ['olive','ivory',5,4,'grip','small','needle'],
    ['ivory','tank',1,6,'tank','small','nozzle'],
    ['ivory','olive',2,4,'tank','small','nozzle'],
    ['ivory','frame',2,6,'tank','small','nozzle'],
    ['olive','olive',2,4,'tank','small','nozzle'],
    ['ivory','orange',1,6,'slantTank','small','ports'],
    ['olive','olive',3,4,'tank','small','orange'],
    ['dark','ivory',4,5,'box','small','none'],
    ['dark','olive',2,6,'grip','small','chambers'],
    ['ivory','olive',2,6,'grip','small','straps'],
    ['dark','olive',3,5,'drum','small','orange'],
    ['dark','olive',2,5,'drum','small','band'],
    ['dark','ivory',3,5,'drum','small','none'],
    ['ivory','olive',2,6,'box','small','orange'],
    ['dark','olive',7,4,'belt','rail','yellow'],
    ['ivory','olive',6,4,'box','rail','nozzle'],
    ['ivory','olive',7,4,'belt','rail','none'],
    ['dark','frame',2,6,'belt','small','none']
  ];
  const guns2=[
    ['dark','frame',4,5,'belt','handle','straps'],
    ['dark','frame',5,5,'belt','handle','yellow'],
    ['ivory','frame',4,4,'drum','rail','orange'],
    ['olive','olive',8,3,'grip','scope','cyan'],
    ['brown','brown',7,3,'grip','scope','orange'],
    ['dark','olive',7,3,'grip','scope','cyanLine'],
    ['olive','olive',5,4,'grip','scope','vents'],
    ['ivory','olive',2,5,'grip','scope','coil'],
    ['ivory','frame',6,3,'grip','scope','cyan'],
    ['olive','olive',6,3,'straight','scope','none'],
    ['ivory','ivory',7,3,'curve','scope','cyan'],
    ['ivory','frame',5,4,'straight','scope','vents'],
    ['ivory','olive',5,4,'straight','scope','vents'],
    ['ivory','frame',3,4,'straight','scope','none'],
    ['olive','ivory',8,3,'grip','scope','none'],
    ['dark','tube',1,7,'grip','handle','orange'],
    ['olive','tube',1,7,'grip','rail','orange'],
    ['ivory','tube',1,6,'grip','rail','point'],
    ['olive','tube',2,6,'grip','rail','cyan'],
    ['dark','tube',1,6,'grip','scope','point'],
    ['dark','tube',2,7,'box','handle','yellow'],
    ['ivory','tube',1,7,'grip','rail','point'],
    ['ivory','brown',6,4,'grip','rail','pump'],
    ['ivory','ivory',6,4,'grip','rail','shells'],
    ['ivory','brown',5,4,'drum','rail','orange'],
    ['ivory','ivory',7,3,'grip','rail','pump'],
    ['ivory','dark',3,5,'box','rail','vents'],
    ['ivory','frame',4,4,'straight','small','cyan'],
    ['ivory','frame',6,4,'drum','rail','yellow']
  ];
  function firearm(model) {
    const p=(model.atlas.includes('firearms-1')?guns1:guns2)[model.index];
    if(!p)throw new Error('Missing firearm profile: '+model.id);
    const [finish,stock,barrel,h,mag,sight,accent]=p,d=drawing();
    const [body,lit]=materials[finish], front=29-barrel, top=12-Math.floor(h/2), bottom=top+h;
    const accentColor=accent.includes('cyan')||['cells','window','ring','coil','needle'].includes(accent)?cyan:accent==='yellow'?'#d4ac48':orange;
    // Stock shape is flat in profile. Open stocks have a genuinely open middle.
    if(stock==='tube') {
      d.rect(2,top+1,27,h,ink);d.rect(3,top+2,24,h-2,body);d.rect(3,top+1,24,1,lit);
      d.rect(2,top,2,h+2,metal);d.rect(26,top,3,h+2,metal);
      d.rect(7,top+1,1,h,accentColor);d.rect(23,top+1,1,h,accentColor);
    } else {
      d.poly([[2,top+1],[8,top+1],[9,top+4],[6,bottom+5],[2,bottom+5]],ink);
      if(stock==='frame') {
        d.poly([[3,top+2],[7,top+2],[7,top+4],[3,bottom+3]],metal);
        d.poly([[4,top+3],[6,top+3],[4,bottom+1]],null);
      } else {
        const [s,sl]=materials[stock]||[orange,'#e2a255'];
        d.poly([[3,top+2],[8,top+2],[7,bottom],[3,bottom+3]],s);d.rect(3,top+2,4,1,sl);
      }
      d.rect(8,top,front-7,h+2,ink);d.rect(9,top+1,front-9,h,body);d.rect(9,top+1,front-9,1,lit);
      d.rect(front-1,11,barrel+2,3,ink);d.rect(front,11,barrel,1,metal);d.rect(28,10,2,4,dark);
    }
    d.poly([[9,bottom],[13,bottom],[11,bottom+7],[8,bottom+7]],ink);
    d.poly([[10,bottom+1],[12,bottom+1],[10,bottom+6],[9,bottom+6]],dark);
    d.dot(10,bottom+1,metal);
    // Use the reference's characteristic magazine or fuel vessel.
    if(mag==='curve') {
      d.poly([[16,bottom],[20,bottom],[20,bottom+5],[22,bottom+8],[18,bottom+8],[16,bottom+5]],ink);
      d.poly([[17,bottom+1],[19,bottom+1],[19,bottom+5],[20,bottom+7],[18,bottom+7]],olive);
      d.rect(17,bottom+1,1,4,oliveLight);
    } else if(['straight','box','belt','drum'].includes(mag)) {
      const w=mag==='drum'?7:mag==='box'?5:4, mx=16;
      d.poly([[mx,bottom-1],[mx+w,bottom-1],[mx+w+1,bottom+6],[mx+w-1,bottom+8],[mx,bottom+7]],ink);
      d.rect(mx+1,bottom,w-1,7,mag==='drum'?dark:olive);d.rect(mx+1,bottom,w-1,1,metal);
      if(mag==='belt')for(let y=bottom+1;y<bottom+7;y+=2)d.rect(mx+1,y,3,1,accent==='yellow'?'#d4ac48':oliveLight);
      if(mag==='drum')for(let y=bottom+2;y<bottom+7;y+=2)d.rect(mx+1,y,w-1,1,metal);
    } else if(mag==='tank'||mag==='slantTank') {
      const mx=mag==='slantTank'?14:16;
      d.poly([[mx,bottom],[mx+5,bottom],[mx+6,bottom+7],[mx+4,bottom+9],[mx,bottom+8],[mx-1,bottom+6]],ink);
      d.rect(mx,bottom+1,4,7,orange);d.rect(mx,bottom+1,1,6,'#efad64');d.rect(mx,bottom+7,4,1,pale);
    } else if(mag==='grip'&&model.category==='energy_rifles')d.rect(17,bottom,3,5,dark);
    if(sight==='scope') {d.rect(10,top-4,9,3,ink);d.rect(11,top-3,6,1,metal);d.rect(18,top-3,1,1,accentColor);d.rect(13,top-1,2,1,dark);}
    if(sight==='small') {d.poly([[10,top],[12,top-3],[16,top-3],[18,top],[16,top+1]],ink);d.rect(12,top-2,3,1,metal);d.dot(15,top-2,accentColor);}
    if(sight==='handle'){d.rect(10,top-4,9,4,ink);d.rect(11,top-3,7,1,metal);d.rect(12,top-2,5,1,null);}
    if(sight==='rail'){d.rect(11,top-2,8,2,ink);d.rect(12,top-2,6,1,metal);}
    if(['cells','window','ring','coil'].includes(accent)) {
      const x=accent==='ring'?15:17;
      d.rect(x-1,top+1,7,Math.max(3,h-1),ink);d.rect(x,top+2,5,Math.max(1,h-3),cyan);
      if(accent==='cells') {d.rect(x+1,top+2,1,h-3,ink);d.rect(x+3,top+2,1,h-3,ink);}
      if(accent==='ring'){d.rect(x+1,top+2,3,2,dark);d.dot(x+2,top+1,cyan);}
    } else if(accent==='cyanLine') d.rect(17,top+2,6,1,cyan);
    else if(accent==='vents'||accent==='ports'||accent==='nozzle') {
      const x=accent==='nozzle'?front-4:17;
      d.rect(x,top+1,5,Math.min(3,h-1),accent==='vents'?pale:orange);
      for(let i=0;i<5;i+=2)d.dot(x+i,top+2,ink);
    } else if(accent==='chambers') {d.rect(18,top+1,6,h-1,dark);for(let i=0;i<6;i+=3)d.rect(18+i,top+2,2,2,pale);}
    else if(accent==='straps'){d.rect(17,top+1,1,h,pale);d.rect(23,top+1,1,h,pale);}
    else if(accent==='pump'){d.rect(17,bottom,8,3,dark);d.rect(18,bottom,6,1,metal);}
    else if(accent==='shells'){for(let i=0;i<4;i++)d.rect(18+i*2,top+1,1,3,orange);}
    else if(accent==='flame'){d.rect(7,top-3,3,4,orange);d.rect(10,top-4,3,4,orange);d.dot(29,12,'#ed9e43');}
    else if(accent==='point'){d.poly([[26,top+1],[30,top+3],[26,bottom-1]],finish==='ivory'?pale:olive);}
    else if(accent!=='none')d.rect(15,top+2,3,1,accentColor);
    return d.finish(model);
  }
  // Flat silhouettes with deliberately large blade faces, no surface texture.
  // Explicit profiles distinguish all 40 hand weapons (shape, reach, guard,
  // blade material and handle material), rather than affix recolours.
  const meleeProfiles=[
    ['axe',13,6,'olive','none',0],['axe',11,6,'olive','orange',1],['axe',10,6,'dark','none',2],['axe',11,6,'brown','orange',3],['hook',12,4,'brown','orange',0],['axe',12,5,'brown','cyan',4],['axe',9,4,'olive','none',5],
    ['knife',10,3,'dark','cyan',0],['knife',11,3,'olive','none',0],['knife',10,3,'olive','none',1],['knife',10,3,'brown','none',2],['knife',11,3,'olive','bone',3],['knife',10,3,'dark','orange',4],['knife',11,4,'dark','orange',5],['knife',10,4,'brown','bone',6],['knife',11,2,'dark','orange',7],['knife',11,2,'dark','orange',8],['knife',10,2,'olive','none',9],['knife',12,1,'dark','orange',10],['sword',12,3,'olive','cyan',0],
    ['poleaxe',14,5,'olive','orange',0],['spear',14,3,'olive','orange',0],['spear',13,3,'dark','cyan',1],['hook',14,4,'brown','none',1],['spear',14,2,'olive','none',2],['poleaxe',13,4,'dark','none',1],['spear',13,3,'brown','orange',3],
    ['sword',14,4,'brown','orange',1],['sword',13,4,'dark','orange',2],['sword',12,3,'dark','none',3],['sword',13,3,'olive','orange',4],['sword',11,3,'olive','orange',5],['sword',14,3,'olive','cyan',6],
    ['hammer',13,5,'olive','orange',0],['club',12,4,'brown','none',0],['club',12,4,'brown','orange',1],['hammer',12,5,'olive','cyan',1],['hammer',11,5,'brown','orange',2],['mace',12,4,'brown','none',0],['mace',12,4,'brown','orange',1]
  ];
  function melee(model) {
    const [shape,length,width,handle,accent,variant]=meleeProfiles[model.index],d=drawing();
    const [wood,woodLight]=materials[handle], blade=accent==='bone'?'#c5b68e':pale, edge=accent==='bone'?'#ece0bb':highlight;
    // Geometry is transformed before native rasterisation, not rotated bitmap art.
    const P=(points,c)=>d.poly(points.map(([x,y])=>[16+(x-y)*.7071,16+(x+y)*.7071]),c);
    const R=(x,y,w,h,c)=>P([[x,y],[x+w,y],[x+w,y+h],[x,y+h]],c);
    const color=accent==='cyan'?cyan:orange;
    if(['axe','poleaxe','spear','hook','hammer','club','mace'].includes(shape)) {
      R(-1.5,-length+5,3,length*2-6,ink);R(-.5,-length+5,1,length*2-7,woodLight);
      R(-1,length-5,2,3,wood);
      if(shape==='axe'||shape==='poleaxe') {
        P([[-2,-length+4],[width,-length],[width+2,-length+4],[width+1,-length+10],[2,-length+12],[-2,-length+8]],ink);
        P([[1,-length+4],[width-1,-length+1],[width,-length+4],[width-1,-length+9],[2,-length+10]],blade);
        P([[width-1,-length+1],[width,-length+4],[width-1,-length+9],[width-3,-length+10],[width-2,-length+5]],edge);
        R(-3,-length+4,5,3,dark);R(-2,-length+4,3,1,metal);
        if(variant===2||variant===3)R(2,-length+4,2,2,dark);
        if(variant===4||shape==='poleaxe')P([[-1,-length+3],[0,-length-2],[2,-length+3]],pale);
        if(accent!=='none')R(-1,-length+5,2,2,color);
      } else if(shape==='spear') {
        P([[0,-length-1],[width+1,-length+5],[1,-length+9],[-width,-length+5]],ink);
        P([[0,-length],[width-1,-length+5],[0,-length+8],[-width+1,-length+5]],blade);
        P([[0,-length],[1,-length+6],[0,-length+8],[-width+1,-length+5]],edge);
        if(variant===1)R(0,-length+4,1,3,cyan);
        if(accent!=='none')R(-1,-length+8,2,2,color);
      } else if(shape==='hook') {
        P([[-2,-length+7],[-2,-length+2],[0,-length],[2,-length+3],[width+2,-length+6],[width+2,-length+11],[width,-length+9],[2,-length+7],[1,-length+11]],ink);
        P([[-1,-length+5],[0,-length+1],[1,-length+4],[width+1,-length+6],[width,-length+9],[width-1,-length+7],[1,-length+6]],pale);
      } else if(shape==='hammer') {
        P([[-width,-length+2],[-width+1,-length],[width,-length+1],[width+1,-length+6],[width,-length+8],[-width,-length+6]],ink);
        R(-width+1,-length+2,width*2-1,3,metal);R(-width+1,-length+1,width*2-2,1,pale);
        if(variant===0)P([[-width,-length+2],[-width-3,-length+3],[-width,-length+5]],pale);
        if(accent!=='none')R(-1,-length+2,2,3,color);
      } else if(shape==='club') {
        P([[-1,-length+10],[-width,-length+4],[-width+1,-length],[width-1,-length],[width,-length+4],[1,-length+10]],ink);
        P([[-.5,-length+9],[-width+1,-length+4],[-width+2,-length+1],[width-2,-length+1],[width-1,-length+4]],wood);
        R(-2,-length+1,1,5,woodLight);
        if(variant===1){R(-3,-length+2,6,2,metal);R(-2,-length+6,4,1,pale);}
      } else {
        P([[-width,-length+2],[-2,-length],[2,-length],[width,-length+2],[width,-length+6],[2,-length+8],[-2,-length+8],[-width,-length+6]],ink);
        R(-width+1,-length+2,width*2-2,4,dark);
        if(variant===1){R(-2,-length+1,1,6,pale);R(1,-length+1,1,6,pale);R(-1,-length+2,2,4,orange);}
        else{R(-2,-length+1,4,1,metal);R(-1,-length+3,2,2,pale);}
      }
    } else {
      const join=shape==='knife'?3:6;
      P([[0,-length],[width+1,-length+4],[width,join],[-width,join],[-width,-length+5]],ink);
      P([[0,-length+1],[width-1,-length+4],[width-1,join-1],[-width+1,join-1],[-width+1,-length+5]],blade);
      P([[0,-length+1],[1,-length+5],[1,join-1],[-width+1,join-1],[-width+1,-length+5]],edge);
      if(variant===2&&shape==='sword')P([[width-1,-length+3],[width+2,-length+6],[width,join],[1,join]],pale);
      if(variant===3&&shape==='knife')P([[0,-length+1],[width-1,-length+3],[width-1,join-1],[-width+1,join-1]],'#95614b');
      if(variant===6&&shape==='knife')for(let y=-length+5;y<join;y+=3)R(width-2,y,2,1,ink);
      const guard=shape==='sword'?5:variant===8?5:variant===10?2:3;
      R(-guard,join,guard*2,2,ink);R(-guard+1,join,guard*2-2,1,metal);
      R(-2,join+2,4,length-join-1,ink);R(-1,join+2,2,length-join-2,wood);R(-1,join+2,1,length-join-3,woodLight);
      R(-2,length-1,4,2,metal);
      if(accent!=='none'&&accent!=='bone') {
        if(accent==='cyan'&&shape==='sword')R(0,-length+5,1,length-2,cyan);
        else R(-1,join,2,2,color);
      }
      if(variant===1&&shape==='knife')R(0,-length+6,1,2,dark);
      if(variant===4&&shape==='knife')R(0,-length+3,1,2,orange);
    }
    return d.finish(model);
  }
  const armorMaterials={
    cerbere:[pale,highlight,dark,orange],cognefer:[pale,highlight,brown,brown],
    chrysalide:[olive,oliveLight,dark,cyan],bastion:[pale,highlight,dark,orange],
    fantome:[dark,metal,ink,cyan],mue:[brown,'#b18b65',dark,brown],
    composite:[pale,highlight,dark,metal],patched:[metal,'#b4b2a2',brown,orange],
    ombre:[dark,metal,brown,brown],membranes:[olive,oliveLight,dark,pale],
    peau:[dark,metal,ink,cyan],veille:[metal,'#b4b2a2',dark,orange],
    matelassee:[pale,highlight,dark,orange],fibres:[olive,oliveLight,dark,orange]
  };
  const armorProfiles=[
    ['body','cerbere',0],['body','cognefer',1],['body','chrysalide',2],['body','composite',3],['body','fantome',4],['coat','mue',0],['body','bastion',5],['body','patched',6],['coat','ombre',1],['coat','membranes',2],['body','peau',7],['coat','veille',3],['body','matelassee',8],['body','fibres',9],
    ['head','cognefer',0],['hood','mue',0],['head','cerbere',1],['head','bastion',2],['head','chrysalide',3],['head','fantome',4],
    ['hands','bastion',0],['hands','chrysalide',1],['hands','cognefer',2],['hands','fantome',3],['hands','mue',4],['hands','cerbere',5],
    ['feet','cerbere',0],['feet','mue',1],['feet','chrysalide',2],['feet','fantome',3],['feet','cognefer',4],['feet','bastion',5]
  ];
  function armor(model) {
    const [type,material,v]=armorProfiles[model.index],[base,light,joints,accent]=armorMaterials[material],d=drawing();
    if(type==='body'||type==='coat') {
      const wide=type==='body'&&v<7?3:0,hem=type==='coat'?29:27;
      d.poly([[11,5],[21,5],[25+wide,10],[25+wide,21],[23,23],[23,hem],[9,hem],[9,23],[7-wide,21],[7-wide,10]],ink);
      d.poly([[8-wide,10],[11,7],[13,9],[11,16],[7-wide,16]],base);
      d.poly([[24+wide,10],[21,7],[19,9],[21,16],[25+wide,16]],base);
      d.rect(8-wide,10,3,1,light);d.rect(22,10,3,1,light);
      d.poly([[12,8],[20,8],[23,13],[21,23],[11,23],[9,13]],joints);
      d.poly([[12,8],[15,10],[17,10],[20,8],[23,12],[21,17],[11,17],[9,12]],base);
      d.rect(12,10,8,1,light);d.rect(14,5,4,3,null);d.rect(13,7,1,2,base);d.rect(18,7,1,2,base);
      d.rect(6-wide,17,4,5,joints);d.rect(22,17,4,5,joints);
      d.rect(10,19,12,3,base);d.rect(11,19,10,1,light);
      d.rect(11,24,4,2,base);d.rect(17,24,4,2,base);d.rect(15,24,2,3,ink);
      if(type==='coat') {
        d.poly([[12,9],[15,12],[15,28],[9,28],[10,17]],base);d.poly([[20,9],[17,12],[17,28],[23,28],[22,17]],base);
        d.rect(15,11,2,17,ink);d.rect(11,24,2,3,light);d.rect(19,24,2,3,joints);
        if(v===0){d.poly([[9,7],[13,5],[16,9],[20,5],[23,7],[18,13],[14,13]],light);d.rect(8,17,2,3,base);}
        if(v===1||v===2){d.poly([[10,12],[12,11],[22,23],[20,25]],light);}
        if(v===3){d.rect(11,14,3,3,light);d.rect(18,14,3,3,light);d.rect(11,19,3,3,light);d.rect(18,19,3,3,light);d.dot(7,13,accent);}
      } else {
        if(v===0||v===5){d.rect(11,15,10,1,joints);d.rect(15,18,2,1,joints);d.rect(9,21,2,3,accent);d.rect(22,19,2,3,accent);}
        if(v===5){d.poly([[13,10],[19,10],[22,13],[20,16],[12,16],[10,13]],light);d.rect(7,9,2,3,orange);d.rect(24,9,2,3,orange);d.rect(11,19,10,2,metal);}
        if(v===1){d.rect(10,11,12,8,base);d.rect(10,11,12,1,light);d.dot(11,13,joints);d.dot(20,17,joints);d.rect(9,21,5,4,joints);d.rect(18,21,5,4,joints);}
        if(v===2){d.poly([[12,10],[16,13],[20,10],[21,20],[16,26],[11,20]],pale);d.poly([[15,15],[17,15],[17,23],[15,24]],light);d.rect(7,9,2,2,cyan);d.rect(23,9,2,2,cyan);d.rect(11,17,2,3,cyan);d.rect(19,17,2,3,cyan);}
        if(v===3){d.rect(10,16,12,1,joints);d.poly([[11,22],[21,22],[19,26],[13,26]],base);}
        if(v===4||v===7){d.rect(11,10,10,11,base);d.rect(11,11,1,8,light);d.rect(10,8,1,5,cyan);d.rect(21,8,1,5,cyan);d.rect(7,20,2,1,cyan);d.rect(23,20,2,1,cyan);}
        if(v===6){d.rect(10,10,6,7,pale);d.rect(16,10,7,6,brown);d.rect(18,11,3,1,'#bd9067');d.rect(11,19,10,1,light);}
        if(v===8){for(let y=11;y<25;y+=5){d.rect(10,y,12,3,base);d.rect(11,y,10,1,light);}d.rect(15,9,2,17,ink);}
        if(v===9){d.poly([[10,10],[12,9],[23,23],[21,25]],light);d.poly([[21,10],[23,11],[11,25],[9,23]],light);d.rect(22,23,2,3,orange);}
      }
    } else if(type==='head'||type==='hood') {
      d.poly([[10,5],[21,5],[26,10],[26,23],[22,28],[10,28],[6,23],[6,11]],ink);
      d.poly([[11,6],[20,6],[24,11],[23,21],[20,25],[11,25],[8,21],[8,11]],base);
      d.poly([[11,6],[20,6],[23,10],[10,10],[8,13],[8,11]],light);
      if(type==='hood') {d.poly([[12,11],[20,11],[23,15],[22,23],[18,26],[12,23],[10,17]],ink);d.poly([[8,23],[11,22],[18,26],[24,23],[26,27],[16,29],[7,27]],base);}
      else if(v===0){d.rect(12,13,12,12,'#493d30');d.rect(9,17,3,6,joints);d.rect(13,13,10,1,ink);}
      else {
        d.rect(9,13,15,5,ink);d.rect(11,14,12,v===4?3:2,accent);d.rect(12,14,8,1,v===3||v===4?'#83ede3':'#efb052');
        if(v===2){d.rect(15,18,3,8,joints);d.rect(18,18,3,7,light);}
        else if(v===3){d.poly([[12,18],[21,18],[19,26],[16,27]],pale);d.poly([[13,18],[21,18],[17,23]],cyan);}
        else{d.rect(13,21,8,4,joints);d.rect(14,21,6,1,metal);}
        d.rect(7,16,4,7,joints);d.rect(8,16,2,5,base);d.rect(23,16,3,6,joints);
      }
    } else if(type==='hands') {
      for(const [x,y] of [[3,5],[18,7]]) {
        d.poly([[x+2,y],[x+8,y],[x+10,y+4],[x+10,y+13],[x+7,y+20],[x+2,y+20],[x,y+15],[x,y+5]],ink);
        d.rect(x+2,y+1,6,5,joints);d.rect(x+3,y+1,4,1,light);
        d.poly([[x+2,y+7],[x+8,y+7],[x+9,y+12],[x+6,y+15],[x+1,y+13]],base);d.rect(x+2,y+7,6,1,light);
        for(let i=0;i<3;i++)d.rect(x+2+i*2,y+14,1,5,base);
        d.rect(x,y+11,2,5,joints);
        if(v===0||v===5)d.rect(x+1,y+6,2,2,accent);
        if(v===1){d.rect(x+4,y+9,2,2,cyan);d.rect(x+2,y+3,4,2,olive);}
        if(v===2){d.dot(x+3,y+9,joints);d.dot(x+7,y+11,joints);}
        if(v===3){d.rect(x+3,y+4,4,1,cyan);d.rect(x+1,y+9,1,3,metal);}
        if(v===4){d.rect(x+2,y+5,6,2,dark);d.rect(x+2,y+14,5,1,brown);}
        if(v===5){d.rect(x+2,y+15,5,3,'#aa8a67');d.rect(x+2,y+18,5,1,ink);d.rect(x+1,y+8,7,2,pale);}
      }
    } else {
      for(const [x,y] of [[3,4],[18,6]]) {
        d.poly([[x,y],[x+8,y],[x+9,y+15],[x+12,y+18],[x+12,y+22],[x+1,y+22],[x-1,y+19]],ink);
        d.rect(x+1,y+1,6,5,joints);d.rect(x+2,y+1,4,1,metal);
        d.poly([[x+1,y+7],[x+7,y+7],[x+8,y+17],[x+10,y+19],[x+2,y+19],[x,y+17]],base);
        d.rect(x+2,y+7,4,1,light);d.rect(x+2,y+8,2,6,light);
        d.poly([[x+2,y+17],[x+7,y+16],[x+10,y+19],[x+10,y+20],[x+2,y+20]],v===2||v===5?pale:base);
        d.rect(x,y+21,11,1,metal);
        if(v===0||v===5)d.rect(x,y+11,2,2,orange);
        if(v===1){d.rect(x,y+7,8,2,dark);d.rect(x+1,y+14,7,1,joints);}
        if(v===2){d.rect(x,y+12,2,3,cyan);d.rect(x+2,y+7,4,2,olive);}
        if(v===3){d.rect(x+1,y+4,5,1,cyan);d.rect(x+2,y+10,1,6,cyan);}
        if(v===4){for(let j=0;j<3;j++){d.rect(x+3,y+10+j*2,4,1,joints);d.dot(x+2,y+10+j*2,light);}}
        if(v===5){d.rect(x+1,y+8,7,5,pale);d.rect(x+2,y+8,6,1,highlight);d.rect(x+2,y+17,8,3,highlight);d.rect(x+1,y+20,10,1,pale);}
      }
    }
    return d.finish(model);
  }
  function build(catalog) {
    const items={};
    for(const model of catalog.models)items[model.id]=model.group==='firearms'?firearm(model):model.group==='melee'?melee(model):armor(model);
    return {version:1,creation:'authored_on_native_pixel_grids',counts:catalog.counts,items};
  }
  if(typeof module==='object'&&module.exports)module.exports={build};
  else root.groundEquipmentArt=build(root.equipmentCatalogData);
})(typeof window!=='undefined'?window:globalThis);
