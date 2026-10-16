"""Design only: accept Contagion transfer rules and propose zone-only Gel lifetime."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
SOURCE = "review-contagion-transfer-rules-INTERACT-CONTAGION01-01.json"
DECISION = "contagion-transfer-rules-acceptance-decisions.json"
ACCEPTED = "review-contagion-transfer-rules-INTERACT-CONTAGION01-02.json"
PROPOSAL = "review-contagion-zone-gel-INTERACT-CONTAGION02-01.json"
PAIRS = [("review-implementation-readiness-21.json", "review-implementation-readiness-22.json"),
         ("review-attribute-model-79.json", "review-attribute-model-80.json"),
         ("review-primary-catalogue-251.json", "review-primary-catalogue-252.json"),
         ("review-complete-skill-design-226.json", "review-complete-skill-design-227.json"),
         ("remaining-individual-skill-review-counts-120.json", "remaining-individual-skill-review-counts-121.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """INTERACT-CONTAGION02 · Gel entretenu par une zone — proposition

Si Contagion transmet un Gel uniquement entretenu par une zone :

- La nouvelle cible peut être gelée même hors de la zone. Ses déplacements sont bloqués ; elle peut toujours attaquer et utiliser ses compétences.
- La durée de référence correspond au temps restant de la zone, limité à 5 tours. Contagion réduit ensuite cette durée selon son taux de conservation, arrondie à l'entier inférieur, avec un minimum de 1 tour si la transmission réussit.
- Ce Gel a ensuite son propre compte à rebours. La disparition de la zone d'origine ne le retire pas. Une Brûlure réussie le supprime, comme les autres Gels.

Exemple avec Contagion au rang effectif 11 : conservation de 80 %. S'il reste 8 tours à la zone, la durée de référence est limitée à 5 : le Gel transmis dure 4 tours.

La zone reste en place. Les résistances, les malus des élites et des boss, ainsi que l'interdiction de retransmettre un effet reçu par Contagion sont conservés.

