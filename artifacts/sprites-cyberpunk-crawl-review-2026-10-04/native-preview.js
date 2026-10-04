(async function () {
  'use strict';
  const art = window.nativeReviewArt, byId = id => document.getElementById(id);
  const ids = Object.keys(art.items);
  const catalog = [...window.equipmentCatalogData.models, ...window.pickupCatalogData.models];
  const models = new Map(ids.map(id => [id, catalog.find(model => model.id === id)]));
  const positions = [[4,4],[5,2],[7,5],[2,6]], heroPosition = [3,4];
  const kinds = ['Soin', 'Munitions', 'Arme · fusil', 'Armure · plastron'];
  const ground = new Set(ids), inventory = new Map(ids.map((id,i) => [id, i === 1 ? 12 : 1]));
  const sources = new Map(), native = new Map();
  let family = 0, gear = 1, target = {kind:'item',id:ids[0]}, ready = false;
  const board = byId('board'), context = board.getContext('2d');
  context.imageSmoothingEnabled = false;
  function canvasFromGrid(sprite) {
    const canvas = document.createElement('canvas'); canvas.width = sprite.width; canvas.height = sprite.height;
    const ctx = canvas.getContext('2d');
    sprite.pixels.forEach((row,y) => row.forEach((index,x) => {
      if (sprite.colors[index]) { ctx.fillStyle = sprite.colors[index]; ctx.fillRect(x,y,1,1); }
    }));
    return canvas;
  }
  ids.forEach(id => native.set(id,canvasFromGrid(art.items[id])));
  const actors = [], floors = art.floors.map(row => row.map(canvasFromGrid)), wall = canvasFromGrid(art.wall);
  const isWall = (x,y) => x === 0 || y === 0 || x === 9 || y === 7 || (x === 6 && y < 4);
  function cellTarget(x,y) {
    const index = positions.findIndex(([a,b]) => a === x && b === y);
    if (index >= 0 && ground.has(ids[index])) return {kind:'item',id:ids[index],x,y};
    if (x === heroPosition[0] && y === heroPosition[1]) return {kind:'hero',x,y};
    return {kind:isWall(x,y) ? 'wall' : 'floor',x,y};
  }
  function drawBoard() {
    context.clearRect(0,0,320,256);
    for (let y=0;y<8;y++) for (let x=0;x<10;x++) {
      context.drawImage(isWall(x,y) ? wall : floors[family][(x*3+y)%4],x*32,y*32);
    }
    ground.forEach(id => {
      const [x,y] = positions[ids.indexOf(id)];
      context.drawImage(native.get(id),x*32,y*32);
    });
    if (actors[gear]) context.drawImage(actors[gear],heroPosition[0]*32,heroPosition[1]*32);
    let position = target.kind === 'item' && ground.has(target.id) ? positions[ids.indexOf(target.id)] : null;
    if (target.kind !== 'item') position = [target.x,target.y];
    if (position && position[0] != null) {
      context.strokeStyle = '#91d7c9'; context.lineWidth = 1;
      context.strokeRect(position[0]*32+.5,position[1]*32+.5,31,31);
    }
    [...byId('cells').children].forEach((button,index) => {
      const x = index%10,y = Math.floor(index/10), value = cellTarget(x,y);
      button.setAttribute('aria-label', (value.kind === 'item' ? models.get(value.id).name : value.kind === 'hero' ? 'BRÈCHE' : value.kind === 'wall' ? 'Mur' : 'Sol') + ` · case ${x},${y}`);
    });
  }
  // These are source-resolution crops. The small native sprites never feed this path.
  async function loadImage(file) {
    const image = new Image(); image.src = file;
    await new Promise((resolve,reject) => {image.onload = resolve; image.onerror = () => reject(new Error('Illustration introuvable : '+file));});
    return image;
  }
  function crop(image,column,row,columns,rows) {
    const x = Math.round(column*image.width/columns), y = Math.round(row*image.height/rows);
    const width = Math.round((column+1)*image.width/columns)-x, height = Math.round((row+1)*image.height/rows)-y;
    const scan = document.createElement('canvas'); scan.width = width; scan.height = height;
    const ctx = scan.getContext('2d'); ctx.drawImage(image,x,y,width,height,0,0,width,height);
    const pixels = ctx.getImageData(0,0,width,height).data;
    let left=width,right=-1,top=height,bottom=-1;
    for(let v=0;v<height;v++) for(let u=0;u<width;u++) if(pixels[(v*width+u)*4+3]>32) {
      left=Math.min(left,u);right=Math.max(right,u);top=Math.min(top,v);bottom=Math.max(bottom,v);
    }
    if (right<left) throw new Error('Illustration vide');
    return {image,x:x+left,y:y+top,width:right-left+1,height:bottom-top+1};
  }
  function drawIllustration(canvas,source) {
    const ctx = canvas.getContext('2d'); ctx.clearRect(0,0,canvas.width,canvas.height);
    if (!source) return;
    const factor = Math.min((canvas.width-10)/source.width,(canvas.height-10)/source.height);
    const width = source.width*factor, height = source.height*factor;
    ctx.imageSmoothingEnabled = true; ctx.imageSmoothingQuality = 'high';
    ctx.drawImage(source.image,source.x,source.y,source.width,source.height,(canvas.width-width)/2,(canvas.height-height)/2,width,height);
    canvas.dataset.source = source.file;
    canvas.dataset.sourceWidth = source.width; canvas.dataset.sourceHeight = source.height;
  }
  function showTarget() {
    const canvas = byId('target-art');
    if (target.kind === 'item') {
      const model = models.get(target.id), i=ids.indexOf(target.id);
      drawIllustration(canvas,sources.get(target.id));
      byId('target-name').textContent = model.name; byId('target-type').textContent = kinds[i];
      byId('target-caption').textContent = 'Illustration de 128 pixels · sprite de 32 pixels sur le plateau.';
    } else if (target.kind === 'hero') {
      drawIllustration(canvas,sources.get('hero:'+gear));
      byId('target-name').textContent = 'BRÈCHE'; byId('target-type').textContent = ['Tenue de base','Lame et plastron','Fusil et armure renforcée'][gear];
      byId('target-caption').textContent = 'Personnage précédent conservé · sprite de 32 pixels.';
    } else {
      const tile = target.kind === 'wall' ? wall : floors[family][((target.x||0)*3+(target.y||0))%4];
      const ctx=canvas.getContext('2d'); ctx.clearRect(0,0,128,128);ctx.imageSmoothingEnabled=false;ctx.drawImage(tile,0,0,128,128);
      canvas.dataset.source = 'native/'; canvas.dataset.sourceWidth=32;canvas.dataset.sourceHeight=32;
      byId('target-name').textContent = target.kind === 'wall' ? 'Mur' : ['Asphalte','Béton','Métal','Ruines'][family];
      byId('target-type').textContent = 'Décor · 32 × 32 pixels';
      byId('target-caption').textContent = 'Agrandissement du motif natif.';
    }
    byId('preview-panel').dataset.target = target.kind === 'item' ? target.id : target.kind;
    byId('pickup').disabled = !ready || target.kind !== 'item' || !ground.has(target.id);
    byId('drop').disabled = !ready || target.kind !== 'item' || ground.has(target.id) || !(inventory.get(target.id)>0);
    document.querySelectorAll('[data-model]').forEach(button => button.setAttribute('aria-pressed',String(target.kind === 'item' && button.dataset.model === target.id)));
  }
  function select(value) {target=value; drawBoard();showTarget();}
  function renderInventory() {
    byId('inventory').replaceChildren();
    ids.forEach((id,i) => {
      const quantity=inventory.get(id); if (!quantity) return;
      const button=document.createElement('button');button.className='inventory-row';button.dataset.model=id;
      const canvas=document.createElement('canvas');canvas.width=64;canvas.height=64;canvas.className='large-art';canvas.setAttribute('aria-label',models.get(id).name);
      drawIllustration(canvas,sources.get(id));
      const label=document.createElement('span'),name=document.createElement('strong'),amount=document.createElement('small');
      name.textContent=models.get(id).name;amount.textContent=kinds[i]+' · × '+quantity;label.append(name,amount);button.append(canvas,label);
      button.addEventListener('click',()=>select({kind:'item',id}));byId('inventory').append(button);
    });
  }
  ids.forEach((id,i)=>{
    const file='native/'+id.slice(5)+'-32.png';
    const button=document.createElement('button');button.className='sample';button.dataset.model=id;
    const image=document.createElement('img');image.src=file;image.alt='';image.width=32;image.height=32;
    const label=document.createElement('span');label.textContent=models.get(id).name;button.append(image,label);button.addEventListener('click',()=>select({kind:'item',id}));byId('samples').append(button);
    const card=document.createElement('div');card.className='zoom-card';
    const zoom=image.cloneNode();zoom.alt=models.get(id).name+' · grille 32 × 32';zoom.removeAttribute('width');zoom.removeAttribute('height');
    const caption=document.createElement('div');caption.textContent=models.get(id).name+' · '+(art.items[id].colors.length-1)+' couleurs';card.append(zoom,caption);byId('zoom-grid').append(card);
  });
  for(let y=0;y<8;y++) for(let x=0;x<10;x++) {
    const button=document.createElement('button');button.className='cell';button.dataset.x=x;button.dataset.y=y;
    const inspect=()=>select(cellTarget(x,y));button.addEventListener('pointerenter',inspect);button.addEventListener('click',inspect);button.addEventListener('focus',inspect);byId('cells').append(button);
  }
  byId('floor-choice').addEventListener('change',event=>{family=Number(event.target.value);drawBoard();showTarget();});
  byId('gear-choice').addEventListener('change',event=>{gear=Number(event.target.value);select({kind:'hero',x:3,y:4});});
  byId('pickup').addEventListener('click',()=>{
    if (target.kind !== 'item' || !ground.has(target.id)) return;
    const id=target.id;ground.delete(id);inventory.set(id,(inventory.get(id)||0)+1);
    renderInventory();drawBoard();showTarget();byId('status').textContent=models.get(id).name+' ajouté à l’inventaire de démonstration.';
  });
  byId('drop').addEventListener('click',()=>{
    if (target.kind !== 'item' || ground.has(target.id) || !(inventory.get(target.id)>0)) return;
    const id=target.id;ground.add(id);inventory.set(id,inventory.get(id)-1);
    renderInventory();drawBoard();showTarget();byId('status').textContent=models.get(id).name+' reposé sur le plateau.';
  });
  byId('reset').addEventListener('click',()=>{
    ids.forEach((id,i)=>{ground.add(id);inventory.set(id,i===1?12:1);});renderInventory();select({kind:'item',id:ids[0]});byId('status').textContent='Quatre objets au sol. Survole une case ou choisis un objet.';
  });
  drawBoard();showTarget();
  try {
    const files = [...new Set(ids.map(id=>models.get(id).atlas)),'classes-v1.png'];
    const images = new Map(await Promise.all(files.map(async file=>[file,await loadImage(file)])));
    ids.forEach(id=>{
      const model=models.get(id),image=images.get(model.atlas),column=model.index%model.columns,row=Math.floor(model.index/model.columns);
      sources.set(id,{...crop(image,column,row,model.columns,model.rows),file:model.atlas});
    });
    for(let row=0;row<3;row++) sources.set('hero:'+row,{...crop(images.get('classes-v1.png'),0,row,3,3),file:'classes-v1.png'});
    // Restore the exact 32px rendering from apercu.html: full atlas cells,
    // nearest-neighbor sampling, no crop normalization or new character art.
    const classImage=images.get('classes-v1.png');
    for(let row=0;row<3;row++) {
      const actor=document.createElement('canvas');actor.width=actor.height=32;
      const ctx=actor.getContext('2d');ctx.imageSmoothingEnabled=false;
      ctx.drawImage(classImage,0,row*classImage.height/3,classImage.width/3,classImage.height/3,0,0,32,32);
      actors.push(actor);
    }
    board.dataset.actorSource='classes-v1.png';board.dataset.actorRendering='previous-full-cell-32';board.dataset.itemSize='32';
    ready=true;renderInventory();drawBoard();showTarget();byId('status').textContent='Quatre objets au sol. Survole une case ou choisis un objet.';
    document.body.dataset.ready='true';
  } catch(error) {byId('status').textContent=error.message;document.body.dataset.ready='error';console.error(error);}
})();
