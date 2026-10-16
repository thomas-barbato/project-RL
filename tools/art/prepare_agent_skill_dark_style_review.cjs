// Isolated numbered style trial: never replaces accepted or runtime icon assets.
const fs = require('node:fs'), path = require('node:path'), crypto = require('node:crypto');
const sharp = require(process.env.RL_SHARP_MODULE || 'sharp');
const root = path.resolve(__dirname, '../..');
const base = path.join(root, 'artifacts/refonte-agent-2026-10-06');
const out = path.join(base, 'skill-icon-review-16');
const read = p => JSON.parse(fs.readFileSync(p, 'utf8').replace(/^\uFEFF/, ''));
const sha = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const write = (p, v) => fs.writeFileSync(p, JSON.stringify(v, null, 2) + '\n');
const esc = s => String(s).replace(/[&<>\"]/g, c => ({'&':'&amp;', '<':'&lt;', '>':'&gt;', '\"':'&quot;'}[c]));
const rel = p => path.relative(root, p).replace(/\\/g, '/');
function protectedCheck(protectedFiles) {
  for (const p of protectedFiles) if (sha(path.join(root, p.file)) !== p.sha256) throw Error('Protected asset changed: ' + p.file);
}
async function main() {
  if (fs.readdirSync(out).some(f => f.startsWith('visual-acceptance') && f.endsWith('.json'))) throw Error('Accepted trial is frozen.');
  const input = read(path.join(out, 'sources.json'));
  const baseline = read(path.join(out, 'protected-baseline.json'));
  protectedCheck(baseline.protected_files);
  const ledger = read(path.join(base, 'skill-icon-acceptance.json'));
  const catalogue = read(path.join(base, 'skill-window-review-05/skills.json')).skills;
  fs.mkdirSync(path.join(out, 'prompts'), {recursive:true});
  const candidates = [];
  for (const c of input.candidates) {
    const skill = catalogue.find(s => s.id === c.id);
    if (!skill) throw Error('Unknown skill ID: ' + c.id);
    const meta = await sharp(c.source).metadata();
    if (meta.width !== meta.height || meta.width < 192) throw Error('Invalid source dimensions');
    const master = c.variant + '-master.png';
    fs.copyFileSync(c.source, path.join(out, master));
    const files = {}, previousFiles = {};
    for (const size of [192,128,64,32,24]) {
      const file = c.variant + '-' + size + '.png';
      await sharp(c.source).resize(size,size,{kernel:'lanczos3'}).png().toFile(path.join(out,file));
      const m = await sharp(path.join(out,file)).metadata();
      if (m.width !== size || m.height !== size) throw Error('Export size mismatch');
      files[size] = {file,sha256:sha(path.join(out,file)),width:size,height:size};
      const old = path.join(base,c.previous_directory,c.previous_variant + '-' + size + '.png');
      previousFiles[size] = {file:rel(old),sha256:sha(old),width:size,height:size};
    }
    fs.writeFileSync(path.join(out,'prompts',c.variant + '.txt'),c.prompt+'\n');
    candidates.push({...c,kind:skill.kind,visual_status:'awaiting_visual_review',
      previous_status:ledger.compact_accepted.some(a => a.id===c.id && a.variant===c.previous_variant)?'accepted':'awaiting_visual_review',
      master:{file:master,sha256:sha(path.join(out,master)),width:meta.width,height:meta.height},files,previous_files:previousFiles});
  }
  const oldUrl = (c,size) => '../'+c.previous_directory+'/'+c.previous_variant+'-'+size+'.png';
  const art = (c,old) => '<section><h3>'+(old?'Version précédente':'Essai sombre')+'</h3><img class="large" src="'+(old?oldUrl(c,128):c.files[128].file)+'" width="128" height="128" alt="'+esc(c.title)+' '+(old?'précédente':'essai')+'"><div class="native">'+[32,64,24].map(size=>'<div><img src="'+(old?oldUrl(c,size):c.files[size].file)+'" width="'+size+'" height="'+size+'" alt="'+esc(c.title)+' '+size+' px"><span>'+size+' px</span></div>').join('')+'</div></section>';
  const cards = candidates.map(c=>'<article id="'+c.variant+'"><small>'+c.number+' · '+c.previous_variant+' → '+c.variant+'</small><h2>'+esc(c.title)+'</h2><div class="compare">'+art(c,true)+art(c,false)+'</div></article>').join('');
  const buttons = candidates.map(c=>'<div class="skill"><img src="'+c.files[64].file+'" width="64" height="64" alt="'+esc(c.title)+'"><span class="rank">1</span><b>'+esc(c.title)+'</b><small>'+(c.kind==='passive'?'Passif':'Actif')+'</small></div>').join('');
  const css = '*{box-sizing:border-box}body{margin:0;padding:20px;background:#0b1017;color:#e1e7ec;font:14px Segoe UI,Arial}header,main,footer{max-width:1040px;margin:auto}h1{font-size:24px;margin:8px 0}p{color:#a6b6c5;line-height:1.5;margin:7px 0 16px}small{color:#baa9a6}h2{font-size:17px;margin:7px 0 10px}h3{font-size:12px;color:#a6b6c5;font-weight:400;margin:0 0 8px}.gallery{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:14px}article{padding:14px;background:#111923;border:1px solid #343e4b;border-radius:8px;min-width:0}.compare{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:16px}.compare section{display:grid;justify-items:center;min-width:0}.large{margin-bottom:10px}img{display:block;flex-shrink:0}.native{display:flex;justify-content:center;align-items:end;gap:20px}.native>div{display:grid;justify-items:center;gap:7px}.native span{font-size:11px;color:#a6b6c5}.context{margin-top:22px;padding:20px;background:#0a0813;border:1px solid #3c3345}.context h2{margin-bottom:20px}.skills{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:20px}.skill{display:grid;grid-template-columns:64px 16px;grid-template-rows:64px auto auto;justify-content:center;justify-items:center;column-gap:6px;row-gap:2px;text-align:center}.skill .rank{color:#d4c6ad;align-self:end}.skill b,.skill small{grid-column:1/3;font-size:12px;max-width:160px}.skill b{font-weight:400;margin-top:7px}footer{margin:18px auto;color:#a6b6c5;font-size:12px;line-height:1.5}a{color:#d8b9ad}@media(max-width:700px){.gallery{grid-template-columns:1fr}.skills{grid-template-columns:repeat(2,minmax(0,1fr))}}';
  const html='<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>SKILLS-IC16 · Essai sombre</title><style>'+css+'</style><header><small>SKILLS-IC16 · QUATRE ESSAIS À EXAMINER</small><h1>Une direction plus sombre et inquiétante</h1><p>Comparaison des dessins · vues 32 / 64 / 24 px à taille réelle, aperçu supérieur à 128 px.</p></header><main><div class="gallery">'+cards+'</div><section class="context"><h2>Aperçu dans la présentation des compétences · 64 px</h2><div class="skills">'+buttons+'</div></section></main><footer>Essais visuels proposés ; textes et effets des compétences conservés. Les dessins précédents gardent leur état de validation.<br><a href="prompts.txt">Prompts Imagegen</a> · <a href="comparison-board.png">Planche de comparaison</a></footer></html>';
  fs.writeFileSync(path.join(out,'index.html'),html);
  const image = (file,x,y,size) => '<image x="'+x+'" y="'+y+'" width="'+size+'" height="'+size+'" href="data:image/png;base64,'+fs.readFileSync(file).toString('base64')+'"/>';
  const pieces=['<rect width="1040" height="850" fill="#0b1017"/>','<text x="24" y="35" font-size="23" fill="#e1e7ec">SKILLS-IC16 · Quatre essais sombres</text>','<text x="24" y="62" font-size="13" fill="#a6b6c5">Comparaison · 32 / 64 / 24 px à taille réelle · aperçu supérieur à 128 px</text>'];
  for (const [i,c] of candidates.entries()) {
    const x=24+(i%2)*508, y=90+Math.floor(i/2)*362;
    pieces.push('<rect x="'+x+'" y="'+y+'" width="492" height="346" rx="8" fill="#111923" stroke="#343e4b"/>','<text x="'+(x+16)+'" y="'+(y+24)+'" fill="#baa9a6" font-size="12">'+c.number+' · '+c.previous_variant+' → '+c.variant+'</text>','<text x="'+(x+16)+'" y="'+(y+48)+'" fill="#e1e7ec" font-size="17">'+esc(c.title)+'</text>');
    for (const old of [true,false]) {
      const left=x+16+(old?0:238);
      pieces.push('<text x="'+(left+111)+'" y="'+(y+72)+'" fill="#a6b6c5" font-size="12" text-anchor="middle">'+(old?'Version précédente':'Essai sombre')+'</text>');
      const fileFor=size=>old?path.join(root,c.previous_files[size].file):path.join(out,c.files[size].file);
      pieces.push(image(fileFor(128),left+47,y+84,128));
      for (const [size,dx] of [[32,25],[64,77],[24,161]]) {
        pieces.push(image(fileFor(size),left+dx,y+230+64-size,size),'<text x="'+(left+dx+size/2)+'" y="'+(y+315)+'" fill="#a6b6c5" font-size="11" text-anchor="middle">'+size+' px</text>');
      }
    }
  }
  pieces.push('<text x="24" y="837" fill="#a6b6c5" font-size="12">Propositions visuelles numérotées · aucun remplacement des dessins du jeu</text>');
  const svg='<svg xmlns="http://www.w3.org/2000/svg" width="1040" height="850"><g font-family="Segoe UI,Arial">'+pieces.join('')+'</g></svg>';
  fs.writeFileSync(path.join(out,'comparison-board.svg'),svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out,'comparison-board.png'));
  fs.writeFileSync(path.join(out,'prompts.txt'),candidates.map(c=>'=== '+c.number+' · '+c.variant+' ===\n'+c.prompt+'\n').join('\n'));
  protectedCheck(baseline.protected_files);
  write(path.join(out,'review.json'),{...input,candidates,protected_hashes:baseline.protected_files,no_new_approval_inferred:true,no_runtime_changes:true});
  write(path.join(out,'validation.json'),{review_id:input.review_id,status:'awaiting_visual_review',candidate_count:candidates.length,dimensions_verified:true,protected_files_preserved:baseline.protected_files.length,no_runtime_changes:true,board:{file:'comparison-board.png',width:1040,height:850},generation_tool:'image_gen.imagegen',generation_mode:'built-in'});
  console.log(JSON.stringify({review_id:input.review_id,candidates:candidates.map(c=>c.variant),protected_files_preserved:baseline.protected_files.length}));
}
main().catch(e=>{console.error(e);process.exitCode=1;});
