// Layout only: embed the unmodified exported PNGs at their stated dimensions.
const fs = require('node:fs'), path = require('node:path');
const sharp = require(process.env.RL_SHARP_MODULE || 'sharp');
const out = __dirname;
const review = JSON.parse(fs.readFileSync(path.join(out, 'review.json'), 'utf8'));
const esc = s => String(s).replace(/[&<>\"]/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','\"':'&quot;'}[c]));
const image = (file,x,y,size) => '<image x="'+x+'" y="'+y+'" width="'+size+'" height="'+size+'" href="data:image/png;base64,'+fs.readFileSync(path.join(out,file)).toString('base64')+'"/>';
async function main() {
  const pieces = ['<rect width="920" height="840" fill="#0b1017"/>', '<text x="22" y="32" font-size="23" fill="#e1e7ec">78–83 · Armes lourdes</text>', '<text x="22" y="59" font-size="14" fill="#a6b6c5">Six propositions à valider · style sombre et symboles compacts</text>', '<text x="22" y="86" font-size="13" fill="#a6b6c5">Détail 128 px · petites vues à taille réelle : 32 / 64 / 24 px</text>'];
  for (const [i, c] of review.candidates.entries()) {
    const x=22+(i%3)*298, y=118+Math.floor(i/3)*348;
    pieces.push('<rect x="'+x+'" y="'+y+'" width="276" height="330" rx="9" fill="#111923" stroke="#3a4654"/>', '<text x="'+(x+16)+'" y="'+(y+26)+'" font-size="13" fill="#b6a8ff">'+c.number+' · '+esc(c.variant)+'</text>', '<text x="'+(x+16)+'" y="'+(y+54)+'" font-size="17" fill="#e1e7ec">'+esc(c.title)+'</text>');
    pieces.push(image(c.files[128].file,x+74,y+82,128));
    for (const [size,dx] of [[32,36],[64,106],[24,212]]) {
      pieces.push(image(c.files[size].file,x+dx,y+292-size,size), '<text x="'+(x+dx+size/2)+'" y="'+(y+315)+'" text-anchor="middle" font-size="11" fill="#a6b6c5">'+size+' px</text>');
    }
  }
  pieces.push('<text x="22" y="826" font-size="12" fill="#a6b6c5">SKILLS-IC44 · dessins proposés avant intégration · aucune modification des textes ou effets</text>');
  const svg='<svg xmlns="http://www.w3.org/2000/svg" width="920" height="840"><g font-family="Segoe UI,Arial">'+pieces.join('')+'</g></svg>';
  fs.writeFileSync(path.join(out,'native-review-board.svg'),svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-review-board.png'));
  const p=path.join(out,'validation.json'), validation=JSON.parse(fs.readFileSync(p,'utf8'));
  validation.native_review_board={width:920,height:840,columns:3,rows:2,icons_embedded_at_native_sizes:true};
  fs.writeFileSync(p,JSON.stringify(validation,null,2)+'\n');
  console.log('Six candidates arranged; icon exports unchanged.');
}
main().catch(error=>{console.error(error);process.exitCode=1});
