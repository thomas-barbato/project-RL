"""Record the HUD request separately from the proposed shared combat phase."""
from copy import deepcopy
from pathlib import Path
import hashlib
import importlib.util
import json

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
PREFIX = "artifacts/refonte-agent-2026-10-06/"
DECISIONS = PREFIX + "hostiles-calm-hud-decisions.json"
PROTOTYPE = "C:/Users/User/.codex/visualizations/2026/10/06/01a11010-433f-7fc2-ac29-ffb696dec5ea/hostiles-et-calme.html"
spec = importlib.util.spec_from_file_location("drone_review", OUT / "build_drone_persistence_review.py")
review = importlib.util.module_from_spec(spec)
spec.loader.exec_module(review)


def read(path):
    return (ROOT / path).read_text(encoding="utf-8")


def digest(path):
    return hashlib.sha256(read(path).encode("utf-8")).hexdigest()


def load(name):
    return json.loads((OUT / name).read_text(encoding="utf-8"))


def save(name, value):
    target = OUT / name
    assert not target.exists(), name
    target.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def main():
    plans = [
        ("review-interface-effects-01.json", "review-interface-effects-02.json", "review-interface-effects-revision-01.md", {"HUD02", "HUD03", "HUD04"}),
        ("review-drone-descriptions-02.json", "review-drone-descriptions-03.json", "review-drone-descriptions-revision-02.md", {"DP01"}),
        ("review-lot-01-revision-04.json", "review-lot-01-revision-05.json", "review-lot-01-revision-04.md", {"E01"})]
    assert all(not (OUT / name).exists() for name in [DECISIONS.split("/")[-1], *[plan[1] for plan in plans]])
    for old_name, _, snapshot, _ in plans:
        assert digest(PREFIX + snapshot) == load(old_name)["document_sha256"]
    checks = load("hostiles-calm-preview-checks.json")
    assert checks["status"] == "passed_in_schematic_preview_not_game_runtime"
    assert digest(PREFIX + "cadrage-before-combat-phase-hud.md") == load("drone-persistence-acceptance-decisions.json")["cadrage_sha256"]
    save(DECISIONS.split("/")[-1], {
        "date": "2026-10-06", "runtime_integration": "not_integrated",
        "user_request": "je veux bien que l'on fasse cette notion de calme mais alors il faudrait indiquer sur l'UX s'il y a des hostiles dans la zone et si le jeu est en phase de combat ou de calme",
        "user_scope_reply": "1 et si on place sa souris sur le status d'hostile repéré, ca met en surbrillance tous les hostiles visibles",
        "accepted_requirements": {"calm_concept_requires_visible_ux": True, "show_currently_observed_hostiles": True,
                                  "omniscient_map_enemy_count": False, "show_combat_or_calm_phase": True,
                                  "hover_hostile_status_highlights_all_visible_hostiles": True},
        "approval_scope": "requested interface capabilities; not approval of prior DP01 text, percentages, delay or new shared phase rules",
        "pending_proposals": ["exact player labels", "numeric visible-hostile counter", "three-phase presentation with three-turn countdown",
                              "shared agent and deployed-drone phase", "same phase for energy and Maintenance",
                              "incoming misses and persistent damage reset common calm", "initial phase and layer-transition rules",
                              "keyboard and touch equivalents", "colors, corner highlights and HUD placement", "native gameplay rendering"],
        "prototype": PROTOTYPE, "prototype_sha256": digest(PROTOTYPE),
        "prototype_checks": PREFIX + "hostiles-calm-preview-checks.json",
        "prototype_checks_sha256": digest(PREFIX + "hostiles-calm-preview-checks.json"),
        "cadrage_document": "docs/REFONTE_AGENT_CRYOGENISE.md", "cadrage_sha256": digest("docs/REFONTE_AGENT_CRYOGENISE.md"),
        "previous_cadrage_snapshot": PREFIX + "cadrage-before-combat-phase-hud.md",
        "previous_cadrage_sha256": digest(PREFIX + "cadrage-before-combat-phase-hud.md"),
        "test_menu_timing": "after all skills and animations; this HUD preview is not that menu"
    })
    for old_name, new_name, snapshot, allowed_changes in plans:
        previous = load(old_name)
        current = deepcopy(previous)
        source = read(previous["document"])
        prior = {row["id"]: row for row in previous["blocks"]}
        rows = []
        for hashes in review.sections(source):
            identifier = hashes["id"]
            if identifier in prior:
                row = deepcopy(prior[identifier])
                if hashes["block_sha256"] != row["block_sha256"]:
                    assert identifier in allowed_changes, identifier
                    row["revision"] += 1
                    for key in ["block_sha256", "text_sha256", "mechanic_sha256"]:
                        if key != "mechanic_sha256" or row.get(key) is not None:
                            row[key] = hashes[key]
                    row["shared_phase_approval"] = "new_common_phase_proposal_pending_user_review"
            else:
                assert identifier in allowed_changes, identifier
                row = {**hashes, "revision": 1, "role": "interface",
                       "runtime_integration": "not_integrated", "text_approval": "pending_user_review",
                       "mechanic_approval": "requested_interface_capability_accepted_other_details_pending" if identifier in ["HUD02", "HUD03"] else "proposed_shared_phase_effects_pending_user_review",
                       "visual_intention_approval": "requested_visible_hostile_highlight_accepted_shape_and_color_pending" if identifier == "HUD02" else "proposed_pending_user_review",
                       "visual_approval": "pending_native_preview_and_user_review"}
            rows.append(row)
        assert set(prior).issubset({row["id"] for row in rows})
        current.update(batch_revision=previous["batch_revision"] + 1,
                       document_sha256=digest(previous["document"]), blocks=rows,
                       previous_manifest=PREFIX + old_name, previous_document_snapshot=PREFIX + snapshot,
                       latest_decisions=DECISIONS, latest_decisions_sha256=digest(DECISIONS),
                       status="hostile_and_phase_ui_requested_shared_rules_and_render_pending")
        if new_name == "review-interface-effects-02.json":
            current["accepted_requirements"].update(load(DECISIONS.split("/")[-1])["accepted_requirements"])
            current.update(prototype=PROTOTYPE, prototype_sha256=digest(PROTOTYPE),
                           prototype_checks=PREFIX + "hostiles-calm-preview-checks.json",
                           prototype_checks_sha256=digest(PREFIX + "hostiles-calm-preview-checks.json"))
        save(new_name, current)
    print(json.dumps({"updated_manifests": 3, "requested_highlight_scope": "currently_visible_hostiles_only",
                      "proposed_common_phase": "agent_and_deployed_drones", "engine_changes": 0,
                      "preview_checks": "passed_at_736_and_320px"}))


if __name__ == "__main__":
    main()
