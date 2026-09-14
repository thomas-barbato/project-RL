// Read-only checks of design documents, NOT tests of the Rust game engine.
// Run from any directory: node <project>/tools/validate_stats_docs.mjs
import assert from 'node:assert/strict';
import { readFileSync, existsSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { inspectProgression } from './stats_progression_checks.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const read = (name) => readFileSync(resolve(root, name), 'utf8').replaceAll('\r\n', '\n');
const catalogue = read('docs/PROPOSITION_COMPETENCES_v0.1.md');
const rules = read('docs/STATISTIQUES_ET_COMPETENCES.md');
const report = read('docs/RAPPORT_SYSTEME_STATISTIQUES_COMPETENCES.md');
const playerTexts = read('docs/TEXTES_JOUEUR_STATISTIQUES_COMPETENCES.md');
const groups = [];
function check(name, test) { test(); groups.push(name); console.log(`PASS ${name}`); }
const cells = (line) => line.trim().slice(1, -1).split('|').map((s) => s.trim());
const idPattern = /^(?:MEL|TIR|DEM|MAN|FUR|REC|ING|INT|DRN|GEL)-(?:V)?\d{2}$/;
const clamp = (x, lo, hi) => Math.min(hi, Math.max(lo, x));
const num = (s) => Number(s.replaceAll('−', '-').replaceAll('%', '').trim());
function tables(doc) {
  const result = []; let current;
  for (const line of doc.split('\n')) {
    if (line.startsWith('|')) {
      if (!current) { current = []; result.push(current); }
      current.push(cells(line));
    } else { current = undefined; }
  }
  return result;
}
const rt = tables(rules);
const table = (firstHeader) => {
  const matches = rt.filter((t) => t[0][0] === firstHeader);
  assert.equal(matches.length, 1, `Unique table: ${firstHeader}`);
  return matches[0].slice(2);
};
const preAnnex = catalogue.split('## 16. Annexe technique')[0];
const annex = catalogue.split('## 16. Annexe technique')[1];
assert.ok(annex, 'Technical annex exists');
const entries = new Map();
for (const line of preAnnex.split('\n').filter((l) => l.startsWith('|'))) {
  const c = cells(line);
  if (idPattern.test(c[0]) && /^[1-5]$/.test(c[1])) {
    assert.ok(!entries.has(c[0]), `Duplicate source ${c[0]}`);
    entries.set(c[0], { id: c[0], minimumLevel: Number(c[1]), row: c });
  }
}
const parents = new Map([
  ['MEL-06', 'MEL-04'], ['TIR-07', 'TIR-01'], ['TIR-08', 'TIR-04'],
  ['DEM-07', 'DEM-02'], ['MAN-08', 'MAN-04'],
  ['FUR-10', 'FUR-05'], ['REC-09', 'REC-01'], ['ING-05', 'ING-01'],
  ['ING-10', 'ING-06'], ['DRN-07', 'DRN-02'],
]);
for (const [id, entry] of entries) {
  if (id.startsWith('GEL-V')) parents.set(id, entry.row[3]);
}
const choiceCostTotals = [0, 1, 2, 4, 6, 9];

check('Markdown, liens locaux et suivi des textes', () => {
  for (const [name, doc] of [
    ['PROPOSITION_COMPETENCES_v0.1.md', catalogue],
    ['STATISTIQUES_ET_COMPETENCES.md', rules],
    ['RAPPORT_SYSTEME_STATISTIQUES_COMPETENCES.md', report],
    ['TEXTES_JOUEUR_STATISTIQUES_COMPETENCES.md', playerTexts],
  ]) {
    assert.equal(doc.split('\n').filter((l) => l.startsWith('```')).length % 2, 0, name);
    for (const t of tables(doc)) {
      for (const row of t) assert.equal(row.length, t[0].length, `${name}: ${row[0]} table width`);
    }
    assert.ok(!doc.split('\n').some((l) => /[ \t]+$/.test(l)), `${name}: trailing whitespace`);
    for (const match of doc.matchAll(/\[[^\]]+\]\(([^)]+)\)/g)) {
      const target = match[1];
      if (/^https?:|^#/.test(target)) continue;
      const file = /^[A-Za-z]:\//.test(target) ? target : resolve(root, 'docs', target.split('#')[0]);
      assert.ok(existsSync(file), `${name}: missing ${target}`);
    }
    assert.match(doc, /[Éé]tape 7/);
  }
});

