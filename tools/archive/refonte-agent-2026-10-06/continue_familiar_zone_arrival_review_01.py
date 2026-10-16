"""Design only: accept the displayed drone first action; propose arrival hazards."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
SOURCE = "review-familiar-first-action-arrival-INTERACT-PETS01-01.json"
DECISION = "drone-first-action-acceptance-decisions.json"
ACCEPTED = "review-familiar-first-action-arrival-INTERACT-PETS01-02.json"
PROPOSAL = "review-familiar-zone-arrival-INTERACT-PETS02-01.json"
PAIRS = [("review-implementation-readiness-28.json", "review-implementation-readiness-29.json"),
         ("review-attribute-model-86.json", "review-attribute-model-87.json"),
         ("review-primary-catalogue-258.json", "review-primary-catalogue-259.json"),
         ("review-complete-skill-design-233.json", "review-complete-skill-design-234.json"),
         ("remaining-individual-skill-review-counts-127.json", "remaining-individual-skill-review-counts-128.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """INTERACT-PETS02 · Arrivée d'un familier dans une zone — proposition

- Invocation, résurrection ou arrivée sur une carte : si le familier apparaît dans une zone qui peut l'affecter, ses effets s'appliquent dès son arrivée, avant sa première action.
- Repositionnement : arriver dans une nouvelle zone suit la même règle. Se déplacer à l'intérieur d'une zone où le familier se trouve déjà ne provoque pas une application supplémentaire.
- La limite reste d'une application par zone, par cible et par tour. Les règles de cible et de dégâts alliés restent propres à chaque compétence.
- Si le familier meurt à l'arrivée, il ne peut pas agir ; ses objets réels tombent sur sa case, comme validé.

Exemple : vous invoquez un drone dans une zone de poison ennemie qui peut l'affecter. Il subit ses effets dès son apparition. S'il survit et peut agir, il joue ensuite pendant la phase des familiers.

Un familier en attente rejoint la carte après votre déplacement effectué, avant les effets périodiques et les actions des familiers, selon l'ordre et les places disponibles déjà validés. Un familier existant conserve son cycle d'action et ses recharges : l'arrivée ne lui donne pas d'action supplémentaire.

Les monstres ressuscités gardent leur première action au tour suivant leur relèvement. Les familiers ne déclenchent pas vos mines. Leur état réel, leurs ressources à la naissance et les durées déjà validées restent conservés.

