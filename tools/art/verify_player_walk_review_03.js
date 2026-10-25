/* Meaningful review checks: idle fidelity, palette ownership and cell timing. */
'use strict';
const assert=require('node:assert/strict');
require('./player_walk_review_03_rig.js');
const rigs=['biped','heavy','robe','quad','crawl','serpent','float','winged','machine','roll'];
const w=64,h=64,source=new Uint8ClampedArray(w*h*4),colors=new Set(['0,0,0,0']);
for(let y=9;y<59;y++)for(let x=15;x<49;x++){
  const k=(y*w+x)*4;source[k]=(x%5)*40;source[k+1]=(y%7)*30;source[k+2]=110;source[k+3]=255;
  colors.add(Array.from(source.slice(k,k+4)).join(','));
}
let checks=0;
for(const rig of rigs){
  const idle=WalkReviewRig.warp(source,w,h,rig,0,0,[32,60],[15,9,49,59],false);
  assert(idle.every((v,i)=>v===source[i]),rig+' changes the front idle source');
  for(const direction of [-1,0,1])for(const phase of [0,.125,.25,.5,.75,.875]){
    const result=WalkReviewRig.warp(source,w,h,rig,direction,phase,[32,60],[15,9,49,59],true);
    assert.equal(result.length,source.length);
    let opaque=0;
    for(let k=0;k<result.length;k+=4){assert(colors.has(Array.from(result.slice(k,k+4)).join(',')),rig+' invents a material color');if(result[k+3])opaque++;}
    assert(opaque>1000&&opaque<2000,rig+' collapses or duplicates the test silhouette');
    checks++;
  }
}
for(const duration of [120,200,400]){
  assert.equal(WalkReviewRig.travel(0,duration),0);
  assert.equal(WalkReviewRig.travel(duration/2,duration),.5);
  assert.equal(WalkReviewRig.travel(duration,duration),1);
  assert.equal(WalkReviewRig.travel(duration+400,duration),1);
}
console.log(JSON.stringify({rigs:rigs.length,animated_cases:checks,front_idle_exact:true,palettes_preserved:true,durations:[120,200,400],max_travel_cells:1}));
