// Package distinct illustrated and compact image_gen masters; no semantic editing.
const fs=require('node:fs');
const path=require('node:path');
const crypto=require('node:crypto');
const sharp=require(process.env.RL_SHARP_MODULE||'sharp');
const root=path.resolve(__dirname,'../..');
const base=path.join(root,'artifacts/refonte-agent-2026-10-06');
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const readJson=p=>JSON.parse(fs.readFileSync(p,'utf8').replace(/^\uFEFF/,''));
const esc=s=>String(s).replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&apos;'}[c]));
async function main(){
  const sourcePath=path.resolve(process.argv[2]||'');
  if(!sourcePath.endsWith('sources.json'))throw Error('Pass sources.json.');
  const input=readJson(sourcePath),out=path.dirname(sourcePath);
  const oldDir=path.join(base,input.source_review_dir||'skill-icon-review-05');
  const prior=readJson(path.join(oldDir,'review.json'));
  const accepted=readJson(path.join(base,'skill-icon-acceptance.json'));
  for(const c of [...accepted.accepted,...(accepted.compact_accepted||[])])for(const f of Object.values(c.files)){
    if(hash(fs.readFileSync(path.join(root,f.file)))!==f.sha256)throw Error('Accepted drawing changed: '+c.id);
  }
  for(const c of prior.candidates)for(const f of Object.values(c.files)){
    if(hash(fs.readFileSync(path.join(oldDir,f.file)))!==f.sha256)throw Error('Prior drawing changed: '+c.id);
  }
  const rows=[];
  for(const c of input.candidates){
    const name=c.variant+'-master.png';
    fs.copyFileSync(c.source,path.join(out,name));
    const meta=await sharp(path.join(out,name)).metadata();
    if(meta.width!==meta.height||meta.width<512)throw Error('Expected square source: '+c.variant);
    const files={};
    for(const size of [192,128,64,32,24]){
      const file=c.variant+'-'+size+'.png';
      if(c.preserved_files){
        const original=c.preserved_files[size];
        if(!original||hash(fs.readFileSync(original.path))!==original.sha256)throw Error('Preserved source changed: '+c.variant);
        fs.copyFileSync(original.path,path.join(out,file));
      }else await sharp(path.join(out,name)).resize(size,size,{kernel:'lanczos3'}).png().toFile(path.join(out,file));
      files[size]={file,sha256:hash(fs.readFileSync(path.join(out,file)))};
    }
    rows.push({number:c.number,id:c.id,variant:c.variant,kind:c.kind,title:c.title,visual_status:'awaiting_visual_review',prompt:c.prompt,master:{file:name,width:meta.width,height:meta.height,sha256:hash(fs.readFileSync(path.join(out,name)))},files});
  }
  const groups=[{key:'illustrated',title:input.illustrated_title||'Illustrations corrigées',y:108},{key:'compact',title:'Versions compactes · dessins distincts',y:466}];
  const text=(x,y,t,size=13,color='#e1e7ec')=>`<text x="${x}" y="${y}" font-size="${size}" fill="${color}">${esc(t)}</text>`;
  const title=input.title||'Corrections et versions compactes';
  const pieces=[text(24,34,input.review_id+' · '+title,25),text(24,61,'Numéros conservés · aperçu 128 px · tailles réelles 64 / 32 / 24 px',13,'#a6b6c5')];
  for(const group of groups){
    pieces.push(text(24,group.y-12,group.title,16,'#d6cdf3'));
    for(const [i,c] of rows.filter(c=>c.kind===group.key).entries()){
      const x=24+i*280,y=group.y;
      pieces.push(`<rect x="${x}" y="${y}" width="266" height="310" rx="9" fill="#111923" stroke="#3a4654"/>`,text(x+13,y+23,String(c.number).padStart(2,'0')+' · '+c.variant,12,'#b6a8ff'),text(x+13,y+44,c.title,12));
      for(const [s,dx,dy]of[[128,69,56],[64,13,203],[32,108,219],[24,175,223]]){
        const img=fs.readFileSync(path.join(out,c.files[s].file)).toString('base64');
        pieces.push(`<image x="${x+dx}" y="${y+dy}" width="${s}" height="${s}" href="data:image/png;base64,${img}"/>`,text(x+dx,y+dy+s+16,s+' px',10,'#a6b6c5'));
      }
    }
  }
  pieces.push(text(24,813,'Illustrations et versions compactes à examiner séparément · revue du dessin uniquement',12,'#a6b6c5'));
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="1160" height="836"><rect width="100%" height="100%" fill="#0b1017"/><g font-family="Segoe UI,Arial,sans-serif">${pieces.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'comparison-board.svg'),svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out,'comparison-board.png'));
  const card=c=>`<article id="${c.variant}"><h3><code>${String(c.number).padStart(2,'0')} · ${c.variant}</code>${esc(c.title)}</h3><img class="large ${c.kind}" src="${c.files[c.kind==='compact'?128:192].file}" width="${c.kind==='compact'?128:192}" height="${c.kind==='compact'?128:192}" alt="${esc(c.title+' · '+c.kind)}"><div class="native">${[64,32,24].map(s=>`<div><img src="${c.files[s].file}" width="${s}" height="${s}" alt=""><span>${s} px</span></div>`).join('')}</div>${c.kind==='illustrated'&&input.show_previous!==false?`<details><summary>Voir la version précédente</summary><img src="../${input.source_review_dir||'skill-icon-review-05'}/${c.id}-128.png" width="128" height="128" alt="Version précédente de ${esc(c.title)}"></details>`:''}</article>`;
  const sections=groups.map(g=>`<section id="${g.key}"><h2>${esc(g.title)}</h2><div class="gallery">${rows.filter(c=>c.kind===g.key).map(card).join('')}</div></section>`).join('');
  fs.writeFileSync(path.join(out,'index.html'),`<!doctype html><html lang="fr"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>${input.review_id} · Corrections et versions compactes</title><style>*{box-sizing:border-box}body{margin:0;padding:24px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial,sans-serif}header,main,footer{max-width:1160px;margin:auto}header{margin-bottom:24px}small,code{color:#b6a8ff}h1{font-size:26px;margin:10px 0}header p,footer{color:#a6b6c5;font-size:13px;line-height:1.5}nav{display:flex;gap:10px;flex-wrap:wrap;margin:18px 0}a{color:#d7cef4;text-decoration:none}nav a,footer a{display:inline-block;padding:7px 10px;border:1px solid #3a4654;border-radius:5px}a:hover{border-color:#b6a8ff}h2{font-size:19px;color:#d6cdf3;margin:0 0 16px}section{margin-bottom:30px}.gallery{display:grid;grid-template-columns:repeat(auto-fit,minmax(265px,1fr));gap:18px}article{min-width:0;background:#111923;border:1px solid #3a4654;border-radius:10px;padding:17px}h3{font-size:15px;font-weight:600;min-height:65px;line-height:1.4;margin:0 0 16px}h3 code{display:block;font:12px Segoe UI,Arial;margin-bottom:5px}.large{display:block;margin:0 auto 22px;border-radius:7px}.large.compact{margin-top:32px;margin-bottom:54px}.native{display:flex;gap:26px;align-items:end;justify-content:center;min-height:88px}.native>div{display:grid;gap:8px;justify-items:center}.native img{display:block}.native span{color:#a6b6c5;font-size:11px}details{font-size:12px;color:#a6b6c5;margin-top:18px}summary{cursor:pointer}details img{display:block;margin:14px auto 0}footer{margin:28px auto}footer a{margin-top:10px}@media(max-width:420px){body{padding:14px}.gallery{grid-template-columns:minmax(0,1fr)}h1{font-size:23px}}</style></head><body><header><small>${input.review_id} · REVUE VISUELLE</small><h1>Corrections et versions compactes</h1><p>Les numéros 01, 02, 04 et 06 reprennent la planche précédente. En haut : les quatre illustrations corrigées. En bas : des dessins distincts, simplifiés pour les petits affichages.</p><nav><a href="#illustrated">Illustrations corrigées</a><a href="#compact">Versions compactes</a><a href="../skill-icon-review-05/index.html">Planche précédente</a></nav></header><main>${sections}</main><footer>Illustrations et versions compactes sont des propositions distinctes. La revue concerne leur dessin.<br><a href="comparison-board.png">Ouvrir la planche des tailles réelles</a></footer></body></html>`);
  fs.writeFileSync(path.join(out,'prompts.txt'),rows.map(c=>`=== ${c.variant} · ${c.title} ===\n${fs.readFileSync(path.join(out,c.prompt),'utf8')}\n`).join('\n'));
  const feedback=readJson(path.join(out,'feedback.json'));
  // Apply the requested review heading without changing the established card layout.
  const htmlPath=path.join(out,'index.html');
  if(input.source_review_dir)fs.writeFileSync(htmlPath,fs.readFileSync(htmlPath,'utf8').replace('../skill-icon-review-05/index.html','../'+input.source_review_dir+'/index.html'));
  fs.writeFileSync(htmlPath,fs.readFileSync(htmlPath,'utf8').replaceAll('Corrections et versions compactes',esc(title)).replace('>Illustrations corrigées</a>','>'+esc(input.illustrated_title||'Illustrations corrigées')+'</a>').replace('Les numéros 01, 02, 04 et 06 reprennent la planche précédente. En haut : les quatre illustrations corrigées. En bas : des dessins distincts, simplifiés pour les petits affichages.',esc(input.description||'Les numéros 01, 02, 04 et 06 reprennent la planche précédente. En haut : les quatre illustrations corrigées. En bas : des dessins distincts, simplifiés pour les petits affichages.')));
  fs.writeFileSync(path.join(out,'review.json'),JSON.stringify({review_id:input.review_id,date:input.date,status:'awaiting_visual_review',source_review:input.source_review||'SKILLS-IC05',scope:input.scope||'Four illustration revisions and four distinct compact icon proposals. No gameplay, skill copy, status applicability or active UI changes.',generation_tool:'image_gen.imagegen',generation_strategy:input.generation_strategy||'One tool call per illustrated revision and one separate call per compact subject.',feedback,candidates:rows,approval_groups:{illustrated:'awaiting_visual_review',compact:'awaiting_visual_review'},unchanged_pending_review:input.unchanged_pending_review||['T01','A03','M01','M03'],preserved_previous_image_hashes:true,preserved_accepted_image_hashes:true,accepted_icon_count:accepted.accepted_count,accepted_compact_icon_count:accepted.compact_accepted_count||0,packaging:'Source masters copied unchanged; preserved exports copied exactly; new compact Lanczos3 reductions; SVG layout only for review.'},null,2)+'\n');
  console.log(JSON.stringify({review_id:input.review_id,illustrated:rows.filter(c=>c.kind==='illustrated').length,compact:rows.filter(c=>c.kind==='compact').length,accepted_preserved:accepted.accepted_count,out}));
}
main().catch(e=>{console.error(e);process.exitCode=1});
