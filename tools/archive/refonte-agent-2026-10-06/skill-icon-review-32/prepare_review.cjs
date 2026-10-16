// Review layout only. No drawing, approval ledger or runtime asset is modified.
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const sharp=require('C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/node/node_modules/sharp');
const out=__dirname,root=path.resolve(out,'../../..');
const read=p=>JSON.parse(fs.readFileSync(p,'utf8').replace(/^\uFEFF/,''));
const hash=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const esc=s=>String(s).replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&apos;'}[c]));
const image=(file,x,y,size)=>`<image x="${x}" y="${y}" width="${size}" height="${size}" href="data:image/png;base64,${fs.readFileSync(file).toString('base64')}"/>`;
const text=(label,x,y,size=13,color='#e1e7ec',anchor='start')=>`<text x="${x}" y="${y}" fill="${color}" font-size="${size}" text-anchor="${anchor}">${esc(label)}</text>`;
function wrap(label,max=21){const lines=[''];for(const word of label.split(' ')){const i=lines.length-1;if(lines[i]&&(lines[i]+' '+word).length>max)lines.push(word);else lines[i]+=(lines[i]?' ':'')+word;}return lines;}
async function main(){
  if(fs.existsSync(path.join(out,'visual-acceptance-01.json')))throw Error('Accepted review is frozen');
  const review=read(path.join(out,'review.json'));
  const pieces=['<rect width="1040" height="510" fill="#0b1017"/>',text('SKILLS-IC32 · Énergie occulte · icônes sombres',24,34,23),text('59–64 · Propositions compactes · vues natives 32 / 64 / 24 px',24,60,13,'#a6b6c5')];
  pieces.push(text('Canalisation validée · référence 32 px',24,100,12,'#a6b6c5'),image(path.join(root,'artifacts/refonte-agent-2026-10-06/skill-icon-review-17/A03-C02-32.png'),300,78,32));
  for(const [i,c] of review.candidates.entries()){
    const x=24+i*166,y=134;
    pieces.push(`<rect x="${x}" y="${y}" width="152" height="330" rx="9" fill="#111923" stroke="#3a4654"/>`,text(c.number+' · '+c.variant,x+12,y+24,11,'#b6a8ff'));
    for(const [n,label] of wrap(c.title,17).entries())pieces.push(text(label,x+12,y+49+n*20,13));
    pieces.push(image(path.join(out,c.files['128'].file),x+12,y+88,128));
    for(const [size,dx] of [[32,4],[64,44],[24,120]])pieces.push(image(path.join(out,c.files[size].file),x+dx,y+232+64-size,size),text(size+' px',x+dx+size/2,y+314,11,'#a6b6c5','middle'));
  }
  pieces.push(text('Dessins soumis à validation · Canalisation est conservée · aucune modification dans le jeu',24,490,12,'#a6b6c5'));
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="1040" height="510"><g font-family="Segoe UI,Arial">${pieces.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'native-review-board.svg'),svg);await sharp(Buffer.from(svg)).png().toFile(path.join(out,'native-review-board.png'));
  const catalogue=read(path.join(root,'artifacts/refonte-agent-2026-10-06/skill-window-review-05/skills.json')).skills.filter(s=>s.branch==='Énergie occulte');
  const context=['<rect width="600" height="495" rx="8" fill="#101416" stroke="#3a4654"/>',text('Énergie occulte',20,27,18)];
  for(const [i,s] of catalogue.entries()){
    const c=review.candidates.find(c=>c.id===s.id),file=c?path.join(out,c.files['64'].file):path.join(root,'assets/skills/agent/A03-compact.png');
    const x=50+(i%3)*190,y=46+Math.floor(i/3)*143;
    context.push(image(file,x,y,64),text('0',x+70,y+39,15,'#a6b6c5'));
    for(const [n,label] of wrap(s.title,22).entries())context.push(text(label,x+32,y+84+n*17,14,'#e1e7ec','middle'));
    context.push(text(s.kind==='passive'?'Passif':'Actif',x+32,y+122,11,'#a6b6c5','middle'));
  }
  const contextSvg=`<svg xmlns="http://www.w3.org/2000/svg" width="600" height="495"><g font-family="Segoe UI,Arial">${context.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'skill-tree-context.svg'),contextSvg);await sharp(Buffer.from(contextSvg)).png().toFile(path.join(out,'skill-tree-context.png'));
  const pagePath=path.join(out,'index.html');let page=fs.readFileSync(pagePath,'utf8');
  const marker='<!-- IC32 context -->';if(page.includes(marker))throw Error('Review layout already prepared');
  page=page.replace('<footer>',marker+'<details><summary>Aperçu dans l’arbre · prototype</summary><p>Les six propositions et Canalisation déjà validée, avec les icônes à 64 px.</p><div class="context-scroll"><img src="skill-tree-context.png" width="600" height="495" alt="Prototype de l’arbre Énergie occulte avec les sept compétences"></div></details><footer>');
  page=page.replace('<a href="prompts.txt">Prompts de génération</a>','<a href="native-review-board.png">Planche à taille native</a> · <a href="prompts.txt">Prompts de génération</a>');
  page+='<style>@media(min-width:1060px){.gallery{grid-template-columns:repeat(6,minmax(0,1fr))}.native{gap:7px}article{padding:10px}h2{font-size:15px}}h2{min-height:44px}.refs{margin:10px 0 14px}details{max-width:1040px;margin:16px auto;color:#a6b6c5}summary{cursor:pointer;font-size:13px}.context-scroll{overflow:auto;max-width:100%}.context-scroll img{max-width:none;width:600px;height:495px;display:block;margin:12px auto}</style>';
  page+='<style>.native span{white-space:nowrap;font-size:11px}</style>';
  fs.writeFileSync(pagePath,page);
  const protected=read(path.join(out,'protected-before.json'));for(const [p,digest] of Object.entries(protected))if(hash(path.join(root,p))!==digest)throw Error('Protected file changed: '+p);
  const validation=read(path.join(out,'validation.json'));Object.assign(validation,{all_prior_protected_hashes_preserved:Object.keys(protected).length,native_review_board:{width:1040,height:510},preview_context:{width:600,height:495,icon_size:64,live_runtime:false},native_identity_copy_preserved:true,no_runtime_changes:true});
  fs.writeFileSync(path.join(out,'validation.json'),JSON.stringify(validation,null,2)+'\n');
  console.log(JSON.stringify({review:review.review_id,candidates:review.candidates.length,protected_files:Object.keys(protected).length,context:'review prototype only'}));
}
main().catch(e=>{console.error(e);process.exitCode=1});
