(()=>{
 const radius=document.getElementById('falloff-radius'),base=document.getElementById('falloff-base'),grid=document.getElementById('falloff-grid'),total=document.getElementById('falloff-total');
 function update(){const r=Number(radius.value),b=Math.floor(Math.max(1,Math.min(1000,Number(base.value)||100))),width=2*r+1;grid.replaceChildren();grid.style.gridTemplateColumns=`repeat(${width},42px)`;let sum=0;
  for(let y=-r;y<=r;y++)for(let x=-r;x<=r;x++){const d=Math.max(Math.abs(x),Math.abs(y)),value=Math.floor(b*(2*r-d)/(2*r)),cell=document.createElement('div');sum+=value;cell.textContent=value;cell.title=`Couche ${d} · ${value} dégâts avant défenses`;cell.style.cssText=`width:42px;height:42px;display:grid;place-items:center;font-weight:bold;background:rgba(244,158,65,${.15+.55*(1-d/(2*r))});border:1px solid ${d===0?'#ffe2a3':'#5c6770'};box-sizing:border-box`;grid.append(cell);}
  total.textContent=`Si chaque case contenait un ennemi : ${sum} dégâts cumulés, contre ${width*width*b} avec des dégâts uniformes. Cela représente ${(sum/b).toFixed(2).replace('.',',')} impacts à pleine puissance.`;
 }
 radius.onchange=update;base.oninput=update;update();
 window.falloffProposal={update,damage:(b,r,x,y)=>Math.floor(b*(2*r-Math.max(Math.abs(x),Math.abs(y)))/(2*r))};
})();
