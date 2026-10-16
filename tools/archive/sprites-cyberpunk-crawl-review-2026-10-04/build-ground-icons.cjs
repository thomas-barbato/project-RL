const fs=require('node:fs'),path=require('node:path'),zlib=require('node:zlib');
const read=file=>JSON.parse(fs.readFileSync(path.join(__dirname,file),'utf8'));
const art=require('./ground-icons.js').build(read('equipment-catalog.json'),read('pickup-catalog.json'));
const output=path.join(__dirname,'ground-icons');fs.mkdirSync(output,{recursive:true});
function crc32(bytes){let c=0xffffffff;for(const b of bytes){c^=b;for(let i=0;i<8;i++)c=(c>>>1)^(c&1?0xedb88320:0);}return(c^0xffffffff)>>>0;}
function chunk(type,data){const n=Buffer.from(type),l=Buffer.alloc(4),c=Buffer.alloc(4);l.writeUInt32BE(data.length);c.writeUInt32BE(crc32(Buffer.concat([n,data])));return Buffer.concat([l,n,data,c]);}
function encode({width,height,pixels,colors}){
  const rows=Buffer.alloc(height*(width*4+1));
  for(let y=0;y<height;y++)for(let x=0;x<width;x++){
    const color=colors[pixels[y][x]],at=y*(width*4+1)+1+x*4;
    if(color){Buffer.from(color.slice(1),'hex').copy(rows,at);rows[at+3]=255;}
  }
  const h=Buffer.alloc(13);h.writeUInt32BE(width,0);h.writeUInt32BE(height,4);h[8]=8;h[9]=6;
  return Buffer.concat([Buffer.from([137,80,78,71,13,10,26,10]),chunk('IHDR',h),chunk('IDAT',zlib.deflateSync(rows)),chunk('IEND',Buffer.alloc(0))]);
}
const files=[];
for(const icon of art.icons){
  const {sprite:s,key,name}=icon,file=key+'-32.png';fs.writeFileSync(path.join(output,file),encode(s));
  files.push({key,name,file,width:32,height:32,models:Object.entries(art.mapping).filter(([,k])=>k===key).map(([id])=>id)});
  const runs=[];s.pixels.forEach((row,y)=>{for(let x=0;x<32;){const index=row[x];let end=x+1;while(end<32&&row[end]===index)end++;if(index)runs.push(`<path fill="${s.colors[index]}" d="M${x} ${y}h${end-x}v1H${x}Z"/>`);x=end;}});
  fs.writeFileSync(path.join(output,key+'.svg'),`<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 32 32" shape-rendering="crispEdges">${runs.join('')}</svg>\n`);
}
fs.writeFileSync(path.join(output,'manifest.json'),JSON.stringify({creation:art.creation,icons:files.length,models:Object.keys(art.mapping).length,files},null,2)+'\n');
fs.writeFileSync(path.join(output,'native-grid-source.json'),JSON.stringify(art,null,2)+'\n');
console.log(JSON.stringify({icons:files.length,models:Object.keys(art.mapping).length}));
