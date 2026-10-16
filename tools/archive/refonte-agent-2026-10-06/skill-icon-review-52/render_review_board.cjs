const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const sharp=require(process.env.RL_SHARP_MODULE||'sharp');
const out=__dirname,root=path.resolve(out,'../../..');
const review=JSON.parse(fs.readFileSync(path.join(out,'review.json'),'utf8'));
const esc=s=>String(s).replace(/[&<>"]/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;'}[c]));
const img=(file,x,y,size)=>'<image x="'+x+'" y="'+y+'" width="'+size+'" height="'+size+'" href="data:image/png;base64,'+fs.readFileSync(path.join(out,file)).toString('base64')+'"/>';
function wrap(text){const lines=[''];for(const word of text.split(' ')){const i=lines.length-1;if(lines[i]&&(lines[i]+' '+word).length>28)lines.push(word);else lines[i]+=(lines[i]?' ':'')+word;}return lines;}
async function main(){
  if(review.candidates.length!==5)throw Error('Expected five grenade candidates.');
  const pieces=['<rect width="1028" height="850" fill="#0b1017"/>','<text x="21" y="32" font-size="23" fill="#e1e7ec">92–96 · Grenades</text>','<text x="21" y="61" font-size="14" fill="#a6b6c5">Cinq nouvelles propositions · dernières icônes compactes du catalogue</text>','<text x="21" y="88" font-size="13" fill="#a6b6c5">Détail 128 px · petites vues à taille réelle : 32 / 64 / 24 px</text>'];
  for(const[i,c]of review.candidates.entries()){
    const x=i<3?21+i*334:188+(i-3)*334,y=i<3?116:464;
    pieces.push('<rect x="'+x+'" y="'+y+'" width="318" height="330" rx="9" fill="#111923" stroke="#3a4654"/>','<text x="'+(x+18)+'" y="'+(y+26)+'" font-size="13" fill="#b6a8ff">'+c.number+' · '+esc(c.variant)+'</text>');
    for(const[j,line]of wrap(c.title).entries())pieces.push('<text x="'+(x+18)+'" y="'+(y+53+j*20)+'" font-size="16" fill="#e1e7ec">'+esc(line)+'</text>');
    pieces.push(img(c.files[128].file,x+95,y+92,128));
    for(const[size,dx]of[[32,40],[64,124],[24,262]])pieces.push(img(c.files[size].file,x+dx,y+294-size,size),'<text x="'+(x+dx+size/2)+'" y="'+(y+317)+'" text-anchor="middle" font-size="11" fill="#a6b6c5">'+size+' px</text>');
  }
  pieces.push('<text x="21" y="829" font-size="12" fill="#a6b6c5">SKILLS-IC52 · cinq dessins soumis avant intégration · noms, règles et effets inchangés</text>');
  const svg='<svg xmlns="http://www.w3.org/2000/svg" width="1028" height="850"><g font-family="Segoe UI,Arial">'+pieces.join('')+'</g></svg>';
  fs.writeFileSync(path.join(out,'native-review-board.svg'),svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-review-board.png'));
  const cards=review.candidates.map(c=>'<article id="'+c.variant+'"><small>'+c.number+' · '+esc(c.variant)+'</small><h2>'+esc(c.title)+'</h2><div class="detail"><img src="'+c.files[128].file+'" width="128" height="128" alt="'+esc(c.title)+'"></div><div class="native">'+[32,64,24].map(size=>'<div><img src="'+c.files[size].file+'" width="'+size+'" height="'+size+'" alt="'+esc(c.title)+' '+size+' px"><span>'+size+' px</span></div>').join('')+'</div></article>').join('');
  fs.writeFileSync(path.join(out,'index.html'),'<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>92–96 · Grenades</title><style>*{box-sizing:border-box}body{margin:0;padding:22px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial}header,main,footer{max-width:1028px;margin:auto}h1{font-size:24px}p,small{color:#a6b6c5;line-height:1.6}a{color:#b6a8ff}.gallery{display:flex;justify-content:center;flex-wrap:wrap;gap:16px}article{flex:0 1 calc((100% - 32px)/3);padding:18px;border:1px solid #3a4654;border-radius:9px;background:#111923;min-width:260px}h2{font-size:16px;min-height:44px}small{color:#b6a8ff}.detail{display:flex;justify-content:center;margin:20px 0}.native{display:flex;gap:32px;align-items:end;justify-content:center}.native>div{display:grid;gap:10px;justify-items:center;font-size:11px;color:#a6b6c5}img{display:block;flex-shrink:0}footer{margin-top:24px;line-height:1.7}@media(max-width:860px){article{flex-basis:calc((100% - 16px)/2)}}@media(max-width:550px){article{flex-basis:100%;min-width:0}}</style><header><small>SKILLS-IC52</small><h1>92–96 · Grenades</h1><p>Cinq nouvelles propositions : dernières icônes compactes du catalogue.<br>Détail 128 px et vues natives 32 / 64 / 24 px.</p></header><main class="gallery">'+cards+'</main><footer>Ces cinq dessins attendent leur validation.<br><a href="../skill-window-native-21/index.html">Furtivité intégrée · capture réelle</a> · <a href="prompts.txt">Prompts exacts · Imagegen intégré</a></footer></html>');
  for(const p of review.protected_hashes){const hash=crypto.createHash('sha256').update(fs.readFileSync(path.join(root,p.file))).digest('hex');if(hash!==p.sha256)throw Error('Protected file changed: '+p.file);}
  const file=path.join(out,'validation.json'),v=JSON.parse(fs.readFileSync(file,'utf8'));
  v.native_review_board={width:1028,height:850,columns:3,rows:2,lower_row_centered:true,icons_embedded_at_native_sizes:true};
  fs.writeFileSync(file,JSON.stringify(v,null,2)+'\n');
  console.log('Five grenade proposals arranged at native sizes; accepted files unchanged.');
}
main().catch(e=>{console.error(e);process.exitCode=1});
