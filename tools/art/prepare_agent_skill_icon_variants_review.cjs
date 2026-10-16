// Package reviewed originals and two image_gen revisions for side-by-side review.
// No semantic image editing and no activation in the game.
const fs=require('node:fs');
const path=require('node:path');
const crypto=require('node:crypto');
const sharp=require(process.env.RL_SHARP_MODULE||'sharp');
const root=path.resolve(__dirname,'../..');
const base=path.join(root,'artifacts/refonte-agent-2026-10-06');
const old=path.join(base,'skill-icon-review-02');
const out=path.join(base,'skill-icon-review-03');
const esc=t=>String(t).replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&apos;'}[c]));
const hash=b=>crypto.createHash('sha256').update(b).digest('hex');
const readJson=p=>JSON.parse(fs.readFileSync(p,'utf8').replace(/^\uFEFF/,''));
const text=(x,y,t,size=13,color='#e1e7ec',weight='normal')=>`<text x="${x}" y="${y}" font-size="${size}" fill="${color}" font-weight="${weight}">${esc(t)}</text>`;
async function main(){
  const [drone,stealth,discarded]=process.argv.slice(2);
  if(!drone||!stealth)throw Error('Pass the two absolute source paths returned by image_gen.');
  fs.mkdirSync(out,{recursive:true});
  const specs=[
    {id:'D59-v1',skill:'D59',title:'Artilleur · précédent',old:true},
    {id:'D59-v3',skill:'D59',title:'Artilleur · canon corrigé',source:drone},
    {id:'SF00-v1',skill:'SF00',title:'Furtivité · précédente',old:true},
    {id:'SF00-v2',skill:'SF00',title:'Furtivité · variante futuriste',source:stealth}
  ];
  const oldReview=readJson(path.join(old,'review.json'));
  const approved=Object.fromEntries(oldReview.candidates.filter(c=>['B01','ME04','SU04','FE05','FR03','BM02'].includes(c.id)).map(c=>[c.id,c.files]));
  for(const c of specs){
    c.files={};
    if(c.source){
      const name=c.id+'-master.png';fs.copyFileSync(c.source,path.join(out,name));
      const m=await sharp(path.join(out,name)).metadata();
      if(Math.abs(m.width/m.height-1)>.01)throw Error('Icon must be square: '+c.id);
      c.master={file:name,width:m.width,height:m.height,sha256:hash(fs.readFileSync(path.join(out,name)))};
    }
    for(const size of [192,128,64,32,24]){
      const name=c.id+'-'+size+'.png';
      if(c.old)fs.copyFileSync(path.join(old,c.skill+'-'+size+'.png'),path.join(out,name));
      else await sharp(path.join(out,c.master.file)).resize(size,size,{kernel:'lanczos3'}).png().toFile(path.join(out,name));
      c.files[size]={file:name,sha256:hash(fs.readFileSync(path.join(out,name)))};
    }
    delete c.source;
  }
  if(discarded)fs.copyFileSync(discarded,path.join(out,'unused-D59-v2-master.png'));
  const pieces=[text(24,34,'SKILLS-IC03 · Canon et furtivité',25,'#e1e7ec','600'),text(24,61,'Correction ciblée du drone · variante technologique de Furtivité',13,'#a6b6c5')];
  for(const [i,c] of specs.entries()){
    const x=24+(i%2)*354,y=86+Math.floor(i/2)*382;
    pieces.push(`<rect x="${x}" y="${y}" width="340" height="366" rx="9" fill="#111923" stroke="#3a4654"/>`);
    pieces.push(text(x+15,y+25,c.id,12,'#b6a8ff','600'),text(x+15,y+48,c.title,15,'#e1e7ec','600'));
    for(const [size,dx,dy] of [[192,74,62],[64,24,277],[32,149,293],[24,249,297]]){
      const src=fs.readFileSync(path.join(out,c.files[size].file)).toString('base64');
      pieces.push(`<image x="${x+dx}" y="${y+dy}" width="${size}" height="${size}" href="data:image/png;base64,${src}"/>`,text(x+dx,y+dy+size+16,size+' px',10,'#a6b6c5'));
    }
  }
  pieces.push(text(24,872,'Les six autres icônes validées restent intactes. Revue du dessin uniquement.',12,'#a6b6c5'));
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="744" height="896"><rect width="100%" height="100%" fill="#0b1017"/><g font-family="Segoe UI,Arial,sans-serif">${pieces.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'comparison-board.svg'),svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out,'comparison-board.png'));
  const cards=specs.map(c=>`<article><h2><code>${c.id}</code><span>${esc(c.title)}</span></h2><img class="large" src="${c.files[192].file}" width="192" height="192" alt="${esc(c.title)}"><div class="native">${[64,32,24].map(s=>`<div><img src="${c.files[s].file}" width="${s}" height="${s}" alt=""><span>${s} px</span></div>`).join('')}</div></article>`);
  const css=`*{box-sizing:border-box}body{margin:0;padding:24px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial,sans-serif}header,main,footer{max-width:920px;margin:auto}header{margin-bottom:26px}small,code{color:#b6a8ff}h1{font-size:26px;margin:9px 0}header p,footer{font-size:13px;color:#a6b6c5;line-height:1.5}.pair{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:18px;margin-bottom:24px}article{min-width:0;background:#111923;border:1px solid #3a4654;border-radius:10px;padding:18px}h2{font-size:16px;font-weight:600;margin:0 0 17px;line-height:1.4}h2 code{display:block;font:12px Segoe UI,Arial;margin-bottom:5px}.large{display:block;border-radius:7px;margin:0 auto 22px}.native{display:flex;gap:27px;align-items:end;justify-content:center;min-height:88px}.native>div{display:grid;gap:8px;justify-items:center}.native img{display:block;border-radius:3px}.native span{font-size:11px;color:#a6b6c5}footer a{color:#d7cef4;padding:7px 10px;border:1px solid #3a4654;border-radius:5px;text-decoration:none;display:inline-block;margin-top:10px}@media(max-width:570px){body{padding:14px}.pair{grid-template-columns:minmax(0,1fr)}h1{font-size:23px}}`;
  fs.writeFileSync(path.join(out,'index.html'),`<!doctype html><html lang="fr"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>SKILLS-IC03 · Canon et furtivité</title><style>${css}</style></head><body><header><small>SKILLS-IC03 · REVUE VISUELLE</small><h1>Drone et Furtivité</h1><p>Le canon gauche est réorienté dans la même direction que le droit. Pour Furtivité, la variante conserve la capuche et ajoute des signes technologiques. Les six autres dessins acceptés restent intacts.</p></header><main><section class="pair" aria-label="Comparaison du drone">${cards.slice(0,2).join('')}</section><section class="pair" aria-label="Comparaison de Furtivité">${cards.slice(2).join('')}</section></main><footer>Vues 64, 32 et 24 px à taille réelle. Nouvelles propositions à valider avant intégration.<br><a href="comparison-board.png">Ouvrir la planche de comparaison</a></footer></body></html>`);
  const review=readJson(path.join(out,'review.json'));
  Object.assign(review,{status:'awaiting_visual_review',user_clarification:'en faite concernant le drone ce qui me gene c\'est qu\'un de ses canons est mal orienté (celui sur la gauche)',D59:{status:'awaiting_visual_review',candidate:'D59-v3',scope:'Left barrel orientation only; prior framing retained.',earlier_reframe:'D59-v2 discarded after clarification'},SF00:{status:'awaiting_visual_comparison',candidate:'SF00-v2',previous_version:'SF00-v1 retained',approval:false},candidates:specs,accepted_source_assets:approved,packaging:'Unmodified source copy and Lanczos3 reductions; no semantic edits.'});
  for(const [id,files] of Object.entries(approved))for(const f of Object.values(files))if(hash(fs.readFileSync(path.join(old,f.file)))!==f.sha256)throw Error('Accepted icon changed: '+id);
  fs.writeFileSync(path.join(out,'review.json'),JSON.stringify(review,null,2)+'\n');
  console.log(JSON.stringify({review_id:review.review_id,variants:4,accepted_preserved:6,out}));
}
main().catch(e=>{console.error(e);process.exitCode=1});
