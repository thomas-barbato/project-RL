"""Design only: accept refresh and expanded Contagion, propose transfer rules."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
SOURCE = "review-persistent-status-reapplication-INTERACT-TIME02-01.json"
DECISION = "persistent-status-reapplication-acceptance-decisions.json"
ACCEPTED = "review-persistent-status-reapplication-INTERACT-TIME02-02.json"
COMPAT_DECISION = "contagion-expanded-status-compatibility-user-decisions.json"
COMPAT = "review-contagion-compatibility-AF06-COMPAT02-01.json"
PROPOSAL = "review-contagion-transfer-rules-INTERACT-CONTAGION01-01.json"
PAIRS = [("review-implementation-readiness-20.json", "review-implementation-readiness-21.json"),
         ("review-attribute-model-78.json", "review-attribute-model-79.json"),
         ("review-primary-catalogue-250.json", "review-primary-catalogue-251.json"),
         ("review-complete-skill-design-225.json", "review-complete-skill-design-226.json"),
         ("remaining-individual-skill-review-counts-119.json", "remaining-individual-skill-review-counts-120.json")]
NEW_FILES = [DECISION, ACCEPTED, COMPAT_DECISION, COMPAT, PROPOSAL] + [new for _, new in PAIRS]
OLD_COMPAT = ["Interférence", "Faille d’armure", "Silence", "Rupture de régénération", "Désorientation"]
ADDED_COMPAT = ["Poison", "Brûlure", "Saignement", "Gel"]
COPY = """Contagion couvre désormais les cinq affaiblissements déjà validés, ainsi que Poison, Brûlure, Saignement et Gel. Un seul effet est transmis par mort.

INTERACT-CONTAGION01 · Sélection et transmission — proposition

- Contagion choisit le plus récent de vos effets compatibles directement appliqués, encore actif juste avant les dégâts mortels.
- Désorientation peut donc être transmise si le coup mortel l'interrompt. Si un coup précédent l'avait déjà supprimée, elle n'est plus disponible.
- La cible est l'ennemi visible le plus proche à portée, avec un trajet dégagé depuis la victime. Une seule tentative : si la transmission échoue, elle ne cherche pas une autre cible.
- L'effet transmis s'applique immédiatement, avec la durée et la puissance prévues par Contagion. Poison, Brûlure et Saignement produisent leur premier dégât immédiatement ; cette application compte dans le total transmis, sans autre application automatique ensuite pendant ce tour.
- Un effet reçu par Contagion ne peut pas être transmis à nouveau.

Si la cible porte déjà le même statut, la nouvelle application réussie le remplace avec sa puissance et sa durée transmises. Les résistances et règles propres à chaque contrôle restent conservées. Les zones restent à leur emplacement : elles ne sont pas transférées.

Ces précisions de sélection et de transmission restent à valider. Le cas d'un Gel uniquement maintenu par une zone sera précisé séparément."""


def overrides(data):
    rules = data.setdefault("active_rule_overrides", {})
    rules["persistent_status_reapplication"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "new_attack_or_cast": "successful_Poison_Burn_Bleed_reapplication_replaces_power_restarts_full_duration_and_immediately_deals_first_of_renewed_applications;no_later_same_turn_automatic_same_kind_damage",
        "existing_zone_refresh": "already_present_status_duration_and_stored_power_refreshed_without_extra_immediate_status_damage;ordinary_periodic_application_retained_under_single_kind_tick_accounting",
        "preserved": "own_damage_of_independent_zones_cumulative;failed_status_application_no_replace_refresh_or_status_damage;new_status_from_zone_uses_TIME01;complete_phase_order_pending"
    }
    rules["contagion_compatible_statuses"] = {
        **ref(COMPAT), "acceptance": ref(COMPAT_DECISION),
        "compatible_statuses": OLD_COMPAT + ADDED_COMPAT, "effects_transmitted_per_death": 1,
        "precedence": "extends_historical_five_status_compatibility_only;AF06_identity_name_RP_and_provisional_rank_curves_unchanged",
        "pending": "selection_source_eligibility_application_and_no_chain_rules_proposed_under_INTERACT_CONTAGION01;zone_only_Gel_and_exact_phase_order_pending"
    }


