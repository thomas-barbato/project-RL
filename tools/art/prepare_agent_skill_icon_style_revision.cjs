// SKILLS-IC12 / UI03: review-only style revisions and larger interactive icons.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const sharp=require(process.env.RL_SHARP_MODULE||'sharp');
const root=path.resolve(__dirname,'../..'),base=path.join(root,'artifacts/refonte-agent-2026-10-06');
const out=path.join(base,'skill-icon-review-12'),ui=path.join(base,'skill-window-review-03');
const read=p=>JSON.parse(fs.readFileSync(p,'utf8').replace(/^\uFEFF/,''));
const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const write=(p,v)=>fs.writeFileSync(p,JSON.stringify(v,null,2)+'\n');
const esc=s=>String(s).replace(/[&<>"]/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;'}[c]));
async function main(){
 const input=read(path.join(out,'sources.json')),prior=path.join(base,'skill-icon-review-11');
 const ledger=read(path.join(base,'skill-icon-acceptance.json'));let preserved=0;
 for(const c of [...ledger.accepted,...ledger.compact_accepted])for(const f of Object.values(c.files)){
  if(hash(path.join(root,f.file))!==f.sha256)throw Error('Approved source changed: '+f.file);preserved++;
 }
 const candidates=[];
 for(const c of input.candidates){
  const master=c.variant+'-master.png';fs.copyFileSync(c.source,path.join(out,master));
  const meta=await sharp(c.source).metadata();if(meta.width!==meta.height)throw Error('Non-square source');
  const files={};for(const size of [192,128,64,32,24]){
   const file=c.variant+'-'+size+'.png';await sharp(c.source).resize(size,size,{kernel:'lanczos3'}).png().toFile(path.join(out,file));
   files[size]={file,sha256:hash(path.join(out,file))};
  }
  candidates.push({...c,visual_status:'awaiting_visual_review',generation_tool:'image_gen.imagegen',generation_mode:'built-in',
   source_sha256:hash(c.source),style_reference_hashes:c.style_refs.map(p=>({file:p,sha256:hash(p)})),
   master:{file:master,sha256:hash(path.join(out,master)),width:meta.width,height:meta.height},files});
 }
 const referenceIds=['B03','O03','FE01','M03'];
 const refs=referenceIds.map(id=>'<div><img src="../skill-icon-review-11/'+id+'-C01-64.png" width="64" height="64" alt="'+id+'"><img src="../skill-icon-review-11/'+id+'-C01-32.png" width="32" height="32" alt=""><small>'+id+' · référence validée</small></div>').join('');
 const cards=candidates.map(c=>'<article id="'+c.variant+'"><small>'+c.number+' · '+c.variant+' · à examiner</small><h2>'+esc(c.title)+'</h2><img class="zoom" src="'+c.files[128].file+'" width="128" height="128" alt="'+esc(c.title)+'"><div class="native">'+[32,64,24].map(s=>'<div><img src="'+c.files[s].file+'" width="'+s+'" height="'+s+'" alt=""><span>'+s+' px</span></div>').join('')+'</div><div class="previous"><span>Version précédente</span><img src="../skill-icon-review-11/'+c.id+'-C01-32.png" width="32" height="32" alt="Ancienne version"></div></article>').join('');
 fs.writeFileSync(path.join(out,'index.html'),'<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>SKILLS-IC12 · Style des compactes</title><style>*{box-sizing:border-box}body{margin:0;padding:24px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial}header,main,footer{max-width:1160px;margin:auto}h1{font-size:25px}p,small{color:#a6b6c5;line-height:1.6}a{color:#d7cef4}header>a{display:inline-block;margin:10px 0 18px;padding:9px;border:1px solid #3a4654;border-radius:5px}.refs{display:flex;flex-wrap:wrap;gap:24px;margin:15px 0 24px;padding:15px;background:#111923;border:1px solid #3a4654;border-radius:8px}.refs>div{display:flex;align-items:center;gap:10px}.refs small{font-size:11px}.gallery{display:grid;grid-template-columns:repeat(auto-fit,minmax(250px,1fr));gap:18px}article{min-width:0;background:#111923;border:1px solid #3a4654;border-radius:10px;padding:16px}article small{font-size:11px;color:#b6a8ff}h2{font-size:14px;min-height:38px;line-height:1.4;margin:10px 0}.zoom{display:block;margin:10px auto 20px}.native{display:flex;gap:24px;align-items:end;justify-content:center}.native>div{display:grid;gap:8px;justify-items:center;font-size:11px;color:#a6b6c5}.native img{display:block}.previous{display:flex;align-items:center;justify-content:center;gap:15px;font-size:11px;color:#9facb9;margin-top:20px;padding-top:12px;border-top:1px solid #34414e}footer{margin:22px auto;color:#a6b6c5;font-size:12px}@media(max-width:420px){body{padding:14px}.gallery{grid-template-columns:1fr}}</style><header><small>SKILLS-IC12 · CORRECTION DE STYLE</small><h1>13–20 · compactes reprises</h1><p>Les premières icônes validées servent de références : contours nets, aplats colorés et formes anguleuses. Chaque nouveau dessin reste à examiner.</p><a href="../skill-window-review-03/index.html?skill=FE05">Voir les icônes agrandies dans la fenêtre</a><div class="refs">'+refs+'</div></header><main class="gallery">'+cards+'</main><footer>Vues 32 / 64 / 24 px à taille réelle. Les sources déjà validées restent identiques.</footer></html>');
 const pieces=['<rect width="1160" height="1050" fill="#0b1017"/>','<text x="24" y="34" font-size="25" fill="#e1e7ec">SKILLS-IC12 · Compactes reprises 13–20</text>','<text x="24" y="62" font-size="13" fill="#a6b6c5">Contours et couleurs des premières · vues natives 32 / 64 / 24 px · dessins à examiner</text>'];
 for(const [i,id]of referenceIds.entries()){
  const x=24+i*280;pieces.push('<image x="'+x+'" y="84" width="64" height="64" href="data:image/png;base64,'+fs.readFileSync(path.join(prior,id+'-C01-64.png')).toString('base64')+'"/>','<text x="'+(x+76)+'" y="118" font-size="12" fill="#a6b6c5">'+id+' · référence validée</text>');
 }
 for(const [i,c]of candidates.entries()){
  const x=24+(i%4)*280,y=180+Math.floor(i/4)*418;
  pieces.push('<rect x="'+x+'" y="'+y+'" width="266" height="396" rx="9" fill="#111923" stroke="#3a4654"/>','<text x="'+(x+13)+'" y="'+(y+22)+'" font-size="12" fill="#b6a8ff">'+c.number+' · '+c.variant+'</text>','<text x="'+(x+13)+'" y="'+(y+44)+'" font-size="12" fill="#e1e7ec">'+esc(c.title)+'</text>');
  for(const [s,dx,dy]of [[128,69,62],[32,20,223],[64,92,210],[24,196,228]]){
   pieces.push('<image x="'+(x+dx)+'" y="'+(y+dy)+'" width="'+s+'" height="'+s+'" href="data:image/png;base64,'+fs.readFileSync(path.join(out,c.files[s].file)).toString('base64')+'"/>','<text x="'+(x+dx)+'" y="'+(y+dy+s+17)+'" font-size="10" fill="#a6b6c5">'+s+' px</text>');
  }
  pieces.push('<text x="'+(x+20)+'" y="'+(y+343)+'" font-size="11" fill="#a6b6c5">Version précédente</text>','<image x="'+(x+202)+'" y="'+(y+322)+'" width="32" height="32" href="data:image/png;base64,'+fs.readFileSync(path.join(prior,c.id+'-C01-32.png')).toString('base64')+'"/>');
 }
 const svg='<svg xmlns="http://www.w3.org/2000/svg" width="1160" height="1050"><g font-family="Segoe UI,Arial">'+pieces.join('')+'</g></svg>';
 fs.writeFileSync(path.join(out,'comparison-board.svg'),svg);await sharp(Buffer.from(svg)).png().toFile(path.join(out,'comparison-board.png'));
 const baseline=path.join(base,'skill-window-review-02');fs.mkdirSync(ui,{recursive:true});
 for(const name of ['skills.json','skills-data.js'])fs.copyFileSync(path.join(baseline,name),path.join(ui,name));
 fs.cpSync(path.join(baseline,'illustrations'),path.join(ui,'illustrations'),{recursive:true});
 const artMap=read(path.join(baseline,'manifest.json')).art_map;
 for(const c of candidates)Object.assign(artMap[c.id],{compact:'../skill-icon-review-12/'+c.files[32].file,compactFiles:Object.fromEntries([24,32,64].map(s=>[s,'../skill-icon-review-12/'+c.files[s].file])),compact_status:'awaiting_visual_review',compact_sha256:c.files[32].sha256});
 fs.writeFileSync(path.join(ui,'art-map.js'),'window.SKILL_ART_REVIEW='+JSON.stringify(artMap,null,2)+';\n');
 const html=fs.readFileSync(path.join(baseline,'index.html'),'utf8').replaceAll('SKILLS-UI02','SKILLS-UI03').replaceAll('SKILLS-IC11','SKILLS-IC12').replace('Icônes compactes et illustrations sous la description · prototype','Compactes reprises et cartes agrandies · prototype').replace('<button id="mode">','<a class="revision-link" href="../skill-icon-review-12/index.html">Revue 13–20</a><button id="mode">');
 fs.writeFileSync(path.join(ui,'index.html'),html);
 const js=fs.readFileSync(path.join(baseline,'review.js'),'utf8').replace('icon.src=compactIcon(s.id);','icon.src=compactIcon(s.id,64);').replace("$('detail-icon').src=compactIcon(s.id);","$('detail-icon').src=compactIcon(s.id,64);");
 fs.writeFileSync(path.join(ui,'review.js'),js);
 fs.writeFileSync(path.join(ui,'style.css'),fs.readFileSync(path.join(baseline,'style.css'),'utf8')+'\n/* UI03: more room for the compact drawings; use 64px exports, preserve status-bar size. */\n.game-shell .skill-card>img{width:64px;height:64px;max-width:100%;object-fit:contain}.game-shell .skill-card{min-height:144px;gap:6px;padding:10px 6px}.game-shell .detail-top img{width:48px;height:48px}.revision-link{color:var(--accent);font-size:12px;white-space:nowrap;text-decoration:none}.review-actions{flex-wrap:wrap}\n');
 write(path.join(ui,'manifest.json'),{review_id:'SKILLS-UI03',date:input.date,status:'awaiting_layout_review',source_review:'SKILLS-UI02',scope:'Larger compact card icons and eight pending style revisions. 192px centered descriptive illustration preserved. Review prototype only.',tree_icon_display_px:64,tree_icon_source_px:64,detail_header_display_px:48,status_icon_display_px:32,illustration_size_px:192,skill_data_sha256:hash(path.join(ui,'skills.json')),accepted_export_hashes_preserved:preserved,art_map:artMap});
 write(path.join(out,'review.json'),{review_id:'SKILLS-IC12',date:input.date,status:'awaiting_visual_review',feedback:input.feedback,candidates,layout_review:'SKILLS-UI03',accepted_export_hashes_preserved:preserved,no_new_approval_inferred:true});
 fs.writeFileSync(path.join(out,'prompts.txt'),candidates.map(c=>'=== '+c.variant+' · '+c.title+' ===\n'+fs.readFileSync(path.join(out,c.prompt),'utf8')+'\n').join('\n'));
 console.log(JSON.stringify({review_id:'SKILLS-IC12',revised_compacts:candidates.length,layout:'SKILLS-UI03',preserved_accepted_exports:preserved}));
}
main().catch(e=>{console.error(e);process.exitCode=1});
