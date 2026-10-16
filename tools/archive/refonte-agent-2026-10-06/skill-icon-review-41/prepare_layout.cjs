// Review layout only. Generated artwork is not repainted or recolored here.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const sharp=require(process.env.RL_SHARP_MODULE||'sharp');
const out=__dirname,root=path.resolve(out,'../../..');
const read=p=>JSON.parse(fs.readFileSync(p,'utf8').replace(/^\uFEFF/,''));
const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
async function main(){
  const review=read(path.join(out,'review.json'));
  if(review.status!=='awaiting_visual_review'||review.candidates.length!==1)throw Error('Expected one pending candidate');
  const pagePath=path.join(out,'index.html');
  let page=fs.readFileSync(pagePath,'utf8');
  page=page.replace('max-width:1040px','max-width:540px').replace('repeat(4,minmax(0,1fr))','1fr').replace('PROPOSITIONS 69–69','PROPOSITION 69');
  page=page.replace('</footer>','<br>Échange de position et Lien de domination restent inchangés et à valider. <a href="../skill-icon-review-39/index.html">Revoir les autres pouvoirs hybrides</a>.</footer>');
  page+='<style>.gallery{grid-template-columns:1fr}.refs{flex-wrap:wrap}h2{min-height:0}</style>';
  fs.writeFileSync(pagePath,page);
  const c=review.candidates[0];
  const svgImage=(size,x,y)=>`<image x="${x}" y="${y}" width="${size}" height="${size}" href="data:image/png;base64,${fs.readFileSync(path.join(out,c.files[size].file)).toString('base64')}"/>`;
  const pieces=['<rect width="540" height="420" fill="#0b1017"/>','<text x="24" y="34" font-size="22" fill="#e1e7ec">69 · Décharge mutagène</text>','<text x="24" y="61" font-size="13" fill="#b6a8ff">M02-C03 · énergie verte, sans rouge</text>',svgImage(128,206,86)];
  for(const[s,x]of[[32,138],[64,240],[24,380]])pieces.push(svgImage(s,x,294-s),`<text x="${x+s/2}" y="319" text-anchor="middle" font-size="12" fill="#a6b6c5">${s} px</text>`);
  pieces.push('<text x="24" y="369" font-size="13" fill="#a6b6c5">Petites vues à taille réelle · proposition à valider.</text>');
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="540" height="420"><g font-family="Segoe UI,Arial">${pieces.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'native-review-board.svg'),svg);await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-review-board.png'));
  for(const f of Object.values(c.files))if(hash(path.join(out,f.file))!==f.sha256)throw Error('Artwork changed');
  for(const f of review.protected_hashes)if(hash(path.join(root,f.file))!==f.sha256)throw Error('Active bank changed');
  const validation=read(path.join(out,'validation.json'));validation.native_review_board={width:540,height:420};
  fs.writeFileSync(path.join(out,'validation.json'),JSON.stringify(validation,null,2)+'\n');
  fs.writeFileSync(path.join(out,'README.md'),'# SKILLS-IC41 · Décharge mutagène sans rouge\n\nLe joueur n’apprécie pas la palette verte et bordeaux de M02-C02. Nouvelle proposition 69 M02-C03 : une décharge verte plus compacte, sans rouge, avec cœur pâle et ombres vertes. Ce changement concerne uniquement l’icône, pas le type de dégâts ni l’animation du sort.\n\nImagegen intégré ; prompt exact dans prompts/M02-C03.txt, master et cinq exports archivés. Aucun redessin par code. Vues natives 32, 64 et 24 px, détail 128 px. Les dessins précédents sont conservés.\n\nM02-C03 reste à valider. La banque active de 66 compactes et 16 illustrations ne change pas. HY04-C01 et HY06-C01 restent inchangés et non validés.\n');
  console.log(JSON.stringify({review:'SKILLS-IC41',candidate:'M02-C03',protected_files:review.protected_hashes.length}));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
