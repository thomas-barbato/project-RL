// Artifact layout only; all icon PNGs are embedded at their exact export sizes.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const sharp=require(process.env.RL_SHARP_MODULE||'sharp');
const out=__dirname,root=path.resolve(out,'../../..');
const review=JSON.parse(fs.readFileSync(path.join(out,'review.json'),'utf8'));
const esc=s=>String(s).replace(/[&<>"]/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;'}[c]));
const image=(file,x,y,size)=>'<image x="'+x+'" y="'+y+'" width="'+size+'" height="'+size+'" href="data:image/png;base64,'+fs.readFileSync(path.join(out,file)).toString('base64')+'"/>';
async function main(){
  const pieces=['<rect width="920" height="840" fill="#0b1017"/>','<text x="22" y="32" font-size="23" fill="#e1e7ec">Armes lourdes · cinq icônes restantes</text>','<text x="22" y="59" font-size="14" fill="#a6b6c5">78, 79, 80, 82, 83 · propositions à valider</text>','<text x="22" y="86" font-size="13" fill="#a6b6c5">Détail 128 px · petites vues à taille réelle : 32 / 64 / 24 px</text>'];
  for(const [i,c]of review.candidates.entries()){
    const x=22+(i%3)*298,y=118+Math.floor(i/3)*348;
    pieces.push('<rect x="'+x+'" y="'+y+'" width="276" height="330" rx="9" fill="#111923" stroke="#3a4654"/>','<text x="'+(x+16)+'" y="'+(y+26)+'" font-size="13" fill="#b6a8ff">'+c.number+' · '+esc(c.variant)+'</text>','<text x="'+(x+16)+'" y="'+(y+54)+'" font-size="17" fill="#e1e7ec">'+esc(c.title)+'</text>',image(c.files[128].file,x+74,y+82,128));
    for(const[size,dx]of[[32,36],[64,106],[24,212]])pieces.push(image(c.files[size].file,x+dx,y+292-size,size),'<text x="'+(x+dx+size/2)+'" y="'+(y+315)+'" text-anchor="middle" font-size="11" fill="#a6b6c5">'+size+' px</text>');
  }
  pieces.push('<text x="22" y="826" font-size="12" fill="#a6b6c5">SKILLS-IC49 · dessins inchangés depuis IC44 · Maîtrise des armes lourdes déjà intégrée</text>');
  const svg='<svg xmlns="http://www.w3.org/2000/svg" width="920" height="840"><g font-family="Segoe UI,Arial">'+pieces.join('')+'</g></svg>';
  fs.writeFileSync(path.join(out,'native-review-board.svg'),svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-review-board.png'));
  const cards=review.candidates.map(c=>'<article id="'+c.variant+'"><small>'+c.number+' · '+esc(c.variant)+'</small><h2>'+esc(c.title)+'</h2><div class="detail"><img src="'+c.files[128].file+'" width="128" height="128" alt="'+esc(c.title)+'"></div><div class="native">'+[32,64,24].map(size=>'<div><img src="'+c.files[size].file+'" width="'+size+'" height="'+size+'" alt="'+esc(c.title)+' '+size+' px"><span>'+size+' px</span></div>').join('')+'</div></article>').join('');
  fs.writeFileSync(path.join(out,'index.html'),'<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>SKILLS-IC49 · Armes lourdes restantes</title><style>*{box-sizing:border-box}body{margin:0;padding:22px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial}header,main,footer{max-width:920px;margin:auto}h1{font-size:24px}p,small{color:#a6b6c5;line-height:1.6}a{color:#b6a8ff}.gallery{display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:22px}article{padding:16px;border:1px solid #3a4654;border-radius:9px;background:#111923}h2{font-size:17px}small{color:#b6a8ff}.detail{display:flex;justify-content:center;margin:24px 0}.native{display:flex;gap:38px;align-items:end;justify-content:center}.native>div{display:grid;gap:10px;justify-items:center;font-size:11px;color:#a6b6c5}img{display:block}footer{margin-top:24px;line-height:1.7}@media(max-width:760px){.gallery{grid-template-columns:repeat(2,minmax(0,1fr))}}@media(max-width:490px){.gallery{grid-template-columns:1fr}}</style><header><small>SKILLS-IC49</small><h1>Armes lourdes · cinq icônes restantes</h1><p>Propositions 78, 79, 80, 82 et 83, inchangées depuis IC44.<br>Détail 128 px et vues natives 32 / 64 / 24 px. Maîtrise des armes lourdes est déjà validée et intégrée.</p></header><main class="gallery">'+cards+'</main><footer>Ces cinq dessins attendent leur validation.<br><a href="../skill-window-native-18/index.html">Voir les compétences dans le jeu</a></footer></html>');
  for(const p of review.protected_hashes){const hash=crypto.createHash('sha256').update(fs.readFileSync(path.join(root,p.file))).digest('hex');if(hash!==p.sha256)throw Error('Protected file changed: '+p.file);}
  const file=path.join(out,'validation.json'),v=JSON.parse(fs.readFileSync(file,'utf8'));
  v.native_review_board={width:920,height:840,columns:3,rows:2,icons_embedded_at_native_sizes:true};
  fs.writeFileSync(file,JSON.stringify(v,null,2)+'\n');
  console.log('Five pending drawings presented at native sizes; active assets unchanged.');
}
main().catch(e=>{console.error(e);process.exitCode=1});
