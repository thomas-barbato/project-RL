// Presentation only: keep the exported drawings unchanged at their exact dimensions.
const fs=require('node:fs'),path=require('node:path');
const sharp=require(process.env.RL_SHARP_MODULE||'sharp');
const out=__dirname;
const r=JSON.parse(fs.readFileSync(path.join(out,'review.json'),'utf8'));
const c=r.candidates[0];
if(c.variant!=='HL04-C04')throw Error('Unexpected candidate');
const png=(size,x,y)=>'<image x="'+x+'" y="'+y+'" width="'+size+'" height="'+size+'" href="data:image/png;base64,'+fs.readFileSync(path.join(out,c.files[size].file)).toString('base64')+'"/>';
async function main(){
  const pieces=['<rect width="460" height="420" fill="#0b1017"/>','<text x="22" y="30" font-size="14" fill="#b6a8ff">81 · HL04-C04 · nouvelle proposition</text>','<text x="22" y="61" font-size="22" fill="#e1e7ec">Maîtrise des armes lourdes</text>','<text x="22" y="88" font-size="14" fill="#a6b6c5">Le même chevron bleu que la maîtrise à distance.</text>',png(128,166,109),'<text x="230" y="260" text-anchor="middle" font-size="12" fill="#a6b6c5">Détail · 128 px</text>'];
  for(const [size,x]of [[32,74],[64,198],[24,350]]){pieces.push(png(size,x,351-size),'<text x="'+(x+size/2)+'" y="376" text-anchor="middle" font-size="12" fill="#a6b6c5">'+size+' px</text>');}
  pieces.push('<text x="22" y="405" font-size="11" fill="#a6b6c5">SKILLS-IC47 · vues natives · à valider avant intégration</text>');
  const svg='<svg xmlns="http://www.w3.org/2000/svg" width="460" height="420"><g font-family="Segoe UI,Arial">'+pieces.join('')+'</g></svg>';
  fs.writeFileSync(path.join(out,'native-review-board.svg'),svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-review-board.png'));
  fs.writeFileSync(path.join(out,'index.html'),'<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>81 · Maîtrise des armes lourdes · chevron bleu</title><style>*{box-sizing:border-box}body{padding:24px;margin:0;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial}main{max-width:600px;margin:auto}h1{font-size:25px}p,small{color:#a6b6c5;line-height:1.6}a{color:#d7cef4}article{max-width:460px;background:#111923;border:1px solid #3a4654;border-radius:10px;padding:22px}.detail{display:flex;justify-content:center;margin:20px 0}.native{display:flex;align-items:end;justify-content:center;gap:46px}.native>div{display:grid;gap:10px;justify-items:center;font-size:12px;color:#a6b6c5}img{display:block}footer{margin-top:22px;line-height:1.7}</style><main><small>SKILLS-IC47 · 81 · HL04-C04</small><h1>Maîtrise des armes lourdes</h1><p>Le double chevron bleu reprend celui de Maîtrise des attaques à distance.</p><article><div class="detail"><img src="'+c.files[128].file+'" width="128" height="128" alt="Maîtrise des armes lourdes avec un chevron bleu"></div><div class="native">'+[32,64,24].map(size=>'<div><img src="'+c.files[size].file+'" width="'+size+'" height="'+size+'" alt="Icône '+size+' px"><span>'+size+' px</span></div>').join('')+'</div></article><footer>Cette nouvelle variante attend sa validation visuelle.<br><a href="../skill-icon-review-44/index.html">Série Armes lourdes · les cinq autres dessins restent à valider</a><br><a href="prompts.txt">Prompt exact · Imagegen intégré</a></footer></main></html>');
  const p=path.join(out,'validation.json'),v=JSON.parse(fs.readFileSync(p,'utf8'));
  v.native_review_board={width:460,height:420,icons_embedded_at_native_sizes:true};
  fs.writeFileSync(p,JSON.stringify(v,null,2)+'\n');
  console.log('Candidate 81 laid out at native sizes.');
}
main().catch(e=>{console.error(e);process.exitCode=1});
