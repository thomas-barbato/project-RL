// Design tooling only. Does not load content into the game, edit saves or write files.
// node tools/equipment_catalogue.mjs [source-profile] [depth] [seed]
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
export const loadCatalogue = () => JSON.parse(readFileSync(resolve(root, 'docs/catalogues/equipements.json'), 'utf8'));
export const allBases = (catalogue) => catalogue.families.flatMap((family) =>
  family.bases.map((base) => ({ ...base, family_id: family.id, family })));
const unique = (values, label) => assert.equal(new Set(values).size, values.length, `Duplicate ${label}`);
const integer = (n, minimum, maximum = Number.MAX_SAFE_INTEGER) => Number.isSafeInteger(n) && n >= minimum && n <= maximum;

export function validateCatalogue(c) {
  assert.equal(c.schema_version, 1);
  assert.equal(c.status, 'design_only_not_loaded_by_game');
  assert.equal(c.deep_equipment_direction, 'mixed_human_exotic_organic_living');
  assert.equal(c.power_tiers_are_player_levels, false);
  assert.equal(c.player_level_cap, null);
  const tiers = c.tiers.map((t) => t.id);
  assert.ok(tiers.length >= 1);
  assert.deepEqual(tiers, tiers.map((_, i) => i + 1), 'Consecutive power tiers');
  unique(c.families.map((f) => f.id), 'family');
  unique(c.loot_sources.map((s) => s.id), 'source');
  unique(c.affixes.map((a) => a.id), 'affix');
  const bases = allBases(c);
  unique(bases.map((b) => b.id), 'base ID');
  unique(bases.map((b) => b.name), 'base name');
  const sourceById = new Map(c.loot_sources.map((s) => [s.id, s]));
  assert.deepEqual(sourceById.get('robot')?.forms, ['machine_module']);
  assert.deepEqual(sourceById.get('humanoid_equipped')?.forms, ['humanoid_equipment']);
  for (const source of c.loot_sources) {
    assert.ok(['enemy', 'site'].includes(source.kind));
    unique(source.forms, 'source form');
    assert.ok(source.forms.every((f) => ['humanoid_equipment', 'machine_module'].includes(f)));
  }
  for (const family of c.families) {
    assert.ok(['weapon', 'armor'].includes(family.kind));
    assert.ok(['melee', 'ranged', 'none'].includes(family.delivery));
    assert.equal(family.delivery === 'none', family.kind === 'armor');
    assert.ok(['profile_possible', 'engine_work_needed', 'slot_work_needed'].includes(family.readiness));
    assert.ok(['humanoid_equipment', 'machine_module'].includes(family.form));
    assert.ok(family.role && family.notes && family.slot);
    assert.deepEqual(family.bases.map((b) => b.tier), tiers, `Coverage: ${family.id}`);
    for (const base of family.bases) {
      assert.match(base.id, /^catalog:[a-z0-9_]+$/);
      assert.ok(base.name.trim() === base.name && base.name.length > 3);
      assert.ok(['ms', 'fs', 'mp', 'fp'].includes(base.grammar));
      assert.ok(['manufactured', 'organic', 'living', 'anomalous'].includes(base.nature));
      assert.ok(base.loot_sources.length > 0, `Orphan base ${base.id}`);
      unique(base.loot_sources, 'base source');
      for (const id of base.loot_sources) {
        const source = sourceById.get(id);
        assert.ok(source, `Unknown source ${id}`);
        assert.ok(source.forms.includes(family.form), `Incompatible source ${id}: ${base.id}`);
        if (id === 'robot') assert.ok(!['organic', 'living'].includes(base.nature), 'Organic robot loot is not approved');
      }
    }
  }
  const stats = ['power', 'coordination', 'resilience', 'perception', 'processing', 'accuracy',
    'armor_penetration', 'maximum_hit_points', 'energy_capacity', 'heat_dissipation'];
  for (const a of c.affixes) {
    assert.match(a.id, /^affix:[a-z0-9_]+$/);
    assert.ok(integer(a.weight, 0, 0xffffffff), `Invalid draw weight ${a.id}`);
    assert.ok(a.group && ['prefix', 'suffix'].includes(a.placement));
    assert.ok(integer(a.priority, 0));
    assert.ok(a.forms.all || ['ms', 'fs', 'mp', 'fp'].every((g) => typeof a.forms[g] === 'string' && a.forms[g].length));
    assert.ok(a.compatibility.length && a.compatibility.every((k) => ['weapon', 'armor'].includes(k)));
    if (a.category === 'stat') {
      assert.ok(stats.includes(a.property), `Unsupported property ${a.property}`);
      assert.equal(a.scope, 'all_equipped');
      assert.ok(a.stat_label);
      assert.deepEqual(a.ranges.map((r) => r.tier), tiers);
      for (const r of a.ranges) {
        const max = stats.indexOf(a.property) < 5 ? 255 : 65535;
        assert.ok(integer(r.minimum, 1, max) && integer(r.maximum, r.minimum, max), `Invalid range ${a.id}`);
      }
    } else {
      assert.equal(a.category, 'special_effect');
      assert.equal(a.scope, 'used_weapon_only');
      assert.deepEqual(a.compatibility, ['weapon']);
      assert.ok(a.deliveries.length && a.deliveries.every((d) => ['melee', 'ranged'].includes(d)));
      assert.ok(tiers.includes(a.minimum_tier));
      assert.ok(a.effect && a.lab_profile && a.origin);
      assert.ok(!c.excluded_effects.some((e) => e.lab_profile === a.lab_profile), 'Excluded effect reintroduced');
      if (a.allowed_families) {
        unique(a.allowed_families, 'allowed family');
        assert.ok(a.allowed_families.every((id) => c.families.some((f) => f.id === id && a.deliveries.includes(f.delivery))));
      }
    }
  }
  const lab = readFileSync(resolve(root, 'src/effects_lab.rs'), 'utf8');
  const profiles = [...lab.match(/const LAB_KINDS:[\s\S]*?=\s*\[([\s\S]*?)\];/)[1].matchAll(/"([a-z_]+)"/g)].map((m) => m[1]);
  assert.ok(c.affixes.filter((a) => a.category === 'special_effect').every((a) => profiles.includes(a.lab_profile)), 'Effect missing from laboratory');
  assert.equal(c.distribution.roll_order[0], 'source_profile');
  assert.equal(c.distribution.on_death, 'transfer_remaining_existing_possessions_not_reroll');
  assert.equal(c.distribution.no_eligible_item, 'no_equipment_drop_no_global_fallback');
  assert.equal(c.distribution.material_does_not_determine_source, true);
  assert.equal(c.generation_proposal.duplicate_groups_allowed, false);
  assert.equal(c.generation_proposal.white_items_have_affixes, false);
  assert.equal(c.generation_proposal.draws_persist, true);
  assert.ok(integer(c.generation_proposal.minimum_affixes, 1));
  assert.ok(integer(c.generation_proposal.maximum_affixes, c.generation_proposal.minimum_affixes, c.affixes.length));
  assert.equal(c.generation_proposal.maximum_special_effects, 1);
  let previousMean = 0;
  for (const [i, row] of c.distribution.depth_weights.entries()) {
    assert.equal(row.depth, i);
    assert.equal(row.weights.length, tiers.length);
    assert.ok(row.weights.every((w) => integer(w, 1, 10000)), 'Every tier stays possible');
    assert.equal(row.weights.reduce((a, b) => a + b, 0), 10000);
    const mean = row.weights.reduce((total, w, j) => total + w * tiers[j], 0) / 10000;
    assert.ok(mean > previousMean, 'Depth must favor stronger tiers');
    previousMean = mean;
  }
  assert.ok(c.distribution.depth_weights.length);
  return { families: c.families.length, bases: bases.length, affixes: c.affixes.length, sources: c.loot_sources.length };
}

