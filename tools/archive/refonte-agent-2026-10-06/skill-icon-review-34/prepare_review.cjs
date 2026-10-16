// Review presentation only; candidate artwork is never painted or altered here.
const fs = require('node:fs'), path = require('node:path'), crypto = require('node:crypto');
const sharp = require(process.env.RL_SHARP_MODULE || 'sharp');
const out = __dirname, root = path.resolve(out,'../../..');
const read = f => JSON.parse(fs.readFileSync(f,'utf8').replace(/^\uFEFF/,''));
const digest = f => crypto.createHash('sha256').update(fs.readFileSync(f)).digest('hex');
const review = read(path.join(out,'review.json'));
const ledger = read(path.join(out,'../skill-icon-acceptance.json'));
const kept = ['M01','M03'].map(id=>ledger.compact_accepted.find(e=>e.id===id));
const img = (f,x,y,s) => `<image x="${x}" y="${y}" width="${s}" height="${s}" href="data:image/png;base64,${fs.readFileSync(f).toString('base64')}"/>`;
async function main() {
  const boardPath=path.join(out,'native-review-board.svg');
  const svg=fs.readFileSync(boardPath,'utf8').replaceAll('height="860"','height="520"').replace('y="842"','y="500"');
  fs.writeFileSync(boardPath,svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-review-board.png'));
  const all=[...kept.map(e=>({title:e.title,file:path.resolve(root,e.files['64'].file),approved:true})),...review.candidates.map(c=>({title:c.title,file:path.join(out,c.files['64'].file),approved:false}))];
  const parts=['<rect width="480" height="334" fill="#10171e"/>','<text x="20" y="28" font-size="19" fill="#dde5e9">Transformations · aperçu des icônes</text>'];
  all.forEach((c,i)=>{
    const x=28+(i%3)*150,y=52+Math.floor(i/3)*126;
    parts.push(img(c.file,x+34,y,64));
    const words=c.title.split(' '), lines=c.title.length>20?[words.slice(0,-1).join(' '),words.at(-1)]:[c.title];
    lines.forEach((l,j)=>parts.push(`<text x="${x+66}" y="${y+85+j*17}" text-anchor="middle" font-size="13" fill="#dde5e9">${l}</text>`));
  });
  parts.push('<text x="20" y="318" font-size="12" fill="#a4b2bd">64 px · deux dessins validés et quatre propositions</text>');
  const tree=`<svg xmlns="http://www.w3.org/2000/svg" width="480" height="334"><g font-family="Segoe UI,Arial">${parts.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'skill-tree-context.svg'),tree);
  await sharp(Buffer.from(tree)).png().toFile(path.join(out,'skill-tree-context.png'));
  let html=fs.readFileSync(path.join(out,'index.html'),'utf8');
  html=html.replace('<footer>','<section class="context"><h2>Dans l’arbre · icônes à 64 px</h2><p>Mue prédatrice et Puissance métamorphe conservent leurs dessins validés.</p><img src="skill-tree-context.png" width="480" height="334" alt="Six icônes de transformation à 64 px"></section><footer>');
  html=html.replace('</style>','.context{max-width:1040px;margin:24px auto}.context>img{width:480px;height:334px;max-width:100%;object-fit:contain;object-position:left top}.context h2{min-height:0}.context p{font-size:13px}</style>');
  fs.writeFileSync(path.join(out,'index.html'),html);
  for(const f of review.protected_hashes) if(digest(path.resolve(root,f.file))!==f.sha256)throw Error('Protected file changed: '+f.file);
  for(const c of review.candidates) for(const f of c.reference_hashes) if(digest(f.file)!==f.sha256)throw Error('Design/style reference changed: '+f.file);
  const validation=read(path.join(out,'validation.json'));
  validation.native_review_board={width:1040,height:520};
  validation.skill_tree_context={width:480,height:334,native_icon_size:64};
  validation.existing_transformation_icons_preserved=kept.map(c=>c.variant);
  fs.writeFileSync(path.join(out,'validation.json'),JSON.stringify(validation,null,2)+'\n');
  fs.writeFileSync(path.join(out,'README.md'),'# IC34 · Transformations\n\nPropositions compactes 65–68 : Colosse biomécanique TR61-C01, Cuirassé TR63-C01, Possédé TR64-C01 et Abysse TR65-C01. Aucun de ces quatre dessins n’est intégré avant sa validation propre.\n\nSources des formes existantes : artifacts/allies-transformations-proposition-2026-10-06/sources/. Seule leur figure vivante sert de référence de dessin. Mue prédatrice et Puissance métamorphe conservent les compactes exactes acceptées.\n\nGénération intégrée image_gen.imagegen, une image par compétence ; prompts exacts dans prompts.txt et prompts/. Masters copiés dans le dossier de revue. Exports Lanczos à 192, 128, 64, 32 et 24 px ; aucune peinture, rotation ou modification des dessins par code.\n\nRevue : index.html, native-review-board.png et skill-tree-context.png. Les formes sont des emblèmes rapprochés lisibles à 32 px. Cette revue ne modifie ni les sprites des transformations, ni leurs textes, ni leurs règles de jeu.\n');
  console.log(JSON.stringify({review_id:review.review_id,candidates:4,protected_files:review.protected_hashes.length,board:validation.native_review_board}));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
