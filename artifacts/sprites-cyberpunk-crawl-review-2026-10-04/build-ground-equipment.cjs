const fs=require('node:fs'),path=require('node:path'),zlib=require('node:zlib'),crypto=require('node:crypto');
const catalog=JSON.parse(fs.readFileSync(path.join(__dirname,'equipment-catalog.json'),'utf8'));
const art=require('./ground-equipment.js').build(catalog),output=path.join(__dirname,'native-equipment');
fs.mkdirSync(output,{recursive:true});
function crc32(bytes){let c=0xffffffff;for(const b of bytes){c^=b;for(let i=0;i<8;i++)c=(c>>>1)^(c&1?0xedb88320:0);}return(c^0xffffffff)>>>0;}
function chunk(type,data){const n=Buffer.from(type),l=Buffer.alloc(4),c=Buffer.alloc(4);l.writeUInt32BE(data.length);c.writeUInt32BE(crc32(Buffer.concat([n,data])));return Buffer.concat([l,n,data,c]);}
function encode(sprite){
  const {width,height,pixels,colors}=sprite,rows=Buffer.alloc(height*(width*4+1));let opaque=0;
  if(pixels.length!==height||pixels.some(row=>row.length!==width))throw Error('Invalid grid');
  for(let y=0;y<height;y++)for(let x=0;x<width;x++){
    const color=colors[pixels[y][x]],at=y*(width*4+1)+1+x*4;
    if(color){Buffer.from(color.slice(1),'hex').copy(rows,at);rows[at+3]=255;opaque++;}
  }
  if(!opaque)throw Error('Empty sprite');
  const h=Buffer.alloc(13);h.writeUInt32BE(width,0);h.writeUInt32BE(height,4);h[8]=8;h[9]=6;
  return Buffer.concat([Buffer.from([137,80,78,71,13,10,26,10]),chunk('IHDR',h),chunk('IDAT',zlib.deflateSync(rows)),chunk('IEND',Buffer.alloc(0))]);
}
const files=[],hashes=new Map(),duplicates=[];
for(const [id,sprite] of Object.entries(art.items)){
  const png=encode(sprite),file=id.slice(5)+'-32.png',hash=crypto.createHash('sha256').update(png).digest('hex');
  if(hashes.has(hash))duplicates.push([hashes.get(hash),id]);else hashes.set(hash,id);
  fs.writeFileSync(path.join(output,file),png);
  const positions=sprite.pixels.flatMap((row,y)=>row.map((p,x)=>p?[x,y]:null).filter(Boolean));
  files.push({id,file,width:32,height:32,colors:sprite.colors.length-1,opaque:positions.length,bounds:[Math.min(...positions.map(p=>p[0])),Math.min(...positions.map(p=>p[1])),Math.max(...positions.map(p=>p[0])),Math.max(...positions.map(p=>p[1]))]});
}
const report={creation:art.creation,counts:art.counts,models:files.length,unique:hashes.size,duplicates,files};
fs.writeFileSync(path.join(output,'manifest.json'),JSON.stringify(report,null,2)+'\n');
fs.writeFileSync(path.join(output,'native-grid-source.json'),JSON.stringify(art,null,2)+'\n');
// Contact sheet rendered from the grids, at 1x and 2x, for pixel-art inspection.
const width=800,height=Math.ceil(files.length/8)*170,pixels=Array.from({length:height},()=>Array(width).fill(1));
const colors=[null,'#28343d'];
files.forEach(({id},index)=>{
  const s=art.items[id],ox=index%8*100,oy=Math.floor(index/8)*170;
  for(const [scale,dy] of [[1,4],[2,40]])for(let y=0;y<32;y++)for(let x=0;x<32;x++){
    const c=s.colors[s.pixels[y][x]];if(!c)continue;let p=colors.indexOf(c);if(p<0){p=colors.length;colors.push(c);}
    for(let v=0;v<scale;v++)for(let u=0;u<scale;u++)pixels[oy+dy+y*scale+v][ox+18+x*scale+u]=p;
  }
});
fs.writeFileSync(path.join(output,'contact-sheet.png'),encode({width,height,pixels,colors}));
console.log(JSON.stringify({models:report.models,unique:report.unique,duplicates:report.duplicates,borderTouches:files.filter(f=>f.bounds.some((p,i)=>i<2?p===0:p===31)).map(f=>f.id)}));