export function compatibleBases(c, sourceId) {
  const source = c.loot_sources.find((s) => s.id === sourceId);
  assert.ok(source, `Unknown source ${sourceId}`);
  return allBases(c).filter((b) => b.loot_sources.includes(sourceId) && source.forms.includes(b.family.form));
}

export function compatibleAffixes(c, base) {
  return c.affixes.filter((a) => a.compatibility.includes(base.family.kind)
    && (a.category === 'stat' ? a.ranges.some((r) => r.tier === base.tier)
      : base.tier >= a.minimum_tier && a.deliveries.includes(base.family.delivery)
        && (!a.allowed_families || a.allowed_families.includes(base.family_id))));
}

// A separate reproducible RNG for offline catalogue examples, NOT the game RNG.
export function previewRng(seed) {
  assert.ok(integer(seed, 0, 0xffffffff));
  let state = seed;
  return { get state() { return state; }, below(bound) {
    assert.ok(integer(bound, 1, 0xffffffff));
    const limit = 0x100000000 - (0x100000000 % bound);
    let n;
    do { state = (Math.imul(state, 1664525) + 1013904223) >>> 0; n = state; } while (n >= limit);
    return n % bound;
  } };
}

export function describeItem(c, base, rolls) {
  const allowed = compatibleAffixes(c, base);
  const details = rolls.map((roll) => {
    const a = allowed.find((a) => a.id === roll.affix_id);
    assert.ok(a, `Unknown or incompatible affix ${roll.affix_id}`);
    assert.equal(roll.tier, base.tier);
    if (a.category === 'stat') {
      const range = a.ranges.find((r) => r.tier === roll.tier);
      assert.ok(integer(roll.value, range.minimum, range.maximum), `Invalid rolled value ${a.id}`);
    } else assert.equal(roll.value, undefined, 'Special effect values are not generated');
    const label = a.forms.all ?? a.forms[base.grammar];
    const description = a.category === 'stat' ? `${a.stat_label} +${roll.value}` : a.effect;
    return { ...roll, label, description, placement: a.placement, priority: a.priority, group: a.group };
  });
  unique(details.map((d) => d.group), 'rolled affix group');
  const strongestName = (placement) => details.filter((a) => a.placement === placement)
    .sort((a, b) => b.priority - a.priority || (a.affix_id < b.affix_id ? -1 : a.affix_id > b.affix_id ? 1 : 0))[0]?.label;
  // French adjectives follow the noun here, even when mechanically a prefix.
  const name = [base.name, strongestName('prefix'), strongestName('suffix')].filter(Boolean).join(' ');
  return { name, details };
}

