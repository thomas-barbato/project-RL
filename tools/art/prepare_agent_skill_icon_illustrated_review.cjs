// Package a generated review contact sheet; no redraw, recoloring or game integration.
const fs=require('node:fs');
const path=require('node:path');
const crypto=require('node:crypto');
const sharp=require(process.env.RL_SHARP_MODULE||'sharp');
const root=path.resolve(__dirname,'../..');
const base=path.join(root,'artifacts/refonte-agent-2026-10-06');
const out=path.join(base,'skill-icon-review-02');
const ids=['B01','ME04','SU04','D59','FE05','FR03','BM02','SF00'];
const data=JSON.parse(fs.readFileSync(path.join(base,'skill-window-review-01/skills.json'),'utf8'));
const esc=t=>String(t).replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&apos;'}[c]));
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const text=(x,y,t,size=13,color='#e1e7ec',weight='normal')=>`<text x="${x}" y="${y}" font-size="${size}" fill="${color}" font-weight="${weight}">${esc(t)}</text>`;
async function main(){
  const source=process.argv[2];
  if(!source||!path.isAbsolute(source))throw Error('Pass the absolute path returned by image_gen.');
  fs.mkdirSync(out,{recursive:true});
  const master=path.join(out,'illustrated-master-01.png');
  fs.copyFileSync(source,master);
  const meta=await sharp(master).metadata();
  if(Math.abs(meta.width/meta.height-2)>.01)throw Error('Expected four by two square cells.');
  const cell=meta.width/4;
  const rows=[];
  for(const [i,id] of ids.entries()){
    const s=data.skills.find(s=>s.id===id);
    if(!s)throw Error('Unknown reviewed skill '+id);
    const col=i%4,row=Math.floor(i/4),left=Math.round(col*cell),top=Math.round(row*cell);
    const area={left:left+3,top:top+3,width:Math.round((col+1)*cell)-left-6,height:Math.round((row+1)*cell)-top-6};
    const files={};
    for(const size of [192,128,64,32,24]){
      const name=`${id}-${size}.png`;
      const buf=await sharp(master).extract(area).resize(size,size,{kernel:'lanczos3'}).png().toBuffer();
      fs.writeFileSync(path.join(out,name),buf);
      files[size]={file:name,sha256:hash(buf)};
    }
    rows.push({id,title:s.title,roleplay:s.roleplay,source_area:area,files});
  }
  const pieces=[text(24,34,'SKILLS-IC02 · Essai illustré',25,'#e1e7ec','600'),text(24,61,'Illustration de l’action · aperçu 128 px · tailles réelles 64 / 32 / 24 px',13,'#a6b6c5')];
  for(const [i,s] of rows.entries()){
    const x=24+(i%4)*280,y=86+Math.floor(i/4)*326;
    pieces.push(`<rect x="${x}" y="${y}" width="266" height="310" rx="9" fill="#111923" stroke="#3a4654"/>`);
    pieces.push(text(x+13,y+23,s.id,12,'#b6a8ff','600'));
    pieces.push(text(x+13,y+44,s.title,12,'#e1e7ec','600'));
    for(const [size,dx,dy] of [[128,69,56],[64,13,203],[32,108,219],[24,175,223]]){
      const content=fs.readFileSync(path.join(out,s.files[size].file)).toString('base64');
      pieces.push(`<image x="${x+dx}" y="${y+dy}" width="${size}" height="${size}" href="data:image/png;base64,${content}"/>`);
      pieces.push(text(x+dx,y+dy+size+16,size+' px',10,'#a6b6c5'));
    }
  }
  pieces.push(text(24,768,'Propositions non intégrées · mêmes textes et effets validés · revue du dessin uniquement',12,'#a6b6c5'));
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="1160" height="788"><rect width="100%" height="100%" fill="#0b1017"/><g font-family="Segoe UI,Arial,sans-serif">${pieces.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'native-review-board.svg'),svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-review-board.png'));
  const cards=rows.map(s=>`<article id="${s.id}"><h2><code>${s.id}</code> ${esc(s.title)}</h2><img class="large" src="${s.files[192].file}" width="192" height="192" alt="Proposition ${esc(s.title)}"><div class="native">${[64,32,24].map(size=>`<div><img src="${s.files[size].file}" width="${size}" height="${size}" alt=""><span>${size} px</span></div>`).join('')}</div><p>${esc(s.roleplay)}</p></article>`).join('');
  fs.writeFileSync(path.join(out,'index.html'),`<!doctype html><html lang="fr"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>SKILLS-IC02 · Icônes illustrées</title><style>*{box-sizing:border-box}body{margin:0;padding:24px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial,sans-serif}header{max-width:1160px;margin:0 auto 25px}header small{color:#b6a8ff;font-weight:600}h1{font-size:26px;margin:10px 0}header p{color:#a6b6c5;line-height:1.5}nav{display:flex;gap:8px;flex-wrap:wrap;margin-top:20px}a{color:#d7cef4;padding:7px 12px;border:1px solid #3a4654;border-radius:5px;text-decoration:none}a:hover{border-color:#b6a8ff}.gallery{display:grid;grid-template-columns:repeat(auto-fit,minmax(265px,1fr));gap:18px;max-width:1160px;margin:auto}article{background:#111923;border:1px solid #3a4654;border-radius:10px;padding:17px;scroll-margin-top:15px;min-width:0}h2{font-size:16px;font-weight:600;margin:0 0 16px;line-height:1.4}code{font:12px Segoe UI,Arial;color:#b6a8ff;margin-right:5px}.large{display:block;border-radius:8px;margin:0 auto 22px}.native{display:flex;gap:26px;align-items:end;justify-content:center;min-height:88px}.native>div{display:grid;gap:8px;justify-items:center}.native img{border-radius:3px;display:block}.native span{font-size:11px;color:#a6b6c5}article p{font-style:italic;color:#b4c1cd;font-size:13px;line-height:1.5;margin:20px 0 0}footer{max-width:1160px;margin:28px auto 0;font-size:13px;color:#a6b6c5}footer a{display:inline-block}@media(max-width:420px){body{padding:14px}.gallery{grid-template-columns:minmax(0,1fr)}h1{font-size:23px}}</style></head><body><header><small>SKILLS-IC02 · REVUE VISUELLE</small><h1>Une icône qui montre l’action</h1><p>Huit propositions au style BD du jeu. Le grand dessin permet d’examiner la qualité ; les vues 64, 32 et 24 px montrent la taille réelle. Les textes et les effets de ces compétences restent ceux déjà validés.</p><nav aria-label="Compétences proposées">${rows.map(s=>`<a href="#${s.id}">${s.id}</a>`).join('')}</nav></header><main class="gallery">${cards}</main><footer>Propositions à valider avant intégration.<p><a href="native-review-board.png">Ouvrir la planche des tailles réelles</a></p></footer></body></html>`);
  const review=JSON.parse(fs.readFileSync(path.join(out,'review.json'),'utf8'));
  Object.assign(review,{status:'awaiting_visual_review',master:{file:path.basename(master),width:meta.width,height:meta.height,sha256:hash(fs.readFileSync(master))},candidates:rows,packaging:'Exact grid crop, three-pixel separator removal, Lanczos3 reduction; no semantic image edits.'});
  fs.writeFileSync(path.join(out,'review.json'),JSON.stringify(review,null,2)+'\n');
  console.log(JSON.stringify({review_id:review.review_id,source_size:[meta.width,meta.height],candidates:rows.length,out}));
}
main().catch(e=>{console.error(e);process.exitCode=1});
