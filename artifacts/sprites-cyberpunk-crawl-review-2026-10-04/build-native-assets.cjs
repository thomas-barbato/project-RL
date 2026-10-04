// Encode the exact authored pixel grids. No image scaling or image library.
const fs = require('node:fs');
const path = require('node:path');
const zlib = require('node:zlib');
const art = require('./native-art.js');
const output = path.join(__dirname, 'native');
fs.mkdirSync(output, { recursive: true });
function crc32(bytes) {
  let crc = 0xffffffff;
  for (const byte of bytes) { crc ^= byte; for (let i=0;i<8;i++) crc = (crc>>>1) ^ (crc&1 ? 0xedb88320 : 0); }
  return (crc ^ 0xffffffff) >>> 0;
}
function chunk(type, data) {
  const name = Buffer.from(type), length = Buffer.alloc(4), checksum = Buffer.alloc(4);
  length.writeUInt32BE(data.length); checksum.writeUInt32BE(crc32(Buffer.concat([name,data])));
  return Buffer.concat([length,name,data,checksum]);
}
function encode(sprite) {
  const {width,height,pixels,colors} = sprite;
  if (pixels.length !== height || pixels.some(row => row.length !== width)) throw new Error('Invalid native grid');
  const rows = Buffer.alloc(height * (width*4+1));
  let opaque = 0;
  for (let y=0;y<height;y++) for(let x=0;x<width;x++) {
    const index = pixels[y][x], color = colors[index];
    if (!Number.isInteger(index) || index < 0 || index >= colors.length) throw new Error('Invalid palette index');
    const at = y*(width*4+1)+1+x*4;
    if (color) { const rgb = Buffer.from(color.slice(1),'hex'); rgb.copy(rows,at); rows[at+3]=255; opaque++; }
  }
  if (!opaque) throw new Error('Empty native sprite');
  const header = Buffer.alloc(13); header.writeUInt32BE(width,0); header.writeUInt32BE(height,4); header[8]=8; header[9]=6;
  return Buffer.concat([Buffer.from([137,80,78,71,13,10,26,10]),chunk('IHDR',header),chunk('IDAT',zlib.deflateSync(rows)),chunk('IEND',Buffer.alloc(0))]);
}
const files = [];
function save(file,sprite) {
  fs.writeFileSync(path.join(output,file),encode(sprite));
  files.push({file,width:sprite.width,height:sprite.height,colors:sprite.colors.length-1,kind:sprite.kind});
}
for(const [id,sprite] of Object.entries(art.items)) save(id.split(':')[1]+'-'+sprite.width+'.png',sprite);
art.floors.forEach((family,f)=>family.forEach((sprite,v)=>save('floor-'+f+'-'+v+'-32.png',sprite)));
save('wall-32.png',art.wall);
fs.writeFileSync(path.join(output,'native-grid-source.json'),JSON.stringify(art,null,2)+'\n');
fs.writeFileSync(path.join(output,'manifest.json'),JSON.stringify({creation:art.creation,files},null,2)+'\n');
console.log(JSON.stringify({native_items:4,floors:16,walls:1,item_pixels:32,actor_source:'classes-v1.png (previous appearances)',source:art.creation}));
