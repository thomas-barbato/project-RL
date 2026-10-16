// SKILLS-IC13: non-destructive review of compact candidates 16 and 20.
const fs = require('node:fs'), path = require('node:path'), crypto = require('node:crypto');
const sharp = require(process.env.RL_SHARP_MODULE || 'sharp');
const root = path.resolve(__dirname, '../..');
const base = path.join(root, 'artifacts/refonte-agent-2026-10-06');
const out = path.join(base, 'skill-icon-review-13');
const read = p => JSON.parse(fs.readFileSync(p, 'utf8').replace(/^\uFEFF/, ''));
const hash = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const write = (p, v) => fs.writeFileSync(p, JSON.stringify(v, null, 2) + '\n');
const esc = s => String(s).replace(/[&<>\"]/g, c => ({'&':'&amp;', '<':'&lt;', '>':'&gt;', '\"':'&quot;'}[c]));
function filesUnder(dir) {
  if (!fs.existsSync(dir)) return [];
  return fs.readdirSync(dir, {withFileTypes: true}).flatMap(e => {
    const p = path.join(dir, e.name);
    return e.isDirectory() ? filesUnder(p) : [p];
  });
}
async function main() {
  if (fs.existsSync(path.join(out, 'visual-acceptance-01.json'))) {
    throw Error('IC13 is accepted and frozen; create a new review for any drawing revision.');
  }
  const input = read(path.join(out, 'sources.json'));
  const protectedPaths = [
    ...filesUnder(path.join(base, 'skill-icon-review-12')),
    ...filesUnder(path.join(root, 'assets/skills/agent')),
    path.join(base, 'skill-icon-acceptance.json'),
  ];
  const protectedHashes = protectedPaths.map(file => ({file, sha256: hash(file)}));
  const ledger = read(path.join(base, 'skill-icon-acceptance.json'));
  const accepted = [...ledger.accepted, ...ledger.compact_accepted].flatMap(c => Object.values(c.files));
  for (const f of accepted) {
    if (hash(path.join(root, f.file)) !== f.sha256) throw Error('Accepted source changed: ' + f.file);
  }
  fs.mkdirSync(path.join(out, 'prompts'), {recursive: true});
  const candidates = [];
  for (const c of input.candidates) {
    const meta = await sharp(c.source).metadata();
    if (meta.width !== meta.height) throw Error('Expected square image: ' + c.variant);
    const master = c.variant + '-master.png';
    fs.copyFileSync(c.source, path.join(out, master));
    const files = {};
    for (const size of [192, 128, 64, 32, 24]) {
      const file = c.variant + '-' + size + '.png';
      await sharp(c.source).resize(size, size, {kernel: 'lanczos3'}).png().toFile(path.join(out, file));
      const m = await sharp(path.join(out, file)).metadata();
      if (m.width !== size || m.height !== size) throw Error('Export size mismatch');
      files[size] = {file, sha256: hash(path.join(out, file)), width: size, height: size};
    }
    const prompt = 'prompts/' + c.variant + '.txt';
    fs.writeFileSync(path.join(out, prompt), c.prompt + '\n');
    candidates.push({...c, prompt, visual_status: 'awaiting_visual_review', generation_tool: 'image_gen.imagegen', generation_mode: 'built-in',
      source_sha256: hash(c.source), master: {file: master, sha256: hash(path.join(out, master)), width: meta.width, height: meta.height}, files,
      style_reference_hashes: c.refs.map(file => ({file, sha256: hash(file)}))});
  }
  const refs = ['B03', 'FE01', 'M03'].map(id => '<div><img src="../skill-icon-review-11/' + id + '-C01-32.png" width="32" height="32" alt=""><span>' + id + '</span></div>').join('');
  const cards = candidates.map(c => '<article id="' + c.variant + '"><small>' + c.number + ' · ' + c.variant + ' · à examiner</small><h2>' + esc(c.title) + '</h2><div class="zoom"><img src="' + c.files[128].file + '" width="128" height="128" alt="' + esc(c.title) + '"></div><div class="native">' + [32, 64, 24].map(size => '<div><img src="' + c.files[size].file + '" width="' + size + '" height="' + size + '" alt="' + esc(c.title) + ' ' + size + ' px"><span>' + size + ' px</span></div>').join('') + '</div><div class="previous"><span>Version précédente</span><img src="../skill-icon-review-12/' + c.previous_variant + '-64.png" width="64" height="64" alt="Version précédente"><img src="../skill-icon-review-12/' + c.previous_variant + '-32.png" width="32" height="32" alt="Version précédente 32 px"></div></article>').join('');
  fs.writeFileSync(path.join(out, 'index.html'), '<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>SKILLS-IC13 · Artilleur et Furtivité</title><style>*{box-sizing:border-box}body{margin:0;padding:24px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial}header,main,footer{max-width:880px;margin:auto}h1{font-size:25px}p,small{color:#a6b6c5;line-height:1.6}a{color:#d7cef4}.refs{display:flex;align-items:center;gap:20px;margin:20px 0;color:#a6b6c5;font-size:12px}.refs>div{display:flex;align-items:center;gap:8px}.gallery{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:20px}article{background:#111923;border:1px solid #3a4654;border-radius:10px;padding:20px}article small{color:#b6a8ff}h2{font-size:17px}.zoom{display:flex;justify-content:center;margin:24px 0}.native{display:flex;gap:28px;justify-content:center;align-items:end}.native>div{display:grid;gap:10px;justify-items:center;color:#a6b6c5;font-size:12px}img{display:block;flex-shrink:0}.previous{display:flex;align-items:center;justify-content:center;gap:16px;font-size:12px;color:#a6b6c5;margin-top:24px;padding-top:18px;border-top:1px solid #34414e}footer{margin:24px auto;color:#a6b6c5;font-size:12px;line-height:1.7}@media(max-width:680px){.gallery{grid-template-columns:1fr}.refs{flex-wrap:wrap}.previous{gap:10px}body{padding:16px}}</style><header><small>SKILLS-IC13 · DEUX NOUVELLES PROPOSITIONS</small><h1>Artilleur et Furtivité</h1><p>16 : un drone flottant avec un canon bien visible. 20 : un agent qui disparaît par camouflage optique.</p><div class="refs"><span>Style des compactes validées · 32 px</span>' + refs + '</div></header><main class="gallery">' + cards + '</main><footer>Vues 32 / 64 / 24 px à taille réelle ; vue supérieure à 128 px. Propositions à valider avant intégration.<br><a href="../skill-icon-review-12/index.html">Revenir aux huit compactes de la revue précédente</a></footer></html>');
  const pieces = ['<rect width="900" height="514" fill="#0b1017"/>', '<text x="24" y="34" font-size="23" fill="#e1e7ec">SKILLS-IC13 · Artilleur et Furtivité</text>', '<text x="24" y="61" font-size="13" fill="#a6b6c5">Deux propositions à examiner · tailles réelles 32 / 64 / 24 px</text>'];
  const img = (file, x, y, size) => '<image x="' + x + '" y="' + y + '" width="' + size + '" height="' + size + '" href="data:image/png;base64,' + fs.readFileSync(file).toString('base64') + '"/>';
  for (const [i, c] of candidates.entries()) {
    const x = 24 + i * 436, y = 88;
    pieces.push('<rect x="' + x + '" y="' + y + '" width="412" height="398" rx="9" fill="#111923" stroke="#3a4654"/>', '<text x="' + (x + 18) + '" y="' + (y + 25) + '" font-size="13" fill="#b6a8ff">' + c.number + ' · ' + c.variant + '</text>', '<text x="' + (x + 18) + '" y="' + (y + 48) + '" font-size="16" fill="#e1e7ec">' + esc(c.title) + '</text>');
    pieces.push(img(path.join(out, c.files[128].file), x + 142, y + 66, 128));
    for (const [size, dx, dy] of [[32, 94, 218], [64, 174, 202], [24, 290, 222]]) {
      pieces.push(img(path.join(out, c.files[size].file), x + dx, y + dy, size), '<text x="' + (x + dx) + '" y="' + (y + 286) + '" font-size="11" fill="#a6b6c5">' + size + ' px</text>');
    }
    pieces.push('<line x1="' + (x + 18) + '" y1="' + (y + 306) + '" x2="' + (x + 394) + '" y2="' + (y + 306) + '" stroke="#34414e"/>', '<text x="' + (x + 38) + '" y="' + (y + 354) + '" font-size="12" fill="#a6b6c5">Version précédente</text>', img(path.join(base, 'skill-icon-review-12', c.previous_variant + '-64.png'), x + 214, y + 320, 64), img(path.join(base, 'skill-icon-review-12', c.previous_variant + '-32.png'), x + 306, y + 336, 32));
  }
  const svg = '<svg xmlns="http://www.w3.org/2000/svg" width="900" height="514"><g font-family="Segoe UI,Arial">' + pieces.join('') + '</g></svg>';
  fs.writeFileSync(path.join(out, 'comparison-board.svg'), svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out, 'comparison-board.png'));
  for (const p of protectedHashes) if (hash(p.file) !== p.sha256) throw Error('Protected file changed: ' + p.file);
  write(path.join(out, 'review.json'), {...input, candidates, accepted_export_hashes_preserved: accepted.length, protected_files_preserved: protectedHashes.length, no_new_approval_inferred: true, no_runtime_changes: true});
  write(path.join(out, 'validation.json'), {review_id: input.review_id, export_dimensions_passed: true, accepted_exports_verified: accepted.length, previous_review_runtime_and_ledger_files_preserved: protectedHashes.length, candidate_count: candidates.length, status: 'awaiting_visual_review'});
  fs.writeFileSync(path.join(out, 'prompts.txt'), input.candidates.map(c => '=== ' + c.variant + ' ===\n' + c.prompt + '\n').join('\n'));
  console.log(JSON.stringify({review_id: input.review_id, candidate_count: candidates.length, accepted_exports_preserved: accepted.length, protected_files_preserved: protectedHashes.length}));
}
main().catch(e => {console.error(e); process.exitCode = 1;});