Cette durée de référence plafonnée et son autonomie restent proposées, en attente de validation."""


def overrides(data):
    data.setdefault("active_rule_overrides", {})["contagion_transfer_rules"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "source": "latest_directly_owned_compatible_active_status_before_fatal_damage;received_via_Contagion_ineligible",
        "recipient": "closest_visible_enemy_in_AF06_range_with_unblocked_effect_path;one_attempt_no_fallback",
        "application": "immediate_with_retained_power_duration_and_current_recipient_defenses;Poison_Burn_Bleed_first_damage_counted_in_total;no_later_same_turn_automatic_same_kind_damage",
        "same_kind_and_no_chain": "latest_successful_transmission_replaces_same_kind_power_duration;received_instance_cannot_be_retransmitted",
        "zones": "remain_in_place;zone_only_Gel_lifetime_proposed_separately_under_INTERACT_CONTAGION02",
        "pending": "exact_geometry_ties_full_phase_order_and_unshown_retention_bindings;individual_tooltip_and_native_feedback"
    }


def validate():
    checked, errors = [], []
    def walk(value):
        if isinstance(value, dict):
            for key, expected in value.items():
                if key.endswith("_sha256") and key != "calculations_sha256":
                    source = value.get(key[:-7], value.get(key[:-7] + "_markdown"))
                    if not isinstance(source, str):
                        continue
                    content = source
                    if len(source) < 240 and "\n" not in source:
                        path = ROOT / source
                        if path.is_file():
                            content = path.read_text(encoding="utf-8")
                        elif source.startswith(("artifacts/", "docs/")):
                            errors.append(("missing", source))
                            continue
                    checked.append(key)
                    if hashlib.sha256(content.encode("utf-8")).hexdigest() != expected:
                        errors.append((key, source[:100]))
            for nested in value.values():
                walk(nested)
        elif isinstance(value, list):
            for nested in value:
                walk(nested)
    for name in NEW_FILES:
        walk(load(name))
    for old_name, new_name in PAIRS:
        old, new = load(old_name), load(new_name)
        preserve(old, new)
        for key, value in old.get("active_rule_overrides", {}).items():
            assert new["active_rule_overrides"][key] == value
        assert new["pending_contagion_transfer_rules"] is None
        assert new["pending_contagion_zone_gel"] == ref(PROPOSAL)
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load(SOURCE)["proposed_rule"]
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-121.json")["remaining_first_pass_total"] == 0
    duration = lambda remaining, percent: min(remaining, max(1, min(remaining, 5) * percent // 100))
    assert duration(8, 80) == 4
    assert duration(2, 80) == 1
    assert duration(1, 50) == 1
    assert not errors, errors
    return len(checked)


def main():
    source = load(SOURCE)
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-CONTAGION01-ACCEPT",
        "source_proposal": ref(SOURCE), "user_acceptance": "parfait on continue",
        "shown_explanation": source["shown_copy_markdown"], "approved_rule": source["proposed_rule"],
        "approval_boundary": "source_pre_fatal_selection_recipient_single_attempt_immediate_application_same_kind_replacement_and_no_retransmission_as_shown;zone_only_Gel_lifetime_and_unshown_full_retention_bindings_or_native_art_not_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="Contagion_transfer_rules_explicitly_accepted;zone_only_Gel_and_other_unshown_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="shown_design_explanation_accepted;individual_skill_copy_not_newly_approved")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-CONTAGION02", "revision": 1,
        "title": "Contagion : durée du Gel entretenu par une zone",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref(ACCEPTED), ref("review-contagion-compatibility-AF06-COMPAT02-01.json"),
                         ref("review-individual-description-SM05-02.json"), ref("review-mine-residue-access-loss-INTERACT-RANK05-02.json"),
                         ref("skill-rank-curves-provisional-02.json")],
        "scope": "only_transfer_lifetime_for_selected_Gel_solely_maintained_by_a_still_active_owned_zone;ordinary_timed_Gel_unchanged",
        "proposed_rule": {
            "eligibility": "accepted_INTERACT_CONTAGION01_selection;source_Gel_still_active_in_prefatal_snapshot_and_solely_maintained_by_selected_source_zone",
            "reference_duration": "min(remaining_source_zone_turns_at_transfer,5);source_zone_must_still_be_active_with_positive_remaining_time",
            "transmitted_duration": "min(remaining_source_zone_turns,max(1,floor(reference_duration*current_AF06_retained_percent/100)))_on_success",
            "duration_basis": "new_five_turn_cap_for_zone_only_transfer_proposed;AF06_current_provisional_retention_curve_unchanged",
            "recipient": "Gel_can_apply_outside_zone;blocks_movement_not_attacks_or_other_actions;existing_current_defenses_and_importance_chance_modifiers_preserved",
            "independent_lifetime": "successful_transferred_Gel_has_own_timer;original_zone_expiry_or_removal_does_not_cancel_this_received_status;successful_Burn_removes_Gel_as_already_accepted",
            "preserved": "zone_not_moved_or_copied;one_status_per_death;received_Contagion_instance_cannot_be_retransmitted;direct_Gel_and_source_zone_only_Gel_exit_rules_unchanged"
        },
        "example": {"Contagion_effective_rank": 11, "retained_percent": 80, "remaining_zone_turns": 8,
                    "capped_reference_turns": 5, "transmitted_Gel_turns": 4, "scope": "illustrates_proposed_duration_binding;not_chance_damage_or_final_balance"},
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["multiple_Gel_source_provenance_and_latest_successful_source_reconciliation",
                              "complete_turn_phase_order", "unshown_other_status_retention_bindings", "modified_individual_tooltip_and_native_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;name_and_RP_unchanged",
        "visual_approval": "no_new_native_rendering_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data.update(date="2026-10-09", runtime_integration="not_integrated")
        data["previous_pending_contagion_transfer_rules"] = deepcopy(data["pending_contagion_transfer_rules"])
        data["pending_contagion_transfer_rules"] = None
        data["accepted_contagion_transfer_rules"] = ref(ACCEPTED)
        data["latest_contagion_transfer_acceptance"] = ref(DECISION)
        data["pending_contagion_zone_gel"] = ref(PROPOSAL)
        if old_name.startswith("review-implementation"):
            data.update(revision=22, status="Contagion_transfer_rules_accepted;zone_only_Gel_duration_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-CONTAGION01"] = {"source": ref(ACCEPTED),
                "rule": "latest_owned_direct_status_prefatal;closest_visible_valid_recipient;one_attempt;immediate_application;no_chain",
                "unshown": "zone_only_Gel_duration_and_other_unshown_retention_bindings"}
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if s != "Contagion_selection_first_application_and_no_chain_proposed_under_INTERACT_CONTAGION01"]
                    group["remaining"].append("Contagion_zone_only_Gel_duration_proposed_under_INTERACT_CONTAGION02;other_unshown_retention_bindings_geometry_ties_and_phase_order_pending")
            data["recommendation"] = "review_zone_only_Gel_transfer_and_remaining_core_turn_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            overrides(data)
            data.update(review_id="STAT-MODEL77", status="Contagion_transfer_rules_accepted;zone_only_Gel_duration_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-22.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            overrides(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-80.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-22.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "Contagion_zone_only_Gel_reference_duration_cap_retention_and_independent_lifetime"
            data["next_review_kind"] = "contagion_zone_gel_INTERACT_CONTAGION02"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-80.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-22.json")
            data["latest_feedback"] = "user_accepts_Contagion_transfer_rules;zone_only_Gel_duration_proposed"
            data["next_review_kind"] = "contagion_zone_gel_INTERACT_CONTAGION02"
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("contagion-zone-gel-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-CONTAGION01-CONTAGION02-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "CONTAGION01": "accepted", "CONTAGION02": "proposal_only", "duration_examples": "passed",
        "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": checked, "errors": 0,
                      "Contagion_transfer": "accepted", "zone_Gel": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
