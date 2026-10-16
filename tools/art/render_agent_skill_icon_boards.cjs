// Render the code-native SVG candidates at their actual review sizes.
// These contact sheets do not activate any asset in the game.
const fs = require('node:fs');
const path = require('node:path');
const sharp = require(process.env.RL_SHARP_MODULE || 'sharp');
const root = path.resolve(__dirname, '../..');
const out = path.join(root, 'artifacts/refonte-agent-2026-10-06/skill-window-review-01');
const data = JSON.parse(fs.readFileSync(path.join(out, 'skills.json'), 'utf8'));
const pages = [
  ['Combat et technologie', ['Mêlée', 'Survie', 'Drones', 'Affaiblissements', 'Attaques à distance', 'Esquive']],
  ['Magie et métamorphose', ['Feu', 'Froid', 'Énergie occulte', 'Transformations', 'Pouvoirs hybrides']],
  ['Renforcements et outils', ['Buffs de mêlée', 'Buffs à distance', 'Armes lourdes', 'Furtivité', 'Mines', 'Grenades']],
];
const esc = text => String(text).replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&apos;'}[c]));
const text = (x, y, value, size=13, color='#e1e7ec', weight='normal') => `<text x="${x}" y="${y}" font-size="${size}" fill="${color}" font-weight="${weight}">${esc(value)}</text>`;
function wrap(value, length=31) {
  const lines = [''];
  for (const word of value.split(' ')) {
    if (lines.at(-1) && (lines.at(-1)+' '+word).length>length) lines.push(word);
    else lines[lines.length-1] += (lines.at(-1)?' ':'')+word;
  }
  return lines;
}
async function main() {
  const rendered = new Map();
  const crypto = require('node:crypto');
  const assets = [];
  for (const size of [64,32,24]) fs.mkdirSync(path.join(out,`icons-${size}`),{recursive:true});
  for (const s of data.skills) {
    for (const size of [64,32,24]) {
      const png = await sharp(path.join(out, 'icons', `${s.id}.svg`)).resize(size,size).png().toBuffer();
      rendered.set(`${s.id}:${size}`, `data:image/png;base64,${png.toString('base64')}`);
      const file=`icons-${size}/${s.id}.png`;
      fs.writeFileSync(path.join(out,file),png);
      assets.push({id:s.id,size,file,sha256:crypto.createHash('sha256').update(png).digest('hex')});
    }
  }
  const boards = [];
  for (const [index,[title,branches]] of pages.entries()) {
    const pieces = [text(26,36,'SKILLS-IC01 · '+title,24,'#e1e7ec','600'),text(26,62,'Propositions à valider · 64 px / 32 px / 24 px · mêmes dessins dans la fenêtre et la barre d’état',13,'#97a7b5')];
    let y=104, count=0;
    for (const branch of branches) {
      pieces.push(text(26,y,branch,18,'#b6a8ff','600')); y+=18;
      const skills=data.skills.filter(s=>s.branch===branch); count+=skills.length;
      for (const [i,s] of skills.entries()) {
        const x=26+(i%4)*278, cy=y+Math.floor(i/4)*154;
        pieces.push(`<rect x="${x}" y="${cy}" width="264" height="142" rx="8" fill="#101721" stroke="#354350"/>`);
        for (const [size,dx] of [[64,16],[32,108],[24,166]]) {
          pieces.push(`<image x="${x+dx}" y="${cy+13+(64-size)/2}" width="${size}" height="${size}" href="${rendered.get(`${s.id}:${size}`)}"/>`);
          pieces.push(text(x+dx,cy+94,size+' px',10,'#97a7b5'));
        }
        pieces.push(text(x+214,cy+30,s.id,12,'#b6a8ff','600'));
        wrap(s.title).forEach((line,j)=>pieces.push(text(x+16,cy+115+j*17,line,12)));
      }
      y+=Math.ceil(skills.length/4)*154+28;
    }
    pieces.push(text(26,y,'Planche '+(index+1)+' / 3 · '+count+' icônes · repères = identifiants des compétences',12,'#97a7b5'));
    const svg=`<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="1150" height="${y+26}" viewBox="0 0 1150 ${y+26}"><rect width="100%" height="100%" fill="#0b1017"/><g font-family="Segoe UI,Arial,sans-serif">${pieces.join('')}</g></svg>`;
    const name=`icon-board-${index+1}`;
    fs.writeFileSync(path.join(out,`${name}.svg`),svg);
    await sharp(Buffer.from(svg)).png().toFile(path.join(out,`${name}.png`));
    boards.push({file:`${name}.png`,title,count,width:1150,height:y+26});
  }
  fs.writeFileSync(path.join(out,'icon-review.json'),JSON.stringify({review_id:'SKILLS-IC01',status:'awaiting_visual_review',scope:'icon_drawings_only',count:94,sizes:[64,32,24],boards,icons:data.skills.map(s=>({id:s.id,title:s.title,file:`icons/${s.id}.svg`})),raster_assets:assets},null,2)+'\n');
  console.log(JSON.stringify({review_id:'SKILLS-IC01',boards}));
}
main().catch(error=>{console.error(error);process.exitCode=1;});
