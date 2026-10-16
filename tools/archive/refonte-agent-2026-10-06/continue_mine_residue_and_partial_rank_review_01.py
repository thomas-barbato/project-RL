"""Design-only acceptance of residue cleanup and proposal for partial rank loss."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy
import sys

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
DECISION = "mine-residue-access-loss-acceptance-decisions.json"
ACCEPTED = "review-mine-residue-access-loss-INTERACT-RANK05-02.json"
PROPOSAL = "review-mine-passive-rank-dependencies-INTERACT-RANK06-01.json"
PAIRS = [("review-implementation-readiness-09.json", "review-implementation-readiness-10.json"),
         ("review-attribute-model-67.json", "review-attribute-model-68.json"),
         ("review-primary-catalogue-239.json", "review-primary-catalogue-240.json"),
         ("review-complete-skill-design-214.json", "review-complete-skill-design-215.json"),
         ("remaining-individual-skill-review-counts-108.json", "remaining-individual-skill-review-counts-109.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """Pour terminer les règles des passifs de mines, je propose de supprimer les créations concernées à chaque baisse de leur rang effectif, même si le passif reste disponible :

- Mines de poison : mines et nuages de poison.
- Mines de glace : mines et zones de glace.
- Résidu persistant : mines configurées pour laisser une zone et zones déjà créées.
- Explosion étendue : mines et zones utilisant son rayon amélioré.

Exemple : Mines de poison passe du rang 12 au rang 10 ; vos mines et nuages de poison existants disparaissent. Vos mines de feu ou de glace ne sont pas concernées par cette seule baisse.

La suppression ne provoque aucune explosion ni remboursement d'énergie. Réseau de mines garde sa règle validée : seule la capacité change et les mines en trop disparaissent. Les statuts déjà appliqués suivent les règles de durée précédemment validées."""


def common(data):
    data["date"] = "2026-10-09"
    data["previous_pending_mine_residue_access_loss"] = deepcopy(data["pending_mine_residue_access_loss"])
    data["pending_mine_residue_access_loss"] = None
    data["accepted_mine_residue_access_loss"] = ref(ACCEPTED)
    data["latest_mine_residue_acceptance"] = ref(DECISION)
    data["pending_mine_passive_dependency_rank_loss"] = ref(PROPOSAL)
    data["runtime_integration"] = "not_integrated"


def override(data):
    data.setdefault("active_rule_overrides", {})["mine_residue_access_loss"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "rule": "loss_of_SM02_access_removes_own_SM00_poison_residue_zones;loss_of_SM03_access_removes_own_SM00_cold_residue_zones;loss_of_SM05_access_removes_all_own_SM00_residue_zones",
        "statuses": "independent_explosion_Poisoned_or_Gel_keeps_ordinary_remaining_duration;Gel_maintained_only_by_removed_zone_ends;do_not_restore_previously_overwritten_status",
        "cleanup": "no_new_explosion_damage_or_energy_refund;other_skill_and_other_caster_zones_retained",
        "scope": "supporting_passive_unavailable;partial_rank_loss_and_SM04_radius_passive_proposed_separately_under_INTERACT_RANK06"
    }


