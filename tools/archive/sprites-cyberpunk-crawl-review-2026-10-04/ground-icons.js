// Flat pictograms drawn on the final 32-pixel grid. One clear symbol per
// category; the exact model is identified by the inventory/inspection art.
(function(root){
  'use strict';
  const light='#f3dfb0',cyan='#67d5d3',gold='#efbc68',red='#f18b78',violet='#c7a8eb';
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
        const u=x+dx,v=y+dy;if(u>=0&&v>=0&&u<32&&v<32&&!grid[v][u])pixels[v][u]='#111b23';
      }
      const colors=[null];pixels.forEach(row=>row.forEach(c=>{if(!colors.includes(c))colors.push(c);}));
      return {name,kind:'icon',width:32,height:32,colors,pixels:pixels.map(row=>row.map(c=>colors.indexOf(c)))};
    }
    return {rect,poly,circle,finish};
  }
  const definitions=[
    ['ranged','Arme à distance','Fusil', 'core:modele_brenek_a80',d=>{
      d.poly([[3,10],[8,10],[11,12],[9,16],[3,18]]);d.rect(8,10,14,5);d.rect(22,11,7,3);
      d.poly([[11,15],[15,15],[13,23],[10,23]]);d.poly([[18,15],[22,15],[23,23],[19,23]]);
      d.rect(13,9,7,1);d.rect(14,12,5,1,cyan);
    }],
    ['melee','Arme de mêlée','Lame', 'core:melee_epee_longue',d=>{
      d.poly([[24,5],[26,6],[24,12],[15,21],[11,17],[20,8]]);
      d.poly([[9,15],[17,23],[15,25],[7,17]],gold);d.poly([[10,19],[13,22],[8,27],[5,24]],gold);
    }],
    ['body','Armure de torse','Plastron', 'core:plastron_bastion',d=>{
      d.poly([[11,6],[14,6],[14,10],[18,10],[18,6],[21,6],[27,12],[23,16],[21,13],[21,27],[11,27],[11,13],[9,16],[5,12]]);
      d.rect(15,15,2,8,cyan);
    }],
    ['head','Casque','Protection de la tête', 'core:heaume_bastion',d=>{
      d.poly([[11,6],[21,6],[25,11],[25,23],[21,27],[11,27],[7,23],[7,11]]);
      d.rect(10,14,12,4,null);d.rect(12,15,8,2,cyan);d.rect(14,21,4,4,null);
    }],
    ['hands','Gants','Protection des mains', 'core:gantelets_bastion',d=>{
      d.rect(10,11,17,12);d.rect(10,7,3,6);d.rect(15,5,3,8);d.rect(20,7,3,6);d.rect(24,9,3,6);
      d.poly([[10,15],[6,12],[4,15],[10,22]]);d.rect(12,23,9,4,cyan);
    }],
    ['feet','Bottes','Protection des pieds', 'core:solerets_bastion',d=>{
      d.poly([[9,6],[19,6],[19,19],[25,21],[27,24],[27,27],[7,27],[7,22],[9,19]]);
      d.rect(10,8,8,3,cyan);d.rect(9,24,16,2,null);
    }],
    ['health','Soin','Croix', 'core:repair_patch',d=>{
      d.rect(12,5,8,22,red);d.rect(5,12,22,8,red);d.rect(14,9,4,14);d.rect(9,14,14,4);
    }],
    ['ammo','Munitions','Trois cartouches', 'core:weapon_matter',d=>{
      for(const x of [5,13,21]){d.poly([[x+3,5],[x+6,10],[x+6,25],[x,25],[x,10]],gold);d.rect(x-1,24,8,3);}
    }],
    ['explosive','Explosif','Bombe et mèche', 'core:fragmentation_charge',d=>{
      d.circle(15,20,8);d.rect(13,9,5,5);d.poly([[17,9],[19,6],[23,6],[23,9],[20,9],[18,12]],gold);d.rect(24,4,3,3,red);
    }],
    ['battery','Batterie','Pile et éclair', 'core:charged_battery',d=>{
      d.rect(10,7,13,20);d.rect(13,4,7,3);d.rect(12,9,9,16,null);
      d.poly([[17,10],[14,17],[17,17],[15,24],[20,15],[17,15]],cyan);
    }],
    ['beacon','Balise','Antenne', 'core:saturation_beacon',d=>{
      d.rect(14,13,4,11);d.rect(9,24,14,3);d.circle(16,10,3,cyan);
      d.poly([[9,6],[7,9],[7,12],[10,15],[8,17],[4,13],[4,8],[7,4]]);
      d.poly([[23,6],[25,9],[25,12],[22,15],[24,17],[28,13],[28,8],[25,4]]);
    }],
    ['sound','Leurre sonore','Haut-parleur', 'core:sound_decoy',d=>{
      d.rect(5,12,5,9);d.poly([[9,12],[17,6],[17,27],[9,21]]);
      d.poly([[21,9],[25,12],[27,16],[25,21],[21,24],[20,21],[23,18],[23,14],[20,12]],cyan);
    }],
    ['camouflage','Camouflage','Œil barré', 'core:camouflage_bundle',d=>{
      d.poly([[4,16],[10,9],[22,9],[28,16],[22,23],[10,23]]);
      d.poly([[8,16],[12,13],[20,13],[24,16],[20,19],[12,19]],null);d.circle(16,16,3,cyan);
      d.poly([[4,5],[7,3],[29,25],[26,28]],'#111b23');d.poly([[5,6],[7,5],[27,25],[25,27]]);
    }],
    ['parts','Pièces de réparation','Engrenage', 'core:repair_parts',d=>{
      d.circle(16,16,8);d.rect(13,5,6,5);d.rect(13,22,6,5);d.rect(5,13,5,6);d.rect(22,13,5,6);
      for(const [x,y] of [[7,7],[20,7],[7,20],[20,20]])d.rect(x,y,5,5);
      d.circle(16,16,4,null);d.circle(16,16,2,cyan);
    }],
    ['component','Composant récupéré','Puce', 'core:salvaged_component',d=>{
      d.rect(9,9,14,14);d.rect(12,12,8,8,null);d.rect(14,14,4,4,cyan);
      for(const x of [11,17]){d.rect(x,5,3,4);d.rect(x,23,3,4);d.rect(5,x,4,3);d.rect(23,x,4,3);}
    }],
    ['tuning','Pièces de réglage','Trois curseurs', 'core:tuning_parts',d=>{
      for(const x of [8,15,22])d.rect(x,6,2,21);
      d.rect(6,10,6,5,cyan);d.rect(13,20,6,5);d.rect(20,7,6,5,cyan);
    }],
    ['tool','Outil d’ingénierie','Clé', 'core:engineering_tool',d=>{
      d.poly([[17,6],[22,4],[20,10],[23,12],[27,9],[27,15],[22,19],[18,18],[10,27],[5,22],[15,13],[14,9]]);
      d.rect(7,21,3,3,null);
    }],
    ['regulator','Régulateur de puissance','Prise', 'core:power_regulator',d=>{
      d.rect(11,5,3,7,cyan);d.rect(19,5,3,7,cyan);d.rect(8,12,17,7);
      d.poly([[10,19],[23,19],[19,23],[18,23],[18,27],[14,27],[14,23]]);
    }],
    ['fragment','Fragment instable','Éclat', 'core:unstable_fragment',d=>{
      d.poly([[17,4],[25,12],[22,24],[11,28],[6,17],[10,9]],violet);
      d.poly([[17,7],[18,17],[11,25],[13,15]]);
    }]
  ];
  const supplyKeys={repair_patch:'health',weapon_matter:'ammo',charged_battery:'battery',saturation_beacon:'beacon',sound_decoy:'sound',camouflage_bundle:'camouflage',repair_parts:'parts',salvaged_component:'component',tuning_parts:'tuning',engineering_tool:'tool',power_regulator:'regulator',unstable_fragment:'fragment',configurable_charge:'explosive',breach_charge:'explosive',structural_charge:'explosive',fragmentation_charge:'explosive',proximity_mine:'explosive'};
  function build(equipment,pickups){
    const icons=definitions.map(([key,name,description,representative,draw])=>{const d=drawing();draw(d);return {key,name,description,representative,sprite:d.finish(name)};});
    const mapping={};
    for(const model of [...equipment.models,...pickups.models]){
      const key=model.group==='firearms'?'ranged':model.group==='melee'?'melee':model.group==='armor'?model.category:supplyKeys[model.id.slice(5)];
      if(!icons.some(icon=>icon.key===key))throw Error('Missing icon: '+model.id);
      mapping[model.id]=key;
    }
    return {version:1,creation:'authored_flat_pictograms_on_native_pixel_grid',icons,mapping};
  }
  if(typeof module==='object'&&module.exports)module.exports={build};
  else root.groundLootIcons=build(root.equipmentCatalogData,root.pickupCatalogData);
})(typeof window!=='undefined'?window:globalThis);
