// Compact skill icons and separate descriptive illustrations: review artifacts only.
const fs=require('node:fs');
const path=require('node:path');
const crypto=require('node:crypto');
const sharp=require(process.env.RL_SHARP_MODULE||'sharp');
const root=path.resolve(__dirname,'../..');
const base=path.join(root,'artifacts/refonte-agent-2026-10-06');
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const read=p=>JSON.parse(fs.readFileSync(p,'utf8').replace(/^\uFEFF/,''));
const esc=s=>String(s).replace(/[&<>"]/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;'}[c]));
const verify=(p,h)=>{if(hash(fs.readFileSync(p))!==h)throw Error('Source changed: '+p);};
async function main(){
 const inputPath=path.resolve(process.argv[2]);
 const input=read(inputPath),out=path.dirname(inputPath),ui=path.join(base,'skill-window-review-02');
 fs.mkdirSync(path.join(ui,'illustrations'),{recursive:true});
 const ledger=read(path.join(base,'skill-icon-acceptance.json'));
 let preserved=0;
 for(const c of [...ledger.accepted,...ledger.compact_accepted])for(const f of Object.values(c.files)){
  verify(path.join(root,f.file),f.sha256);preserved++;
 }
 const candidates=[];
 for(const c of [...input.existing,...input.new_candidates]){
  const master=c.variant+'-master.png';
  fs.copyFileSync(c.source,path.join(out,master));
  const meta=await sharp(path.join(out,master)).metadata();
  if(meta.width!==meta.height||meta.width<512)throw Error('Non-square compact master: '+c.variant);
  const files={};
  for(const size of [192,128,64,32,24]){
   const file=c.variant+'-'+size+'.png',target=path.join(out,file);
   if(c.files){verify(c.files[size].path,c.files[size].sha256);fs.copyFileSync(c.files[size].path,target);}
   else await sharp(c.source).resize(size,size,{kernel:'lanczos3'}).png().toFile(target);
   const exportMeta=await sharp(target).metadata();
   if(exportMeta.width!==size||exportMeta.height!==size)throw Error('Bad native size: '+file);
   files[size]={file,sha256:hash(fs.readFileSync(target))};
  }
  candidates.push({id:c.id,number:c.number,variant:c.variant,title:c.title,kind:'compact',
   visual_status:c.status||'awaiting_visual_review',prompt:c.prompt||null,
   master:{file:master,sha256:hash(fs.readFileSync(path.join(out,master))),width:meta.width,height:meta.height},files});
 }
 candidates.sort((a,b)=>a.number-b.number);
 const card=c=>'<article id="'+c.variant+'"><small>'+String(c.number).padStart(2,'0')+' · '+esc(c.variant)+' · '+(c.visual_status==='accepted'?'déjà validée':'à examiner')+'</small><h3>'+esc(c.title)+'</h3><img class="zoom" src="'+c.files[128].file+'" width="128" height="128" alt="'+esc(c.title)+'"><div class="native">'+[32,64,24].map(s=>'<div><img src="'+c.files[s].file+'" width="'+s+'" height="'+s+'" alt=""><span>'+s+' px</span></div>').join('')+'</div></article>';
 const sections=[
  {id:'new',title:'13–20 · nouvelles versions compactes',rows:candidates.filter(c=>c.number>=13)},
  {id:'pending',title:'09–12 · propositions de mêlée à examiner',rows:candidates.filter(c=>c.number>=9&&c.number<=12)},
  {id:'accepted',title:'01–08 · icônes compactes déjà validées',rows:candidates.filter(c=>c.visual_status==='accepted')}
 ];
 fs.writeFileSync(path.join(out,'index.html'),'<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>SKILLS-IC11 · Icônes compactes</title><style>*{box-sizing:border-box}body{margin:0;padding:24px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial}header,main,footer{max-width:1160px;margin:auto}h1{font-size:26px}p,small{color:#a6b6c5;line-height:1.6}nav{display:flex;gap:10px;flex-wrap:wrap;margin:20px 0}a{color:#d7cef4}nav a,footer a{padding:8px 10px;border:1px solid #3a4654;border-radius:5px;text-decoration:none}h2{font-size:19px;color:#d6cdf3;margin:25px 0 16px}.gallery{display:grid;grid-template-columns:repeat(auto-fit,minmax(245px,1fr));gap:18px}article{min-width:0;background:#111923;border:1px solid #3a4654;border-radius:10px;padding:16px}article small{font-size:11px;color:#b6a8ff}h3{font-size:14px;min-height:40px;line-height:1.4;margin:12px 0}.zoom{display:block;margin:10px auto 22px}.native{display:flex;gap:24px;align-items:end;justify-content:center}.native>div{display:grid;gap:8px;justify-items:center;font-size:11px;color:#a6b6c5}.native img{display:block}footer{margin:28px auto}@media(max-width:420px){body{padding:14px}.gallery{grid-template-columns:1fr}}</style><header><small>SKILLS-IC11 · REVUE VISUELLE</small><h1>Icônes compactes</h1><p>Les petits affichages utilisent les dessins compacts. Les illustrations détaillées sont réservées aux fiches. Les vues de 32 px sont affichées à taille réelle.</p><nav><a href="../skill-window-review-02/index.html">Voir une fiche illustrée</a><a href="#new">Nouvelles icônes</a><a href="#pending">Mêlée</a><a href="#accepted">Déjà validées</a></nav></header><main>'+sections.map(s=>'<section id="'+s.id+'"><h2>'+s.title+'</h2><div class="gallery">'+s.rows.map(card).join('')+'</div></section>').join('')+'</main><footer>Les destinations visuelles sont acceptées ; chaque nouveau dessin reste à valider.</footer></html>');
 const fresh=candidates.filter(c=>c.number>=13);
 const pieces=['<rect width="1160" height="836" fill="#0b1017"/>',
  '<text x="24" y="34" font-size="25" fill="#e1e7ec">SKILLS-IC11 · Nouvelles icônes compactes</text>',
  '<text x="24" y="62" font-size="13" fill="#a6b6c5">13–20 · aperçu 128 px · tailles réelles 32 / 64 / 24 px · dessins à examiner</text>'];
 for(const [i,c] of fresh.entries()){
  const x=24+(i%4)*280,y=94+Math.floor(i/4)*356;
  pieces.push('<rect x="'+x+'" y="'+y+'" width="266" height="326" rx="9" fill="#111923" stroke="#3a4654"/>',
   '<text x="'+(x+13)+'" y="'+(y+23)+'" font-size="12" fill="#b6a8ff">'+c.number+' · '+esc(c.variant)+'</text>',
   '<text x="'+(x+13)+'" y="'+(y+44)+'" font-size="12" fill="#e1e7ec">'+esc(c.title)+'</text>');
  for(const [s,dx,dy] of [[128,69,63],[32,17,228],[64,88,213],[24,194,232]]){
   pieces.push('<image x="'+(x+dx)+'" y="'+(y+dy)+'" width="'+s+'" height="'+s+'" href="data:image/png;base64,'+fs.readFileSync(path.join(out,c.files[s].file)).toString('base64')+'"/>',
    '<text x="'+(x+dx)+'" y="'+(y+dy+s+16)+'" font-size="10" fill="#a6b6c5">'+s+' px</text>');
  }
 }
 const svg='<svg xmlns="http://www.w3.org/2000/svg" width="1160" height="836"><g font-family="Segoe UI,Arial,sans-serif">'+pieces.join('')+'</g></svg>';
 fs.writeFileSync(path.join(out,'comparison-board.svg'),svg);
 await sharp(Buffer.from(svg)).png().toFile(path.join(out,'comparison-board.png'));
 const artMap={};
 for(const c of candidates){
  const illustration=input.illustrations.find(i=>i.id===c.id);
  if(!illustration)throw Error('Missing illustration: '+c.id);
  verify(illustration.source,illustration.sha256);
  const illustrationFile=illustration.variant+'.png';
  fs.copyFileSync(illustration.source,path.join(ui,'illustrations',illustrationFile));
  artMap[c.id]={compact:'../skill-icon-review-11/'+c.files[32].file,
   compactFiles:Object.fromEntries([24,32,64].map(s=>[s,'../skill-icon-review-11/'+c.files[s].file])),
   illustration:'illustrations/'+illustrationFile,compact_status:c.visual_status,
   illustration_status:illustration.status,compact_sha256:c.files[32].sha256,
   illustration_sha256:illustration.sha256};
 }
 const original=path.join(base,'skill-window-review-01');
 for(const name of ['skills.json','skills-data.js'])fs.copyFileSync(path.join(original,name),path.join(ui,name));
 fs.writeFileSync(path.join(ui,'art-map.js'),'window.SKILL_ART_REVIEW='+JSON.stringify(artMap,null,2)+';\n');
 let html=fs.readFileSync(path.join(original,'index.html'),'utf8')
  .replaceAll('SKILLS-UI01','SKILLS-UI02')
  .replace('Fenêtre et icônes proposées · pas encore intégrées','Icônes compactes et illustrations sous la description · prototype')
  .replaceAll('Voir les 94 icônes','Voir les 20 icônes compactes')
  .replace('94 compétences','20 compétences en aperçu')
  .replace('SKILLS-IC01','SKILLS-IC11')
  .replace('94 icônes · taille native et réductions','20 icônes compactes · tailles réelles')
  .replace('width="64" height="64" alt=""','width="32" height="32" alt=""')
  .replace('<p class="roleplay" id="roleplay"></p>','<p class="roleplay" id="roleplay"></p><figure class="skill-illustration" id="skill-illustration"><img id="detail-illustration" width="192" height="192" alt=""></figure>')
  .replace('<script src="skills-data.js"></script>','<script src="skills-data.js"></script><script src="art-map.js"></script>');
 fs.writeFileSync(path.join(ui,'index.html'),html);
 let js=fs.readFileSync(path.join(original,'review.js'),'utf8')
  .replace('const allSkills=window.SKILL_REVIEW.skills;','const allSkills=window.SKILL_REVIEW.skills.filter(s=>window.SKILL_ART_REVIEW[s.id]);\nconst compactIcon=(id,size=32)=>window.SKILL_ART_REVIEW[id].compactFiles[size];')
  .replaceAll("'icons/'+s.id+'.svg'","compactIcon(s.id)")
  .replaceAll("'icons/'+id+'.svg'","compactIcon(id)")
  .replace("for(const b of branches){","for(const b of branches.filter(b=>allSkills.some(s=>s.branch===b))){")
  .replace("for(const [index,branch] of [...primaryBranches,...secondaryBranches].entries()){","for(const [index,branch] of [...primaryBranches,...secondaryBranches].filter(b=>allSkills.some(s=>s.branch===b)).entries()){")
  .replace("img.src=compactIcon(s.id);img.width=size;","img.src=compactIcon(s.id,size);img.width=size;")
  .replace("$('roleplay').textContent=s.roleplay;","$('roleplay').textContent=s.roleplay;$('detail-illustration').src=window.SKILL_ART_REVIEW[s.id].illustration;$('detail-illustration').alt='Illustration de '+s.title;")
  .replace("for(const [id,name,turns,description,negative] of statuses){","for(const [id,name,turns,description,negative] of statuses.filter(s=>window.SKILL_ART_REVIEW[s[0]])){")
  .replaceAll("'Voir les 94 icônes'","'Voir les 20 icônes compactes'")
  .replace("gallery();render();if(","gallery();render();select(new URLSearchParams(location.search).get('skill')||'FE01');if(");
 fs.writeFileSync(path.join(ui,'review.js'),js);
 const css=fs.readFileSync(path.join(original,'style.css'),'utf8')+'\n/* SKILLS-UI02: compact interactive icons, illustration below RP in existing scroll pane. */\n.game-shell .skill-card>img,.game-shell .detail-top img{width:32px;height:32px;flex:none}.game-shell .skill-card{min-height:114px}.skill-illustration{margin:0 0 22px;display:flex;justify-content:center}.skill-illustration img{display:block;width:192px;height:192px;max-width:100%;object-fit:contain;border-radius:7px}.detail-scroll .roleplay{margin-bottom:16px}\n';
 fs.writeFileSync(path.join(ui,'style.css'),css);
 const manifest={review_id:'SKILLS-UI02',date:input.date,status:'awaiting_layout_review',
  source_review:'SKILLS-UI01',scope:'Layout prototype: 20 existing reviewed skills, compact 32px icons and separate centered illustration under RP. No gameplay or active Rust UI change.',
  compact_usage:['trees','detail heading','status bar'],illustration_size_px:192,
  illustration_position:'immediately after RP, before numeric effects, inside detail-scroll',
  source_skill_data_sha256:hash(fs.readFileSync(path.join(original,'skills.json'))),
  copied_skill_data_sha256:hash(fs.readFileSync(path.join(ui,'skills.json'))),
  accepted_export_hashes_preserved:preserved,art_map:artMap};
 const acceptanceFile=path.join(ui,'visual-acceptance-01.json');
 if(fs.existsSync(acceptanceFile)){
  const acceptance=read(acceptanceFile);
  const unchanged=Object.entries(acceptance.files).every(([file,sha256])=>
   fs.existsSync(path.join(ui,file))&&hash(fs.readFileSync(path.join(ui,file)))===sha256);
  if(unchanged){
   manifest.status='layout_accepted';
   manifest.layout_acceptance=acceptance.review_id;
   manifest.layout_acceptance_reference='visual-acceptance-01.json';
  }
 }
 fs.writeFileSync(path.join(ui,'manifest.json'),JSON.stringify(manifest,null,2)+'\n');
 fs.writeFileSync(path.join(out,'review.json'),JSON.stringify({review_id:'SKILLS-IC11',date:input.date,status:'awaiting_visual_review',scope:'Compact-only gallery: 8 existing accepted compact designs, 4 existing pending melee designs and 8 new compact counterparts. Detailed drawings reserved for the separate skill description.',
  generation_tool:'image_gen.imagegen',generation_mode:'built-in',candidates,
  new_ids:input.new_candidates.map(c=>c.id),layout_review:'SKILLS-UI02',preserved_accepted_exports:preserved,
  no_new_approval_inferred:true},null,2)+'\n');
 fs.writeFileSync(path.join(out,'prompts.txt'),input.new_candidates.map(c=>'=== '+c.variant+' · '+c.title+' ===\n'+fs.readFileSync(path.join(out,c.prompt),'utf8')+'\n').join('\n'));
 console.log(JSON.stringify({review_id:'SKILLS-IC11',new_compact:fresh.length,total_compact:candidates.length,layout:'SKILLS-UI02',preserved_accepted_exports:preserved}));
}
main().catch(e=>{console.error(e);process.exitCode=1});

