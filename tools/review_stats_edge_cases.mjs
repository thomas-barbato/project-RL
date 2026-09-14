// Read-only regression models of five adopted DOCUMENTARY corrections.
// Two unchanged balance observations remain; a successful run does NOT certify gameplay.
// No Rust import, game execution, random simulation, save migration or file write.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { inspectProgression } from './stats_progression_checks.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const read = (file) => readFileSync(resolve(root, file), 'utf8').replaceAll('\r\n', '\n');
const rules = read('docs/STATISTIQUES_ET_COMPETENCES.md');
const catalogue = read('docs/PROPOSITION_COMPETENCES_v0.1.md');
const playerTexts = read('docs/TEXTES_JOUEUR_STATISTIQUES_COMPETENCES.md');
const annex = catalogue.split('## 16. Annexe technique')[1];
assert.ok(annex);
const cells = (line) => line.trim().slice(1, -1).split('|').map((cell) => cell.trim());
const profiles = new Map(annex.split('\n').filter((line) => /^\| (?:[A-Z]{3})-(?:V)?\d{2} \|/.test(line))
  .map(cells).map((row) => [row[0], row]));
const observations = [];
const corrections = [];
function verify(id, fn) {
  const result = fn();
  corrections.push({ id, ...result });
  console.log(`PASS ${id} : ${JSON.stringify(result)}`);
}
function observe(id, fn) {
  const result = fn();
  observations.push({ id, ...result });
  console.log(`CONSTAT ${id} : ${JSON.stringify(result)}`);
}
const close = (actual, expected) => assert.ok(Math.abs(actual - expected) < 1e-9);

verify('CHALEUR', () => {
  const limits = rules.match(/(\d+) pour le mode ordinaire, (\d+) pour le Surcadencement compatible/).slice(1).map(Number);
  assert.deepEqual(limits, [100, 140]);
  assert.ok(rules.includes('Refus_thermique = (Apport_H_action > 0) ET (H_projetee > Limite_H_mode)'));
  const refusal = (heat, addition, limit) => addition > 0 && Math.max(0, heat + addition) > limit;
  for (const limit of limits) {
    assert.equal(refusal(limit + 15, 0, limit), false);
    assert.equal(refusal(limit + 15, -5, limit), false);
    assert.equal(refusal(limit + 15, 1, limit), true);
    assert.equal(refusal(limit - 5, 6, limit), true);
    assert.equal(refusal(limit - 5, 5, limit), false);
  }
  assert.equal(refusal(10, -30, 100), false);
  // Historical counterexample: the removed broad gate wrongly refused this wait.
  assert.equal(115 + 0 > 100, true);
  assert.ok(/MSG-REFUS-CHALEUR[^\n]+Apport propre strictement positif[^\n]+Cette action ajoute de la chaleur/.test(playerTexts));
  return { limites: limits, apportNulOuRefroidissantAutorise: true,
    portee: 'validation thermique seulement ; autres conditions et degats maintenus' };
});

