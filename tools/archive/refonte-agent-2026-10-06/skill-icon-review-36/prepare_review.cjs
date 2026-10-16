// Resampling and review composition only; image_gen provides all artwork.
const fs = require('node:fs'), path = require('node:path'), crypto = require('node:crypto');
const sharp = require(process.env.RL_SHARP_MODULE || 'sharp');
const out = __dirname, root = path.resolve(out,'../../..'), base = path.resolve(out,'..');
const read = f => JSON.parse(fs.readFileSync(f,'utf8').replace(/^\uFEFF/,''));
const hash = f => crypto.createHash('sha256').update(fs.readFileSync(f)).digest('hex');
const rel = f => path.relative(root,f).replaceAll('\\','/');
const prior = read(path.join(base,'skill-icon-review-35/review.json')).candidates[0];
const ledger = read(path.join(base,'skill-icon-acceptance.json'));
const active = ledger.compact_accepted.find(c=>c.id==='M03');
if (active.variant!=='M03-C02' || prior.variant!=='M03-C03') throw Error('Unexpected M03 revision history');
const protectionFile = path.join(out,'protected-hashes.json');
if (!fs.existsSync(protectionFile)) {
  const protectedFiles = read(path.join(base,'skill-icon-review-35/protected-hashes.json'));
  for (const e of fs.readdirSync(path.join(base,'skill-icon-review-35'),{withFileTypes:true})) {
    const f=path.join(base,'skill-icon-review-35',e.name);
    if(e.isFile()) protectedFiles.push({file:rel(f),sha256:hash(f)});
  }
  const prompt=path.join(base,'skill-icon-review-35/prompts/M03-C03.txt');
  protectedFiles.push({file:rel(prompt),sha256:hash(prompt)});
  fs.writeFileSync(protectionFile,JSON.stringify(protectedFiles,null,2)+'\n');
}
const protection = read(protectionFile);
const verify = () => { for(const f of protection) if(hash(path.resolve(root,f.file))!==f.sha256) throw Error('Protected file changed: '+f.file); };
const img = (f,x,y,s) => `<image x="${x}" y="${y}" width="${s}" height="${s}" href="data:image/png;base64,${fs.readFileSync(f).toString('base64')}"/>`;
async function main() {
  verify();
  const source = read(path.join(out,'sources.json'));
  const master = path.join(out,'M03-C04-master.png');
  fs.copyFileSync(source.source_path,master);
  const files={};
  for(const s of [192,128,64,32,24]) {
    const file=`M03-C04-${s}.png`, dest=path.join(out,file);
    await sharp(master).resize(s,s,{fit:'fill',kernel:'lanczos3'}).png().toFile(dest);
    const meta=await sharp(dest).metadata();
    if(meta.width!==s || meta.height!==s) throw Error('Incorrect native export size');
    files[s]={file,sha256:hash(dest),width:s,height:s};
  }
  const candidate={id:'M03',number:8,variant:'M03-C04',kind:'compact',title:'Puissance métamorphe',visual_status:'pending',revision_scope:'Natural organic claw anatomy, keeping four claws and the established dark palette',previous_proposal:'M03-C03',files,master:{file:path.basename(master),sha256:hash(master)},reference:{file:rel(path.join(base,'skill-icon-review-35/M03-C03-master.png')),sha256:hash(path.join(base,'skill-icon-review-35/M03-C03-master.png'))}};
  const review={review_id:'SKILLS-IC36',user_feedback:'la griff ne fait pas assez naturelle',candidates:[candidate],active_variant:active.variant,prior_proposal_not_approved:true,IC34_candidates_still_pending:['TR61-C01','TR63-C01','TR64-C01','TR65-C01'],protected_hashes:protection,generation:{tool:'image_gen.imagegen',use_case:'precise-object-edit',prompt_file:'prompts/M03-C04.txt',source_path:source.source_path,source_sha256:hash(source.source_path),drawing_edits_by_code:false}};
  fs.writeFileSync(path.join(out,'review.json'),JSON.stringify(review,null,2)+'\n');
  const versions=[{label:'Essai précédent',variant:'M03-C03',paths:Object.fromEntries(Object.entries(prior.files).map(([s,f])=>[s,path.join(base,'skill-icon-review-35',f.file)]))},{label:'Griffes plus naturelles',variant:'M03-C04',paths:Object.fromEntries(Object.entries(files).map(([s,f])=>[s,path.join(out,f.file)]))}];
  const cards=versions.map(v=>`<article><h2>${v.label}</h2><div class="large"><img src="${path.relative(out,v.paths[128]).replaceAll('\\','/')}" width="128" height="128" alt="${v.label}"><span>128 px · détail</span></div><div class="native">${[32,64,24].map(s=>`<div><img src="${path.relative(out,v.paths[s]).replaceAll('\\','/')}" width="${s}" height="${s}" alt="${v.label} — ${s} px"><span>${s} px</span></div>`).join('')}</div><small>${v.variant}</small></article>`).join('');
  fs.writeFileSync(path.join(out,'index.html'),`<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>IC36 · Puissance métamorphe</title><style>*{box-sizing:border-box}body{margin:0;padding:28px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial}main{max-width:950px;margin:auto}h1{font-size:26px;margin:8px 0 14px}p{color:#a6b6c5;line-height:1.6}small{color:#b6a8ff}.compare{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:22px;margin:25px 0}article{padding:22px;background:#111923;border:1px solid #3a4654;border-radius:10px;text-align:center}h2{font-size:19px;margin:0 0 24px}.large{display:grid;justify-items:center;gap:10px}.native{display:flex;justify-content:center;gap:40px;align-items:end;margin:26px 0 22px}.native>div{display:grid;justify-items:center;gap:10px}span{color:#a6b6c5;font-size:12px}img{display:block;max-width:none}footer{font-size:12px;line-height:1.8;color:#a6b6c5}a{color:#d7cef4}@media(max-width:680px){body{padding:16px}.compare{grid-template-columns:1fr}.native{gap:32px}}</style><main><small>SKILLS-IC36 · ICÔNE 8 · M03-C04</small><h1>Puissance métamorphe · griffes naturelles</h1><p>Quatre griffes effilées, plus fluides et mieux rattachées aux doigts. La chair rouge et les accents occultes violets gardent la même direction sombre.</p><section class="compare">${cards}</section><p>Nouvelle proposition à valider. Les autres icônes restent inchangées.</p><footer>Vues natives à 32, 64 et 24 px ; détail à 128 px.<br>La banque du jeu conserve la dernière version validée. Textes, illustration et effets inchangés.<br><a href="prompts/M03-C04.txt">Prompt · Imagegen intégré</a> · <a href="../skill-icon-review-34/index.html">Quatre transformations à examiner</a></footer></main></html>`);
  const parts=['<rect width="1000" height="440" fill="#0b1017"/>','<text x="24" y="34" font-size="22" fill="#e1e7ec">8 · Puissance métamorphe</text>'];
  versions.forEach((v,i)=>{
    const x=30+i*490;
    parts.push(`<text x="${x}" y="72" font-size="17" fill="#b6a8ff">${v.label}</text>`,img(v.paths[128],x+165,92,128));
    for(const [s,dx] of [[32,95],[64,197],[24,337]]) parts.push(img(v.paths[s],x+dx,322-s,s),`<text x="${x+dx+s/2}" y="347" font-size="13" text-anchor="middle" fill="#a6b6c5">${s} px</text>`);
  });
  parts.push('<text x="24" y="407" font-size="13" fill="#a6b6c5">M03-C04 · quatre griffes · proposition en attente de validation</text>');
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="440"><g font-family="Segoe UI,Arial">${parts.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'native-review-board.svg'),svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-review-board.png'));
  verify();
  fs.writeFileSync(path.join(out,'validation.json'),JSON.stringify({review_id:review.review_id,exports:[192,128,64,32,24],protected_files_preserved:protection.length,active_bank_unchanged:true,IC34_candidates_unchanged:true,visual_inspection:'Pending',code_drawing_edits:false},null,2)+'\n');
  fs.writeFileSync(path.join(out,'README.md'),'# IC36 · Griffes naturelles\n\nIcône 8, M03-C04 : révision de l’anatomie de Puissance métamorphe en conservant quatre griffes. Le joueur trouve M03-C03 insuffisamment naturelle ; aucun accord sur cette version n’est déduit.\n\nImagegen intégré avec M03-C03-master.png comme cible. Prompt exact dans prompts/M03-C04.txt. Source copiée, exports Lanczos uniquement ; aucun dessin par code.\n\nRevue à 128, 32, 64 et 24 px. Nouvelle variante non intégrée avant validation ; textes, mécanique et illustration inchangés. Banque active et quatre propositions IC34 préservées par empreintes.\n');
  console.log(JSON.stringify({review:review.review_id,variant:candidate.variant,protected_files:protection.length}));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
