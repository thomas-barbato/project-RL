// Review-only redraw of the existing compact bank. Accepted drawings and game files stay intact.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const sharp=require(process.env.RL_SHARP_MODULE||'sharp');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06'),out=path.join(base,'skill-icon-review-17');
const read=p=>JSON.parse(fs.readFileSync(p,'utf8').replace(/^\uFEFF/,''));
const sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const write=(p,v)=>fs.writeFileSync(p,JSON.stringify(v,null,2)+'\n');
const esc=s=>String(s).replace(/[&<>\"]/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','\"':'&quot;'}[c]));
const rel=p=>path.relative(root,p).replace(/\\/g,'/');
const check=files=>{for(const p of files)if(sha(path.join(root,p.file))!==p.sha256)throw Error('Protected asset changed: '+p.file);};
async function main(){
  if(fs.readdirSync(out).some(f=>f.startsWith('visual-acceptance')&&f.endsWith('.json')))throw Error('Accepted review is frozen.');
  const input=read(path.join(out,'sources.json')),baseline=read(path.join(out,'protected-baseline.json'));
  check(baseline.protected_files);
  const catalogue=read(path.join(base,'skill-window-review-05/skills.json')).skills;
  const ledger=read(path.join(base,'skill-icon-acceptance.json'));
  if(input.candidates.length!==30||new Set(input.candidates.map(c=>c.id)).size!==30||new Set(input.candidates.map(c=>c.number)).size!==30)throw Error('Expected exactly 30 unique existing skills and stable numbers.');
  fs.mkdirSync(path.join(out,'prompts'),{recursive:true});
  const candidates=[];
  for(const c of input.candidates){
    const skill=catalogue.find(s=>s.id===c.id);
    if(!skill||skill.title!==c.title||skill.kind!==c.kind||skill.roleplay!==c.roleplay)throw Error('Skill identity or copy differs: '+c.id);
    if(!c.source)throw Error('Missing generated source: '+c.id);
    const meta=await sharp(c.source).metadata();
    if(meta.width!==meta.height||meta.width<192)throw Error('Invalid source dimensions: '+c.id);
    const master=c.variant+'-master.png';fs.copyFileSync(c.source,path.join(out,master));
    const files={};
    for(const size of [192,128,64,32,24]){
      const file=c.variant+'-'+size+'.png';
      await sharp(c.source).resize(size,size,{kernel:'lanczos3'}).png().toFile(path.join(out,file));
      const m=await sharp(path.join(out,file)).metadata();
      if(m.width!==size||m.height!==size)throw Error('Export dimensions differ');
      files[size]={file,sha256:sha(path.join(out,file)),width:size,height:size};
    }
    const previous={};
    for(const size of [128,64,32,24]){
      const old=path.join(root,path.dirname(c.previous_file),c.previous_variant+'-'+size+'.png');
      if(!fs.existsSync(old))throw Error('Missing previous comparison: '+old);
      previous[size]={file:rel(old),sha256:sha(old)};
    }
    fs.writeFileSync(path.join(out,'prompts',c.variant+'.txt'),c.prompt+'\n');
    candidates.push({...c,files,previous_files:previous,master:{file:master,sha256:sha(path.join(out,master)),width:meta.width,height:meta.height},
      previous_status:ledger.compact_accepted.some(a=>a.id===c.id&&a.variant===c.previous_variant)?'accepted':'awaiting_visual_review',visual_status:'awaiting_visual_review',
      reference_hashes:c.refs.map(file=>({file,sha256:sha(file)}))});
  }
  const oldUrl=(c,size)=>'../../'+c.previous_files[size].file.replace('artifacts/refonte-agent-2026-10-06/','');
  // HTML is inside the artifact root: use sibling review directories for old exports.
  const oldRelative=(c,size)=>'../'+c.previous_files[size].file.split('/').slice(-2).join('/');
  const native=(c,old)=>'<div class="native">'+[32,64,24].map(size=>'<div><img src="'+(old?oldRelative(c,size):c.files[size].file)+'" width="'+size+'" height="'+size+'" alt="'+esc(c.title)+' '+size+' px"><span>'+size+' px</span></div>').join('')+'</div>';
  const cards=candidates.map(c=>'<button type="button" class="skill" data-group="'+c.group+'" data-open="'+c.variant+'" aria-label="Comparer '+esc(c.title)+'"><small>'+c.number+' · '+c.variant+'</small><div class="thumbs">'+[64,32,24].map(size=>'<div><img src="'+c.files[size].file+'" width="'+size+'" height="'+size+'" alt="'+esc(c.title)+' '+size+' px"><span>'+size+' px</span></div>').join('')+'</div><b>'+esc(c.title)+'</b><span class="kind">'+(c.kind==='passive'?'Passif':'Actif')+'</span></button>').join('');
  const dialogs=candidates.map(c=>'<dialog id="'+c.variant+'"><header><small>'+c.number+' · '+c.previous_variant+' → '+c.variant+'</small><button type="button" data-close aria-label="Fermer la comparaison">×</button><h2>'+esc(c.title)+'</h2></header><div class="compare"><section><h3>Version précédente</h3><img class="large" src="'+oldRelative(c,128)+'" width="128" height="128" alt="Version précédente de '+esc(c.title)+'">'+native(c,true)+'</section><section><h3>Essai sombre</h3><img class="large" src="'+c.files[128].file+'" width="128" height="128" alt="Essai sombre de '+esc(c.title)+'">'+native(c,false)+'</section></div><p>Comparaison visuelle · '+(c.kind==='passive'?'Passif':'Actif')+' · '+esc(c.branch)+'</p></dialog>').join('');
  const nav='<button type="button" data-filter="all" aria-pressed="true">Toutes · 30</button>'+input.groups.map(g=>'<button type="button" data-filter="'+g.id+'" aria-pressed="false">'+esc(g.title)+' · '+g.ids.length+'</button>').join('');
  const css='*{box-sizing:border-box}body{margin:0;padding:20px;background:#0b1017;color:#e1e7ec;font:14px Segoe UI,Arial}header,main,footer{max-width:1040px;margin:auto}h1{font-size:24px;margin:7px 0}p{color:#a6b6c5;line-height:1.5;margin:8px 0}a{color:#d8b9ad}small{color:#baa9a6;font-size:11px}nav{display:flex;gap:8px;flex-wrap:wrap;margin:16px 0 20px}nav button{background:#141a23;color:#bfc8d2;border:1px solid #46505e;padding:8px 11px;border-radius:4px;cursor:pointer}nav button[aria-pressed=true]{background:#35262b;border-color:#a4706a;color:#f0e3dc}.gallery{display:grid;grid-template-columns:repeat(6,minmax(0,1fr));gap:18px 12px}.skill{display:flex;flex-direction:column;align-items:center;padding:4px;border:0;background:transparent;color:inherit;min-width:0;cursor:pointer;gap:5px}.skill:hover,.skill:focus-visible{background:#161e28;outline:1px solid #68717c}.skill[hidden]{display:none}.thumbs{display:flex;align-items:end;gap:8px;min-height:85px}.thumbs>div{display:grid;justify-items:center;gap:5px}.thumbs span,.kind{font-size:10px;color:#a6b6c5}.skill b{font-size:12px;font-weight:400;line-height:1.3;min-height:32px;text-align:center}img{display:block;flex-shrink:0}footer{margin:22px auto 8px;font-size:12px;color:#a6b6c5;line-height:1.5}dialog{color:#e1e7ec;background:#111923;border:1px solid #6b6269;border-radius:8px;padding:22px;width:min(580px,95vw)}dialog::backdrop{background:#000b}dialog header{position:relative}dialog h2{font-size:20px;margin:9px 0 20px}dialog h3{font-size:13px;font-weight:400;color:#a6b6c5}dialog [data-close]{position:absolute;right:0;top:-8px;background:transparent;border:0;color:#eee;font-size:28px;cursor:pointer}.compare{display:grid;grid-template-columns:1fr 1fr;gap:24px}.compare section{display:grid;justify-items:center}.large{margin-bottom:16px}.native{display:flex;align-items:end;gap:18px}.native>div{display:grid;justify-items:center;gap:7px}.native span{font-size:11px;color:#a6b6c5}dialog p{font-size:12px;margin-top:20px;text-align:center}@media(max-width:900px){.gallery{grid-template-columns:repeat(4,minmax(0,1fr))}}@media(max-width:600px){.gallery{grid-template-columns:repeat(2,minmax(0,1fr))}.compare{gap:10px}body{padding:14px}}';
  const script='document.querySelectorAll("[data-filter]").forEach(b=>b.addEventListener("click",()=>{document.querySelectorAll("[data-filter]").forEach(x=>x.setAttribute("aria-pressed",String(x===b)));document.querySelectorAll(".skill").forEach(c=>c.hidden=b.dataset.filter!=="all"&&c.dataset.group!==b.dataset.filter);}));document.querySelectorAll("[data-open]").forEach(b=>b.addEventListener("click",()=>document.getElementById(b.dataset.open).showModal()));document.querySelectorAll("[data-close]").forEach(b=>b.addEventListener("click",()=>b.closest("dialog").close()));';
  fs.writeFileSync(path.join(out,'index.html'),'<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>SKILLS-IC17 · Les 30 autres icônes</title><style>'+css+'</style><header><small>SKILLS-IC17 · 30 NOUVEAUX CANDIDATS</small><h1>La banque compacte reprend la direction sombre</h1><p>Vues 64 / 32 / 24 px à taille réelle. Clique sur une compétence pour comparer avec son dessin précédent.</p><nav aria-label="Groupes de compétences">'+nav+'</nav></header><main class="gallery">'+cards+'</main>'+dialogs+'<footer>Les nouveaux dessins sont proposés à la revue ; les numéros et les compétences restent les mêmes.<br><a href="../skill-icon-review-16/index.html">Les quatre essais de référence</a> · <a href="native-overview.png">Planche à taille native</a> · <a href="prompts.txt">Prompts Imagegen</a></footer><script>'+script+'</script></html>');
  const pieces=['<rect width="1040" height="930" fill="#0b1017"/>','<text x="24" y="34" fill="#e1e7ec" font-size="23">SKILLS-IC17 · Les 30 autres icônes compactes</text>','<text x="24" y="59" fill="#a6b6c5" font-size="13">Dessins proposés · vues 64 / 32 / 24 px à taille réelle</text>'];
  const img=(file,x,y,size)=>'<image x="'+x+'" y="'+y+'" width="'+size+'" height="'+size+'" href="data:image/png;base64,'+fs.readFileSync(file).toString('base64')+'"/>';
  const wrap=(s,n=24)=>{const lines=[''];for(const word of s.split(' ')){const i=lines.length-1;if(lines[i]&&(lines[i]+' '+word).length>n)lines.push(word);else lines[i]+=(lines[i]?' ':'')+word;}return lines;};
  for(const [i,c] of candidates.entries()){
    const x=24+(i%6)*168,y=82+Math.floor(i/6)*166;
    pieces.push('<text x="'+x+'" y="'+(y+13)+'" fill="#baa9a6" font-size="11">'+c.number+' · '+c.variant+'</text>');
    for(const [size,dx] of [[64,0],[32,72],[24,112]])pieces.push(img(path.join(out,c.files[size].file),x+dx,y+26+64-size,size),'<text x="'+(x+dx+size/2)+'" y="'+(y+104)+'" text-anchor="middle" fill="#a6b6c5" font-size="10">'+size+' px</text>');
    for(const [line,label] of wrap(c.title).entries())pieces.push('<text x="'+x+'" y="'+(y+125+line*15)+'" fill="#e1e7ec" font-size="11">'+esc(label)+'</text>');
  }
  pieces.push('<text x="24" y="921" fill="#a6b6c5" font-size="12">Style issu des quatre essais SKILLS-IC16 · aucun remplacement dans le jeu</text>');
  const svg='<svg xmlns="http://www.w3.org/2000/svg" width="1040" height="930"><g font-family="Segoe UI,Arial">'+pieces.join('')+'</g></svg>';
  fs.writeFileSync(path.join(out,'native-overview.svg'),svg);await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-overview.png'));
  for(const group of input.groups){
    const selected=candidates.filter(c=>c.group===group.id),rows=Math.ceil(selected.length/4),height=80+rows*170;
    const parts=['<rect width="1040" height="'+height+'" fill="#0b1017"/>','<text x="24" y="35" fill="#e1e7ec" font-size="23">'+esc(group.title)+' · SKILLS-IC17</text>'];
    for(const [i,c] of selected.entries()){
      const x=24+(i%4)*254,y=70+Math.floor(i/4)*170;
      parts.push('<text x="'+x+'" y="'+(y+14)+'" fill="#baa9a6" font-size="12">'+c.number+' · '+c.variant+'</text>');
      for(const [size,dx] of [[64,0],[32,80],[24,130]])parts.push(img(path.join(out,c.files[size].file),x+dx,y+26+64-size,size),'<text x="'+(x+dx+size/2)+'" y="'+(y+108)+'" text-anchor="middle" fill="#a6b6c5" font-size="11">'+size+' px</text>');
      for(const [line,label] of wrap(c.title,33).entries())parts.push('<text x="'+x+'" y="'+(y+133+line*16)+'" fill="#e1e7ec" font-size="12">'+esc(label)+'</text>');
    }
    await sharp(Buffer.from('<svg xmlns="http://www.w3.org/2000/svg" width="1040" height="'+height+'"><g font-family="Segoe UI,Arial">'+parts.join('')+'</g></svg>')).png().toFile(path.join(out,group.id+'-native-board.png'));
  }
  fs.writeFileSync(path.join(out,'prompts.txt'),candidates.map(c=>'=== '+c.number+' · '+c.variant+' ===\n'+c.prompt+'\n').join('\n'));
  check(baseline.protected_files);
  write(path.join(out,'review.json'),{...input,candidates,protected_hashes:baseline.protected_files,no_new_individual_approval_inferred:true,no_runtime_changes:true});
  write(path.join(out,'validation.json'),{review_id:input.review_id,status:'awaiting_visual_review',candidate_count:30,dimensions_verified:true,unique_skill_ids:30,stable_numbers:30,protected_files_preserved:baseline.protected_files.length,skill_names_kinds_and_roleplay_preserved:true,no_runtime_changes:true,generation_tool:'image_gen.imagegen',generation_mode:'built-in'});
  console.log(JSON.stringify({review_id:input.review_id,candidates:30,protected_files_preserved:baseline.protected_files.length}));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
