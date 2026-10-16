// Re-present existing candidates unchanged. No artwork generation or editing.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const sharp=require(process.env.RL_SHARP_MODULE||'sharp');
const out=__dirname,root=path.resolve(out,'../../..'),base=path.resolve(out,'..');
const read=f=>JSON.parse(fs.readFileSync(f,'utf8').replace(/^\uFEFF/,''));
const hash=f=>crypto.createHash('sha256').update(fs.readFileSync(f)).digest('hex');
const rel=f=>path.relative(root,f).replaceAll('\\','/');
const prior=read(path.join(base,'skill-icon-review-34/review.json'));
const ledger=read(path.join(base,'skill-icon-acceptance.json'));
const kept=['M01','M03'].map(id=>ledger.compact_accepted.find(c=>c.id===id));
if(kept[1].variant!=='M03-C05')throw Error('The accepted M03 symbol must be used');
const protectionFile=path.join(out,'protected-hashes.json');
if(!fs.existsSync(protectionFile)){
  const all=new Set([path.join(base,'skill-icon-acceptance.json'),path.join(base,'skill-window-review-05/skills.json')]);
  const collect=dir=>{for(const e of fs.readdirSync(dir,{withFileTypes:true})){const f=path.join(dir,e.name);if(e.isDirectory())collect(f);else all.add(f);}};
  collect(path.join(root,'assets/skills/agent'));collect(path.join(base,'skill-icon-review-34'));
  for(const c of kept){all.add(path.resolve(root,c.master.file));for(const f of Object.values(c.files))all.add(path.resolve(root,f.file));}
  fs.writeFileSync(protectionFile,JSON.stringify([...all].sort().map(f=>({file:rel(f),sha256:hash(f)})),null,2)+'\n');
}
const protection=read(protectionFile);
const verify=()=>{for(const f of protection)if(hash(path.resolve(root,f.file))!==f.sha256)throw Error('Protected file changed: '+f.file);};
const svgImage=(f,x,y,s)=>`<image x="${x}" y="${y}" width="${s}" height="${s}" href="data:image/png;base64,${fs.readFileSync(f).toString('base64')}"/>`;
async function main(){
  verify();
  const candidates=prior.candidates.map(c=>{
    if(ledger.compact_accepted.some(e=>e.id===c.id))throw Error('This candidate is already accepted');
    const copy=JSON.parse(JSON.stringify(c));
    for(const f of [...Object.values(copy.files),copy.master]){
      const source=path.join(base,'skill-icon-review-34',f.file);
      if(hash(source)!==f.sha256)throw Error('Original art changed');
      f.file='../skill-icon-review-34/'+f.file;
    }
    copy.prompt='../skill-icon-review-34/'+copy.prompt;
    return copy;
  });
  const review={review_id:'SKILLS-IC38',title:'Transformations · suite de la revue',date:'2026-10-11',status:'awaiting_visual_review',previous_candidate_review:'SKILLS-IC34',new_art_generation:false,candidates,accepted_context:kept.map(c=>({id:c.id,variant:c.variant,title:c.title,files:c.files})),protected_hashes:protection,scope:'Same exact four candidates and stable review numbers; only the review context now uses accepted M03-C05.'};
  fs.writeFileSync(path.join(out,'review.json'),JSON.stringify(review,null,2)+'\n');
  const cards=candidates.map(c=>`<article id="${c.variant}"><small>${c.number} · ${c.variant}</small><h2>${c.title}</h2><div class="zoom"><img src="${c.files[128].file}" width="128" height="128" alt="${c.title}"></div><div class="native">${[32,64,24].map(s=>`<div><img src="${c.files[s].file}" width="${s}" height="${s}" alt="${c.title} ${s} px"><span>${s} px</span></div>`).join('')}</div></article>`).join('');
  fs.writeFileSync(path.join(out,'index.html'),`<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>IC38 · Transformations</title><style>*{box-sizing:border-box}body{margin:0;padding:24px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial}header,main,footer{max-width:1040px;margin:auto}h1{font-size:25px;margin:12px 0}p,small{color:#a6b6c5;line-height:1.6}a{color:#d7cef4}.gallery{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:16px}article{background:#111923;border:1px solid #3a4654;border-radius:10px;padding:16px;min-width:0}article small{color:#b6a8ff}h2{font-size:16px;min-height:44px;line-height:1.35;margin:9px 0}.zoom{display:flex;justify-content:center;margin:16px 0 20px}.native{display:flex;gap:22px;justify-content:center;align-items:end}.native>div{display:grid;gap:9px;justify-items:center;color:#a6b6c5;font-size:12px}img{display:block;flex-shrink:0}.context{max-width:1040px;margin:22px auto}.context h2{min-height:0}.context p{font-size:13px}.context img{width:480px;height:334px}footer{margin:24px auto;color:#a6b6c5;font-size:12px;line-height:1.7}@media(max-width:920px){.gallery{grid-template-columns:repeat(2,minmax(0,1fr))}}@media(max-width:520px){.gallery{grid-template-columns:1fr}body{padding:16px}.context{overflow-x:auto}}</style><header><small>SKILLS-IC38 · PROPOSITIONS 65–68</small><h1>Transformations · les quatre icônes à valider</h1><p>Les quatre propositions conservent leurs dessins et leurs numéros. Puissance métamorphe utilise maintenant le symbole validé de quatre griffures.</p></header><main class="gallery">${cards}</main><section class="context"><h2>Ensemble dans l’arbre · 64 px</h2><p>Mue prédatrice et Puissance métamorphe sont validées. Les quatre autres icônes restent proposées à la revue.</p><img src="skill-tree-context.png" width="480" height="334" alt="Six icônes de transformation à 64 px, dont Puissance métamorphe validée"></section><footer>Petites vues à taille réelle : 32, 64 et 24 px. Vue supérieure : 128 px.<br><a href="../skill-window-native-13/index.html">Puissance métamorphe intégrée en jeu</a> · <a href="../skill-icon-review-34/prompts.txt">Prompts des quatre dessins · Imagegen intégré</a></footer></html>`);
  const parts=['<rect width="1040" height="460" fill="#0b1017"/>','<text x="24" y="34" font-size="22" fill="#e1e7ec">Transformations · propositions 65–68</text>','<text x="24" y="62" font-size="13" fill="#a6b6c5">IC38 · mêmes dessins · vues natives 32 / 64 / 24 px</text>'];
  candidates.forEach((c,i)=>{
    const x=24+i*252;
    parts.push(`<text x="${x}" y="112" font-size="13" fill="#b6a8ff">${c.number} · ${c.variant}</text>`,`<text x="${x}" y="142" font-size="15" fill="#e1e7ec">${c.title}</text>`,svgImage(path.join(out,c.files[128].file),x+52,163,128));
    for(const[s,dx]of[[32,34],[64,97],[24,190]])parts.push(svgImage(path.join(out,c.files[s].file),x+dx,382-s,s),`<text x="${x+dx+s/2}" y="406" text-anchor="middle" font-size="12" fill="#a6b6c5">${s} px</text>`);
  });
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="1040" height="460"><g font-family="Segoe UI,Arial">${parts.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'native-review-board.svg'),svg);await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-review-board.png'));
  const context=[...kept.map(c=>({title:c.title,file:path.resolve(root,c.files[64].file)})),...candidates.map(c=>({title:c.title,file:path.join(out,c.files[64].file)}))];
  const treeParts=['<rect width="480" height="334" fill="#10171e"/>','<text x="20" y="28" font-size="19" fill="#dde5e9">Transformations · aperçu des icônes</text>'];
  context.forEach((c,i)=>{
    const x=28+(i%3)*150,y=52+Math.floor(i/3)*126;
    treeParts.push(svgImage(c.file,x+34,y,64));
    const words=c.title.split(' '),lines=c.title.length>20?[words.slice(0,-1).join(' '),words.at(-1)]:[c.title];
    lines.forEach((l,j)=>treeParts.push(`<text x="${x+66}" y="${y+85+j*17}" text-anchor="middle" font-size="13" fill="#dde5e9">${l}</text>`));
  });
  treeParts.push('<text x="20" y="318" font-size="12" fill="#a4b2bd">64 px · deux icônes validées et quatre propositions</text>');
  const tree=`<svg xmlns="http://www.w3.org/2000/svg" width="480" height="334"><g font-family="Segoe UI,Arial">${treeParts.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'skill-tree-context.svg'),tree);await sharp(Buffer.from(tree)).png().toFile(path.join(out,'skill-tree-context.png'));
  verify();
  fs.writeFileSync(path.join(out,'validation.json'),JSON.stringify({review_id:review.review_id,protected_files_preserved:protection.length,original_master_and_export_hashes_unchanged:true,active_bank_unchanged:true,candidate_numbers_unchanged:true,no_new_art_generated:true,accepted_context_variants:kept.map(c=>c.variant),native_sizes:[32,64,24],native_review_board:[1040,460],context_board:[480,334]},null,2)+'\n');
  fs.writeFileSync(path.join(out,'README.md'),'# IC38 · reprise des quatre transformations\n\nLes candidats 65–68 TR61-C01, TR63-C01, TR64-C01 et TR65-C01 sont repris exactement depuis IC34, sans nouveau dessin ni redimensionnement. Leurs sources et leurs prompts restent archivés dans IC34. Les identifiants de revue sont conservés.\n\nLa présentation montre le contexte actualisé avec M01-C02 et M03-C05, acceptées. Les quatre candidats restent en attente de validation visuelle, non intégrés. Textes, effets et banque du jeu inchangés.\n\nindex.html et native-review-board.png montrent les vues natives ; skill-tree-context.png montre six icônes à 64 px. Les planches composent uniquement les exports existants. Empreintes vérifiées dans validation.json.\n');
  console.log(JSON.stringify({review:review.review_id,candidates:candidates.map(c=>c.variant),protected_files:protection.length,new_art:false}));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