check('Couverture exacte : 94 techniques et 10 variantes', () => {
  assert.equal(entries.size, 104);
  assert.equal([...entries.keys()].filter((id) => id.startsWith('GEL-V')).length, 10);
  const rows = annex.split('\n').filter((l) => l.startsWith('|')).map(cells)
    .filter((c) => idPattern.test(c[0]));
  assert.equal(rows.length, 104);
  assert.equal(new Set(rows.map((r) => r[0])).size, 104);
  assert.deepEqual(rows.map((r) => r[0]).sort(), [...entries.keys()].sort());
  for (const r of rows) {
    assert.equal(r.length, 6, r[0]);
    assert.ok(r.every((c) => c.length > 0), `Empty profile field ${r[0]}`);
  }
  for (const id of ['REC-06', 'REC-07', 'REC-10', 'MAN-07']) assert.ok(!entries.has(id));
  assert.match(preAnnex, /\| MAN-07 \| Fusionnée, identifiant réservé \|/);
  assert.equal([...entries.keys()].filter((id) => id.startsWith('MAN-')).length, 9);
});

const textRows = tables(playerTexts).flatMap((t) => t.slice(2));
const techniqueTexts = textRows.filter((r) => idPattern.test(r[0]));
const interfaceTexts = textRows.filter((r) => /^(UI|MSG)-/.test(r[0]));
check('Textes joueur : 104 descriptions, noms et prérequis conservés', () => {
  assert.equal(techniqueTexts.length, entries.size);
  assert.equal(new Set(techniqueTexts.map((r) => r[0])).size, entries.size);
  assert.deepEqual(techniqueTexts.map((r) => r[0]).sort(), [...entries.keys()].sort());
  for (const row of techniqueTexts) {
    assert.equal(row.length, 4, row[0]);
    assert.equal(row[1], entries.get(row[0]).row[2], `${row[0]}: unchanged name`);
    assert.ok(row.every((c) => c.length > 0), `${row[0]}: complete description and limit`);
    const parent = parents.get(row[0]);
    if (parent) assert.ok(row[3].includes(entries.get(parent).row[2]), `${row[0]}: named prerequisite`);
  }
  const disciplines = interfaceTexts.filter((r) => r[0].startsWith('UI-DISC-'));
  assert.deepEqual(disciplines.map((r) => r[0].slice(8)).sort(),
    ['MEL', 'TIR', 'DEM', 'MAN', 'FUR', 'REC', 'ING', 'INT', 'GEL', 'DRN'].sort());
  // A summary cannot certify semantics. It must at least preserve all named options.
});

