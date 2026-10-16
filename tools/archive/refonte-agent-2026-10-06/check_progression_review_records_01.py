"""Document integrity and design arithmetic only; no engine test claim."""
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
PREFIX = "artifacts/refonte-agent-2026-10-06/"
FILES = [
    "skill-rank-balancing-delegation-decisions.json",
    "current-level-skill-power-acceptance-decisions.json",
    "review-current-level-skill-power-01.json",
    "skill-rank-curves-provisional-01.json",
    "skill-rank-curves-validation-01.json",
    "review-implementation-readiness-03.json",
    "review-attribute-model-61.json",
    "review-primary-catalogue-233.json",
    "review-complete-skill-design-208.json",
    "remaining-individual-skill-review-counts-102.json"
]


def load(name):
    return json.loads((BASE / name).read_text(encoding="utf-8"))


def digest(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def main():
    checked, errors = [], []

    def walk(value, context):
        if isinstance(value, dict):
            for key, expected in value.items():
                if key.endswith("_sha256") and key != "calculations_sha256":
                    stem = key[:-7]
                    source = value.get(stem, value.get(stem + "_markdown"))
                    if not isinstance(source, str):
                        continue
                    path = ROOT / source
                    content = source
                    if len(source) < 240 and "\n" not in source:
                        if path.is_file():
                            content = path.read_text(encoding="utf-8")
                        elif source.startswith(("artifacts/", "docs/")):
                            errors.append((context, "missing reference", source))
                            continue
                    actual = digest(content)
                    checked.append((context, key))
                    if actual != expected:
                        errors.append((context, key, source[:100]))
            for key, nested in value.items():
                walk(nested, context + "/" + key)
        elif isinstance(value, list):
            for index, nested in enumerate(value):
                walk(nested, context + "/" + str(index))

    for name in FILES:
        walk(load(name), name)
    for old_name, new_name in [
        ("review-attribute-model-60.json", "review-attribute-model-61.json"),
        ("review-primary-catalogue-232.json", "review-primary-catalogue-233.json"),
        ("review-complete-skill-design-207.json", "review-complete-skill-design-208.json"),
        ("remaining-individual-skill-review-counts-101.json", "remaining-individual-skill-review-counts-102.json")
    ]:
        old, new = load(old_name), load(new_name)
        for key, value in old.items():
            if key.startswith("accepted_") or key in {"branches", "blocks", "removed_skills", "removed_branches", "purchase_rules", "candidate_count", "learned_rank_cap", "cost_per_rank", "accepted_coefficients"}:
                assert new.get(key) == value, (new_name, key)
        for key, value in old.get("active_rule_overrides", {}).items():
            assert new["active_rule_overrides"][key] == value, (new_name, key)
        assert new["runtime_integration"] == "not_integrated"

    curves = load("skill-rank-curves-provisional-01.json")
    primary = load("review-primary-catalogue-233.json")
    primary_ids = {r["id"] for b in primary["branches"] for r in b["candidates"]}
    skill_ids = {r["skill_id"] for r in curves["skills"]}
    assert len(skill_ids) == len(curves["skills"]) == 94
    assert len(primary_ids) == 79 and len(primary["branches"]) == 14
    assert primary_ids <= skill_ids
    removed = set()
    for r in primary.get("removed_skills", []):
        if isinstance(r, str):
            removed.add(r)
        elif isinstance(r, dict):
            removed.add(r.get("id", r.get("skill_id")))
    assert not skill_ids.intersection(removed)
    assert not any(s.startswith("FO") for s in skill_ids)
    assert load("remaining-individual-skill-review-counts-102.json")["remaining_first_pass_total"] == 0
    assert curves["global_constraints"]["learned_cap"] == 10
    assert curves["global_constraints"]["secondary_active_learned_cap"] == 1
    assert curves["global_constraints"]["secondary_passive_purchase_requires_actual_learned_active"]
    assert load("review-current-level-skill-power-01.json")["coefficient_approval"].startswith("proposed_reference_only")
    assert load("review-attribute-model-61.json")["pending_skill_progression"] is None

    prose = ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md"
    lines = prose.read_text(encoding="utf-8").splitlines()
    assert not any(line.rstrip() != line for line in lines[-110:])
    assert not errors, errors
    report = {
        "date": "2026-10-09", "review_id": "SKILL-RANKS01-DOC-CHECK01",
        "scope": "versioned_design_manifest_integrity_coverage_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": FILES,
        "hash_references_checked": len(checked), "hash_errors": errors,
        "coverage": {"branches": 14, "primary": 79, "secondary": 14, "repair_complement": 1, "total": 94},
        "level_bound_damage_skill_count": sum(bool(row["current_level_damage_parameters"]) for row in curves["skills"]),
        "source_accepted_fields_and_previous_active_overrides": "unchanged",
        "rank_caps_and_secondary_prerequisite": "preserved",
        "removed_skill_and_Foudre_reintroduction": False,
        "current_level_direction": "explicitly_accepted",
        "exact_level_coefficient": "separate_proposed_reference;not_silently_approved",
        "rank_arithmetic_report": {"reference": PREFIX + "skill-rank-curves-validation-01.json",
                                  "reference_sha256": digest((BASE / "skill-rank-curves-validation-01.json").read_text(encoding="utf-8"))},
        "runtime_integration": "not_integrated"
    }
    out = BASE / "skill-rank-curves-document-integrity-01.json"
    assert not out.exists(), out
    out.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps({"files": len(FILES), "checked_hashes": len(checked), "errors": 0,
                      "skills": 94, "level_bound_skills": report["level_bound_damage_skill_count"],
                      "preserved": "accepted_rules_caps_catalogue_and_previous_overrides", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
