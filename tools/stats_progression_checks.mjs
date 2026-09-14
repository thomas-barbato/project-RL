// Pure documentary model, not the Rust progression system.
// Check every reachable prefix after a version mask and optional ordered class grants.
export function inspectProgression(pool, parents, disabled = new Set(), initial = []) {
  const byId = new Map(pool.map((entry) => [entry.id, entry]));
  const invalid = (reason) => ({ validInitial: false, reason, allPathsComplete: false,
    countsByChoiceCount: Array(6).fill(0), deadEnds: [], reachedIds: [], availableIds: [], unavailableIds: [] });
  if (byId.size !== pool.length
      || pool.some((e) => !Number.isInteger(e.minimumLevel) || e.minimumLevel < 1)) {
    return invalid('invalid-catalogue');
  }
  const available = new Set(pool.filter((e) => !disabled.has(e.id)).map((e) => e.id));
  let changed;
  do {
    changed = false;
    for (const id of available) {
      const parent = parents.get(id);
      if (parent && !available.has(parent)) { available.delete(id); changed = true; }
    }
  } while (changed);
  const learned = new Set();
  for (const id of initial) {
    if (!available.has(id)) return invalid('initial-choice-unavailable');
    if (learned.has(id)) return invalid('duplicate-initial-choice');
    if (parents.has(id) && !learned.has(parents.get(id))) return invalid('initial-prerequisite-missing');
    learned.add(id);
  }
  const key = (state) => [...state].sort().join(',');
  const levels = Array.from({ length: Math.max(6, learned.size + 1) }, () => new Map());
  if (learned.size >= 5) {
    levels[5].set(key(learned), learned);
  } else {
    levels[learned.size].set(key(learned), learned);
  }
  const deadEnds = [];
  const reached = new Set(learned);
  for (let choiceCount = learned.size; choiceCount < 5; choiceCount += 1) {
    for (const state of levels[choiceCount].values()) {
      let successors = 0;
      for (const id of available) {
        if (state.has(id)) continue;
        if (parents.has(id) && !state.has(parents.get(id))) continue;
        const next = new Set([...state, id]);
        levels[choiceCount + 1].set(key(next), next);
        reached.add(id);
        successors += 1;
      }
      if (successors === 0) deadEnds.push([...state].sort());
    }
  }
  return {
    validInitial: true,
    allPathsComplete: deadEnds.length === 0 && levels[5].size > 0,
    countsByChoiceCount: levels.map((states) => states.size),
    deadEnds,
    reachedIds: [...reached].sort(),
    availableIds: [...available].sort(),
    unavailableIds: pool.filter((e) => !available.has(e.id)).map((e) => e.id).sort(),
  };
}
