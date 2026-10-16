// Package one image_gen source per icon into a numbered review. No gameplay edits.
const fs=require('node:fs');
const path=require('node:path');
const crypto=require('node:crypto');
const sharp=require(process.env.RL_SHARP_MODULE||'sharp');
const root=path.resolve(__dirname,'../..');
const base=path.join(root,'artifacts/refonte-agent-2026-10-06');
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const readJson=p=>JSON.parse(fs.readFileSync(p,'utf8').replace(/^\uFEFF/,''));
const esc=s=>String(s).replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&apos;'}[c]));
const relative=p=>path.relative(root,p).replace(/\\/g,'/');
const sizes=[192,128,64,32,24];
async function main(){
  const inputPath=path.resolve(process.argv[2]||'');
  if(!fs.existsSync(inputPath)||!inputPath.endsWith('.json'))throw Error('Pass sources.json.');
  const input=readJson(inputPath),out=path.dirname(inputPath);
  const skills=readJson(path.join(base,'skill-window-review-01/skills.json')).skills;
  const review2=readJson(path.join(base,'skill-icon-review-02/review.json'));
  const review3=readJson(path.join(base,'skill-icon-review-03/review.json'));
  const review4=readJson(path.join(base,'skill-icon-review-04/review.json'));
  if(review4.status!=='accepted'||!review3.SF00.approval)throw Error('Accepted revisions must be recorded.');
  const accepted=review2.approval.accepted_icon_ids.map(id=>({id,variant:id,review:'SKILLS-IC02',candidate:review2.candidates.find(c=>c.id===id),dir:path.join(base,'skill-icon-review-02')}));
  accepted.push({id:'D59',variant:'D59-v4',review:'SKILLS-IC04',candidate:review4.candidates.find(c=>c.id==='D59-v4'),dir:path.join(base,'skill-icon-review-04')});
  accepted.push({id:'SF00',variant:'SF00-v2',review:'SKILLS-IC03',candidate:review3.candidates.find(c=>c.id==='SF00-v2'),dir:path.join(base,'skill-icon-review-03')});
  const approved=[];
  for(const c of accepted){
    const files={};
    for(const [size,f] of Object.entries(c.candidate.files)){
      const full=path.join(c.dir,f.file);
      if(hash(fs.readFileSync(full))!==f.sha256)throw Error('Accepted asset changed: '+c.id);
      files[size]={file:relative(full),sha256:f.sha256};
    }
    approved.push({id:c.id,variant:c.variant,title:skills.find(s=>s.id===c.id).title,visual_status:'accepted',acceptance_review:c.review,acceptance_reference:relative(path.join(c.dir,'review.json')),files});
  }
  fs.writeFileSync(path.join(base,'skill-icon-acceptance.json'),JSON.stringify({date:input.date,scope:'Exact approved icon drawings only; no skill numbers, copy or window-layout approval implied.',accepted_count:approved.length,accepted:approved},null,2)+'\n');
  const rows=[];
  for(const [i,c] of input.candidates.entries()){
    const skill=skills.find(s=>s.id===c.id);
    if(!skill)throw Error('Unknown skill '+c.id);
    if(approved.some(s=>s.id===c.id))throw Error('Batch would replace accepted art '+c.id);
    const name=c.id+'-master.png';
    fs.copyFileSync(c.source,path.join(out,name));
    const meta=await sharp(path.join(out,name)).metadata();
    if(meta.width!==meta.height||meta.width<512)throw Error('Expected square illustration master for '+c.id);
    const files={};
    for(const size of sizes){
      const file=c.id+'-'+size+'.png';
      await sharp(path.join(out,name)).resize(size,size,{kernel:'lanczos3'}).png().toFile(path.join(out,file));
      files[size]={file,sha256:hash(fs.readFileSync(path.join(out,file)))};
    }
    rows.push({number:i+1,id:c.id,title:skill.title,kind:skill.kind,branch:skill.branch,roleplay:skill.roleplay,skill_source:skill.source,visual_status:'awaiting_visual_review',prompt:c.prompt,master:{file:name,width:meta.width,height:meta.height,sha256:hash(fs.readFileSync(path.join(out,name)))},files});
  }
  const cols=4,cardW=266,colPitch=280,rowPitch=326,boardH=86+Math.ceil(rows.length/cols)*rowPitch+50;
  const text=(x,y,t,size=13,color='#e1e7ec')=>`<text x="${x}" y="${y}" font-size="${size}" fill="${color}">${esc(t)}</text>`;
  const parts=[text(24,34,input.review_id+' · '+input.title,25),text(24,61,'Huit nouvelles propositions · aperçu 128 px · tailles réelles 64 / 32 / 24 px',13,'#a6b6c5')];
  for(const [i,c] of rows.entries()){
    const x=24+(i%cols)*colPitch,y=86+Math.floor(i/cols)*rowPitch;
    parts.push(`<rect x="${x}" y="${y}" width="${cardW}" height="310" rx="9" fill="#111923" stroke="#3a4654"/>`,text(x+13,y+23,String(c.number).padStart(2,'0')+' · '+c.id,12,'#b6a8ff'),text(x+13,y+44,c.title,12));
    for(const [size,dx,dy] of [[128,69,56],[64,13,203],[32,108,219],[24,175,223]]){
      const png=fs.readFileSync(path.join(out,c.files[size].file)).toString('base64');
      parts.push(`<image x="${x+dx}" y="${y+dy}" width="${size}" height="${size}" href="data:image/png;base64,${png}"/>`,text(x+dx,y+dy+size+16,size+' px',10,'#a6b6c5'));
    }
  }
  parts.push(text(24,boardH-24,'Revue du dessin uniquement · textes et règles conservés · huit icônes précédentes validées',12,'#a6b6c5'));
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="1160" height="${boardH}"><rect width="100%" height="100%" fill="#0b1017"/><g font-family="Segoe UI,Arial,sans-serif">${parts.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'native-review-board.svg'),svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-review-board.png'));
  const cards=rows.map(c=>`<article id="${c.id}"><h2><code>${String(c.number).padStart(2,'0')} · ${c.id}</code>${esc(c.title)}</h2><img class="large" src="${c.files[192].file}" width="192" height="192" alt="${esc(c.title)}"><div class="native">${[64,32,24].map(s=>`<div><img src="${c.files[s].file}" width="${s}" height="${s}" alt=""><span>${s} px</span></div>`).join('')}</div><p>${esc(c.roleplay)}</p></article>`).join('');
  fs.writeFileSync(path.join(out,'index.html'),`<!doctype html><html lang="fr"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>${esc(input.review_id+' · '+input.title)}</title><style>*{box-sizing:border-box}body{margin:0;padding:24px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial,sans-serif}header,main,footer{max-width:1160px;margin:auto}header{margin-bottom:24px}small,code{color:#b6a8ff}h1{font-size:26px;margin:10px 0}header p,footer{color:#a6b6c5;font-size:13px;line-height:1.5}nav{display:flex;gap:8px;flex-wrap:wrap;margin:18px 0}a{color:#d7cef4;text-decoration:none}nav a,footer a{display:inline-block;padding:7px 10px;border:1px solid #3a4654;border-radius:5px}a:hover{border-color:#b6a8ff}.gallery{display:grid;grid-template-columns:repeat(auto-fit,minmax(265px,1fr));gap:18px}article{min-width:0;background:#111923;border:1px solid #3a4654;border-radius:10px;padding:17px;scroll-margin-top:15px}h2{font-size:15px;font-weight:600;min-height:65px;line-height:1.4;margin:0 0 16px}h2 code{display:block;font:12px Segoe UI,Arial;margin-bottom:5px}.large{display:block;border-radius:7px;margin:0 auto 22px}.native{display:flex;gap:26px;align-items:end;justify-content:center;min-height:88px}.native>div{display:grid;gap:8px;justify-items:center}.native img{display:block}.native span{color:#a6b6c5;font-size:11px}article p{font-size:13px;font-style:italic;color:#b4c1cd;line-height:1.5;margin:20px 0 0}footer{margin:28px auto}footer a{margin-top:10px}@media(max-width:420px){body{padding:14px}.gallery{grid-template-columns:minmax(0,1fr)}h1{font-size:23px}}</style></head><body><header><small>${esc(input.review_id)} · REVUE VISUELLE</small><h1>${esc(input.title)}</h1><p>Huit nouvelles icônes dans le style BD accepté. Les huit dessins de la première planche sont conservés, avec les versions corrigées du drone et de Furtivité.</p><nav aria-label="Compétences proposées">${rows.map(c=>`<a href="#${c.id}">${String(c.number).padStart(2,'0')} · ${c.id}</a>`).join('')}</nav></header><main class="gallery">${cards}</main><footer>Vues 64, 32 et 24 px à taille réelle. Revue du dessin uniquement ; les textes et les effets restent ceux déjà définis.<br><a href="native-review-board.png">Ouvrir la planche des tailles réelles</a></footer></body></html>`);
  const prompts=rows.map(c=>`=== ${c.id} · ${c.title} ===\n${fs.readFileSync(path.join(out,c.prompt),'utf8')}\n`).join('\n');
  fs.writeFileSync(path.join(out,'prompts.txt'),prompts);
  fs.writeFileSync(path.join(out,'review.json'),JSON.stringify({review_id:input.review_id,date:input.date,status:'awaiting_visual_review',scope:'New icon drawings only. No active asset, skill copy, mechanics, monster identity or UI behavior changes.',generation_tool:'image_gen.imagegen',generation_strategy:'One independent tool call and one source master per skill.',candidate_count:rows.length,candidates:rows,accepted_prior_icon_ids:approved.map(c=>c.id),accepted_prior_hashes_preserved:true,approval_ledger:relative(path.join(base,'skill-icon-acceptance.json')),packaging:'Masters copied unchanged; Lanczos3 reductions. SVG used only to lay out the review board.'},null,2)+'\n');
  console.log(JSON.stringify({review_id:input.review_id,candidates:rows.length,accepted_preserved:approved.length,out}));
}
main().catch(e=>{console.error(e);process.exitCode=1});
