import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { loadCatalogue, validateCatalogue, allBases, compatibleBases, compatibleAffixes, previewRng, drawPreview, describeItem } from './equipment_catalogue.mjs';

test('the design catalogue covers all tiers, names, grammar and supported lab effects', () => {
  assert.deepEqual(validateCatalogue(loadCatalogue()), { families: 24, bases: 144, affixes: 22, sources: 6 });
});

test('affix weights are independent from item quality and zero disables a property', () => {
  const c = loadCatalogue();
  for (const affix of c.affixes) affix.weight = 0;
  c.affixes.find(a => a.id === 'affix:power').weight = 100;
  c.affixes.find(a => a.id === 'affix:vitality').weight = 10;
  validateCatalogue(c);
  const rng = previewRng(42), counts = { 'affix:power': 0, 'affix:vitality': 0 };
  for (let i = 0; i < 12000; i++) {
    const item = drawPreview(c, { source: 'humanoid_site', affixCount: 1 }, rng);
    assert.equal(item.affixes.length, 1);
    counts[item.affixes[0].affix_id]++;
  }
  assert.ok(counts['affix:power'] > counts['affix:vitality'] * 8);
  assert.ok(counts['affix:power'] < counts['affix:vitality'] * 12);
});

test('the readable catalogue contains every base and affix', () => {
  const c = loadCatalogue();
  const bases = readFileSync(new URL('../docs/CATALOGUE_BASES_EQUIPEMENT.md', import.meta.url), 'utf8');
  const affixes = readFileSync(new URL('../docs/AFFIXES_ET_PROVENANCE_EQUIPEMENT.md', import.meta.url), 'utf8');
  for (const b of allBases(c)) assert.ok(bases.includes(`| P${b.tier} | ${b.name} |`), b.id);
  for (const a of c.affixes) assert.ok(affixes.includes(a.forms.all ?? `${a.forms.ms} / ${a.forms.fs}`), a.id);
});

test('a robot never receives humanoid equipment, including rare high-tier rolls', () => {
  const c = loadCatalogue(), rng = previewRng(9025);
  assert.equal(compatibleBases(c, 'robot').length, 24);
  const seen = new Set();
  for (let depth = 0; depth < 10; depth++) for (let i = 0; i < 200; i++) {
    const item = drawPreview(c, { source: 'robot', depth }, rng);
    const base = allBases(c).find((b) => b.id === item.base_id);
    assert.equal(base.family.form, 'machine_module');
    assert.ok(!['organic', 'living'].includes(base.nature));
    assert.ok(!base.loot_sources.includes('humanoid_equipped'));
    seen.add(base.id);
  }
  assert.equal(seen.size, 24);
});

test('an empty or unknown creature profile cannot fall back to the whole catalogue', () => {
  const c = loadCatalogue(), rng = previewRng(123), state = rng.state;
  for (const source of ['organic_creature', 'anomalous_creature']) {
    assert.equal(drawPreview(c, { source, depth: 100 }, rng), null);
    assert.equal(rng.state, state);
  }
  assert.throws(() => drawPreview(c, { source: 'unknown' }, rng), /Unknown source/);
  assert.equal(rng.state, state);
});

test('material and provenance are independent: organic clothing is not generic animal loot', () => {
  const c = loadCatalogue();
  const clothing = compatibleBases(c, 'humanoid_equipped').find((b) => b.name === 'Veste de peau seconde');
  assert.equal(clothing.nature, 'living');
  assert.ok(!clothing.loot_sources.includes('organic_creature'));
  assert.equal(compatibleBases(c, 'humanoid_site').length, 120);
  assert.equal(compatibleBases(c, 'machine_site').length, 24);
});

test('white items keep their base name and have no affixes', () => {
  const c = loadCatalogue();
  const item = drawPreview(c, { source: 'humanoid_equipped', white: true }, previewRng(10));
  assert.deepEqual(item.affixes, []);
  assert.deepEqual(item.details, []);
  assert.equal(item.name, allBases(c).find((b) => b.id === item.base_id).name);
});

