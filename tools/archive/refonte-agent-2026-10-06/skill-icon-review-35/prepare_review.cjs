// Review composition and resampling only. The artwork comes exclusively from image_gen.
const fs = require('node:fs'), path = require('node:path'), crypto = require('node:crypto');
const sharp = require(process.env.RL_SHARP_MODULE || 'sharp');
const out = __dirname, root = path.resolve(out, '../../..');
const read = f => JSON.parse(fs.readFileSync(f, 'utf8').replace(/^\uFEFF/, ''));
const hash = f => crypto.createHash('sha256').update(fs.readFileSync(f)).digest('hex');
const rel = f => path.relative(root, f).replaceAll('\\', '/');
const base = path.resolve(out, '..');
const ledgerFile = path.join(base, 'skill-icon-acceptance.json');
const ledger = read(ledgerFile), previous = ledger.compact_accepted.find(c => c.id === 'M03');
if (previous.variant !== 'M03-C02' || previous.number !== 8) throw Error('Unexpected accepted M03 variant');
const protectionFile = path.join(out, 'protected-hashes.json');
if (!fs.existsSync(protectionFile)) {
  const files = new Set([ledgerFile, path.join(base, 'skill-window-review-05/skills.json')]);
  function collect(dir) { for (const e of fs.readdirSync(dir, {withFileTypes:true})) {
    const f = path.join(dir,e.name); if (e.isDirectory()) collect(f); else files.add(f);
  }}
  collect(path.join(root,'assets/skills/agent'));
  collect(path.join(base,'skill-icon-review-34'));
  for (const c of [...ledger.accepted, ...ledger.compact_accepted]) {
    for (const item of Object.values(c.files || {})) files.add(path.resolve(root, item.file));
    if (c.master) files.add(path.resolve(root,c.master.file));
  }
  fs.writeFileSync(protectionFile,JSON.stringify([...files].sort().map(f=>({file:rel(f),sha256:hash(f)})),null,2)+'\n');
}
const protectedFiles = read(protectionFile);
const verify = () => { for (const f of protectedFiles) if (hash(path.resolve(root,f.file)) !== f.sha256) throw Error('Protected file changed: '+f.file); };
const svgImage = (f,x,y,s) => `<image x="${x}" y="${y}" width="${s}" height="${s}" href="data:image/png;base64,${fs.readFileSync(f).toString('base64')}"/>`;
async function main() {
  verify();
  const source = read(path.join(out,'sources.json'));
  const master = path.join(out,'M03-C03-master.png');
  fs.copyFileSync(source.source_path,master);
  const files = {};
  for (const s of [192,128,64,32,24]) {
    const name = `M03-C03-${s}.png`, dest = path.join(out,name);
    await sharp(master).resize(s,s,{fit:'fill',kernel:'lanczos3'}).png().toFile(dest);
    const meta = await sharp(dest).metadata();
    if (meta.width!==s || meta.height!==s) throw Error('Wrong export size');
    files[s]={file:name,sha256:hash(dest),width:s,height:s};
  }
  const candidate = {id:'M03',number:8,variant:'M03-C03',title:previous.title,kind:'compact',visual_status:'pending',revision_scope:'Four claws instead of three; existing style and colors retained',files,master:{file:path.basename(master),sha256:hash(master)},previous_variant:previous.variant,reference:{file:rel(path.resolve(root,previous.files[192].file)),sha256:previous.files[192].sha256}};
  const review = {review_id:'SKILLS-IC35',candidates:[candidate],active_previous_variant:'M03-C02',unapproved_IC34_candidates:['TR61-C01','TR63-C01','TR64-C01','TR65-C01'],protected_hashes:protectedFiles,generation:{tool:'image_gen.imagegen',source_sha256:hash(source.source_path),prompt_file:'prompts/M03-C03.txt',drawing_edits_by_code:false}};
  fs.writeFileSync(path.join(out,'review.json'),JSON.stringify(review,null,2)+'\n');
  const versions=[{name:'Version actuelle · 3 griffes',variant:'M03-C02',paths:Object.fromEntries(Object.entries(previous.files).map(([s,f])=>[s,path.resolve(root,f.file)]))},{name:'Proposition · 4 griffes',variant:'M03-C03',paths:Object.fromEntries(Object.entries(files).map(([s,f])=>[s,path.join(out,f.file)]))}];
  const cards=versions.map(v=>`<article><h2>${v.name}</h2><div class="large"><img src="${path.relative(out,v.paths[128]).replaceAll('\\','/')}" width="128" height="128" alt="${v.name}"><span>128 px · détail</span></div><div class="native">${[32,64,24].map(s=>`<div><img src="${path.relative(out,v.paths[s]).replaceAll('\\','/')}" width="${s}" height="${s}" alt="${v.name} — ${s} px"><span>${s} px</span></div>`).join('')}</div><small>${v.variant}</small></article>`).join('');
  fs.writeFileSync(path.join(out,'index.html'),`<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>IC35 · Puissance métamorphe</title><style>*{box-sizing:border-box}body{margin:0;padding:28px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial}main{max-width:950px;margin:auto}h1{font-size:26px;margin:8px 0 14px}p{color:#a6b6c5;line-height:1.6}small{color:#b6a8ff}.compare{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:22px;margin:25px 0}article{padding:22px;background:#111923;border:1px solid #3a4654;border-radius:10px;text-align:center}h2{font-size:19px;margin:0 0 24px}.large{display:grid;justify-items:center;gap:10px}.native{display:flex;justify-content:center;gap:40px;align-items:end;margin:26px 0 22px}.native>div{display:grid;justify-items:center;gap:10px}span{color:#a6b6c5;font-size:12px}img{display:block;max-width:none}footer{font-size:12px;line-height:1.8;color:#a6b6c5}a{color:#d7cef4}@media(max-width:680px){body{padding:16px}.compare{grid-template-columns:1fr}.native{gap:32px}}</style><main><small>SKILLS-IC35 · ICÔNE 8 · M03-C03</small><h1>Puissance métamorphe · quatre griffes</h1><p>Même direction sombre, mêmes couleurs. La nouvelle proposition remplace les trois griffes par quatre griffes distinctes.</p><section class="compare">${cards}</section><p>La version à quatre griffes attend ta validation. Les quatre transformations proposées dans IC34 restent à examiner.</p><footer>Petites vues à taille réelle, sans agrandissement.<br>Texte, effets et illustration inchangés. La version actuelle reste en jeu pendant cette revue.<br><a href="prompts/M03-C03.txt">Prompt · Imagegen intégré</a> · <a href="../skill-icon-review-34/index.html">Revenir aux quatre transformations</a></footer></main></html>`);
  const parts=['<rect width="1000" height="440" fill="#0b1017"/>','<text x="24" y="34" font-size="22" fill="#e1e7ec">8 · Puissance métamorphe</text>'];
  versions.forEach((v,i)=>{
    const x=30+i*490;
    parts.push(`<text x="${x}" y="72" font-size="17" fill="#b6a8ff">${v.name}</text>`,svgImage(v.paths[128],x+165,92,128));
    for (const [s,dx] of [[32,95],[64,197],[24,337]]) parts.push(svgImage(v.paths[s],x+dx,322-s,s),`<text x="${x+dx+s/2}" y="347" font-size="13" text-anchor="middle" fill="#a6b6c5">${s} px</text>`);
  });
  parts.push('<text x="24" y="407" font-size="13" fill="#a6b6c5">M03-C03 en attente de validation · textes, effets et autres dessins inchangés</text>');
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="440"><g font-family="Segoe UI,Arial">${parts.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'native-review-board.svg'),svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-review-board.png'));
  verify();
  fs.writeFileSync(path.join(out,'validation.json'),JSON.stringify({review_id:review.review_id,native_export_sizes:[192,128,64,32,24],protected_files_preserved:protectedFiles.length,active_bank_unchanged:true,IC34_drawings_and_statuses_unchanged:true,code_drawing_edits:false,visual_claw_count:'Requires visual inspection: exactly four intended'},null,2)+'\n');
  fs.writeFileSync(path.join(out,'README.md'),'# IC35 · Puissance métamorphe\n\nRévision ciblée de l’icône 8 : M03-C03 propose quatre griffes au lieu de trois. Même style sombre et mêmes couleurs. L’illustration détaillée, le texte et les règles restent inchangés.\n\nImagegen intégré, référence exacte M03-C02-192.png ; prompt archivé dans prompts/M03-C03.txt. Masters copiés et redimensionnement Lanczos uniquement. Aucun dessin modifié par code.\n\nComparaison à 128 px et vues natives 32, 64 et 24 px dans index.html. La nouvelle variante est en attente de validation, non intégrée. Les quatre candidats IC34 restent non validés.\n');
  console.log(JSON.stringify({review_id:review.review_id,candidate:candidate.variant,protected_files:protectedFiles.length}));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