export function drawPreview(c, { source, depth = 0, white = false, affixCount }, rng) {
  assert.ok(integer(depth, 0));
  const eligible = compatibleBases(c, source);
  if (!eligible.length) return null; // No fallback, no RNG consumption.
  const familyIds = [...new Set(eligible.map((b) => b.family_id))];
  const family = familyIds[rng.below(familyIds.length)];
  const familyBases = eligible.filter((b) => b.family_id === family);
  const profile = c.distribution.depth_weights[Math.min(depth, c.distribution.depth_weights.length - 1)];
  const availableTiers = [...new Set(familyBases.map((b) => b.tier))];
  let draw = rng.below(availableTiers.reduce((n, tier) => n + profile.weights[tier - 1], 0));
  const tier = availableTiers.find((t) => (draw -= profile.weights[t - 1]) < 0);
  const candidates = familyBases.filter((b) => b.tier === tier);
  const base = candidates[rng.below(candidates.length)];
  const rules = c.generation_proposal;
  const count = white ? 0 : affixCount ?? rules.minimum_affixes + rng.below(rules.maximum_affixes - rules.minimum_affixes + 1);
  assert.ok(integer(count, white ? 0 : rules.minimum_affixes, white ? 0 : rules.maximum_affixes));
  const rolls = [], groups = new Set(); let special = 0;
  for (let i = 0; i < count; i++) {
    const pool = compatibleAffixes(c, base).filter((a) => a.weight > 0 && !groups.has(a.group)
      && (a.category !== 'special_effect' || special < rules.maximum_special_effects));
    assert.ok(pool.length, 'Not enough compatible affixes');
    let ticket = rng.below(pool.reduce((sum, a) => sum + a.weight, 0));
    const a = pool.find((a) => (ticket -= a.weight) < 0);
    groups.add(a.group);
    const roll = { affix_id: a.id, tier };
    if (a.category === 'stat') {
      const range = a.ranges.find((r) => r.tier === tier);
      roll.value = range.minimum + rng.below(range.maximum - range.minimum + 1);
    } else special++;
    rolls.push(roll);
  }
  return { base_id: base.id, source, tier, nature: base.nature, affixes: rolls, ...describeItem(c, base, rolls) };
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const c = loadCatalogue();
  console.log('Catalogue de conception uniquement :', validateCatalogue(c));
  const source = process.argv[2] ?? 'robot';
  const depth = Number(process.argv[3] ?? 2);
  const rng = previewRng(Number(process.argv[4] ?? 42));
  for (let i = 0; i < 8; i++) {
    const item = drawPreview(c, { source, depth }, rng);
    console.log(item ? `[P${item.tier}] ${item.name}\n  ${item.details.map((d) => `${d.label} : ${d.description}`).join(' ; ')}` : 'Aucun équipement compatible.');
  }
}
