"""Design-only receipts: capacity approval, direct variant rule, residue proposal."""
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
DECISION = "mine-capacity-and-variant-loss-acceptance-decisions.json"
CAPACITY = "review-mine-passive-capacity-loss-INTERACT-RANK03-02.json"
VARIANT = "review-mine-variant-access-loss-INTERACT-RANK04-01.json"
PROPOSAL = "review-mine-residue-access-loss-INTERACT-RANK05-01.json"
PAIRS = [("review-implementation-readiness-08.json", "review-implementation-readiness-09.json"),
         ("review-attribute-model-66.json", "review-attribute-model-67.json"),
         ("review-primary-catalogue-238.json", "review-primary-catalogue-239.json"),
         ("review-complete-skill-design-213.json", "review-complete-skill-design-214.json"),
         ("remaining-individual-skill-review-counts-107.json", "remaining-individual-skill-review-counts-108.json")]
NEW_FILES = [DECISION, CAPACITY, VARIANT, PROPOSAL] + [new for _, new in PAIRS]
COPY = """Pour les mines qui ont déjà explosé, je propose que perdre le passif nécessaire supprime aussi les zones concernées : nuages de poison si Mines de poison devient indisponible, zones de glace si Mines de glace devient indisponible, et toutes les zones laissées par vos mines si Résidu persistant devient indisponible.

Les statuts déjà appliqués par l'explosion, comme Empoisonnement ou Gel, continuent jusqu'à leur fin normale. Le Gel maintenu uniquement par une zone supprimée prend fin avec elle.

Cette suppression ne produit pas de nouvelle explosion ni de remboursement d'énergie. Elle concerne les zones de vos mines, sans supprimer les zones provenant d'autres sorts."""


def common(data):
    data["date"] = "2026-10-09"
    data["previous_pending_mine_passive_capacity_loss"] = deepcopy(data["pending_mine_passive_capacity_loss"])
    data["pending_mine_passive_capacity_loss"] = None
    data["accepted_mine_passive_capacity_loss"] = ref(CAPACITY)
    data["accepted_mine_variant_access_loss"] = ref(VARIANT)
    data["latest_mine_capacity_and_variant_decisions"] = ref(DECISION)
    data["pending_mine_residue_access_loss"] = ref(PROPOSAL)
    data["runtime_integration"] = "not_integrated"


def overrides(data):
    rules = data.setdefault("active_rule_overrides", {})
    rules["mine_passive_capacity_loss"] = {
        **ref(CAPACITY), "acceptance": ref(DECISION),
        "rule": "SM01_capacity_loss_uses_new_limit_immediately;remove_only_excess_own_placed_mines_oldest_first;armed_and_unarmed_count;no_explosion_or_energy_refund",
        "precedence": "SM00_creator_rank_loss_still_removes_all_own_creations"
    }
    rules["mine_variant_access_loss"] = {
        **ref(VARIANT), "direct_user_decision": ref(DECISION),
        "rule": "loss_of_SM02_SM03_or_SM05_access_removes_own_placed_mines_depending_on_lost_poison_cold_or_residue_passive;no_conversion_to_base_mine",
        "scope": "supporting_passive_unavailable;partial_rank_loss_with_passive_still_available_and_already_created_residue_zones_not_resolved",
        "creator": "SM00_remains_creator;this_rule_is_a_passive_dependency_cleanup_not_a_change_of_creator_identity"
    }


