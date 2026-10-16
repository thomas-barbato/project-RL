// Package the image_gen revision and record the exact, separately approved drawings.
// This script changes review artifacts only, never active skill or UI assets.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const sharp = require(process.env.RL_SHARP_MODULE || 'sharp');
const root = path.resolve(__dirname, '../..');
const base = path.join(root, 'artifacts/refonte-agent-2026-10-06');
const priorDir = path.join(base, 'skill-icon-review-06');
const out = path.join(base, 'skill-icon-review-07');
const read = p => JSON.parse(fs.readFileSync(p, 'utf8').replace(/^\uFEFF/, ''));
const write = (p, v) => fs.writeFileSync(p, JSON.stringify(v, null, 2) + '\n');
const hash = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const rel = p => path.relative(root, p).replaceAll('\\', '/');
const esc = t => String(t).replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&apos;'}[c]));
async function main() {
  const source = process.argv[2];
  if (!source || !fs.existsSync(source)) throw Error('Pass the image_gen output path.');
  const prior = read(path.join(priorDir, 'review.json'));
  const ledgerPath = path.join(base, 'skill-icon-acceptance.json');
  const ledger = read(ledgerPath);
  for (const c of [...ledger.accepted, ...(ledger.compact_accepted || [])]) {
    for (const f of Object.values(c.files)) if (hash(path.join(root, f.file)) !== f.sha256) throw Error('Accepted image changed: ' + c.variant);
  }
  for (const c of prior.candidates) {
    for (const f of [c.master, ...Object.values(c.files)]) if (hash(path.join(priorDir, f.file)) !== f.sha256) throw Error('Prior image changed: ' + c.variant);
  }
  fs.mkdirSync(out, {recursive: true});
  const masterFile = 'B03-v3-master.png';
  fs.copyFileSync(source, path.join(out, masterFile));
  const meta = await sharp(path.join(out, masterFile)).metadata();
  if (meta.width !== meta.height || meta.width < 512) throw Error('Expected a square source master.');
  const files = {};
  for (const s of [256, 192, 128, 64, 32, 24]) {
    const file = 'B03-v3-' + s + '.png';
    await sharp(path.join(out, masterFile)).resize(s, s, {kernel:'lanczos3'}).png().toFile(path.join(out, file));
    files[s] = {file, sha256: hash(path.join(out, file))};
  }
  const feedback = {
    date: '2026-10-10', source_review: 'SKILLS-IC06',
    user_message: '1 - Dans la version "non compacte" peut-être qu’on devrait retirer la partie "armurée" :) pour que le corps soient "nue" dans le sens ou l’ont voit ses bras au lieu de l’armure. Je valide le reste',
    revision_required: ['B03-v2'],
    accepted_illustrated: ['O03-v2', 'T03-v2', 'FE01-v2'],
    accepted_compact: ['B03-C01', 'O03-C01', 'T03-C01', 'FE01-C01'],
    unchanged_pending_review: ['T01', 'A03', 'M01', 'M03'],
    scope: 'Approval applies only to the other seven drawings shown in IC06; unchanged IC05 candidates remain pending. No gameplay, copy, or status applicability approval implied.'
  };
  for (const c of prior.candidates) {
    c.visual_status = c.variant === 'B03-v2' ? 'revision_requested' : 'accepted';
    if (c.visual_status === 'accepted') {
      c.approval_date = feedback.date;
      c.user_approval = 'Je valide le reste';
      const item = {
        id:c.id, variant:c.variant, kind:c.kind, title:c.title, visual_status:'accepted',
        acceptance_review:'SKILLS-IC06', acceptance_reference:rel(path.join(priorDir,'visual-acceptance-01.json')),
        files:Object.fromEntries(Object.entries(c.files).map(([s,f]) => [s,{...f,file:rel(path.join(priorDir,f.file))}])),
        master:{...c.master,file:rel(path.join(priorDir,c.master.file))}
      };
      const list = c.kind === 'compact' ? (ledger.compact_accepted ||= []) : ledger.accepted;
      if (!list.some(x => x.variant === item.variant)) list.push(item);
    }
  }
  prior.status = 'partially_accepted';
  prior.approval_groups = {illustrated:'three_accepted_one_revision_requested', compact:'accepted'};
  prior.latest_user_feedback = feedback;
  ledger.accepted_count = ledger.accepted.length;
  ledger.compact_accepted_count = ledger.compact_accepted.length;
  ledger.accepted_drawing_count = ledger.accepted_count + ledger.compact_accepted_count;
  ledger.scope = 'Exact approved icon drawings; illustrated and compact drawings are tracked separately. No gameplay, skill copy, window layout or status applicability approval implied.';
  write(path.join(priorDir, 'visual-acceptance-01.json'), feedback);
  write(path.join(priorDir, 'review.json'), prior);
  write(ledgerPath, ledger);
  write(path.join(out, 'review.json'), {
    review_id:'SKILLS-IC07', date:feedback.date, status:'awaiting_visual_review', source_review:'SKILLS-IC06',
    scope:'B03 illustrated outer armor replaced by bare shoulders and arms; heart and organic chest retained. All seven IC06 accepted drawings stay unchanged.',
    generation_tool:'image_gen.imagegen', prompt:'prompt-B03-v3.txt', source, feedback,
    candidates:[{number:1,id:'B03',variant:'B03-v3',kind:'illustrated',title:'Vitalité améliorée',visual_status:'awaiting_visual_review',master:{file:masterFile,width:meta.width,height:meta.height,sha256:hash(path.join(out,masterFile))},files}],
    accepted_preserved:ledger.accepted_drawing_count,
    packaging:'Unmodified generated master; Lanczos3 reductions; review composition only.'
  });
  const specs = [
    {variant:'B03-v2', title:'Version précédente · armurée', dir:priorDir, file:s=>'B03-v2-'+s+'.png'},
    {variant:'B03-v3', title:'Nouvelle proposition · corps nu', dir:out, file:s=>'B03-v3-'+s+'.png'},
    {variant:'B03-C01', title:'Version compacte · validée', dir:priorDir, file:s=>'B03-C01-'+s+'.png'}
  ];
  const txt = (x,y,t,s=13,color='#e1e7ec') => `<text x="${x}" y="${y}" font-size="${s}" fill="${color}">${esc(t)}</text>`;
  const bits = [txt(24,34,'SKILLS-IC07 · 01 · Vitalité améliorée',25),txt(24,62,'Armure retirée · épaules et bras nus · vues 64 / 32 / 24 px à taille réelle',13,'#a6b6c5')];
  for (const [i,c] of specs.entries()) {
    const x=24+i*356,y=86;
    bits.push(`<rect x="${x}" y="${y}" width="340" height="376" rx="9" fill="#111923" stroke="#3a4654"/>`,txt(x+14,y+24,c.variant,12,'#b6a8ff'),txt(x+14,y+47,c.title,15));
    for (const [s,dx,dy] of [[192,74,61],[64,23,281],[32,151,297],[24,252,301]]) {
      bits.push(`<image x="${x+dx}" y="${y+dy}" width="${s}" height="${s}" href="data:image/png;base64,${fs.readFileSync(path.join(c.dir,c.file(s))).toString('base64')}"/>`,txt(x+dx,y+dy+s+16,s+' px',10,'#a6b6c5'));
    }
  }
  bits.push(txt(24,495,'Seule la nouvelle illustration B03-v3 reste à examiner sur cette planche.',12,'#a6b6c5'));
  const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="520"><rect width="100%" height="100%" fill="#0b1017"/><g font-family="Segoe UI,Arial,sans-serif">${bits.join('')}</g></svg>`;
  fs.writeFileSync(path.join(out,'comparison-board.svg'),svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out,'comparison-board.png'));
  const src=(c,s)=>rel(path.join(c.dir,c.file(s))).replace('artifacts/refonte-agent-2026-10-06/','../');
  const card=c=>`<article><h2><code>01 · ${c.variant}</code>${esc(c.title)}</h2><img class="large" src="${src(c,192)}" width="192" height="192" alt="${esc(c.title)}"><div class="native">${[64,32,24].map(s=>`<div><img src="${src(c,s)}" width="${s}" height="${s}" alt=""><span>${s} px</span></div>`).join('')}</div></article>`;
  fs.writeFileSync(path.join(out,'index.html'),`<!doctype html><html lang="fr"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>SKILLS-IC07 · Vitalité améliorée</title><style>*{box-sizing:border-box}body{margin:0;padding:24px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial}header,main,footer{max-width:1100px;margin:auto}header{margin-bottom:24px}small,code{color:#b6a8ff}h1{font-size:26px;margin:10px 0}p,footer{font-size:13px;line-height:1.5;color:#a6b6c5}.gallery{display:grid;grid-template-columns:repeat(auto-fit,minmax(265px,1fr));gap:18px}article{min-width:0;background:#111923;border:1px solid #3a4654;border-radius:10px;padding:17px}h2{font-size:15px;min-height:52px;line-height:1.4;margin:0 0 16px}code{display:block;font:12px Segoe UI;margin-bottom:5px}.large{display:block;margin:0 auto 22px;border-radius:7px}.native{display:flex;gap:27px;justify-content:center;align-items:end;min-height:88px}.native>div{display:grid;justify-items:center;gap:8px}.native span{color:#a6b6c5;font-size:11px}.native img{display:block}footer{margin:26px auto}a{color:#d7cef4;display:inline-block;padding:7px 10px;border:1px solid #3a4654;border-radius:5px;text-decoration:none}@media(max-width:420px){body{padding:14px}.gallery{grid-template-columns:minmax(0,1fr)}}</style></head><body><header><small>SKILLS-IC07 · REVUE VISUELLE</small><h1>01 · Vitalité améliorée</h1><p>La nouvelle illustration remplace l’armure par des épaules et des bras nus. La version compacte validée reste identique.</p></header><main class="gallery">${specs.map(card).join('')}</main><footer>Seule l’illustration B03-v3 reste à examiner sur cette planche.<br><a href="comparison-board.png">Ouvrir la planche des tailles réelles</a> <a href="../skill-icon-review-06/index.html">Revenir aux autres icônes</a></footer></body></html>`);
  console.log(JSON.stringify({review_id:'SKILLS-IC07',accepted_illustrated:ledger.accepted_count,accepted_compact:ledger.compact_accepted_count,new_candidate:'B03-v3',out}));
}
main().catch(e=>{console.error(e);process.exitCode=1});
