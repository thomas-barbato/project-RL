// Flat pictograms drawn on the final 32-pixel grid. One clear symbol per
// category; the exact model is identified by the inventory/inspection art.
(function(root){
  'use strict';
  const light='#cbd3d5',cyan='#56c5c8',gold='#e3a63d',red='#d74e43',violet='#a27aca';
  const steel='#83949d',shade='#3c505a',edge='#edf0e9',amberLight='#ffdb73',olive='#7f9e65';
  function drawing(){
    const grid=Array.from({length:32},()=>Array(32).fill(null));
    const dot=(x,y,c)=>{if(x>=0&&y>=0&&x<32&&y<32)grid[y][x]=c;};
    const rect=(x,y,w,h,c=light)=>{for(let j=y;j<y+h;j++)for(let i=x;i<x+w;i++)dot(i,j,c);};
    function poly(points,c=light){
      for(let y=0;y<32;y++)for(let x=0;x<32;x++){
        let inside=false;
        for(let i=0,j=points.length-1;i<points.length;j=i++){
          const [ax,ay]=points[i],[bx,by]=points[j];
          if((ay>y+.5)!==(by>y+.5)&&x+.5<(bx-ax)*(y+.5-ay)/(by-ay)+ax)inside=!inside;
        }
        if(inside)dot(x,y,c);
      }
    }
    function circle(cx,cy,r,c=light){for(let y=0;y<32;y++)for(let x=0;x<32;x++)if((x+.5-cx)**2+(y+.5-cy)**2<=r*r)dot(x,y,c);}
    function finish(name){
      const pixels=grid.map(row=>row.slice());
      // A single dark contour separates the light symbol from every floor.
      for(let y=0;y<32;y++)for(let x=0;x<32;x++)if(grid[y][x])for(let dy=-1;dy<=1;dy++)for(let dx=-1;dx<=1;dx++){
        const u=x+dx,v=y+dy;if(u>=0&&v>=0&&u<32&&v<32&&!grid[v][u])pixels[v][u]='#0a1116';
      }
      const colors=[null];pixels.forEach(row=>row.forEach(c=>{if(!colors.includes(c))colors.push(c);}));
      return {name,kind:'icon',width:32,height:32,colors,pixels:pixels.map(row=>row.map(c=>colors.indexOf(c)))};
    }
    return {rect,poly,circle,finish};
  }
  const definitions=[
    ['ranged','Arme à distance','Fusil', 'core:modele_brenek_a80',d=>{
      // A long, shallow diagonal profile: distinct muzzle, closed stock,
      // trigger grip and curved magazine. Saturated amber, as a loot marker.
      d.poly([[3,19],[7,17],[10,18],[9,21],[4,24]],gold);
      d.poly([[8,17],[21,10],[24,13],[12,21]],gold);
      d.poly([[21,11],[27,7],[29,10],[24,14]],gold);
      d.poly([[10,20],[13,19],[15,23],[12,25]],gold);
      d.poly([[17,17],[21,15],[24,21],[22,25],[19,24]],gold);
      d.poly([[9,17],[20,11],[21,12],[11,18]],amberLight);
      d.poly([[22,11],[27,8],[28,9],[23,12]],amberLight);
      d.poly([[18,19],[20,18],[22,23],[20,23]],'#aa702b');
      d.poly([[12,15],[17,12],[18,14],[13,17]],'#c3842b');
    }],
    ['melee','Arme de mêlée','Lame', 'core:melee_epee_longue',d=>{
      d.poly([[24,4],[27,5],[25,12],[15,22],[10,17],[20,7]],steel);
      d.poly([[24,5],[25,6],[22,11],[12,20],[11,18],[21,8]],edge);
      d.poly([[8,15],[18,25],[16,27],[6,17]],shade);
      d.poly([[9,16],[17,24],[16,25],[8,17]],gold);
      d.poly([[10,19],[13,22],[8,27],[5,24]],steel);d.poly([[7,22],[9,24],[7,26],[5,24]],gold);
    }],
    ['armor','Armure','Protection', 'core:plastron_bastion',d=>{
      // A rigid plate carrier, with a neck recess and separate shoulder plates.
      d.poly([[10,7],[13,5],[14,10],[18,10],[19,5],[22,7],[23,19],[20,27],[12,27],[9,19]],steel);
      d.poly([[11,10],[14,12],[18,12],[21,10],[21,17],[19,20],[13,20],[11,17]],light);
      d.poly([[11,10],[14,12],[18,12],[21,10],[21,12],[18,14],[14,14],[11,12]],edge);
      d.poly([[5,8],[9,6],[10,12],[7,16],[4,13]],light);d.poly([[23,6],[27,8],[28,13],[25,16],[22,12]],light);
      d.rect(13,22,6,3,light);d.rect(13,22,6,1,edge);d.rect(8,18,3,4,shade);d.rect(21,18,3,4,shade);
    }],
    ['health','Soin','Kit médical', 'core:repair_patch',d=>{
      d.rect(5,8,22,18,'#973d35');d.rect(6,9,20,15,red);d.rect(7,9,18,2,'#f18966');
      d.rect(13,12,6,10,edge);d.rect(10,15,12,4,edge);d.rect(12,5,8,3,steel);d.rect(14,6,4,2,null);
    }],
    ['ammo','Munitions','Deux cartouches', 'core:weapon_matter',d=>{
      for(const [x,y] of [[7,5],[18,7]]){
        d.poly([[x+3,y],[x+6,y+5],[x+6,y+19],[x+7,y+19],[x+7,y+21],[x-1,y+21],[x-1,y+19],[x,y+19],[x,y+5]],'#a9702d');
        d.poly([[x+3,y+1],[x+5,y+5],[x+1,y+5]],'#e7bb78');d.rect(x,y+6,6,13,gold);
        d.rect(x+1,y+6,2,12,amberLight);d.rect(x-1,y+19,8,1,amberLight);
      }
    }],
    ['explosive','Explosif','Grenade', 'core:fragmentation_charge',d=>{
      d.poly([[11,10],[21,10],[25,17],[23,25],[19,28],[12,27],[8,23],[8,16]],olive);
      d.poly([[11,11],[14,10],[14,25],[12,25],[10,22],[10,16]],'#bad18a');
      d.rect(10,16,14,2,shade);d.rect(10,22,13,2,shade);d.rect(16,12,2,14,shade);
      d.rect(12,5,8,5,steel);d.rect(14,5,4,1,edge);d.poly([[19,5],[23,6],[26,17],[23,18],[21,9],[18,8]],light);
    }],
    ['battery','Batterie','Pile et éclair', 'core:charged_battery',d=>{
      d.rect(7,8,18,19,steel);d.rect(8,9,16,17,shade);d.rect(9,10,14,14,cyan);d.rect(10,10,12,2,'#a5e9df');
      d.rect(10,4,4,4,gold);d.rect(19,4,4,4,gold);d.rect(10,4,3,1,amberLight);d.rect(19,4,3,1,amberLight);
      d.poly([[16,13],[13,19],[16,19],[14,23],[20,16],[17,16]],edge);
    }],
    ['beacon','Balise','Antenne', 'core:saturation_beacon',d=>{
      d.rect(13,5,6,10,steel);d.rect(14,6,4,7,cyan);d.rect(14,6,4,2,'#a5e9df');
      d.rect(11,15,10,7,steel);d.rect(12,15,8,2,light);d.rect(15,21,2,7,light);
      d.poly([[12,20],[15,21],[8,28],[5,28]],light);d.poly([[18,20],[20,20],[27,28],[24,28]],light);
    }],
    ['sound','Leurre sonore','Émetteur sonore', 'core:sound_decoy',d=>{
      d.rect(8,5,16,22,steel);d.rect(9,6,14,20,shade);d.rect(10,6,12,2,light);
      d.circle(16,17,6,light);d.circle(16,17,4,shade);d.circle(16,17,2,cyan);d.rect(13,9,6,2,gold);
    }],
    ['camouflage','Camouflage','Rouleau de tissu', 'core:camouflage_bundle',d=>{
      d.poly([[5,10],[25,10],[28,14],[27,23],[7,23],[4,20]],olive);d.rect(7,11,18,2,'#bad18a');
      d.rect(9,10,3,13,shade);d.rect(20,10,3,13,shade);d.rect(9,11,3,1,steel);d.rect(20,11,3,1,steel);
      d.poly([[5,20],[8,20],[12,25],[7,25]],'#a4b77b');
    }],
    ['parts','Pièces de réparation','Engrenage', 'core:repair_parts',d=>{
      d.circle(16,16,8,steel);d.rect(13,5,6,5,steel);d.rect(13,22,6,5,steel);d.rect(5,13,5,6,steel);d.rect(22,13,5,6,steel);
      for(const [x,y] of [[7,7],[20,7],[7,20],[20,20]])d.rect(x,y,5,5,steel);
      d.circle(16,16,6,light);d.circle(16,16,3,shade);d.rect(13,5,6,1,edge);d.rect(7,7,5,1,edge);
    }],
    ['component','Composant récupéré','Puce', 'core:salvaged_component',d=>{
      d.rect(8,8,17,17,olive);d.rect(9,9,15,2,'#bad18a');d.rect(12,12,9,9,shade);d.rect(13,13,7,2,steel);
      d.rect(15,16,3,3,cyan);for(const x of [10,16,22]){d.rect(x,5,2,3,gold);d.rect(x,25,2,3,gold);}
    }],
    ['tuning','Pièces de réglage','Interfaces', 'core:tuning_parts',d=>{
      d.rect(6,7,9,18,steel);d.rect(7,8,7,15,light);d.rect(8,10,5,7,shade);d.rect(8,11,5,3,cyan);d.rect(9,5,3,2,gold);
      d.rect(19,10,8,15,steel);d.rect(20,11,6,12,light);d.rect(21,13,4,5,shade);d.rect(21,14,4,2,gold);d.rect(22,7,2,3,gold);
    }],
    ['tool','Outil d’ingénierie','Clé', 'core:engineering_tool',d=>{
      d.poly([[17,6],[23,4],[20,9],[23,12],[28,9],[27,15],[23,19],[18,18],[10,27],[5,22],[15,13],[14,9]],steel);
      d.poly([[17,7],[21,6],[19,9],[22,13],[26,11],[25,15],[22,17],[17,16],[9,24],[7,22],[16,13]],light);
      d.poly([[7,20],[11,23],[8,26],[5,23]],gold);
    }],
    ['regulator','Régulateur de puissance','Module d’alimentation', 'core:power_regulator',d=>{
      d.poly([[10,8],[22,8],[25,12],[25,22],[21,26],[11,26],[7,22],[7,12]],steel);
      d.rect(10,10,12,12,light);d.rect(12,12,8,8,shade);d.rect(14,14,4,4,cyan);
      d.rect(13,4,6,4,gold);d.rect(4,14,3,6,gold);d.rect(25,14,3,6,gold);
    }],
    ['fragment','Fragment instable','Éclat', 'core:unstable_fragment',d=>{
      d.poly([[17,4],[26,11],[23,23],[11,28],[5,18],[9,10]],'#644d8c');
      d.poly([[17,5],[21,12],[15,21],[10,26],[8,18],[11,11]],violet);
      d.poly([[17,5],[16,13],[9,19],[10,12]],'#dec7ed');d.poly([[22,12],[25,12],[22,22],[17,25]],'#826bb5');
    }]
  ];
  const supplyKeys={repair_patch:'health',weapon_matter:'ammo',charged_battery:'battery',saturation_beacon:'beacon',sound_decoy:'sound',camouflage_bundle:'camouflage',repair_parts:'parts',salvaged_component:'component',tuning_parts:'tuning',engineering_tool:'tool',power_regulator:'regulator',unstable_fragment:'fragment',configurable_charge:'explosive',breach_charge:'explosive',structural_charge:'explosive',fragmentation_charge:'explosive',proximity_mine:'explosive'};
  function build(equipment,pickups){
    const icons=definitions.map(([key,name,description,representative,draw])=>{const d=drawing();draw(d);return {key,name,description,representative,sprite:d.finish(name)};});
    const mapping={};
    for(const model of [...equipment.models,...pickups.models]){
      const key=model.group==='firearms'?'ranged':model.group==='melee'?'melee':model.group==='armor'?'armor':supplyKeys[model.id.slice(5)];
      if(!icons.some(icon=>icon.key===key))throw Error('Missing icon: '+model.id);
      mapping[model.id]=key;
    }
    return {version:1,creation:'authored_industrial_loot_icons_on_native_pixel_grid',assetDirectory:'ground-icons-jupiter',icons,mapping};
  }
  if(typeof module==='object'&&module.exports)module.exports={build};
  else root.groundLootIcons=build(root.equipmentCatalogData,root.pickupCatalogData);
})(typeof window!=='undefined'?window:globalThis);
