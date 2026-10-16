// Condition image_gen output for a targeted icon review; no gameplay activation.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const sharp = require(process.env.RL_SHARP_MODULE || 'sharp');
const base = path.resolve(__dirname, '../../artifacts/refonte-agent-2026-10-06');
const previous = path.join(base, 'skill-icon-review-03');
const out = path.join(base, 'skill-icon-review-04');
const hash = b => crypto.createHash('sha256').update(b).digest('hex');
const readJson = p => JSON.parse(fs.readFileSync(p, 'utf8').replace(/^\uFEFF/, ''));
const esc = s => String(s).replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&apos;'}[c]));
async function main() {
  const source = process.argv[2];
  if (!source) throw Error('Pass the image_gen output path.');
  fs.mkdirSync(out, {recursive:true});
  const prior = readJson(path.join(previous, 'review.json'));
  if (!prior.SF00.approval) throw Error('SF00-v2 approval must be recorded first.');
  const specs = [
    {id:'D59-v3', title:'Version précédente', status:'revision_required'},
    {id:'D59-v4', title:'Flamme et métal corrigés', status:'awaiting_visual_review'}
  ];
  for (const c of specs) {
    c.files = {};
    if (c.id === 'D59-v4') {
      fs.copyFileSync(source, path.join(out, 'D59-v4-master.png'));
      const meta = await sharp(source).metadata();
      if (meta.width !== meta.height) throw Error('The icon must be square.');
      c.master = {file:'D59-v4-master.png', width:meta.width, height:meta.height, sha256:hash(fs.readFileSync(source))};
    }
    for (const size of [192,128,64,32,24]) {
      const file = `${c.id}-${size}.png`;
      if (c.id === 'D59-v3') fs.copyFileSync(path.join(previous, file), path.join(out, file));
      else await sharp(source).resize(size, size, {kernel:'lanczos3'}).png().toFile(path.join(out, file));
      c.files[size] = {file, sha256:hash(fs.readFileSync(path.join(out, file)))};
    }
  }
  for (const c of prior.candidates) for (const f of Object.values(c.files)) {
    if (hash(fs.readFileSync(path.join(previous, f.file))) !== f.sha256) throw Error('Earlier candidate changed: '+c.id);
  }
  for (const [id, files] of Object.entries(prior.accepted_source_assets)) for (const f of Object.values(files)) {
    if (hash(fs.readFileSync(path.join(base, 'skill-icon-review-02', f.file))) !== f.sha256) throw Error('Accepted icon changed: '+id);
  }
  const sf = prior.candidates.find(c => c.id === 'SF00-v2');
  const review = {
    review_id:'SKILLS-IC04', date:'2026-10-10', status:'awaiting_drone_visual_review',
    skill_id:'D59', user_feedback:"Concernant le drone, le feu de son canon n'englobe pas son canon et il y a des artefacte jaune sur le s2 armes",
    scope:'Right muzzle flash geometry and yellow artifacts on both weapons only. Barrels retain their image-right direction. No gameplay or skill copy changes.',
    generation_tool:'image_gen.imagegen', prompt:'prompt-D59.txt', candidates:specs,
    accepted_unchanged:[...prior.accepted_unchanged, 'SF00-v2'],
    SF00:{status:'accepted', candidate:'SF00-v2', user_approval:'je valide la furtivité aussi.', approval_date:'2026-10-10', source_review:'SKILLS-IC03', files:sf.files, master:sf.master},
    accepted_source_assets:prior.accepted_source_assets,
    verification:{previous_candidate_hashes_preserved:true, seven_accepted_icons_preserved:true},
    packaging:'Source copied unchanged; Lanczos3 size exports. No semantic edits during packaging.'
  };
  fs.writeFileSync(path.join(out,'review.json'), JSON.stringify(review,null,2)+'\n');
  const text = (x,y,t,size=13,color='#e1e7ec') => `<text x="${x}" y="${y}" font-size="${size}" fill="${color}">${esc(t)}</text>`;
  const pieces = [text(24,34,'SKILLS-IC04 · Drone Artilleur',25),text(24,60,'Flamme raccordée à la bouche du canon · traces jaunes retirées du métal',13,'#a6b6c5')];
  for (const [i,c] of specs.entries()) {
    const x=24+i*354, y=85;
    pieces.push(`<rect x="${x}" y="${y}" width="340" height="366" rx="9" fill="#111923" stroke="#3a4654"/>`,text(x+15,y+25,c.id,12,'#b6a8ff'),text(x+15,y+48,c.title,15));
    for (const [s,dx,dy] of [[192,74,62],[64,24,277],[32,149,293],[24,249,297]]) {
      const png=fs.readFileSync(path.join(out,c.files[s].file)).toString('base64');
      pieces.push(`<image x="${x+dx}" y="${y+dy}" width="${s}" height="${s}" href="data:image/png;base64,${png}"/>`,text(x+dx,y+dy+s+16,s+' px',10,'#a6b6c5'));
    }
  }
  pieces.push(text(24,480,'Furtivité SF00-v2 validée. Les sept icônes acceptées restent intactes.',12,'#a6b6c5'));
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="744" height="500"><rect width="100%" height="100%" fill="#0b1017"/><g font-family="Segoe UI,Arial,sans-serif">${pieces.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'comparison-board.svg'),svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out,'comparison-board.png'));
  const cards=specs.map(c=>`<article><h2><code>${c.id}</code>${esc(c.title)}</h2><img class="large" src="${c.files[192].file}" width="192" height="192" alt="${esc(c.title)}"><div class="native">${[64,32,24].map(s=>`<div><img src="${c.files[s].file}" width="${s}" height="${s}" alt=""><span>${s} px</span></div>`).join('')}</div></article>`).join('');
  fs.writeFileSync(path.join(out,'index.html'),`<!doctype html><html lang="fr"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>SKILLS-IC04 · Canon et flamme</title><style>*{box-sizing:border-box}body{margin:0;padding:24px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial,sans-serif}header,main,footer{max-width:920px;margin:auto}header{margin-bottom:24px}small,code{color:#b6a8ff}h1{font-size:26px;margin:10px 0}p,footer{color:#a6b6c5;font-size:13px;line-height:1.5}.pair{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:18px}article{min-width:0;background:#111923;border:1px solid #3a4654;border-radius:10px;padding:18px}h2{font-size:16px;line-height:1.4;margin:0 0 17px}h2 code{display:block;font:12px Segoe UI,Arial;margin-bottom:5px}.large{display:block;margin:0 auto 22px;border-radius:7px}.native{display:flex;gap:27px;align-items:end;justify-content:center;min-height:88px}.native>div{display:grid;gap:8px;justify-items:center}.native img{display:block}.native span{color:#a6b6c5;font-size:11px}.accepted{display:flex;align-items:center;gap:16px;margin:24px 0;padding:14px;border:1px solid #365c49;border-radius:8px}.accepted img{display:block}footer a{display:inline-block;color:#d7cef4;margin:12px 0}@media(max-width:570px){body{padding:14px}.pair{grid-template-columns:minmax(0,1fr)}h1{font-size:23px}}</style></head><body><header><small>SKILLS-IC04 · REVUE VISUELLE</small><h1>Drone Artilleur</h1><p>La flamme rejoint la bouche du canon. Les deux armes retrouvent un métal gris sans traces jaunes, en conservant leur orientation.</p></header><main><section class="pair" aria-label="Comparaison du drone">${cards}</section><aside class="accepted"><img src="../skill-icon-review-03/SF00-v2-64.png" width="64" height="64" alt="Furtivité validée"><div><strong>SF00-v2 · Furtivité validée</strong><p>Cette version et les six autres icônes acceptées sont conservées.</p></div></aside></main><footer>Vues 64, 32 et 24 px à taille réelle.<br><a href="comparison-board.png">Ouvrir la planche de comparaison</a></footer></body></html>`);
  console.log(JSON.stringify({review_id:review.review_id, variants:2, approved_preserved:7, out}));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
