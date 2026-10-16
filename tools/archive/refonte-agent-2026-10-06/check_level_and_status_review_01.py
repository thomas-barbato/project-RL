"""Review reference and arithmetic checks only; no engine tests."""
from fractions import Fraction
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
FILES = [
    "current-level-skill-power-numeric-acceptance-decisions.json",
    "last-status-cast-power-acceptance-decisions.json",
    "review-current-level-skill-power-02.json",
    "skill-rank-curves-provisional-02.json",
    "skill-rank-curves-validation-02.json",
    "review-persistent-power-snapshot-INTERACT-POWER01-01.json",
    "review-implementation-readiness-04.json",
    "review-attribute-model-62.json",
    "review-primary-catalogue-234.json",
    "review-complete-skill-design-209.json",
    "remaining-individual-skill-review-counts-103.json"
]


def load(name):
    return json.loads((BASE / name).read_text(encoding="utf-8"))


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
                    content = source
                    if len(source) < 240 and "\n" not in source:
                        path = ROOT / source
                        if path.is_file():
                            content = path.read_text(encoding="utf-8")
                        elif source.startswith(("artifacts/", "docs/")):
                            errors.append((context, "missing reference", source))
                            continue
                    checked.append((context, key))
                    if hashlib.sha256(content.encode("utf-8")).hexdigest() != expected:
                        errors.append((context, key, source[:100]))
            for key, nested in value.items():
                walk(nested, context + "/" + key)
        elif isinstance(value, list):
            for index, nested in enumerate(value):
                walk(nested, context + "/" + str(index))

    for name in FILES:
        walk(load(name), name)
    for old_name, new_name in [
        ("review-attribute-model-61.json", "review-attribute-model-62.json"),
        ("review-primary-catalogue-233.json", "review-primary-catalogue-234.json"),
        ("review-complete-skill-design-208.json", "review-complete-skill-design-209.json"),
        ("remaining-individual-skill-review-counts-102.json", "remaining-individual-skill-review-counts-103.json"),
        ("review-implementation-readiness-03.json", "review-implementation-readiness-04.json")
    ]:
        old, new = load(old_name), load(new_name)
        for key, value in old.items():
            if key.startswith("accepted_") or key in {"branches", "blocks", "removed_skills", "removed_branches", "purchase_rules", "candidate_count", "learned_rank_cap", "cost_per_rank"}:
                assert new.get(key) == value, (new_name, key)
        for key, value in old.get("active_rule_overrides", {}).items():
            if key not in {"current_level_nonweapon_power", "delegated_provisional_skill_rank_curves"}:
                assert new["active_rule_overrides"][key] == value, (new_name, key)
        assert new["pending_level_power_coefficient"] is None
        assert new["runtime_integration"] == "not_integrated"
    old_curves = load("skill-rank-curves-provisional-01.json")
    curves = load("skill-rank-curves-provisional-02.json")
    assert curves["skills"] == old_curves["skills"]
    assert curves["total_skill_count"] == 94
    level = load("review-current-level-skill-power-02.json")
    assert level["coefficient_approval"].startswith("accepted_as_initial_provisional")
    for row in level["Braise_neutral_profile_examples"]:
        raw = Fraction(8 + 3 * (row["effective_rank"] - 1)) * Fraction(row["character_current_level"] + 9, 10)
        assert row["fire_damage_integer"] == int(raw), row
    level11 = {r["effective_rank"]: r["fire_damage_integer"] for r in level["Braise_neutral_profile_examples"] if r["character_current_level"] == 11}
    assert level11 == {1: 16, 10: 70}
    status = load("last-status-cast-power-acceptance-decisions.json")
    assert status["strongest_power_selection"] is False
    assert status["status_power_addition_by_reapplication"] is False
    proposal = load("review-persistent-power-snapshot-INTERACT-POWER01-01.json")
    assert proposal["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-103.json")["remaining_first_pass_total"] == 0
    lines = (ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md").read_text(encoding="utf-8").splitlines()
    assert not any(line.rstrip() != line for line in lines[-80:])
    assert not errors, errors
    report = {
        "date": "2026-10-09", "review_id": "SKILL-LEVEL01-STATUS02-CHECK01",
        "scope": "review_reference_integrity_and_arithmetic;not_engine_or_balance_tests",
        "files_checked": FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "Braise_level11_without_bonuses": level11,
        "rank_curves_94_parameters": "unchanged",
        "other_accepted_rules_catalogues_caps_and_existing_overrides": "preserved",
        "level_coefficient": "accepted_initial_provisional_curve",
        "latest_status_cast_power": "accepted_even_if_lower;duration_restart_preserved",
        "persistent_effect_snapshot": "proposal_only;not_accepted_or_runtime_integrated",
        "runtime_integration": "not_integrated"
    }
    out = BASE / "level-and-status-review-validation-01.json"
    assert not out.exists(), out
    out.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps({"files": len(FILES), "checked_hashes": len(checked), "errors": 0,
                      "Braise_level11": level11, "rank_curves": "94_unchanged",
                      "next": "INTERACT_POWER01_proposal", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
