(async function () {
  'use strict';
  const art = window.nativeReviewArt, byId = id => document.getElementById(id);
  const {classes,loadouts,enemies} = window.faceReviewCatalog;
  const atlasFiles = {front:{class:'classes-face-v1.png',enemy:'bestiaire-face-v2.png'},previous:{class:'classes-v1.png',enemy:'bestiaire-v2.png'}};
  const enemySpots = [[2,2,0],[4,1,-1],[7,2,2],[7,4,9],[5,6,23]];
  const ids = Object.keys(art.items);
  const equipment=window.groundEquipmentArt;
  const catalog = [...window.equipmentCatalogData.models, ...window.pickupCatalogData.models];
  const models = new Map(catalog.map(model=>[model.id,model]));
  const positions = [[4,4],[5,2],[7,5],[2,6]], heroPosition = [3,4];
  const kind=id=>id===ids[0]?'Soin':id===ids[1]?'Munitions':models.get(id).group==='armor'?'Armure · '+({body:'torse',head:'tête',hands:'mains',feet:'pieds'}[models.get(id).category]):'Arme · '+(models.get(id).group==='melee'?'mêlée':'distance');
  const ground = new Set(ids), inventory = new Map(ids.map((id,i) => [id, i === 1 ? 12 : 1]));
  const sources = new Map(), native = new Map();
  let family = 0, gear = 1, classIndex = 0, view = 'front', enemySelection = 1, galleryMode = 'enemy', shadows=true;
  let target = {kind:'hero',x:3,y:4}, ready = false;
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
  Object.entries(equipment.items).forEach(([id,sprite])=>native.set(id,canvasFromGrid(sprite)));
  const contacts=new WeakMap();
  function cacheContact(sprite) {
    const pixels=sprite.getContext('2d').getImageData(0,0,32,32).data;
    let bottom=-1,left=32,right=-1;
    for(let y=31;y>=0&&bottom<0;y--)for(let x=0;x<32;x++)if(pixels[(y*32+x)*4+3]>128)bottom=y;
    for(let y=Math.max(0,bottom-3);y<=bottom;y++)for(let x=0;x<32;x++)if(pixels[(y*32+x)*4+3]>128){left=Math.min(left,x);right=Math.max(right,x);}
    const width=Math.max(8,Math.min(20,right-left+3));
    contacts.set(sprite,{x:Math.max(Math.ceil(width/2),Math.min(32-Math.ceil(width/2),Math.round((left+right)/2))),y:Math.min(30,bottom),width});
  }
  function drawFigure(sprite,x,y) {
    if(!sprite)return;
    if(shadows) {
      const p=contacts.get(sprite),ox=x*32,oy=y*32,w=p.width;
      context.fillStyle='rgba(10,15,17,.24)';
      for(const [dy,width] of [[-1,w-4],[0,w],[1,w-4]])context.fillRect(ox+p.x-Math.floor(width/2),oy+p.y+dy,width,1);
      context.fillStyle='rgba(8,12,14,.38)';context.fillRect(ox+p.x-Math.floor((w-6)/2),oy+p.y,w-6,1);
    }
    context.drawImage(sprite,x*32,y*32);
  }
  const actors = {front:[],previous:[]}, creatures = {front:[],previous:[]};
  const floors = art.floors.map(row => row.map(canvasFromGrid)), wall = canvasFromGrid(art.wall);
  const actorIndex = () => gear*3+classIndex;
  const sourceKey = (kind,index) => view+':'+kind+':'+index;
  const isWall = (x,y) => x === 0 || y === 0 || x === 9 || y === 7 || (x === 6 && y < 4);
  function cellTarget(x,y) {
    const index = positions.findIndex(([a,b]) => a === x && b === y);
    if (index >= 0 && ground.has(ids[index])) return {kind:'item',id:ids[index],x,y};
    if (x === heroPosition[0] && y === heroPosition[1]) return {kind:'hero',x,y};
    const spot=enemySpots.find(([a,b])=>a===x&&b===y);
    if(spot) return {kind:'enemy',index:spot[2]===-1?enemySelection:spot[2],x,y};
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
    drawFigure(actors[view][actorIndex()],...heroPosition);
    for(const [x,y,index] of enemySpots) {
      const sprite=creatures[view][index===-1?enemySelection:index];
      drawFigure(sprite,x,y);
    }
    board.dataset.actorSource=atlasFiles[view].class;board.dataset.enemySource=atlasFiles[view].enemy;
    board.dataset.view=view;board.dataset.actorIndex=actorIndex();board.dataset.enemyCount=enemySpots.length;board.dataset.itemSize='32';
    board.dataset.shadows=String(shadows);board.dataset.weapon=ids[2];board.dataset.armor=ids[3];
    let position = target.kind === 'item' && ground.has(target.id) ? positions[ids.indexOf(target.id)] : null;
    if (target.kind !== 'item') position = [target.x,target.y];
    if (position && position[0] != null) {
      context.strokeStyle = '#91d7c9'; context.lineWidth = 1;
      context.strokeRect(position[0]*32+.5,position[1]*32+.5,31,31);
    }
    [...byId('cells').children].forEach((button,index) => {
      const x = index%10,y = Math.floor(index/10), value = cellTarget(x,y);
      button.setAttribute('aria-label', (value.kind === 'item' ? models.get(value.id).name : value.kind === 'hero' ? classes[classIndex] : value.kind === 'enemy' ? enemies[value.index][0] : value.kind === 'wall' ? 'Mur' : 'Sol') + ` · case ${x},${y}`);
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
    return cropFrame(image,{x,y,width,height});
  }
  function cropFrame(image,{x,y,width,height}) {
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
      const model = models.get(target.id);
      drawIllustration(canvas,sources.get(target.id));
      byId('target-name').textContent = model.name; byId('target-type').textContent = kind(target.id);
      byId('target-caption').textContent = 'Illustration de 128 pixels · sprite de 32 pixels sur le plateau.';
    } else if (target.kind === 'hero') {
      drawIllustration(canvas,sources.get(sourceKey('hero',actorIndex())));
      byId('target-name').textContent = classes[classIndex]; byId('target-type').textContent = loadouts[classIndex][gear];
      byId('target-caption').textContent = (view==='front'?'Vue de face':'Version précédente')+' · sprite de 32 pixels.';
    } else if (target.kind === 'enemy') {
      drawIllustration(canvas,sources.get(sourceKey('enemy',target.index)));
      byId('target-name').textContent=enemies[target.index][0];byId('target-type').textContent=enemies[target.index][1];
      byId('target-caption').textContent=(view==='front'?'Vue de face':'Version précédente')+' · sprite de 32 pixels.';
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
    document.querySelectorAll('.creature-card').forEach(button=>{
      const selected=button.dataset.kind==='class' ? target.kind==='hero'&&Number(button.dataset.index)===actorIndex() : target.kind==='enemy'&&Number(button.dataset.index)===target.index;
      button.setAttribute('aria-pressed',String(selected));
    });
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
      name.textContent=models.get(id).name;amount.textContent=kind(id)+' · × '+quantity;label.append(name,amount);button.append(canvas,label);
      button.addEventListener('click',()=>select({kind:'item',id}));byId('inventory').append(button);
    });
  }
  function chooseEnemy(index) {
    enemySelection=index;byId('enemy-choice').value=String(index);select({kind:'enemy',index,x:4,y:1});
  }
  function updateGearLabels() {
    [...byId('gear-choice').options].forEach((option,index)=>option.textContent=loadouts[classIndex][index]);
  }
  function renderGallery() {
    const list=byId('creature-gallery');list.replaceChildren();
    const sprites=galleryMode==='class'?actors[view]:creatures[view];
    sprites.forEach((sprite,index)=>{
      const button=document.createElement('button');button.className='creature-card';button.dataset.kind=galleryMode;button.dataset.index=index;
      const images=document.createElement('span');images.className='creature-images';
      for(const size of [64,32]) {
        const canvas=document.createElement('canvas');canvas.width=canvas.height=size;
        const ctx=canvas.getContext('2d');ctx.imageSmoothingEnabled=false;ctx.drawImage(sprite,0,0,size,size);images.append(canvas);
      }
      const name=document.createElement('strong'),caption=document.createElement('small');
      name.textContent=galleryMode==='class'?classes[index%3]:enemies[index][0];
      caption.textContent=galleryMode==='class'?['Tenue de base','Équipé · référence','Équipé · variante'][Math.floor(index/3)]:(index>=17?'Proposition de bestiaire':'Créature existante');
      button.append(images,name,caption);button.addEventListener('click',()=>{
        if(galleryMode==='enemy') chooseEnemy(index);
        else {classIndex=index%3;gear=Math.floor(index/3);byId('class-choice').value=String(classIndex);byId('gear-choice').value=String(gear);updateGearLabels();select({kind:'hero',x:3,y:4});}
      });list.append(button);
    });
    byId('gallery-count').textContent=sprites.length+' dessins · '+(view==='front'?'de face':'version précédente');
  }
  function renderSamples() {
    byId('samples').replaceChildren();byId('zoom-grid').replaceChildren();
    ids.forEach((id,i)=>{
    const file=(i<2?'native/':'native-equipment/')+id.slice(5)+'-32.png';
    const button=document.createElement('button');button.className='sample';button.dataset.model=id;
    const image=document.createElement('img');image.src=file;image.alt='';image.width=32;image.height=32;
    const label=document.createElement('span');label.textContent=models.get(id).name;button.append(image,label);button.addEventListener('click',()=>select({kind:'item',id}));byId('samples').append(button);
    const card=document.createElement('div');card.className='zoom-card';
    const zoom=image.cloneNode();zoom.alt=models.get(id).name+' · grille 32 × 32';zoom.removeAttribute('width');zoom.removeAttribute('height');
    const caption=document.createElement('div');caption.textContent=models.get(id).name+' · '+((equipment.items[id]||art.items[id]).colors.length-1)+' couleurs';card.append(zoom,caption);byId('zoom-grid').append(card);
    });
  }
  function chooseEquipment(id) {
    const slot=models.get(id).group==='armor'?3:2,previous=ids[slot];
    ground.delete(previous);inventory.delete(previous);ids[slot]=id;ground.add(id);inventory.set(id,1);
    byId(slot===2?'weapon-choice':'armor-choice').value=id;
    renderSamples();renderInventory();select({kind:'item',id});
    byId('status').textContent=models.get(id).name+' placé sur le plateau · dessin natif de 32 pixels.';
  }
  function renderEquipment() {
    const list=byId('equipment-gallery');list.replaceChildren();
    const filter=byId('equipment-filter').value;
    const entries=window.equipmentCatalogData.models.filter(m=>filter==='all'||m.group===filter);
    for(const model of entries) {
      const button=document.createElement('button');button.className='creature-card equipment-card';button.dataset.model=model.id;
      const images=document.createElement('span');images.className='creature-images';
      const large=document.createElement('canvas');large.width=large.height=64;drawIllustration(large,sources.get(model.id));
      const small=document.createElement('canvas');small.width=small.height=32;small.dataset.model=model.id;
      const ctx=small.getContext('2d');ctx.imageSmoothingEnabled=false;ctx.drawImage(floors[family][0],0,0);ctx.drawImage(native.get(model.id),0,0);
      const name=document.createElement('strong'),caption=document.createElement('small');name.textContent=model.name;caption.textContent=kind(model.id);
      images.append(large,small);button.append(images,name,caption);button.addEventListener('click',()=>chooseEquipment(model.id));list.append(button);
    }
    byId('equipment-count').textContent=entries.length+' modèles';
  }
  for(const [slot,element] of [[2,'weapon-choice'],[3,'armor-choice']]) {
    for(const model of window.equipmentCatalogData.models.filter(m=>(m.group==='armor')===(slot===3))) {
      const option=document.createElement('option');option.value=model.id;option.textContent=model.name;byId(element).append(option);
    }
    byId(element).value=ids[slot];byId(element).addEventListener('change',event=>chooseEquipment(event.target.value));
  }
  byId('equipment-filter').addEventListener('change',()=>{renderEquipment();showTarget();});
  byId('shadow-toggle').addEventListener('click',()=>{shadows=!shadows;byId('shadow-toggle').setAttribute('aria-pressed',String(shadows));byId('shadow-toggle').textContent='Ombres : '+(shadows?'oui':'non');drawBoard();});
  renderSamples();
  for(let y=0;y<8;y++) for(let x=0;x<10;x++) {
    const button=document.createElement('button');button.className='cell';button.dataset.x=x;button.dataset.y=y;
    const inspect=()=>select(cellTarget(x,y));button.addEventListener('pointerenter',inspect);button.addEventListener('click',inspect);button.addEventListener('focus',inspect);byId('cells').append(button);
  }
  byId('floor-choice').addEventListener('change',event=>{family=Number(event.target.value);drawBoard();showTarget();if(ready)renderEquipment();});
  byId('gear-choice').addEventListener('change',event=>{gear=Number(event.target.value);select({kind:'hero',x:3,y:4});});
  byId('class-choice').addEventListener('change',event=>{classIndex=Number(event.target.value);updateGearLabels();select({kind:'hero',x:3,y:4});});
  byId('view-choice').addEventListener('change',event=>{view=event.target.value;renderGallery();drawBoard();showTarget();});
  enemies.forEach(([name],index)=>{const option=document.createElement('option');option.value=index;option.textContent=name;byId('enemy-choice').append(option);});
  byId('enemy-choice').value=String(enemySelection);byId('enemy-choice').addEventListener('change',event=>chooseEnemy(Number(event.target.value)));
  byId('gallery-mode').addEventListener('change',event=>{galleryMode=event.target.value;renderGallery();showTarget();});
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
    const sourceIds=[...new Set([...ids,...Object.keys(equipment.items)])];
    const files = [...new Set(sourceIds.map(id=>models.get(id).atlas)),...Object.values(atlasFiles).flatMap(pair=>Object.values(pair))];
    const images = new Map(await Promise.all(files.map(async file=>[file,await loadImage(file)])));
    sourceIds.forEach(id=>{
      const model=models.get(id),image=images.get(model.atlas),column=model.index%model.columns,row=Math.floor(model.index/model.columns);
      sources.set(id,{...crop(image,column,row,model.columns,model.rows),file:model.atlas});
    });
    // The generated front bestiary has unequal row heights; explicit UV
    // rectangles preserve whole figures instead of cutting the human heads.
    for(const [orientation,pair] of Object.entries(atlasFiles)) {
      for(const [kind,file,columns,rows,list] of [['hero',pair.class,3,3,actors[orientation]],['enemy',pair.enemy,6,4,creatures[orientation]]]) {
        const image=images.get(file);
        for(let row=0;row<rows;row++) for(let column=0;column<columns;column++) {
          const index=row*columns+column,tile=document.createElement('canvas');tile.width=tile.height=32;
          const ctx=tile.getContext('2d');ctx.imageSmoothingEnabled=false;
          let source;
          if(orientation==='front'&&kind==='enemy') {
            const frame=window.faceBestiaryFrames[index],factor=Math.min(30/frame.width,30/frame.height);
            const width=Math.round(frame.width*factor),height=Math.round(frame.height*factor);
            ctx.drawImage(image,frame.x,frame.y,frame.width,frame.height,Math.floor((32-width)/2),Math.floor((32-height)/2),width,height);
            source=cropFrame(image,frame);
          } else {
            ctx.drawImage(image,column*image.width/columns,row*image.height/rows,image.width/columns,image.height/rows,0,0,32,32);
            source=crop(image,column,row,columns,rows);
          }
          cacheContact(tile);list.push(tile);sources.set(orientation+':'+kind+':'+index,{...source,file});
        }
      }
    }
    ready=true;renderInventory();renderEquipment();renderGallery();drawBoard();showTarget();byId('status').textContent='133 modèles disponibles. Choisis une arme ou une armure pour l’examiner sur le plateau.';
    document.body.dataset.ready='true';document.body.dataset.equipmentCount=Object.keys(equipment.items).length;
  } catch(error) {byId('status').textContent=error.message;document.body.dataset.ready='error';console.error(error);}
})();