Cette proposition reste à valider."""


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
        assert new["pending_familiar_first_action_arrival"] is None
        assert new["pending_familiar_zone_arrival"] == ref(PROPOSAL)
        assert "familiar_zone_arrival" not in new.get("active_rule_overrides", {})
        assert new["runtime_integration"] == "not_integrated"
    accepted, source = load(ACCEPTED), load(SOURCE)
    assert accepted["accepted_rule"] == {"new_drone": source["proposed_rule"]["new_drone"]}
    assert load(PROPOSAL)["proposed_rule"] == {key: value for key, value in source["proposed_rule"].items() if key != "new_drone"}
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-128.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    return len(checked)


def main():
    source = load(SOURCE)
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-PETS01-ACCEPT",
        "source_proposal": ref(SOURCE), "user_acceptance": "ok on continue :) .",
        "displayed_acceptance_anchor": "La première action des drones pendant leur tour d’invocation reste une proposition à valider.",
        "preceding_displayed_explanation": "Un drone pouvant agir dès son invocation donnerait aussi un intérêt immédiat à son déploiement.",
        "approved_rule": {"new_drone": source["proposed_rule"]["new_drone"]},
        "approval_boundary": "displayed_new_drone_first_action_in_normal_familiar_phase_only;full_unshown_zone_arrival_and_phase_details_not_inferred_accepted",
        "preserved": "already_accepted_raised_first_action_next_birth_turn;normal_actor_speeds_budgets_and_conditions;familiars_before_enemies",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="displayed_drone_first_action_accepted_only;remaining_arrival_details_proposed_under_PETS02;not_runtime_integrated",
                    mechanic_approval="partial_acceptance_new_drone_only", text_approval="displayed_first_action_explanation_accepted;unshown_full_copy_not_accepted")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = {"new_drone": accepted["proposed_rule"].pop("new_drone")}
    accepted["unapproved_or_already_accepted_reminders_from_source"] = accepted.pop("proposed_rule")
    accepted["shown_copy_approval_boundary"] = "historical_prepared_full_copy_retained_as_source;only_displayed_drone_first_action_accepted"
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-PETS02", "revision": 1,
        "title": "Arrivée d'un familier dans une zone", "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "previous_combined_proposal": ref(SOURCE), "first_action_acceptance": ref(DECISION),
        "dependencies": source["dependencies"] + [ref(ACCEPTED)],
        "already_accepted": source["already_accepted"],
        "proposed_rule": deepcopy(accepted["unapproved_or_already_accepted_reminders_from_source"]),
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "shown_example": "eligible_enemy_poison_zone_affects_new_drone_on_appearance;survivor_acts_later_in_normal_familiar_phase",
        "not_resolved_here": source["not_resolved_here"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_RP_or_name",
        "visual_approval": "no_new_native_rendering_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old, data = load(old_name), deepcopy(load(old_name))
        data.update(date="2026-10-09", runtime_integration="not_integrated")
        data["previous_pending_familiar_first_action_arrival"] = deepcopy(data["pending_familiar_first_action_arrival"])
        data["pending_familiar_first_action_arrival"] = None
        data["accepted_drone_first_action"] = ref(ACCEPTED)
        data["latest_drone_first_action_acceptance"] = ref(DECISION)
        data["pending_familiar_zone_arrival"] = ref(PROPOSAL)
        if old_name.startswith("review-implementation"):
            data.update(revision=29, status="displayed_drone_first_action_accepted;zone_arrival_details_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-PETS01"] = {"source": ref(ACCEPTED),
                "rule": "new_drone_can_act_in_invocation_turn_normal_familiar_phase_if_otherwise_eligible",
                "unshown": "zone_arrival_and_progressive_arrival_phase_proposed_under_INTERACT_PETS02"}
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = ["familiar_zone_arrival_and_progressive_arrival_phase_proposed_under_INTERACT_PETS02" if s == "familiar_first_action_arrival_and_zone_hazards_proposed_under_INTERACT_PETS01" else s for s in group["remaining"]]
            data["recommendation"] = "review_zone_arrival_then_remaining_command_costs_and_class_starts_first_mission"
        else:
            if old_name.startswith(("review-attribute", "review-primary", "review-complete")):
                data.setdefault("active_rule_overrides", {})["drone_first_action"] = {
                    **ref(ACCEPTED), "acceptance": ref(DECISION), "rule": source["proposed_rule"]["new_drone"],
                    "preserved": "raised_first_action_next_birth_turn;normal_actor_conditions_and_budgets",
                    "pending": "zone_arrival_and_other_combat_command_costs"}
            if old_name.startswith("review-attribute"):
                data.update(review_id="STAT-MODEL84", status="displayed_drone_first_action_accepted;zone_arrival_proposed;other_rules_preserved")
                pair(data, "previous_model", old_name)
            elif old_name.startswith(("review-primary", "review-complete")):
                data["batch_revision"] += 1
                pair(data, "previous_manifest", old_name)
                pair(data, "latest_decisions", DECISION)
                pair(data, "latest_proposal", PROPOSAL)
                data["latest_proposal_scope"] = "eligible_familiar_zone_arrival_hazards_and_progressive_arrival_phase"
            else:
                pair(data, "previous_count", old_name)
                data["latest_feedback"] = "displayed_drone_first_action_accepted;zone_arrival_details_proposed"
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-29.json")
            if not old_name.startswith("review-attribute"):
                pair(data, "current_attribute_model", "review-attribute-model-87.json")
            data["next_review_kind"] = "familiar_zone_arrival_INTERACT_PETS02"
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("familiar-zone-arrival-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-PETS01-PETS02-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "PETS01": "displayed_new_drone_first_action_only_accepted", "PETS02": "proposal_only",
        "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": checked, "errors": 0,
                      "drone_first_action": "accepted_only", "zone_arrival": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