check('Textes joueur : clés uniques, modèles et quinze accords distincts', () => {
  assert.equal(textRows.length, techniqueTexts.length + interfaceTexts.length, 'No uncategorized text row');
  assert.equal(new Set(interfaceTexts.map((r) => r[0])).size, interfaceTexts.length);
  const approved = [...rules.matchAll(/^#### Texte (\d+) — ([^\n]+)\n+Statut[^\n]+\n+((?:>[^\n]*\n)+)/gm)];
  assert.deepEqual(approved.map((m) => Number(m[1])), Array.from({ length: 15 }, (_, i) => i + 1));
  assert.match(playerTexts, /rédigé sous cette délégation/);
  const allowedParameters = new Set([
    'budget', 'minimum', 'maximum', 'plafond', 'technique', 'rang', 'discipline', 'cout', 'restants',
    'preparation', 'execution', 'recuperation', 'energie', 'moment', 'chaleur', 'bande', 'reservation',
    'quantite', 'objet', 'geometrie', 'portee', 'duree', 'delai', 'chance', 'source', 'date', 'attitude',
    'valeur_signee', 'rayon', 'disponibles', 'projection', 'limite', 'materiel', 'fonction', 'depenses',
    'cause', 'cible', 'type', 'droits', 'programme', 'lot', 'element', 'type_trace', 'direction',
    'anciennete', 'etat', 'processus', 'drone', 'niveau', 'attribut', 'valeur', 'recompenses', 'effet',
  ]);
  for (const row of interfaceTexts) {
    assert.equal(row.length, 3, row[0]);
    assert.match(row[0], /^(UI|MSG)-[A-Z0-9-]+$/);
    assert.ok(row.every((c) => c.length > 0), `${row[0]}: complete text`);
    for (const match of row[2].matchAll(/\{([a-z_]+)\}/g)) {
      assert.ok(allowedParameters.has(match[1]), `${row[0]}: unknown placeholder ${match[1]}`);
    }
    assert.doesNotMatch(row[2].replaceAll(/\{[a-z_]+\}/g, ''), /[{}]/, `${row[0]}: malformed placeholder`);
  }
  for (const row of techniqueTexts) {
    assert.doesNotMatch(row.slice(1).join(' '), /\b(?:MEL|TIR|DEM|MAN|FUR|REC|ING|INT|DRN|GEL)-(?:V)?\d{2}\b/,
      `${row[0]}: design identifier exposed in player copy`);
  }
  for (const key of ['UI-DEFENSE-NUMERIQUE', 'UI-DURABILITE', 'UI-ENERGIE', 'UI-CHALEUR',
    'UI-BANDE-PASSANTE', 'UI-CONTACT-INCERTAIN', 'UI-CONTACT-SOUVENIR', 'UI-RAPPORT-DRONE',
    'MSG-REFUS-CONTACT', 'MSG-REFUS-LIAISON', 'MSG-RESULTAT-ECHEC', 'MSG-RESULTAT-INSPECTION',
    'MSG-RESULTAT-DRONE-PERDU', 'UI-RETARDATEUR', 'UI-DISCIPLINE-DIFFEREE', 'MSG-RESULTAT-FUSIBLE']) {
    assert.ok(interfaceTexts.some((r) => r[0] === key), `Missing essential copy: ${key}`);
  }
});

check('Prérequis, niveaux et cinq premiers apprentissages atteignables', () => {
  for (const [child, parent] of parents) {
    assert.ok(entries.has(child), `${child}: missing child`);
    assert.ok(entries.has(parent), `${child}: missing parent`);
    assert.ok(entries.get(parent).minimumLevel < entries.get(child).minimumLevel,
      `${child}: parent chronology`);
    assert.ok(entries.get(child).row.join(' ').includes(parent), `${child}: documented parent`);
    const seen = new Set([child]); let cur = parent;
    while (cur) { assert.ok(!seen.has(cur)); seen.add(cur); cur = parents.get(cur); }
  }
  for (const discipline of ['MEL', 'TIR', 'DEM', 'MAN', 'FUR', 'REC', 'ING', 'INT', 'GEL', 'DRN']) {
    const pool = [...entries.values()].filter((e) => e.id.startsWith(`${discipline}-`));
    const result = inspectProgression(pool, parents);
    assert.ok(result.validInitial && result.allPathsComplete, `${discipline}: a legal path gets stuck`);
    assert.ok(result.countsByChoiceCount.slice(1, 6).every((count) => count > 0));
    assert.equal(result.reachedIds.length, pool.length, `${discipline}: unreachable entry`);
    if (discipline === 'REC') {
      assert.ok(inspectProgression(pool, parents, new Set(['REC-08'])).allPathsComplete, 'REC without diagnosis');
    }
  }
});

check('Versions partielles, prérequis transitifs et acquis incompatibles', () => {
  assert.match(rules, /différer l'ouverture d'une discipline/);
  const recon = [...entries.values()].filter((e) => e.id.startsWith('REC-'));
  const inspect = (ids, initial = []) => inspectProgression(recon, parents, new Set(ids), initial);
  assert.ok(inspect(['REC-08']).allPathsComplete);
  const incomplete = inspect(['REC-02', 'REC-03', 'REC-08']);
  assert.equal(incomplete.availableIds.length, 4);
  assert.equal(incomplete.allPathsComplete, false);
  assert.equal(incomplete.countsByChoiceCount[5], 0);
  assert.ok(inspect(['REC-01']).unavailableIds.includes('REC-09'));
  assert.ok(inspect([], ['REC-01', 'REC-02']).allPathsComplete);
  assert.equal(inspect(['REC-02'], ['REC-02']).validInitial, false);
  assert.equal(inspect([], ['REC-01', 'REC-01']).validInitial, false);
  assert.equal(inspect([], ['REC-09']).validInitial, false);
  assert.equal(inspect([], ['REC-02', 'REC-03', 'REC-09']).validInitial, false);
  const maneuver = [...entries.values()].filter((e) => e.id.startsWith('MAN-'));
  assert.equal(inspectProgression(maneuver, parents, new Set(), ['MAN-07']).validInitial, false);
  // Minimum level is independent from the number of earlier choices.
  const fiveAtHighLevel = [1, 1, 5, 5, 5]
    .map((minimumLevel, index) => ({ id: `fixture-${index}`, minimumLevel }));
  assert.equal(inspectProgression(fiveAtHighLevel, new Map()).allPathsComplete, true);
  // Closure must continue beyond a direct child, regardless of catalogue order.
  const chain = [
    { id: 'c', minimumLevel: 3 },
    { id: 'b', minimumLevel: 2 },
    { id: 'a', minimumLevel: 1 },
  ];
  assert.deepEqual(inspectProgression(chain, new Map([['c', 'b'], ['b', 'a']]), new Set(['a']))
    .unavailableIds, ['a', 'b', 'c']);
  // Independent ordered recursion checks all 128 hypothetical REC feature masks.
  // It avoids both the helper's transitive filtering and its learned-set deduplication.
  function everyPathFinishes(disabled, chosen = []) {
    if (chosen.length === 5) return true;
    const options = recon.filter((e) => !disabled.has(e.id) && !chosen.includes(e.id)
      && (!parents.has(e.id) || chosen.includes(parents.get(e.id))));
    return options.length > 0 && options.every((e) => everyPathFinishes(disabled, [...chosen, e.id]));
  }
  for (let mask = 0; mask < 2 ** recon.length; mask += 1) {
    const disabled = new Set(recon.filter((_, i) => mask & (1 << i)).map((e) => e.id));
    assert.equal(inspectProgression(recon, parents, disabled).allPathsComplete,
      everyPathFinishes(disabled), `REC feature mask ${mask}`);
  }
});

let numericalRows = 0;
check('49 exemples tabulaires recalculés depuis les règles', () => {
  for (const r of table('Situation')) {
    const [p, e, c, chance] = r.slice(1).map(num);
    assert.equal(clamp(p - e + c, 5, 95), chance); numericalRows += 1;
  }
  for (const r of table('Dégâts bruts D')) {
    const [d, b, f, p, effective, damage] = r.map(num);
    assert.equal(Math.max(0, b - f - p), effective);
    assert.equal(Math.max(0, d - effective), damage); numericalRows += 1;
  }
  for (const r of table('Puissance')) {
    const [p, cap, available, used, bonus, d, b, damage] = r.map(num);
    assert.equal(2 * p, available); assert.equal(Math.min(available, cap), used);
    assert.equal(used - 10, bonus); assert.equal(12 + bonus, d);
    assert.equal(Math.max(0, d - b), damage); numericalRows += 1;
  }
  for (const r of table('Base du corps')) {
    const [base, resilience, material, state, maximum] = r.map(num);
    assert.equal(Math.max(1, base + 5 * (resilience - 5) + material + state), maximum);
    numericalRows += 1;
  }
  for (const r of table('PV avant')) {
    const before = Number(r[0].split('/')[0]);
    assert.equal(Math.min(before, num(r[1])), Number(r[2].split('/')[0])); numericalRows += 1;
  }
  for (const r of table('Dégâts bruts')) {
    const [d, resistance, damage] = r.map(num);
    assert.equal(Math.floor(d * (100 - clamp(resistance, -50, 75)) / 100), damage);
    numericalRows += 1;
  }
  for (const r of table('Physique brut')) {
    const [p, e, b, resistance, finalP, finalE, total] = r.map(num);
    assert.equal(Math.max(0, p - b), finalP);
    assert.equal(Math.floor(e * (100 - resistance) / 100), finalE);
    assert.equal(finalP + finalE, total); numericalRows += 1;
  }
  for (const r of table('Résilience')) {
    const [resilience, bonus, stability, intensity, chance] = r.map(num);
    assert.equal(50 + 5 * (resilience - 5) + bonus, stability);
    assert.equal(clamp(50 + stability - intensity, 5, 95), chance); numericalRows += 1;
  }
  for (const r of table('Traitement attaquant')) {
    const [at, bonus, dt, firewall, attack, defense, chance] = r.map(num);
    assert.equal(Math.max(0, 65 + 5 * (at - 5) + bonus), attack);
    assert.equal(Math.max(0, 50 + 4 * (dt - 5) + firewall), defense);
    assert.equal(clamp(50 + attack - defense, 5, 95), chance); numericalRows += 1;
  }
  for (const r of table('Niveau')) {
    const [level, xp, budget, attributes] = r.map(num);
    assert.equal(100 * (level - 1) + 20 * (level - 1) * (level - 2), xp);
    assert.equal(level + 1, budget); assert.equal(Math.floor(level / 4), attributes);
    numericalRows += 1;
  }
  assert.equal(numericalRows, 49);
});

check('Bornes, monotonie et absence de soin par rééquipement', () => {
  for (let raw = 0; raw <= 100; raw += 1) {
    let previous = Infinity;
    for (let b = 0; b <= 100; b += 1) {
      const d = Math.max(0, raw - b);
      assert.ok(d <= previous && d >= 0 && d <= raw); previous = d;
    }
    previous = Infinity;
    for (let r = -100; r <= 100; r += 1) {
      const d = Math.floor(raw * (100 - clamp(r, -50, 75)) / 100);
      assert.ok(d <= previous && d >= 0); previous = d;
    }
  }
  for (let current = 0; current <= 125; current += 1) {
    const reduced = Math.min(current, 100);
    const equipped = Math.min(reduced, 125);
    assert.ok(equipped <= current);
  }
  for (let t = 1; t <= 10; t += 1) {
    for (let def = 0; def <= 200; def += 1) {
      const chance = clamp(50 + Math.max(0, 65 + 5 * (t - 5)) - def, 5, 95);
      assert.ok(chance >= 5 && chance <= 95);
    }
  }
});

check('Perception, charge et déplacement : exemples isolés', () => {
  const detection = (p, distance) => 50 + 4 * (p - 5) - 2 * Math.max(0, distance - 2);
  assert.equal(detection(5, 2), 50); assert.equal(detection(8, 4), 58);
  const visible = (lineOfSight, p, distance, difficulty) => lineOfSight && detection(p, distance) >= difficulty;
  assert.equal(visible(false, 10, 1, 0), false); // Guard clause in model, not a game FOV test.
  const capacity = (p) => Math.max(1, Math.min(60, 40 + 2 * (p - 5)));
  assert.deepEqual([3, 5, 8].map(capacity), [36, 40, 46]);
  const burden = (mass, cap) => mass <= cap ? 0 : mass <= 1.5 * cap ? 1 : 2;
  assert.deepEqual([40, 50, 60, 61].map((m) => burden(m, 40)), [0, 1, 1, 2]);
  const push = (force, mass, anchor) => force >= Math.ceil(mass / 10) + anchor;
  assert.deepEqual([push(10, 80, 0), push(10, 120, 0), push(10, 80, 10)], [true, false, false]);
  assert.equal(Math.max(2, 2, 2), 2); // Slow movement sources take the maximum.
  assert.deepEqual([10, 6].map((impact) => -Math.max(0, 10 - impact) || 0), [0, -4]);
  assert.equal(Math.min(14, 16 + 4), 14); // Percée cannot exceed hardware.
});

const builds = [
  ['Combattant mobile', [8, 6, 6, 5, 3], [10, 7, 8, 5, 3],
    { MEL: ['01','03','04','06','09'], MAN: ['01','02','05'], FUR: ['01','03'], REC: ['01','04'], ING: ['01','03','05'] }],
  ['Tireur observateur', [3, 8, 5, 8, 4], [4, 10, 6, 9, 4],
    { TIR: ['01','04','06','07','09'], REC: ['01','03','05','08'], MAN: ['01','03','05'], FUR: ['02','03'] }],
  ['Démolisseur discret', [5, 6, 6, 7, 4], [5, 8, 7, 8, 5],
    { DEM: ['01','04','06','08','10'], FUR: ['01','03','06'], MAN: ['01','03','04'], REC: ['01','04','03'] }],
  ['Saboteur électronique', [3, 5, 6, 6, 8], [3, 6, 8, 6, 10],
    { GEL: ['01','02','05','06','08'], INT: ['01','02','05','07'], REC: ['01','04','09'], MAN: ['01','02'] }],
  ['Opérateur de drones', [4, 5, 6, 5, 8], [4, 6, 7, 6, 10],
    { DRN: ['01','02','05','07','10'], ING: ['01','03','05','08'], TIR: ['01','04','02'], REC: ['01','04'] }],
  ['Généraliste', [6, 6, 6, 5, 5], [7, 7, 7, 6, 6],
    { TIR: ['01','02','04'], MAN: ['01','03','05'], REC: ['01','03','09'], ING: ['01','03','05'], GEL: ['01','04','05'], FUR: ['02'] }],
];
check('Six builds : budgets et séquences d’apprentissage', () => {
  for (const [name, start, end, choices] of builds) {
    assert.equal(start.reduce((a,b) => a+b, 0), 28, name);
    assert.equal(end.reduce((a,b) => a+b, 0), 33, name);
    assert.ok(start.every((x) => x >= 3 && x <= 8), name);
    assert.ok(end.every((x, i) => x >= start[i] && x <= 10), name);
    assert.equal(Object.values(choices).reduce((s, ids) => s + choiceCostTotals[ids.length], 0), 21, name);
    for (const [discipline, ids] of Object.entries(choices)) {
      const chosen = new Set();
      ids.forEach((suffix) => {
        const id = `${discipline}-${suffix}`;
        assert.ok(!chosen.has(id), `${name}: duplicate ${id}`);
        assert.ok(!parents.has(id) || chosen.has(parents.get(id)), `${name}: prerequisite ${id}`);
        chosen.add(id);
      });
    }
    assert.ok(report.includes(name));
  }
  const buildRows = tables(report).find((t) => t[0][0] === 'Build').slice(2);
  assert.equal(buildRows.length, builds.length);
  for (const row of buildRows) {
    const b = builds.find(([name]) => name === row[0]); assert.ok(b);
    assert.equal(Number(row[2]), 21);
    assert.equal(Number(row[3]), Object.values(b[3]).reduce((s, ids) => s + ids.length, 0));
  }
});

check('Budget XP et anti-doublon : modèle documentaire', () => {
  let xp = 0;
  for (let level = 1; level < 20; level += 1) {
    xp += 100 + 40 * (level - 1);
    assert.equal(xp, 100 * level + 20 * level * (level - 1));
  }
  assert.equal(xp, 8740); assert.equal(2 * choiceCostTotals[5], 18);
  const paid = new Set(); let received = 0;
  const reward = (key) => { if (!paid.has(key)) { paid.add(key); received += 100; } };
  reward('obstacle:1'); reward('obstacle:1');
  assert.equal(received, 100); // Invariant illustration, not a check of Rust reward routing.
});

check('Chaleur, variantes et comparaisons chiffrées du rapport', () => {
  const thermal = (raw, resistance) => Math.floor(raw * (100 - resistance) / 100);
  const critical = (h) => 5 * Math.ceil(Math.max(0, h - 100) / 10);
  assert.deepEqual([100,115,140].map((h) => [critical(h), h - 5]), [[0,95],[10,110],[20,135]]);
  const overheat = (initial) => {
    let heat = initial, damage = 0;
    for (let t = 0; t < 3; t += 1) { heat += 15; damage += thermal(critical(heat), 25); heat = Math.max(0, heat - 2); }
    return [heat, damage];
  };
  assert.deepEqual(overheat(0), [39,0]); assert.deepEqual(overheat(90), [129,25]);
  assert.equal([16,12,9].reduce((s,d) => s + thermal(d,25), 0), 27);
  assert.equal(thermal(16,25) * 3, 36);
  assert.equal(thermal(2,75) * 3 * 4, 0);
  const expected = [4.9,4.55,3.5,2.25];
  const calculated = [0.7*(12-5),0.7*(18-5)/2,0.7*(10-5),0.9*(10-5)/2];
  const comparison = tables(report).find((t) => t[0][0] === 'Attaque de référence').slice(2);
  assert.equal(comparison.length, 4);
  calculated.forEach((x,i) => {
    assert.ok(Math.abs(x - expected[i]) < 1e-9);
    assert.equal(Number(comparison[i][3].replace(',', '.')), expected[i]);
  });
});

check('Chronologie et bornes : petits modèles indépendants', () => {
  // Effects created inside environment phase n start ticking at n+1.
  const eligible = (phase, applied, duration) => phase > applied && phase <= applied + duration;
  assert.deepEqual([7,8,9,10,11].map((p) => eligible(p,7,3)), [false,true,true,true,false]);
  const visited = new Set(); let hits = 0;
  for (const phase of [1,1,1,2,2]) {
    const key = `actor:field-family:${phase}`;
    if (!visited.has(key)) { visited.add(key); hits += 1; }
  }
  assert.equal(hits, 2);
  // A finite campaign never revisits a host even if a cycle is present.
  const graph = { a:['b','c'], b:['a','d'], c:['d'], d:['a'] };
  const attempted = new Set(['a']); const infected = ['a'];
  for (let i = 0; i < infected.length && infected.length < 3; i += 1) {
    const candidate = graph[infected[i]].find((id) => !attempted.has(id));
    if (candidate) { attempted.add(candidate); infected.push(candidate); }
  }
  assert.equal(infected.length, 3); assert.equal(new Set(infected).size, 3);
});

console.log(`\n${groups.length} groupes réussis ; ${entries.size} profils ; ${numericalRows} lignes numériques ; ${builds.length} builds.`);
console.log(`Textes joueur : ${techniqueTexts.length} descriptions ; ${interfaceTexts.length} autres entrées ; 15 validations individuelles conservées.`);
console.log('Portée : documents et modèles isolés. Aucun test du moteur Rust, du rendu ou de parties jouables.');