def main():
    source = load("review-mine-residue-access-loss-INTERACT-RANK05-01.json")
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-RANK05-ACCEPT",
        "source_proposal": ref("review-mine-residue-access-loss-INTERACT-RANK05-01.json"),
        "user_acceptance": "parfait :) on continue", "shown_explanation": source["shown_copy_markdown"],
        "approved_rule": source["proposed_rule"],
        "approval_boundary": "shown_post_explosion_residue_zone_cleanup_on_loss_of_required_passive_access_and_status_duration_distinction;partial_rank_loss_radius_and_merged_status_provenance_not_newly_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="mine_residue_access_loss_explicitly_accepted;partial_rank_radius_and_provenance_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="design_explanation_accepted;no_new_skill_RP_or_name")
    accepted["previous_proposal"] = ref("review-mine-residue-access-loss-INTERACT-RANK05-01.json")
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-RANK06", "revision": 1,
        "title": "Baisse du rang d'un passif utilisé par une mine",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref("review-mine-variant-access-loss-INTERACT-RANK04-01.json"), ref(ACCEPTED),
                         ref("review-mine-passive-capacity-loss-INTERACT-RANK03-02.json"),
                         ref("review-individual-description-SM04-02.json"), ref("review-invoked-creation-scope-INTERACT-RANK02-02.json")],
        "scope": "any_effective_rank_decrease_of_SM02_SM03_SM04_or_SM05_affecting_own_SM00_mine_or_residue_creation_that_used_this_passive;creator_identity_not_changed",
        "already_accepted": "SM02_SM03_SM05_unavailability_removes_dependent_mines_and_residue_zones_without_base_conversion;SM01_capacity_only_removes_oldest_excess;SM00_creator_rank_loss_removes_all_its_own_creations",
        "proposed_rule": {
            "condition": "any_effective_rank_decrease_even_if_passive_remains_learned_or_gear_available;not_conditioned_on_complete_access_loss_or_actual_numeric_breakpoint_change",
            "SM02": "remove_own_SM00_poison_mines_and_poison_residue_zones_that_used_SM02",
            "SM03": "remove_own_SM00_cold_mines_and_cold_residue_zones_that_used_SM03",
            "SM05": "remove_own_SM00_mines_configured_with_persistent_residue_and_own_existing_residue_zones_that_used_SM05",
            "SM04": "remove_own_SM00_mines_and_residue_zones_that_used_SM04_radius_modifier;includes_final_SM04_access_loss",
            "selection": "only_creations_using_affected_passive;unrelated_mines_zones_other_skills_and_other_casters_retained",
            "cleanup": "no_explosion_new_damage_or_energy_refund;never_convert_removed_mine_to_base_variant",
            "statuses": "preserve_accepted_explosion_status_remaining_duration_and_removed_zone_only_Gel_expiry_rules;no_global_target_status_purge",
            "SM01_exception": "capacity_passive_keeps_accepted_new_limit_and_oldest_excess_only_rule",
            "creator_rule": "SM00_remains_creator;its_own_rank_loss_still_removes_all_own_creations_even_when_a_passive_is_unchanged"
        },
        "shown_example": {"affected_passive": "SM02", "old_effective_rank": 12, "new_effective_rank": 10,
                          "passive_still_available": True, "remove": ["own_poison_mines", "own_poison_residue_zones"],
                          "retain_under_this_single_passive_change": ["own_fire_mines", "own_cold_mines"],
                          "numbers_are_rank_illustration_not_new_damage_balance": True},
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["mine_power_sampling_on_rank_increase_or_caster_level_attribute_change", "merged_multi_source_Gel_provenance",
                              "predeath_resurrection_mark_cleanup_and_birth_sampling", "full_turn_and_equipment_transaction_event_order",
                              "native_removal_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_skill_RP_or_name",
        "visual_approval": "no_new_native_art_or_animation_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        common(data)
        if old_name.startswith("review-implementation"):
            data.update(revision=10, status="mine_residue_access_cleanup_accepted;partial_passive_rank_and_radius_cleanup_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-RANK05"] = {
                "rule": "loss_of_required_variant_or_residue_passive_removes_dependent_own_mine_residue_zones;independent_explosion_status_keeps_duration;zone_only_Gel_ends",
                "source": ref(ACCEPTED), "unshown": "partial_rank_loss_SM04_and_merged_source_provenance"
            }
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if s != "post_explosion_mine_residue_cleanup_proposed_under_INTERACT_RANK05;partial_passive_rank_loss_radius_and_resurrection_marks_pending"]
                    group["remaining"].append("partial_mine_passive_rank_and_radius_cleanup_proposed_under_INTERACT_RANK06;resurrection_marks_and_remaining_provenance_pending")
            data["remaining_after_user_directions"][2] = "partial_mine_passive_rank_radius_cleanup_and_resurrection_mark_birth_cases"
            data["recommendation"] = "review_partial_mine_passive_rank_radius_cleanup_remaining_mine_sampling_and_mark_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            override(data)
            data.update(review_id="STAT-MODEL65", status="mine_residue_cleanup_accepted;partial_passive_rank_cleanup_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-10.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            override(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-68.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-10.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "partial_effective_rank_loss_of_mine_variant_residue_or_radius_passives;SM01_capacity_exception_preserved"
            data["next_review_kind"] = "mine_passive_dependency_rank_loss_INTERACT_RANK06"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-68.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-10.json")
            data["latest_feedback"] = "user_accepts_RANK05;partial_mine_passive_rank_and_radius_cleanup_proposed"
            data["next_review_kind"] = "mine_passive_dependency_rank_loss_INTERACT_RANK06"
        preserve(old, data)
        save(new_name, data)
    print(json.dumps({"accepted": "INTERACT-RANK05", "proposed": "INTERACT-RANK06", "runtime": "not_integrated"}))


def check():
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
                            errors.append((context, "missing", source))
                            continue
                    checked.append((context, key))
                    if hashlib.sha256(content.encode("utf-8")).hexdigest() != expected:
                        errors.append((context, key, source[:100]))
            for key, nested in value.items():
                walk(nested, context + "/" + key)
        elif isinstance(value, list):
            for i, nested in enumerate(value):
                walk(nested, context + "/" + str(i))
    for name in NEW_FILES:
        walk(load(name), name)
    for old_name, new_name in PAIRS:
        old, new = load(old_name), load(new_name)
        preserve(old, new)
        for key, value in old.get("active_rule_overrides", {}).items():
            assert new["active_rule_overrides"][key] == value
        assert new["pending_mine_residue_access_loss"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load("review-mine-residue-access-loss-INTERACT-RANK05-01.json")["proposed_rule"]
    assert load(ACCEPTED)["mechanic_approval"] == "accepted_as_displayed"
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-109.json")["remaining_first_pass_total"] == 0
    tail = (ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md").read_text(encoding="utf-8").splitlines()[-85:]
    assert not any(line.rstrip() != line for line in tail)
    assert not errors, errors
    save("mine-residue-and-partial-rank-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-RANK05-RANK06-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved", "RANK05": "accepted", "RANK06": "proposal_only",
        "SM01_capacity_exception": "preserved", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "residue": "accepted", "partial_passive_rank": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    check() if sys.argv[1:] == ["--check"] else main()
