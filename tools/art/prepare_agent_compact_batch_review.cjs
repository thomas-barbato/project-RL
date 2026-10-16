// Review-only compact icon packager. It never changes approval ledgers or runtime assets.
const fs = require('node:fs'), path = require('node:path'), crypto = require('node:crypto');
const sharp = require(process.env.RL_SHARP_MODULE || 'sharp');
const root = path.resolve(__dirname, '../..');
const base = path.join(root, 'artifacts/refonte-agent-2026-10-06');
const read = p => JSON.parse(fs.readFileSync(p, 'utf8').replace(/^\uFEFF/, ''));
const hash = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const write = (p, v) => fs.writeFileSync(p, JSON.stringify(v, null, 2) + '\n');
const relative = p => path.relative(root, p).replace(/\\/g, '/');
const esc = s => String(s).replace(/[&<>\"]/g, c => ({'&':'&amp;', '<':'&lt;', '>':'&gt;', '\"':'&quot;'}[c]));
function filesUnder(dir) {
  if (!fs.existsSync(dir)) return [];
  return fs.readdirSync(dir, {withFileTypes: true}).flatMap(e => {
    const p = path.join(dir, e.name);
    return e.isDirectory() ? filesUnder(p) : [p];
  });
}
function wrap(text, limit = 29) {
  const lines = [''];
  for (const word of text.split(' ')) {
    const i = lines.length - 1;
    if (lines[i] && (lines[i] + ' ' + word).length > limit) lines.push(word);
    else lines[i] += (lines[i] ? ' ' : '') + word;
  }
  return lines;
}
async function main() {
  const inputPath = path.resolve(root, process.argv[2] || 'artifacts/refonte-agent-2026-10-06/skill-icon-review-14/sources.json');
  const out = path.dirname(inputPath);
  const within = path.relative(base, out);
  if (within.startsWith('..') || path.isAbsolute(within) || !within) throw Error('Review must be a subdirectory of the agent artifact root.');
  if (fs.readdirSync(out).some(f => f.startsWith('visual-acceptance') && f.endsWith('.json'))) {
    throw Error('Accepted review is frozen; create a new numbered review for any drawing revision.');
  }
  const input = read(inputPath);
  const numberRange = input.candidates[0].number + '–' + input.candidates.at(-1).number;
  const cataloguePath = path.join(base, 'skill-window-review-05/skills.json');
  const catalogue = read(cataloguePath).skills;
  const ledgerPath = path.join(base, 'skill-icon-acceptance.json');
  const ledger = read(ledgerPath);
  const accepted = [...ledger.accepted, ...ledger.compact_accepted].flatMap(c => Object.values(c.files));
  for (const f of accepted) if (hash(path.join(root, f.file)) !== f.sha256) throw Error('Accepted export changed: ' + f.file);
  const protectedFiles = [...filesUnder(path.join(root, 'assets/skills/agent')), ledgerPath, cataloguePath];
  const protectedHashes = protectedFiles.map(file => ({file: relative(file), sha256: hash(file)}));
  const ids = new Set(), numbers = new Set();
  if (!input.candidates.length || input.candidates.length > 8) throw Error('Expected one to eight compact candidates.');
  for (const c of input.candidates) {
    if (!/^[A-Z0-9]+-C\d{2}$/.test(c.variant) || !c.variant.startsWith(c.id + '-')) throw Error('Invalid variant ID.');
    if (ids.has(c.id) || numbers.has(c.number)) throw Error('Duplicate ID or review number.');
    ids.add(c.id); numbers.add(c.number);
    if (ledger.compact_accepted.some(a => a.id === c.id)) throw Error('Use a targeted revision for an already accepted compact: ' + c.id);
    if (!catalogue.some(s => s.id === c.id)) throw Error('Skill missing from reviewed catalogue: ' + c.id);
  }
  fs.mkdirSync(path.join(out, 'prompts'), {recursive: true});
  const candidates = [];
  for (const c of input.candidates) {
    const skill = catalogue.find(s => s.id === c.id);
    const source = path.resolve(root, c.source);
    const meta = await sharp(source).metadata();
    if (meta.width !== meta.height || meta.width < 192) throw Error('Expected square high-resolution source: ' + c.variant);
    const master = c.variant + '-master.png';
    fs.copyFileSync(source, path.join(out, master));
    const files = {};
    for (const size of [192, 128, 64, 32, 24]) {
      const file = c.variant + '-' + size + '.png';
      await sharp(source).resize(size, size, {kernel: 'lanczos3'}).png().toFile(path.join(out, file));
      const m = await sharp(path.join(out, file)).metadata();
      if (m.width !== size || m.height !== size) throw Error('Export size mismatch');
      files[size] = {file, sha256: hash(path.join(out, file)), width: size, height: size};
    }
    const prompt = 'prompts/' + c.variant + '.txt';
    fs.writeFileSync(path.join(out, prompt), c.prompt + '\n');
    candidates.push({id: c.id, number: c.number, variant: c.variant, title: skill.title, branch: skill.branch,
      kind: skill.kind, roleplay: skill.roleplay, visual_status: 'awaiting_visual_review',
      generation_tool: 'image_gen.imagegen', generation_mode: 'built-in', prompt,
      source: c.source, source_sha256: hash(source),
      master: {file: master, sha256: hash(path.join(out, master)), width: meta.width, height: meta.height}, files,
      reference_hashes: c.refs.map(file => ({file, sha256: hash(file)}))});
  }
  const styleReferences = input.style_references || ['B03', 'FE01', 'M03'].map(id => ({
    title: id, file: relative(path.join(base, 'skill-icon-review-11', id + '-C01-32.png'))
  }));
  const refs = styleReferences.map(ref => '<div><img src="' + esc(path.relative(out, path.resolve(root, ref.file)).replace(/\\/g, '/')) + '" width="32" height="32" alt="Référence ' + esc(ref.title) + '"><span>' + esc(ref.title) + '</span></div>').join('');
  const cards = candidates.map(c => '<article id="' + c.variant + '"><small>' + c.number + ' · ' + c.variant + '</small><h2>' + esc(c.title) + '</h2><div class="zoom"><img src="' + c.files[128].file + '" width="128" height="128" alt="' + esc(c.title) + '"></div><div class="native">' + [32, 64, 24].map(size => '<div><img src="' + c.files[size].file + '" width="' + size + '" height="' + size + '" alt="' + esc(c.title) + ' ' + size + ' px"><span>' + size + ' px</span></div>').join('') + '</div></article>').join('');
  fs.writeFileSync(path.join(out, 'index.html'), '<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>' + input.review_id + ' · ' + esc(input.title) + '</title><style>*{box-sizing:border-box}body{margin:0;padding:24px;background:#0b1017;color:#e1e7ec;font:15px Segoe UI,Arial}header,main,footer{max-width:1040px;margin:auto}h1{font-size:25px;margin:12px 0}p,small{color:#a6b6c5;line-height:1.6}a{color:#d7cef4}.refs{display:flex;align-items:center;gap:20px;margin:16px 0 24px;color:#a6b6c5;font-size:12px}.refs>div{display:flex;align-items:center;gap:8px}.gallery{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:16px}article{background:#111923;border:1px solid #3a4654;border-radius:10px;padding:16px;min-width:0}article small{color:#b6a8ff}h2{font-size:16px;min-height:44px;line-height:1.35;margin:9px 0}.zoom{display:flex;justify-content:center;margin:16px 0 20px}.native{display:flex;gap:22px;justify-content:center;align-items:end}.native>div{display:grid;gap:9px;justify-items:center;color:#a6b6c5;font-size:12px}img{display:block;flex-shrink:0}footer{margin:24px auto;color:#a6b6c5;font-size:12px;line-height:1.7}@media(max-width:920px){.gallery{grid-template-columns:repeat(2,minmax(0,1fr))}}@media(max-width:480px){.gallery{grid-template-columns:1fr}.refs{flex-wrap:wrap}body{padding:16px}}</style><header><small>' + input.review_id + ' · PROPOSITIONS 21–28</small><h1>' + esc(input.title) + '</h1><p>Quatre modèles de drone, Maintenance, Réparation ciblée, Blindage amélioré et Autodestruction.</p><div class="refs"><span>Style des compactes validées · 32 px</span>' + refs + '</div></header><main class="gallery">' + cards + '</main><footer>Vues 32 / 64 / 24 px à taille réelle ; vue supérieure à 128 px. Dessins proposés à la revue.<br><a href="../skill-icon-review-13/index.html">Artilleur et Furtivité validés</a> · <a href="prompts.txt">Prompts de génération</a></footer></html>');
  const pagePath = path.join(out, 'index.html');
  let page = fs.readFileSync(pagePath, 'utf8').replace('PROPOSITIONS 21–28', 'PROPOSITIONS ' + numberRange);
  if (input.description) page = page.replace('Quatre modèles de drone, Maintenance, Réparation ciblée, Blindage amélioré et Autodestruction.', esc(input.description));
  if (input.previous_review) page = page.replace('../skill-icon-review-13/index.html', esc(input.previous_review.file)).replace('Artilleur et Furtivité validés', esc(input.previous_review.title));
  if (candidates.length >= 5 && candidates.length <= 6) {
    page = page.replace('repeat(4,minmax(0,1fr))', 'repeat(3,minmax(0,1fr))');
    page += '<style>body{padding:20px}h1{margin:8px 0}header p{margin:8px 0}.refs{margin:10px 0 16px}article{padding:12px}h2{min-height:22px}.zoom{margin:12px 0}.native>div{gap:8px}footer{margin:16px auto}</style>';
  }
  fs.writeFileSync(pagePath, page);
  const width = 1040, height = 860;
  const pieces = ['<rect width="' + width + '" height="' + height + '" fill="#0b1017"/>', '<text x="24" y="34" font-size="23" fill="#e1e7ec">' + input.review_id + ' · ' + esc(input.title) + '</text>', '<text x="24" y="60" font-size="13" fill="#a6b6c5">21–28 · Propositions compactes · vues natives 32 / 64 / 24 px</text>'];
  const img = (file, x, y, size) => '<image x="' + x + '" y="' + y + '" width="' + size + '" height="' + size + '" href="data:image/png;base64,' + fs.readFileSync(file).toString('base64') + '"/>';
  pieces.push('<text x="24" y="100" font-size="12" fill="#a6b6c5">Références validées · 32 px</text>');
  for (const [i, ref] of styleReferences.entries()) pieces.push(img(path.resolve(root, ref.file), 214 + i * 56, 78, 32));
  for (const [i, c] of candidates.entries()) {
    const x = 24 + (i % 4) * 252, y = 134 + Math.floor(i / 4) * 348;
    pieces.push('<rect x="' + x + '" y="' + y + '" width="236" height="330" rx="9" fill="#111923" stroke="#3a4654"/>', '<text x="' + (x + 16) + '" y="' + (y + 24) + '" font-size="13" fill="#b6a8ff">' + c.number + ' · ' + c.variant + '</text>');
    for (const [line, title] of wrap(c.title).entries()) pieces.push('<text x="' + (x + 16) + '" y="' + (y + 49 + line * 20) + '" font-size="15" fill="#e1e7ec">' + esc(title) + '</text>');
    pieces.push(img(path.join(out, c.files[128].file), x + 54, y + 88, 128));
    for (const [size, dx] of [[32, 32], [64, 96], [24, 192]]) {
      pieces.push(img(path.join(out, c.files[size].file), x + dx, y + 232 + (64 - size), size), '<text x="' + (x + dx + size / 2) + '" y="' + (y + 314) + '" text-anchor="middle" font-size="11" fill="#a6b6c5">' + size + ' px</text>');
    }
  }
  pieces.push('<text x="24" y="842" font-size="12" fill="#a6b6c5">La taille des aperçus natifs est conservée dans ce fichier. Chaque dessin garde son identifiant de revue.</text>');
  const svg = '<svg xmlns="http://www.w3.org/2000/svg" width="' + width + '" height="' + height + '"><g font-family="Segoe UI,Arial">' + pieces.join('').replace('21–28 · Propositions compactes', numberRange + ' · Propositions compactes') + '</g></svg>';
  fs.writeFileSync(path.join(out, 'native-review-board.svg'), svg);
  await sharp(Buffer.from(svg)).png().toFile(path.join(out, 'native-review-board.png'));
  for (const p of protectedHashes) if (hash(path.join(root, p.file)) !== p.sha256) throw Error('Protected file changed: ' + p.file);
  write(path.join(out, 'review.json'), {...input, candidates, protected_hashes: protectedHashes,
    accepted_export_hashes_preserved: accepted.length, no_new_approval_inferred: true, no_runtime_changes: true});
  write(path.join(out, 'validation.json'), {review_id: input.review_id, export_dimensions_passed: true,
    accepted_exports_verified: accepted.length, runtime_ledger_and_catalogue_files_preserved: protectedHashes.length,
    candidate_count: candidates.length, native_review_board: {width, height}, status: 'awaiting_visual_review'});
  fs.writeFileSync(path.join(out, 'prompts.txt'), input.candidates.map(c => '=== ' + c.number + ' · ' + c.variant + ' ===\n' + c.prompt + '\n').join('\n'));
  console.log(JSON.stringify({review_id: input.review_id, candidate_count: candidates.length,
    accepted_exports_preserved: accepted.length, protected_files_preserved: protectedHashes.length, directory: relative(out)}));
}
main().catch(e => {console.error(e); process.exitCode = 1;});
