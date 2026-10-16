"""Design-only unique mark acceptance and map/owner mark cleanup proposal."""
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
DECISION = "reanimation-mark-reapplications-acceptance-decisions.json"
ACCEPTED = "review-reanimation-mark-reapplications-INTERACT-REANIM03-02.json"
PROPOSAL = "review-reanimation-mark-map-and-owner-loss-INTERACT-REANIM04-01.json"
PAIRS = [("review-implementation-readiness-14.json", "review-implementation-readiness-15.json"),
         ("review-attribute-model-72.json", "review-attribute-model-73.json"),
         ("review-primary-catalogue-244.json", "review-primary-catalogue-245.json"),
         ("review-complete-skill-design-219.json", "review-complete-skill-design-220.json"),
         ("remaining-individual-skill-review-counts-113.json", "remaining-individual-skill-review-counts-114.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """Pour les marques de Réanimation encore en attente, je propose :

- Sur la même carte, s'éloigner ou perdre la cible de vue ne supprime pas la marque.
- Changer de carte annule vos marques laissées sur la carte quittée : descente, refuge ou portail démoniaque, à l'aller comme au retour. Revenir ne restaure pas ces marques.
- Si le lanceur meurt ou est supprimé, ses marques en attente sont annulées. Ce cas concernerait aussi un autre lanceur si ce pouvoir lui est attribué plus tard.

Cette règle concerne les marques avant la mort de leur cible. Les familiers déjà relevés gardent leurs règles validées de passage entre cartes, avec leur durée restante."""


def common(data):
    data["date"] = "2026-10-09"
    data["previous_pending_reanimation_mark_reapplications"] = deepcopy(data["pending_reanimation_mark_reapplications"])
    data["pending_reanimation_mark_reapplications"] = None
    data["accepted_reanimation_mark_reapplications"] = ref(ACCEPTED)
    data["latest_reanimation_mark_reapplications_acceptance"] = ref(DECISION)
    data["pending_reanimation_mark_map_and_owner_loss"] = ref(PROPOSAL)
    data["runtime_integration"] = "not_integrated"


def override(data):
    data.setdefault("active_rule_overrides", {})["reanimation_mark_reapplications"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "unique_mark": "one_live_OC07_mark_per_target_and_at_most_one_raised_from_target_death",
        "same_owner": "refuse_reapplication_without_energy_elapsed_turn_or_cooldown_change;existing_mark_retained",
        "other_owner": "if_other_caster_has_OC07_latest_successful_valid_mark_replaces_old_mark_owner;single_raised_for_remaining_owner_using_current_rank_at_birth",
        "raised_target": "already_reanimated_actor_ineligible_even_when_hostile;no_chain_reanimation",
        "replacement": "no_restoration_of_previous_mark_or_refund_of_previous_cast_costs",
        "scope": "no_new_monster_skill_assignment_faction_rule_or_UI_refusal_text;ordinary_other_status_reapplication_rules_unchanged"
    }


def main():
    source = load("review-reanimation-mark-reapplications-INTERACT-REANIM03-01.json")
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-REANIM03-ACCEPT",
        "source_proposal": ref("review-reanimation-mark-reapplications-INTERACT-REANIM03-01.json"),
        "user_acceptance": "parfait :)", "shown_explanation": source["shown_copy_markdown"],
        "approved_rule": source["proposed_rule"],
        "approval_boundary": "displayed_unique_mark_same_owner_refusal_conditional_other_owner_replacement_and_raised_target_exclusion;owner_loss_map_cleanup_and_native_feedback_not_newly_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="unique_reanimation_marks_and_reapplication_explicitly_accepted;owner_map_and_phase_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="design_explanation_accepted;no_new_skill_RP_name_or_UI_error_copy")
    accepted["previous_proposal"] = ref("review-reanimation-mark-reapplications-INTERACT-REANIM03-01.json")
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-REANIM04", "revision": 1,
        "title": "Marques de Réanimation : carte et disparition du lanceur",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref(ACCEPTED), ref("review-reanimation-mark-and-birth-INTERACT-REANIM01-02.json"),
                         ref("review-familiar-map-transitions-PETS-EQ18-02.json"), ref("review-pending-familiar-arrival-PETS-EQ19-02.json")],
        "scope": "pending_OC07_marks_before_target_death_when_owner_leaves_current_map_dies_or_is_removed;existing_raised_actor_transfer_not_changed",
        "already_accepted": "mark_until_target_death_unless_accepted_rank_loss_or_source_replacement_cleanup;living_familiars_transfer_with_current_state_and_remaining_lifetime;no_new_birth_or_refill_on_transfer",
        "proposed_rule": {
            "same_map": "normal_owner_distance_changes_or_loss_of_target_visibility_do_not_cancel_pending_mark_on_same_map",
            "owner_map_change": "on_successful_owner_map_departure_cancel_own_pending_OC07_marks_on_departure_map;includes_descent_refuge_entry_and_demonic_bonus_portal_entry_or_return",
            "return": "returning_to_departure_map_does_not_restore_cancelled_marks;ordinary_new_cast_can_mark_eligible_target_again",
            "owner_death_or_removal": "cancel_pending_marks_when_actual_owner_dies_or_is_removed_from_simulation;does_not_mean_invisibility_or_temporary_inability_to_cast",
            "source_selection": "only_marks_whose_current_owner_is_affected;do_not_restore_older_replaced_marks_or_remove_other_caster_marks",
            "existing_raised_map_transfer": "preserve_accepted_same_actor_familiar_transfer_waiting_arrival_current_reserves_real_items_and_remaining_lifetime",
            "scope_boundary": "marks_only;no_new_NPC_skill_assignment_player_respawn_or_rule_for_existing_NPC_raised_after_owner_death"
        },
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["target_changes_map_without_owner", "existing_nonplayer_raised_after_owner_death_if_future_monster_has_ability",
                              "exact_transition_death_and_birth_same_event_order", "native_marker_removal_and_inspection_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_skill_RP_name_or_UI_error_copy",
        "visual_approval": "no_new_native_art_or_animation_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        common(data)
        if old_name.startswith("review-implementation"):
            data.update(revision=15, status="unique_reanimation_marks_accepted;mark_owner_and_map_loss_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-REANIM03"] = {
                "rule": "one_mark_per_target;same_owner_duplicate_refused_without_cost_turn_or_cooldown_change;conditional_other_owner_latest_success_replaces;already_raised_ineligible",
                "source": ref(ACCEPTED), "unshown": "owner_loss_map_transition_and_exact_phase_order"
            }
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if s != "unique_reanimation_marks_and_reapplication_proposed_under_INTERACT_REANIM03;remaining_source_and_turn_cases_pending"]
                    group["remaining"].append("pending_reanimation_mark_map_and_owner_cleanup_proposed_under_INTERACT_REANIM04;remaining_source_and_turn_cases_pending")
            data["remaining_after_user_directions"][2] = "reanimation_mark_map_and_owner_cleanup_and_remaining_source_turn_cases"
            data["recommendation"] = "review_pending_reanimation_mark_map_owner_cleanup_and_remaining_turn_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            override(data)
            data.update(review_id="STAT-MODEL70", status="unique_reanimation_marks_accepted;mark_map_owner_cleanup_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-15.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            override(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-73.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-15.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "pending_OC07_marks_on_owner_map_change_death_or_removal;living_familiar_map_transfer_preserved"
            data["next_review_kind"] = "reanimation_mark_map_and_owner_loss_INTERACT_REANIM04"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-73.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-15.json")
            data["latest_feedback"] = "user_accepts_REANIM03;pending_mark_map_and_owner_cleanup_proposed"
            data["next_review_kind"] = "reanimation_mark_map_and_owner_loss_INTERACT_REANIM04"
        preserve(old, data)
        save(new_name, data)
    print(json.dumps({"accepted": "INTERACT-REANIM03", "proposed": "INTERACT-REANIM04", "runtime": "not_integrated"}))


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
        assert new["pending_reanimation_mark_reapplications"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load("review-reanimation-mark-reapplications-INTERACT-REANIM03-01.json")["proposed_rule"]
    assert load(ACCEPTED)["mechanic_approval"] == "accepted_as_displayed"
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-114.json")["remaining_first_pass_total"] == 0
    tail = (ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md").read_text(encoding="utf-8").splitlines()[-90:]
    assert not any(line.rstrip() != line for line in tail)
    assert not errors, errors
    save("reanimation-marks-and-map-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-REANIM03-REANIM04-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved", "REANIM03": "accepted", "REANIM04": "proposal_only",
        "living_familiar_map_transfer": "preserved", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "unique_marks": "accepted", "pending_mark_map_and_owner_loss": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    check() if sys.argv[1:] == ["--check"] else main()