verify('RETARDATEURS', () => {
  assert.match(rules, /Une application pendant les actions ordinaires peut donc agir à la phase environnementale qui suit/);
  assert.ok(rules.includes('la première phase éligible est C+1'));
  assert.ok(rules.includes('phase environnementale C+D'));
  const fuse = Number(profiles.get('DEM-03')[4].match(/détonation après (\d+) UT/)[1]);
  const implantDelay = Number(profiles.get('GEL-08')[1].match(/retard (\d+) UT/)[1]);
  const nativeCleanup = Number(rules.match(/Un nettoyage natif accessible aux non-spécialistes prend (\d+) UT/)[1]);
  assert.equal(fuse, 1); assert.equal(implantDelay, 2); assert.equal(nativeCleanup, 2);
  const origin = 7;
  const arm = (delay, phase) => {
    assert.ok(Number.isInteger(delay) && delay >= 1);
    return { kind: 'announced-fuse', origin, originPhase: phase, due: origin + delay, fired: false };
  };
  const advance = (timer, cycle) => {
    if (timer.fired || cycle < timer.due) return 0;
    timer.fired = true;
    return 1;
  };
  const restored = (timer) => JSON.parse(JSON.stringify(timer));
  for (const phase of ['player-action', 'npc-action', 'environment']) {
    for (const delay of [fuse, implantDelay, 5]) {
      const timer = arm(delay, phase);
      assert.equal(advance(timer, origin), 0);
      assert.equal(timer.due - origin, delay);
      let loaded = restored(timer);
      for (let cycle = origin + 1; cycle < timer.due; cycle += 1) {
        assert.equal(advance(loaded, cycle), 0);
        loaded = restored(loaded); // Saving does not rearm or reset the deadline.
      }
      assert.equal(loaded.due, timer.due);
      assert.equal(advance(loaded, timer.due), 1);
      assert.equal(advance(restored(loaded), timer.due), 0);
      assert.equal(advance(restored(loaded), timer.due + 1), 0);
      const pendingAtLoad = restored(timer);
      assert.equal(advance(pendingAtLoad, timer.due), 1);
      assert.equal(advance(pendingAtLoad, timer.due), 0);
    }
  }
  assert.throws(() => arm(0, 'player-action'));
  assert.ok(fuse < nativeCleanup); // A one-UT fuse does not promise a two-UT response.
  assert.ok(implantDelay >= nativeCleanup); // Two timely steps fit; success is not guaranteed.
  const sequence = [arm(1, 'player-action'), arm(2, 'player-action')];
  assert.deepEqual([origin, origin + 1, origin + 2].map((cycle) =>
    sequence.reduce((count, timer) => count + advance(timer, cycle), 0)), [0, 1, 1]);
  // Periodic ticks retain their earlier convention, not the fuse grace cycle.
  const ticks = (environmentCreated) => Array.from({ length: 3 }, (_, i) => origin + Number(environmentCreated) + i);
  assert.deepEqual(ticks(false), [7, 8, 9]);
  assert.deepEqual(ticks(true), [8, 9, 10]);
  assert.match(rules, /Elle ne décale pas les tics d'Infection/);
  return {
    chargeDelai: fuse, cycleArmementExclu: true,
    implosionDelai: implantDelay, nettoyageNatif: nativeCleanup,
    reponsesOrdinaires: { charge: 1, implosion: 2 }, repriseSansDoubleExecution: true,
  };
});

verify('REGISTRE', () => {
  const accessSteps = Number(rules.match(/Opération native de référence[^\n]+?, (\d+) UT de préparation/)[1]);
  const auditDelay = Number(rules.match(/trace enregistrée immédiatement, examen après (\d+) UT/)[1]);
  assert.equal(profiles.get('INT-08')[1], 'P1+A1');
  const falsificationSteps = 2;
  // Fresh access succeeds first try, with the right and the record already known.
  const earliestFinish = accessSteps + falsificationSteps;
  assert.equal(earliestFinish, 4);
  assert.equal(auditDelay, 5);
  const due = (origin, environmentCreated) => origin + auditDelay - Number(!environmentCreated);
  assert.equal(due(1, false), 5);
  assert.equal(due(1, true), 6);
  assert.ok(earliestFinish < due(1, false));
  assert.ok(earliestFinish + 2 > due(1, false)); // A late falsification cannot cancel a completed audit.
  assert.equal(due(7, false), 11); // No additional fuse grace: not 12.
  const restored = JSON.parse(JSON.stringify({ origin: 1, due: due(1, false), examined: false }));
  assert.equal(restored.due, 5);
  assert.match(rules, /INT-08 utilise le droit de modification déjà acquis/);
  assert.match(rules, /ne crée donc pas[^\n]+une chaîne infinie de traces/);
  return { finAuPlusTotAccesNeuf: earliestFinish, audit: auditDelay,
    margeUT: auditDelay - earliestFinish, doubleGrace: false,
    limite: 'bon droit, preuve connue, aucune suppression de copies ou temoins' };
});

verify('RECONNAISSANCE_PARTIELLE', () => {
  const entries = catalogue.split('## 16. Annexe technique')[0].split('\n')
    .filter((line) => /^\| REC-\d{2} \| [1-5] \|/.test(line)).map(cells)
    .map((row) => ({ id: row[0], minimumLevel: Number(row[1]) }));
  assert.equal(entries.length, 7);
  assert.match(catalogue, /REC-09[^\n]+Demande Analyse de cible/);
  const parents = new Map([['REC-09', 'REC-01']]);
  const explore = (disabled, initial = []) => inspectProgression(entries, parents, disabled, initial);
  const full = explore(new Set());
  const noDiagnostic = explore(new Set(['REC-08']));
  const noTracesSecretsDiagnostic = explore(new Set(['REC-02', 'REC-03', 'REC-08']));
  assert.ok(full.allPathsComplete);
  assert.ok(noDiagnostic.allPathsComplete);
  assert.equal(noTracesSecretsDiagnostic.countsByChoiceCount[5], 0);
  assert.equal(noTracesSecretsDiagnostic.allPathsComplete, false);
  assert.match(rules, /différer l'ouverture d'une discipline/);
  assert.ok(explore(new Set(['REC-01'])).unavailableIds.includes('REC-09'));
  assert.ok(explore(new Set(), ['REC-01', 'REC-02']).allPathsComplete);
  assert.equal(explore(new Set(['REC-02']), ['REC-02']).validInitial, false);
  return { configurationsParNombreDeChoix: { complet: full.countsByChoiceCount.slice(1),
    sansDiagnostic: noDiagnostic.countsByChoiceCount.slice(1),
    sansTracesSecretsDiagnostic: noTracesSecretsDiagnostic.countsByChoiceCount.slice(1) },
    disciplineIncompleteDifferee: true,
    limite: 'masques fictifs, pas detection des fonctionnalites du prototype ni migration de sauvegarde' };
});

verify('RETRAITE_METHODIQUE', () => {
  assert.equal(profiles.has('MAN-07'), false);
  assert.doesNotMatch(playerTexts, /^\| MAN-07 \|/m);
  assert.match(catalogue, /\| MAN-07 \| Fusionnée, identifiant réservé \|/);
  assert.match(profiles.get('MAN-01')[4], /\+20 Esquive uniquement contre une interception/);
  assert.match(profiles.get('MAN-01')[4], /maintien de la consigne inclus sans achat supplémentaire/);
  assert.match(profiles.get('MAN-01')[5], /Chaque pas coûte son action/);
  // Same known route, three unencumbered retreat steps; no special hazard changes.
  const manual = { steps: 3, time: 3, dodgeVsInterception: 20 };
  const retainedCommand = { steps: 3, time: 3, dodgeVsInterception: 20 };
  assert.deepEqual(manual, retainedCommand);
  const entries = catalogue.split('## 16. Annexe technique')[0].split('\n')
    .filter((line) => /^\| MAN-\d{2} \| [1-5] \|/.test(line)).map(cells)
    .map((row) => ({ id: row[0], minimumLevel: Number(row[1]) }));
  assert.equal(entries.length, 9);
  assert.ok(inspectProgression(entries, new Map([['MAN-08', 'MAN-04']])).allPathsComplete);
  return { repetitionManuelle: manual, consigneMaintenue: retainedCommand,
    achatSupplementaire: false, profilsManoeuvre: entries.length, cinqChoixToujoursPossibles: true };
});

observe('ATTAQUES_SITUATIONNELLES', () => {
  assert.match(annex, /mêlée 12 dégâts physiques à Impact 10/);
  assert.match(profiles.get('MEL-01')[4], /×1,5/);
  assert.equal(profiles.get('MEL-01')[1], 'A1 + R1');
  assert.match(profiles.get('TIR-01')[4], /\+20 Précision/);
  assert.equal(profiles.get('TIR-01')[1], 'P1+A1');
  const melee = [0, 6, 8, 12].map((armor) => ({
    blindage: armor,
    ordinaire: 0.7 * Math.max(0, 12 - armor),
    puissante: 0.7 * Math.max(0, 18 - armor) / 2,
  }));
  close(melee[1].ordinaire, melee[1].puissante);
  assert.ok(melee[2].puissante > melee[2].ordinaire);
  const aimed = [10, 20, 70].map((rawChance) => ({
    chanceInitiale: rawChance,
    ordinaire: rawChance / 100 * 10,
    vise: Math.min(95, rawChance + 20) / 100 * 10 / 2,
  }));
  close(aimed[0].ordinaire, 1); close(aimed[0].vise, 1.5);
  close(aimed[1].ordinaire, aimed[1].vise);
  close(aimed[2].ordinaire, 7); close(aimed[2].vise, 4.5);
  return { degatsEsperesParUT: { melee, tirSansBlindage: aimed },
    limites: 'aucune interruption, aucun avantage du soutien pendant R1, pas de simulation ennemie' };
});

observe('INFECTION_ET_CANAUX', () => {
  assert.match(profiles.get('GEL-V07')[4], /2 thermiques bruts\/tic/);
  assert.match(profiles.get('GEL-V07')[5], /3 tics par hôte/);
  assert.match(rules, /maximum de 75 %|plafond.*75 %|75 %/);
  const infection = [0, 49, 50, 75].map((resistance) => ({
    resistance, parTic: Math.floor(2 * (100 - resistance) / 100),
    troisTics: 3 * Math.floor(2 * (100 - resistance) / 100),
  }));
  assert.equal(infection[1].parTic, 1);
  assert.equal(infection[2].parTic, 1);
  assert.equal(Math.floor(2 * (100 - 51) / 100), 0);
  assert.equal(infection[3].troisTics, 0);
  assert.match(rules, /2 drones et 4 B ; une unité active réserve 1 B, une routine avancée 1 B supplémentaire/);
  assert.match(profiles.get('DRN-05')[2], /\+1 B pendant transmission/);
  const used = 2 * (1 + 1);
  assert.equal(used, 4); assert.ok(used + 1 > 4);
  return { infectionContagieuse: infection, nulDesResistanceEntiere51: true,
    canaux: { deuxRoutinesAvancees: used, commandeDemandee: 1, capacite: 4, lancementDirectPossible: false } };
});

console.log(`\n${corrections.length} groupes de corrections vérifiés ; ${observations.length} observations d'équilibrage conservées.`);
console.log('Portée : lectures documentaires et modèles isolés, pas comportement du moteur ni équilibrage validé.');