def main():
    source = load(SOURCE)
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-TIME02-ACCEPT",
        "source_proposal": ref(SOURCE), "user_acceptance": "parfait on continue",
        "shown_explanation": source["shown_copy_markdown"], "approved_rule": source["proposed_rule"],
        "approval_boundary": "new_attack_or_cast_immediate_status_reapplication_and_automatic_existing_zone_refresh_rules;Contagion_transfers_and_complete_phase_order_not_newly_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="persistent_status_reapplication_explicitly_accepted;nonattack_transfer_and_full_phase_order_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="shown_design_explanation_accepted;individual_skill_copy_not_newly_approved")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(COMPAT_DECISION, {
        "date": "2026-10-09", "review_id": "AF06-COMPAT02-ACCEPT", "skill_id": "AF06",
        "previous_compatibility": ref("seal-contagion-disorientation-compatibility-decisions.json"),
        "user_question": "ah oui ca ne transmet pas ces effets là ? ca me semble anormal, qu'en penses tu ?",
        "shown_proposal": "Ajouter Poison, Brûlure, Saignement et Gel créerait des synergies avec les autres branches. Je prépare cette extension avec une transmission limitée à un effet par mort.",
        "user_acceptance": "oui on va changer ca, ca me plait pas trop .",
        "accepted_compatibility": OLD_COMPAT + ADDED_COMPAT, "effects_transmitted_per_death": 1,
        "approval_boundary": "explicit_extension_by_four_named_statuses_and_shown_one_effect_per_death;no_implicit_approval_of_other_statuses_selection_no_chain_transfer_numbers_zone_only_Gel_or_native_art",
        "name_and_roleplay": "existing_user_approved_Contagion_name_and_RP_unchanged",
        "previous_TIME02_acceptance": ref(DECISION), "runtime_integration": "not_integrated"
    })
    save(COMPAT, {
        "date": "2026-10-09", "review_id": "AF06-COMPAT02", "revision": 1, "skill_id": "AF06", "title": "Contagion",
        "status": "expanded_compatibility_and_one_effect_per_death_explicitly_accepted;other_transfer_rules_pending;not_runtime_integrated",
        "acceptance": ref(COMPAT_DECISION), "existing_card": ref("review-individual-description-AF06-03.json"),
        "provisional_rank_curves": ref("skill-rank-curves-provisional-02.json"),
        "accepted_rule": {"compatible_statuses": OLD_COMPAT + ADDED_COMPAT, "added_statuses": ADDED_COMPAT,
                          "effects_transmitted_per_death": 1, "existing_name_RP_rank_curves_and_other_approvals": "preserved"},
        "mechanic_approval": "expanded_status_list_and_single_effect_per_death_accepted",
        "text_approval": "no_new_RP_or_individual_tooltip_copy_presented", "visual_approval": "no_new_native_rendering",
        "not_resolved_here": ["source_selection_targeting_and_transmission_failure", "no_chain_transmission",
                              "persistent_first_application_on_transfer", "zone_only_Gel_transfer_lifetime", "exact_phase_order"],
        "runtime_integration": "not_integrated"
    })
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-CONTAGION01", "revision": 1,
        "title": "Contagion : sélection et transmission",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref(COMPAT), ref(ACCEPTED), ref("review-timer-accounting-INTERACT-TIME01-03.json"),
                         ref("review-individual-description-AF06-03.json"), ref("skill-rank-curves-provisional-02.json")],
        "scope": "source_selection_pre_fatal_snapshot_target_selection_first_application_and_no_retransmission_for_existing_AF06_with_expanded_accepted_status_list",
        "already_accepted": "nine_named_compatible_statuses;one_effect_per_death;name_RP_and_delegated_provisional_numeric_curves;existing_same_kind_latest_power_and_each_status_defense_rules",
        "proposed_rule": {
            "source": "one_most_recent_successfully_applied_compatible_effect_owned_and_directly_applied_by_AF06_owner;must_be_active_immediately_before_fatal_damage;received_via_Contagion_instance_not_eligible",
            "Disorientation_exception": "pre_fatal_damage_active_Disorientation_may_be_selected_even_if_fatal_damage_interrupts_it;effect_previously_removed_by_nonfatal_damage_cannot_be_selected",
            "recipient": "closest_currently_visible_enemy_within_current_AF06_range_from_victim_and_unblocked_effect_path_from_victim;no_new_exact_distance_metric_or_tie_policy_here",
            "attempt": "one_recipient_one_transmission_attempt;failed_resistance_or_immunity_does_not_search_next_candidate;no_candidate_means_no_transfer",
            "application": "successful_transfer_applies_effect_immediately_with_AF06_retained_power_and_duration;new_recipient_current_defenses_and_importance_rules_preserved",
            "periodic": "transferred_Poison_Burn_Bleed_first_damage_immediate_counted_in_transmitted_application_total;no_later_same_turn_automatic_same_kind_damage",
            "same_kind": "successful_received_same_kind_effect_replaces_current_power_and_restarts_duration_using_new_transmitted_duration;no_additive_status_instances_or_restoration_of_old_effect",
            "no_chain": "instance_received_via_Contagion_cannot_be_retransmitted_by_Contagion_at_recipient_death",
            "zones": "transfer_actor_status_not_zone_or_invocation;source_zone_stays_in_place;zone_only_Gel_without_independent_lifetime_requires_separate_review",
            "scope_boundary": "no_new_retention_or_damage_numbers_no_new_compatible_status_and_no_new_RP_or_art;complete_phase_order_and_selection_geometry_ties_pending"
        },
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["zone_only_Gel_transfer_lifetime", "exact_distance_ties_and_line_of_sight_predicate",
                              "duration_and_power_retention_bindings_and_rounding_where_unshown", "complete_death_phase_order",
                              "modified_individual_tooltip_and_native_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;name_and_RP_unchanged",
        "visual_approval": "no_new_native_art_animation_or_UI_layout_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data["date"] = "2026-10-09"
        data["previous_pending_persistent_status_reapplication"] = deepcopy(data["pending_persistent_status_reapplication"])
        data["pending_persistent_status_reapplication"] = None
        data["accepted_persistent_status_reapplication"] = ref(ACCEPTED)
        data["latest_persistent_status_reapplication_acceptance"] = ref(DECISION)
        data["accepted_contagion_expanded_status_compatibility"] = ref(COMPAT)
        data["latest_contagion_compatibility_acceptance"] = ref(COMPAT_DECISION)
        data["pending_contagion_transfer_rules"] = ref(PROPOSAL)
        data["runtime_integration"] = "not_integrated"
        if old_name.startswith("review-implementation"):
            data.update(revision=21, status="TIME02_and_expanded_Contagion_compatibility_accepted;Contagion_transfer_rules_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(COMPAT_DECISION)
            data["resolved_interactions"]["INTERACT-TIME02"] = {"rule": "new_attack_or_cast_immediate_reapplication;existing_zone_refresh_no_extra_immediate_status_damage",
                                                                   "source": ref(ACCEPTED), "unshown": "non_attack_transfers_and_complete_phase_order"}
            data["resolved_interactions"]["AF06-COMPAT02"] = {"rule": "five_original_statuses_plus_Poison_Burn_Bleed_Gel;one_effect_per_death",
                                                               "source": ref(COMPAT), "unshown": "source_targeting_transmission_and_zone_only_Gel_rules"}
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if not s.startswith("existing_persistent_status_reapplication_vs_automatic_zone_refresh_proposed_under_INTERACT_TIME02")]
                    group["remaining"].extend(["Contagion_selection_first_application_and_no_chain_proposed_under_INTERACT_CONTAGION01",
                                               "ordinary_buff_and_cooldown_accounting_unshown_draft_not_accepted"])
            data["recommendation"] = "review_expanded_Contagion_transfer_rules_and_remaining_core_turn_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            overrides(data)
            data.update(review_id="STAT-MODEL76", status="TIME02_and_Contagion_compatibility_extension_accepted;transfer_rules_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-21.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            overrides(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-79.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-21.json")
            pair(data, "latest_decisions", COMPAT_DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "expanded_Contagion_source_target_selection_immediate_transfer_and_no_chain_rules"
            data["next_review_kind"] = "contagion_transfer_rules_INTERACT_CONTAGION01"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-79.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-21.json")
            data["latest_feedback"] = "user_accepts_TIME02_and_expands_Contagion_by_four_statuses;transfer_rules_proposed"
            data["next_review_kind"] = "contagion_transfer_rules_INTERACT_CONTAGION01"
        preserve(old, data)
        save(new_name, data)
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
        assert new["pending_persistent_status_reapplication"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load(SOURCE)["proposed_rule"]
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert len(load(COMPAT)["accepted_rule"]["compatible_statuses"]) == 9
    assert load("remaining-individual-skill-review-counts-120.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    save("refresh-and-contagion-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-TIME02-AF06-COMPAT02-CONTAGION01-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved", "TIME02": "accepted",
        "AF06_COMPAT02": "nine_statuses_and_one_effect_per_death_accepted", "CONTAGION01": "proposal_only",
        "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "status_refresh": "accepted", "compatible_statuses": 9, "Contagion_transfer": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
