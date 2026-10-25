/* Review-only inverse pixel deformation. No simulation or game state. */
(function(root) {
  'use strict';
  const clamp=(x,a,b)=>Math.max(a,Math.min(b,x));
  function warp(source,w,h,rig,dir,phase,foot,bounds,moving=true) {
    const out=new Uint8ClampedArray(source.length), cx=foot[0];
    const [x0,y0,x1,y1]=bounds, height=Math.max(8,y1-y0);
    const hip=y0+height*(rig==='robe'?.58:.63);
    const s=moving?Math.sin(phase*Math.PI*2):0;
    const bob=moving?-Math.abs(Math.sin(phase*Math.PI*2))*(rig==='heavy'?.65:rig==='float'?1.8:.9):0;
    for(let y=0;y<h;y++)for(let x=0;x<w;x++) {
      let sx=x,sy=y-bob;
      if(rig==='serpent') {
        sx-=moving?Math.sin(phase*Math.PI*2+(y-y0)*.12)*2.2*clamp((y-y0)/height,0,1):0;
        sy=y-(moving?Math.sin(phase*Math.PI*2+(x-x0)*.07)*.6:0);
      } else if(rig==='float') {
        sx-=moving?Math.sin(phase*Math.PI*2+(y-y0)*.05)*clamp((y-y0)/height,0,1)*.7:0;
      } else if(rig==='roll') {
        // Keep the complete chassis and track outline together. Translation
        // carries the vehicle; no alternating legs or floating hull.
        sy=y-(moving?Math.sin(phase*Math.PI*4)*.35:0);
      } else if(y>hip) {
        const t=clamp((y-hip)/(y1-hip),0,1);
        if(rig==='robe') {
          sx-=moving?Math.sin(phase*Math.PI*2+(x-cx)*.09)*t*1.2:0;
          sy=y-bob*(1-t);
        } else {
          const multiple=['crawl','quad','machine'].includes(rig);
          const side=x<cx?-1:1;
          const wave=!moving?0:multiple?Math.sin(phase*Math.PI*2+(x-x0)/Math.max(8,(x1-x0))*Math.PI*3):s*side;
          const stride=(rig==='heavy'?2:rig==='crawl'?1.8:2.7)*(dir===0?.48:1);
          sx-=wave*t*stride;
          sy=y-bob*(1-t)+Math.max(0,wave)*t*(rig==='heavy'?1:1.8);
        }
      }
      if(rig==='winged' && Math.abs(x-cx)>height*.20 && y<hip && moving) {
        const wing=clamp((Math.abs(x-cx)-height*.20)/(height*.25),0,1);
        sy-=Math.sin(phase*Math.PI*2)*wing*1.4;
      }
      if(dir!==0) {
        // Keep the source's armor and palette. This is a 2D yaw experiment,
        // not a reconstruction of unseen side/back surfaces.
        const head=1-clamp((sy-y0-height*.15)/(height*.22),0,1);
        const scale=1-(['serpent','crawl','roll'].includes(rig)?.025:.05)-head*.09;
        sx=cx+(sx-cx-dir*(.8+head*1.1))/scale;
      }
      const ix=Math.round(sx),iy=Math.round(sy);
      if(ix<0||ix>=w||iy<0||iy>=h)continue;
      const a=(iy*w+ix)*4,b=(y*w+x)*4;
      out[b]=source[a];out[b+1]=source[a+1];out[b+2]=source[a+2];out[b+3]=source[a+3];
    }
    return out;
  }
  function travel(elapsed,duration) { return clamp(elapsed/duration,0,1); }
  root.WalkReviewRig={warp,travel};
})(typeof globalThis==='object'?globalThis:this);
