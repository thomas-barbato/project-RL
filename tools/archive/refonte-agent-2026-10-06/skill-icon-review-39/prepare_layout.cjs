// Layout and resampling only; the generated paintings remain unchanged.
const fs = require('node:fs'), path = require('node:path'), crypto = require('node:crypto');
const sharp = require(process.env.RL_SHARP_MODULE || 'sharp');
const out = __dirname, root = path.resolve(out, '../../..');
const read = p => JSON.parse(fs.readFileSync(p, 'utf8').replace(/^\uFEFF/, ''));
const digest = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
async function main() {
  const review = read(path.join(out, 'review.json'));
  if (review.status !== 'awaiting_visual_review' || review.candidates.length !== 3) throw Error('Expected three pending candidates');
  const pagePath = path.join(out, 'index.html');
  let page = fs.readFileSync(pagePath, 'utf8');
  page = page.replace('max-width:1040px', 'max-width:804px').replace('repeat(4,minmax(0,1fr))', 'repeat(3,minmax(0,1fr))');
  fs.writeFileSync(pagePath, page);
  const boardPath = path.join(out, 'native-review-board.svg');
  let board = fs.readFileSync(boardPath, 'utf8');
  board = board.replaceAll('width="1040"', 'width="804"').replaceAll('height="860"', 'height="520"').replace('y="842"', 'y="500"');
  fs.writeFileSync(boardPath, board);
  await sharp(Buffer.from(board)).png().toFile(path.join(out, 'native-review-board.png'));
  const validation = read(path.join(out, 'validation.json'));
  validation.native_review_board = {width:804,height:520};
  for (const c of review.candidates) {
    for (const f of Object.values(c.files)) if (digest(path.join(out,f.file)) !== f.sha256) throw Error('Painting export modified');
  }
  for (const p of review.protected_hashes) if (digest(path.join(root,p.file)) !== p.sha256) throw Error('Runtime changed during review');
  fs.writeFileSync(path.join(out, 'validation.json'), JSON.stringify(validation,null,2)+'\n');
  fs.writeFileSync(path.join(out, 'README.md'), '# SKILLS-IC39 · Pouvoirs hybrides\n\nTrois candidats nouveaux, non intégrés et à valider : 69 M02-C01 Décharge mutagène, 70 HY04-C01 Échange de position et 71 HY06-C01 Lien de domination. Les noms et textes restent ceux du catalogue canonique. Aucune mécanique nouvelle.\n\nGénération par Imagegen intégré, une image par compétence. Deux références de style uniquement : Puissance métamorphe M03-C05 et Faille d’armure AF02-C02. Prompts exacts dans prompts.txt et prompts/. Masters copiés dans ce dossier ; exports 192, 128, 64, 32 et 24 px, sans redessin par code.\n\nPetites vues à taille réelle dans index.html et native-review-board.png (804 × 520). La revue contrôle 411 exports antérieurs validés et protège 86 fichiers de la banque actuelle : 66 compactes et 16 illustrations. Les quatre transformations validées sont montrées dans ../skill-window-native-14/index.html.\n');
  console.log(JSON.stringify({candidate_count:3,board:[804,520],protected_files:review.protected_hashes.length}));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
