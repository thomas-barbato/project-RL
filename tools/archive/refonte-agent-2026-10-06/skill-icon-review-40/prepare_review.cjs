// Review-only packaging: resample the generated edit and lay out the comparison.
const fs = require('node:fs'), path = require('node:path'), crypto = require('node:crypto');
const sharp = require(process.env.RL_SHARP_MODULE || 'sharp');
const out=__dirname, root=path.resolve(out,'../../..'), base=path.dirname(out);
const read=f=>JSON.parse(fs.readFileSync(f,'utf8').replace(/^\uFEFF/,''));
const hash=f=>crypto.createHash('sha256').update(fs.readFileSync(f)).digest('hex');
const rel=f=>path.relative(root,f).replaceAll('\\','/');
const write=(f,v)=>fs.writeFileSync(f,JSON.stringify(v,null,2)+'\n');
const previous=path.join(base,'skill-icon-review-39');
const oldReview=read(path.join(previous,'review.json'));
const old=oldReview.candidates.find(c=>c.id==='M02');
const ledger=read(path.join(base,'skill-icon-acceptance.json'));
if(old.variant!=='M02-C01'||old.number!==69||ledger.compact_accepted.some(c=>c.id==='M02')) throw Error('Unexpected M02 state');
if(fs.existsSync(path.join(out,'visual-acceptance-01.json')))throw Error('Accepted review is frozen');
const protectionFile=path.join(out,'protected-hashes.json');
if(!fs.existsSync(protectionFile)) {
  const files=new Map();
  const protect=f=>files.set(rel(f),hash(f));
  const collect=dir=>{for(const e of fs.readdirSync(dir,{withFileTypes:true})) {
    const f=path.join(dir,e.name); if(e.isDirectory())collect(f);else protect(f);
  }};
  collect(previous);collect(path.join(root,'assets/skills/agent'));
  protect(path.join(base,'skill-icon-acceptance.json'));
  protect(path.join(base,'skill-window-review-05/skills.json'));
  for(const c of [...ledger.accepted,...ledger.compact_accepted]) {
    for(const f of [...Object.values(c.files),...(c.master?[c.master]:[])]) {
      const file=path.join(root,f.file);if(hash(file)!==f.sha256)throw Error('Accepted artwork changed');protect(file);
    }
  }
  write(protectionFile,[...files].map(([file,sha256])=>({file,sha256})));
}
const protectedFiles=read(protectionFile);
const verify=()=>{for(const f of protectedFiles)if(hash(path.join(root,f.file))!==f.sha256)throw Error('Protected file changed: '+f.file);};
const svgImage=(f,x,y,s)=>`<image x="${x}" y="${y}" width="${s}" height="${s}" href="data:image/png;base64,${fs.readFileSync(f).toString('base64')}"/>`;
async function main() {
  verify();
  const source=read(path.join(out,'sources.json'));
  const master=path.join(out,'M02-C02-master.png');fs.copyFileSync(source.source_path,master);
  const metadata=await sharp(master).metadata();if(metadata.width!==metadata.height||metadata.width<192)throw Error('Expected square master');
  const files={};
  for(const s of [192,128,64,32,24]) {
    const file=`M02-C02-${s}.png`,dest=path.join(out,file);
    await sharp(master).resize(s,s,{kernel:'lanczos3'}).png().toFile(dest);
    const m=await sharp(dest).metadata();if(m.width!==s||m.height!==s)throw Error('Wrong export size');
    files[s]={file,sha256:hash(dest),width:s,height:s};
  }
  const candidate={id:'M02',number:69,variant:'M02-C02',title:old.title,branch:old.branch,kind:old.kind,
    roleplay:old.roleplay,visual_status:'awaiting_visual_review',revision_scope:'Green palette only',
    generation_tool:'image_gen.imagegen',generation_mode:'built-in',prompt:'prompts/M02-C02.txt',
    source:source.source_path,source_sha256:hash(source.source_path),master:{file:path.basename(master),sha256:hash(master),width:metadata.width,height:metadata.height},files,
    reference_hashes:[{file:rel(path.join(previous,old.master.file)),sha256:old.master.sha256}]};
  const context=oldReview.candidates.filter(c=>c.id!=='M02').map(c=>({id:c.id,number:c.number,variant:c.variant,title:c.title,visual_status:c.visual_status,file:'../skill-icon-review-39/'+c.files['32'].file,sha256:c.files['32'].sha256}));
  write(path.join(out,'review.json'),{review_id:'SKILLS-IC40',date:'2026-10-11',status:'awaiting_visual_review',
    user_feedback:'peut-être que decharge mutagène devrait avoir une palette plus verte ?',candidates:[candidate],
    previous_variant:{variant:old.variant,visual_status:old.visual_status,file:'../skill-icon-review-39/review.json'},
    context_unchanged_pending:context,protected_hashes:protectedFiles,no_runtime_changes:true,no_new_approval_inferred:true,drawing_edits_by_code:false});
  const card=(title,variant,prefix)=>`<article><small>69 · ${variant}</small><h2>${title}</h2><div class="zoom"><img src="${prefix}${variant}-128.png" width="128" height="128" alt="${title}"></div><div class="native">${[32,64,24].map(s=>`<div><img src="${prefix}${variant}-${s}.png" width="${s}" height="${s}" alt="${title} ${s} px"><span>${s} px</span></div>`).join('')}</div></article>`;
  fs.writeFileSync(path.join(out,'index.html'),`<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>IC40 · Décharge mutagène verte</title><style>*{box-sizing:border-box}body{margin:0;padding:24px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial}main{max-width:760px;margin:auto}h1{font-size:25px;margin:12px 0}p,small{color:#a6b6c5;line-height:1.6}a{color:#d7cef4}.comparison{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:24px;margin:24px 0}article{background:#111923;border:1px solid #3a4654;border-radius:10px;padding:20px;min-width:0}article small{color:#b6a8ff}h2{font-size:18px;margin:8px 0;min-height:48px}.zoom{display:flex;justify-content:center;margin:24px 0}.native{display:flex;gap:34px;justify-content:center;align-items:end}.native>div{display:grid;gap:9px;justify-items:center;color:#a6b6c5;font-size:12px}img{display:block;flex-shrink:0}.context{display:flex;flex-wrap:wrap;gap:24px;margin:14px 0 24px;color:#a6b6c5}.context>div{display:flex;gap:10px;align-items:center}footer{font-size:12px;line-height:1.8;color:#a6b6c5}@media(max-width:600px){.comparison{grid-template-columns:1fr}body{padding:16px}}</style><main><small>SKILLS-IC40 · ICÔNE 69 · RÉVISION DE PALETTE</small><h1>Décharge mutagène · variante verte</h1><p>La décharge conserve sa forme déchirante et sa peinture sombre. Le vert devient dominant, avec un cœur vert pâle.</p><section class="comparison">${card('Proposition précédente · violette','M02-C01','../skill-icon-review-39/')}${card('Nouvelle proposition · verte','M02-C02','')}</section><p>Les deux autres propositions sont conservées ; elles restent à valider.</p><div class="context">${context.map(c=>`<div><img src="${c.file}" width="32" height="32" alt="${c.title} inchangée"><span>${c.number} · ${c.title}</span></div>`).join('')}</div><footer>Petites vues à taille réelle : 32 / 64 / 24 px. Détails supérieurs : 128 px.<br>Variante verte proposée à la revue. <a href="prompts/M02-C02.txt">Prompt · Imagegen intégré</a> · <a href="../skill-icon-review-39/index.html">Revoir l’ensemble des pouvoirs hybrides</a></footer></main></html>`);
  const pieces=['<rect width="760" height="520" fill="#0b1017"/>','<text x="24" y="34" font-size="23" fill="#e1e7ec">69 · Décharge mutagène · palette verte</text>','<text x="24" y="62" font-size="13" fill="#a6b6c5">Comparaison · vues natives 32 / 64 / 24 px · détail 128 px</text>'];
  for(const [i,variant]of['M02-C01','M02-C02'].entries()) {
    const x=24+i*368,dir=i?out:previous;
    pieces.push(`<rect x="${x}" y="92" width="344" height="360" rx="9" fill="#111923" stroke="#3a4654"/>`,`<text x="${x+18}" y="120" font-size="15" fill="#e1e7ec">${i?'Nouvelle proposition · verte':'Proposition précédente · violette'}</text>`,`<text x="${x+18}" y="145" font-size="13" fill="#b6a8ff">69 · ${variant}</text>`,svgImage(path.join(dir,variant+'-128.png'),x+108,166,128));
    for(const[s,dx]of[[32,42],[64,136],[24,270]])pieces.push(svgImage(path.join(dir,variant+`-${s}.png`),x+dx,384-s,s),`<text x="${x+dx+s/2}" y="411" text-anchor="middle" font-size="12" fill="#a6b6c5">${s} px</text>`);
  }
  pieces.push('<text x="24" y="490" font-size="13" fill="#a6b6c5">M02-C02 · proposition non intégrée, en attente de validation visuelle.</text>');
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="760" height="520"><g font-family="Segoe UI,Arial">${pieces.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'native-review-board.svg'),svg);await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-review-board.png'));
  verify();
  write(path.join(out,'validation.json'),{review_id:'SKILLS-IC40',new_exports_verified:5,export_sizes:[192,128,64,32,24],protected_files_preserved:protectedFiles.length,active_compact_count:ledger.compact_accepted.length,active_bank_unchanged:true,previous_proposals_unchanged:true,status:'awaiting_visual_review',drawing_edits_by_code:false});
  fs.writeFileSync(path.join(out,'README.md'),'# SKILLS-IC40 · Décharge mutagène verte\n\nRévision demandée par le joueur : palette plus verte. Candidat 69 M02-C02, avec comparaison à M02-C01. Les silhouettes, la direction et la peinture sombre sont conservées dans la demande d’édition ; seuls les coloris changent.\n\nImagegen intégré, cible d’édition M02-C01-master.png. Prompt exact dans prompts/M02-C02.txt ; master généré copié ici, exports 192, 128, 64, 32 et 24 px. Aucun redessin ou changement de couleur par code.\n\nLa nouvelle variante reste à valider et ne remplace aucune ressource active. Échange de position HY04-C01 et Lien de domination HY06-C01 restent inchangés et non validés. Le catalogue et les 66 compactes actives sont préservés.\n');
  console.log(JSON.stringify({review:'SKILLS-IC40',variant:'M02-C02',protected_files:protectedFiles.length,active_compact_count:ledger.compact_accepted.length}));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
