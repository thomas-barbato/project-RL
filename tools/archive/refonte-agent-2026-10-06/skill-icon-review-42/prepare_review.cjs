// Re-present the two existing pending drawings without generating or editing artwork.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const sharp=require(process.env.RL_SHARP_MODULE||'sharp');
const out=__dirname,root=path.resolve(out,'../../..'),base=path.dirname(out);
const read=f=>JSON.parse(fs.readFileSync(f,'utf8').replace(/^\uFEFF/,''));
const hash=f=>crypto.createHash('sha256').update(fs.readFileSync(f)).digest('hex');
const rel=f=>path.relative(root,f).replaceAll('\\','/');
const prior=read(path.join(base,'skill-icon-review-39/review.json'));
const ledger=read(path.join(base,'skill-icon-acceptance.json'));
const accepted=ledger.compact_accepted.find(c=>c.id==='M02');
if(accepted?.variant!=='M02-C03'||ledger.compact_accepted.length!==67)throw Error('Expected accepted green M02');
if(fs.existsSync(path.join(out,'visual-acceptance-01.json')))throw Error('Accepted review is frozen');
const protectionFile=path.join(out,'protected-hashes.json');
if(!fs.existsSync(protectionFile)){
  const all=new Set([path.join(base,'skill-icon-acceptance.json'),path.join(base,'skill-window-review-05/skills.json')]);
  const collect=dir=>{for(const e of fs.readdirSync(dir,{withFileTypes:true})){const f=path.join(dir,e.name);if(e.isDirectory())collect(f);else all.add(f);}};
  collect(path.join(root,'assets/skills/agent'));collect(path.join(base,'skill-icon-review-39'));
  all.add(path.resolve(root,accepted.master.file));for(const f of Object.values(accepted.files))all.add(path.resolve(root,f.file));
  fs.writeFileSync(protectionFile,JSON.stringify([...all].sort().map(f=>({file:rel(f),sha256:hash(f)})),null,2)+'\n');
}
const protection=read(protectionFile);
const verify=()=>{for(const f of protection)if(hash(path.resolve(root,f.file))!==f.sha256)throw Error('Protected file changed: '+f.file);};
const svgImage=(f,x,y,s)=>`<image x="${x}" y="${y}" width="${s}" height="${s}" href="data:image/png;base64,${fs.readFileSync(f).toString('base64')}"/>`;
async function main(){
  verify();
  const candidates=prior.candidates.filter(c=>['HY04','HY06'].includes(c.id)).map(c=>{
    if(ledger.compact_accepted.some(e=>e.id===c.id))throw Error('Already accepted');
    const copy=JSON.parse(JSON.stringify(c));
    for(const f of [...Object.values(copy.files),copy.master]){
      const source=path.join(base,'skill-icon-review-39',f.file);
      if(hash(source)!==f.sha256)throw Error('Original artwork changed');
      f.file='../skill-icon-review-39/'+f.file;
    }
    copy.prompt='../skill-icon-review-39/'+copy.prompt;return copy;
  });
  if(candidates.length!==2||candidates[0].number!==70||candidates[1].number!==71)throw Error('Unexpected candidates');
  const review={review_id:'SKILLS-IC42',title:'Pouvoirs hybrides · deux icônes à valider',date:'2026-10-11',status:'awaiting_visual_review',previous_candidate_review:'SKILLS-IC39',new_art_generation:false,candidates,accepted_context:{id:accepted.id,number:69,variant:accepted.variant,title:accepted.title,files:accepted.files},protected_hashes:protection,no_new_approval_inferred:true,scope:'The two exact IC39 drawings keep their numbers; M02-C03 is already accepted and appears as context only.'};
  fs.writeFileSync(path.join(out,'review.json'),JSON.stringify(review,null,2)+'\n');
  const cards=candidates.map(c=>`<article id="${c.variant}"><small>${c.number} · ${c.variant}</small><h2>${c.title}</h2><div class="zoom"><img src="${c.files[128].file}" width="128" height="128" alt="${c.title}"></div><div class="native">${[32,64,24].map(s=>`<div><img src="${c.files[s].file}" width="${s}" height="${s}" alt="${c.title} ${s} px"><span>${s} px</span></div>`).join('')}</div></article>`).join('');
  const green=path.relative(out,path.resolve(root,accepted.files['64'].file)).replaceAll('\\','/');
  fs.writeFileSync(path.join(out,'index.html'),`<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>IC42 · Pouvoirs hybrides</title><style>*{box-sizing:border-box}body{margin:0;padding:24px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial}header,main,section,footer{max-width:760px;margin:auto}h1{font-size:25px;margin:12px 0}p,small{color:#a6b6c5;line-height:1.6}a{color:#d7cef4}.gallery{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:20px}article{background:#111923;border:1px solid #3a4654;border-radius:10px;padding:18px;min-width:0}article small{color:#b6a8ff}h2{font-size:18px;min-height:44px;line-height:1.35;margin:9px 0}.zoom{display:flex;justify-content:center;margin:16px 0 20px}.native{display:flex;gap:30px;justify-content:center;align-items:end}.native>div{display:grid;gap:9px;justify-items:center;color:#a6b6c5;font-size:12px}img{display:block;flex-shrink:0}.context{display:flex;gap:18px;align-items:center;margin:24px auto;color:#a6b6c5}.context p{margin:0}.context img{width:64px;height:64px}footer{margin:20px auto;color:#a6b6c5;font-size:12px;line-height:1.7}@media(max-width:560px){.gallery{grid-template-columns:1fr}body{padding:16px}}</style><header><small>SKILLS-IC42 · PROPOSITIONS 70–71</small><h1>Pouvoirs hybrides · les deux icônes restantes</h1><p>Échange de position et Lien de domination conservent leurs dessins et leurs numéros.</p></header><main class="gallery">${cards}</main><section class="context"><img src="${green}" width="64" height="64" alt="Décharge mutagène validée et intégrée"><p>69 · Décharge mutagène<br>Version verte sans rouge validée et intégrée.</p></section><footer>Petites vues à taille réelle : 32, 64 et 24 px. Vue supérieure : 128 px.<br><a href="../skill-window-native-15/index.html">Voir Décharge mutagène en jeu</a> · <a href="../skill-icon-review-39/prompts.txt">Prompts originaux des dessins · Imagegen intégré</a></footer></html>`);
  const parts=['<rect width="760" height="460" fill="#0b1017"/>','<text x="24" y="34" font-size="22" fill="#e1e7ec">Pouvoirs hybrides · propositions 70–71</text>','<text x="24" y="62" font-size="13" fill="#a6b6c5">IC42 · mêmes dessins · vues natives 32 / 64 / 24 px</text>'];
  candidates.forEach((c,i)=>{const x=24+i*368;parts.push(`<text x="${x}" y="112" font-size="13" fill="#b6a8ff">${c.number} · ${c.variant}</text>`,`<text x="${x}" y="142" font-size="17" fill="#e1e7ec">${c.title}</text>`,svgImage(path.join(out,c.files[128].file),x+108,163,128));for(const[s,dx]of[[32,40],[64,136],[24,270]])parts.push(svgImage(path.join(out,c.files[s].file),x+dx,382-s,s),`<text x="${x+dx+s/2}" y="406" text-anchor="middle" font-size="12" fill="#a6b6c5">${s} px</text>`);});
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="760" height="460"><g font-family="Segoe UI,Arial">${parts.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'native-review-board.svg'),svg);await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-review-board.png'));
  verify();
  fs.writeFileSync(path.join(out,'validation.json'),JSON.stringify({review_id:review.review_id,protected_files_preserved:protection.length,original_master_and_export_hashes_unchanged:true,active_bank_unchanged:true,candidate_numbers_unchanged:true,no_new_art_generated:true,accepted_context_variant:'M02-C03',native_sizes:[32,64,24],native_review_board:[760,460]},null,2)+'\n');
  fs.writeFileSync(path.join(out,'README.md'),'# SKILLS-IC42 · Pouvoirs hybrides restants\n\nCandidats 70 HY04-C01 Échange de position et 71 HY06-C01 Lien de domination, repris exactement depuis IC39, sans génération, retouche ou redimensionnement. Leurs sources et prompts restent archivés dans IC39. Numéros conservés.\n\nLes deux candidats restent à valider et ne sont pas intégrés. Décharge mutagène M02-C03 est montrée uniquement comme contexte validé ; capture réelle dans skill-window-native-15. Textes, effets et banque du jeu inchangés par cette reprise de revue.\n\nindex.html et native-review-board.png montrent les tailles natives 32, 64 et 24 px, avec détail 128 px. Empreintes vérifiées dans validation.json.\n');
  console.log(JSON.stringify({review:review.review_id,candidates:candidates.map(c=>c.variant),protected_files:protection.length,new_art:false}));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