test('rolls are reproducible, bounded, non-stacking and source-compatible', () => {
  const c = loadCatalogue(), left = previewRng(84), right = previewRng(84);
  const seen = new Set();
  for (let i = 0; i < 1500; i++) {
    const options = { source: i % 2 ? 'robot' : 'humanoid_equipped', depth: i % 6 };
    const item = drawPreview(c, options, left);
    assert.deepEqual(item, drawPreview(c, options, right));
    const base = allBases(c).find((b) => b.id === item.base_id);
    const groups = [], allowed = compatibleAffixes(c, base);
    let special = 0;
    for (const roll of item.affixes) {
      const affix = allowed.find((a) => a.id === roll.affix_id);
      assert.ok(affix); groups.push(affix.group); seen.add(affix.id);
      if (affix.category === 'stat') {
        const range = affix.ranges.find((r) => r.tier === item.tier);
        assert.ok(roll.value >= range.minimum && roll.value <= range.maximum);
      } else { special++; assert.equal(base.family.kind, 'weapon'); }
    }
    assert.equal(new Set(groups).size, groups.length);
    assert.ok(special <= 1 && item.affixes.length >= 1 && item.affixes.length <= 3);
  }
  assert.equal(seen.size, 22);
});

test('ricochet stays on projectile families; armor never gets an offensive trigger', () => {
  const c = loadCatalogue();
  for (const base of allBases(c)) {
    const affixes = compatibleAffixes(c, base);
    if (base.family.kind === 'armor') assert.ok(affixes.every((a) => a.category === 'stat'));
    if (['thermal_projectors', 'electric_projectors', 'knives'].includes(base.family_id)) {
      assert.ok(!affixes.some((a) => a.lab_profile === 'ricochet'));
    }
  }
});

test('French gender/plural and a short title preserve every affix in the details', () => {
  const c = loadCatalogue();
  for (const [family, adjective] of [['knives', 'précis'], ['swords', 'précise'], ['gauntlets', 'précis'], ['boots', 'précises']]) {
    const base = allBases(c).find((b) => b.family_id === family && b.tier === 1);
    const rolls = [{ affix_id: 'affix:accuracy', tier: 1, value: 8 }, { affix_id: 'affix:vitality', tier: 1, value: 10 }, { affix_id: 'affix:power', tier: 1, value: 1 }];
    const before = JSON.stringify(rolls);
    const item = describeItem(c, base, rolls);
    assert.equal(item.name, `${base.name} ${adjective} de vitalité`);
    assert.equal(item.details.length, 3);
    assert.ok(item.details.some((d) => d.description === 'Puissance +1'));
    assert.equal(JSON.stringify(rolls), before);
  }
});

test('invalid or contradictory catalogue entries are refused', () => {
  const mutations = [
    (c) => c.families[1].bases[0].id = c.families[0].bases[0].id,
    (c) => c.families[0].bases[0].loot_sources.push('robot'),
    (c) => c.families[0].bases[0].loot_sources.push('missing'),
    (c) => c.families[0].bases[0].tier = 0,
    (c) => c.affixes[0].ranges[0].minimum = 999,
    (c) => c.affixes[0].property = 'unimplemented_speed_bonus',
    (c) => c.affixes.find((a) => a.lab_profile === 'healing').scope = 'all_equipped',
    (c) => c.distribution.depth_weights[0].weights[5] = 0,
    (c) => c.generation_proposal.duplicate_groups_allowed = true,
  ];
  for (const mutate of mutations) { const c = loadCatalogue(); mutate(c); assert.throws(() => validateCatalogue(c)); }
});

test('formatting cannot invent values, stack duplicate affixes or put a trigger on armor', () => {
  const c = loadCatalogue(), bases = allBases(c);
  const blade = bases.find((b) => b.family_id === 'swords' && b.tier === 2);
  const armor = bases.find((b) => b.family_id === 'jackets' && b.tier === 2);
  assert.throws(() => describeItem(c, blade, [{ affix_id: 'affix:vitality', tier: 2, value: 999 }]));
  const roll = { affix_id: 'affix:vitality', tier: 2, value: 15 };
  assert.throws(() => describeItem(c, blade, [roll, roll]), /Duplicate/);
  assert.throws(() => describeItem(c, armor, [{ affix_id: 'affix:healing', tier: 2 }]), /incompatible/);
});