def main():
    capacity_source = load("review-mine-passive-capacity-loss-INTERACT-RANK03-01.json")
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-RANK03-RANK04-DECISIONS",
        "capacity": {"review_id": "INTERACT-RANK03-ACCEPT", "source_proposal": ref("review-mine-passive-capacity-loss-INTERACT-RANK03-01.json"),
                     "user_acceptance": "oui :)", "shown_rule": capacity_source["shown_copy_markdown"],
                     "approved_rule": capacity_source["proposed_rule"]},
        "variant": {"review_id": "INTERACT-RANK04-DIRECT", "context": "assistant announces review of already placed mines when the passive granting poison, cold or persistent effect is lost",
                    "user_directive": "si ces mines etaient des mines de poison , de froid ou un effet persistant, alors la mine disparait, elle n'est pas remplacée par une mine \"de base\"",
                    "approved_rule": "remove_mines_depending_on_lost_variant_or_persistent_effect_passive;do_not_replace_with_base_mine",
                    "not_inferred": "no_new_rule_for_partial_rank_loss_when_passive_remains_available_or_for_existing_residue_zones"},
        "approval_boundary": "capacity_proposal_accepted_and_direct_variant_loss_instruction_recorded;new_residue_zone_cleanup_still_proposed;no_new_RP_name_or_native_visual",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(capacity_source)
    accepted.update(revision=2, status="mine_passive_capacity_loss_explicitly_accepted;variant_and_other_cases_separate;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="design_explanation_accepted;no_new_skill_RP_or_name")
    accepted["previous_proposal"] = ref("review-mine-passive-capacity-loss-INTERACT-RANK03-01.json")
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(CAPACITY, accepted)
    save(VARIANT, {
        "date": "2026-10-09", "review_id": "INTERACT-RANK04", "revision": 1,
        "title": "Perte du passif autorisant une mine spéciale",
        "status": "direct_user_mechanical_instruction;not_runtime_integrated", "user_decision": ref(DECISION),
        "dependencies": [ref("review-invoked-creation-scope-INTERACT-RANK02-02.json"), ref(CAPACITY),
                         ref("review-equipment-granted-skills-GEAR-SK01-02.json")],
        "accepted_rule": {
            "condition": "supporting_passive_becomes_unavailable;no_personal_rank_and_no_remaining_eligible_equipment_grant",
            "SM02": "remove_own_placed_poison_mines_if_Mines_de_poison_access_is_lost",
            "SM03": "remove_own_placed_cold_mines_if_Mines_de_glace_access_is_lost",
            "SM05": "remove_own_placed_mines_configured_with_persistent_residue_if_Residue_persistant_access_is_lost",
            "replacement": "none;never_convert_removed_mine_to_base_fire_mine",
            "unrelated": "other_own_mines_without_lost_dependency_and_other_casters_creations_follow_existing_rules",
            "creator": "SM00_remains_mine_creator;supporting_passive_dependency_can_require_cleanup_without_becoming_creator"
        },
        "unshown_cases_pending": ["partial_passive_rank_loss_while_passive_still_available", "lost_SM04_radius_passive",
                                  "already_exploded_mine_residue_zone_cleanup", "exact_cleanup_event_order_and_native_feedback"],
        "mechanic_approval": "direct_user_instruction", "text_approval": "no_new_skill_RP_or_name",
        "visual_approval": "no_new_native_art_or_animation", "runtime_integration": "not_integrated"
    })
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-RANK05", "revision": 1,
        "title": "Perte de passif après l'explosion d'une mine",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref(VARIANT), ref("review-individual-description-SM05-02.json"),
                         ref("review-persistent-power-snapshot-INTERACT-POWER01-02.json"), ref("review-invoked-creation-scope-INTERACT-RANK02-02.json")],
        "scope": "SM00_created_residue_zones_when_supporting_variant_or_residue_passive_becomes_unavailable;not_partial_rank_loss_or_other_skills",
        "proposed_rule": {
            "SM02_access_loss": "remove_own_SM00_poison_residue_zones_depending_on_SM02",
            "SM03_access_loss": "remove_own_SM00_cold_residue_zones_depending_on_SM03",
            "SM05_access_loss": "remove_all_own_SM00_residue_zones_depending_on_SM05",
            "independent_explosion_status": "already_applied_explosion_Poisoned_or_Gel_instances_keep_ordinary_remaining_duration_and_existing_reapplication_removal_rules",
            "zone_only_Gel": "Gel_maintained_only_by_removed_zone_ends_under_existing_zone_Gel_lifetime_rule",
            "cleanup": "no_new_explosion_damage_or_energy_refund;do_not_remove_other_skill_zones_or_other_caster_zones",
            "no_restore": "does_not_restore_a_previous_status_instance_already_overwritten_by_later_successful_reapplication"
        },
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["partial_supporting_passive_rank_loss_while_still_available", "lost_SM04_radius_passive",
                              "merged_multi_source_Gel_provenance_and_full_turn_scheduler", "predeath_resurrection_marks_and_birth_sampling",
                              "native_removal_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_skill_RP_or_name",
        "visual_approval": "no_new_native_art_or_animation_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old, data = load(old_name), deepcopy(load(old_name))
        common(data)
        if old_name.startswith("review-implementation"):
            data.update(revision=9, status="mine_capacity_accepted_and_variant_removal_directed;post_explosion_residue_cleanup_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-RANK03"] = {"rule": "new_SM01_capacity_immediate;remove_oldest_excess_only;armed_and_unarmed_count;no_explosion_refund", "source": ref(CAPACITY)}
            data["resolved_interactions"]["INTERACT-RANK04"] = {"rule": "lost_poison_cold_or_persistent_effect_passive_removes_affected_placed_mines_without_base_replacement", "source": ref(VARIANT), "unshown": "partial_rank_loss_and_existing_residue_zones"}
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if s != "mine_passive_capacity_loss_proposed_under_INTERACT_RANK03;variant_passives_and_resurrection_marks_pending"]
                    group["remaining"].append("post_explosion_mine_residue_cleanup_proposed_under_INTERACT_RANK05;partial_passive_rank_loss_radius_and_resurrection_marks_pending")
            data["remaining_after_user_directions"][2] = "post_explosion_mine_residue_cleanup_and_partial_passive_rank_loss_radius_and_resurrection_mark_birth_cases"
            data["recommendation"] = "review_post_explosion_residue_cleanup_remaining_passive_and_mark_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            overrides(data)
            data.update(review_id="STAT-MODEL64", status="mine_capacity_and_direct_variant_cleanup_recorded;residue_cleanup_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-09.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            overrides(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-67.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-09.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "post_explosion_mine_residue_zones_on_variant_or_residue_access_loss_only"
            data["next_review_kind"] = "mine_residue_access_loss_INTERACT_RANK05"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-67.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-09.json")
            data["latest_feedback"] = "user_accepts_RANK03_and_directs_variant_mine_removal;post_explosion_residue_cleanup_proposed"
            data["next_review_kind"] = "mine_residue_access_loss_INTERACT_RANK05"
        preserve(old, data)
        save(new_name, data)
    print(json.dumps({"accepted": "INTERACT-RANK03", "directed": "INTERACT-RANK04", "proposed": "INTERACT-RANK05", "runtime": "not_integrated"}))


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
        assert new["pending_mine_passive_capacity_loss"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load(CAPACITY)["accepted_rule"] == load("review-mine-passive-capacity-loss-INTERACT-RANK03-01.json")["proposed_rule"]
    assert load(CAPACITY)["mechanic_approval"] == "accepted_as_displayed"
    assert load(VARIANT)["mechanic_approval"] == "direct_user_instruction"
    assert load(VARIANT)["accepted_rule"]["replacement"] == "none;never_convert_removed_mine_to_base_fire_mine"
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-108.json")["remaining_first_pass_total"] == 0
    tail = (ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md").read_text(encoding="utf-8").splitlines()[-100:]
    assert not any(line.rstrip() != line for line in tail)
    assert not errors, errors
    save("mine-capacity-variant-and-residue-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-RANK03-RANK04-RANK05-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "RANK03": "accepted", "RANK04": "direct_user_instruction", "RANK05": "proposal_only",
        "partial_passive_rank_loss": "not_inferred", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "capacity": "accepted", "variant_mines": "direct_user_instruction", "residue": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    check() if sys.argv[1:] == ["--check"] else main()
