"""Design only: accept Contagion completion and propose the Bastion start profile."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
SOURCE = "review-contagion-retention-targeting-INTERACT-CONTAGION03-01.json"
DECISION = "contagion-retention-targeting-acceptance-decisions.json"
ACCEPTED = "review-contagion-retention-targeting-INTERACT-CONTAGION03-02.json"
PROPOSAL = "review-class-start-bastion-START-B01-01.json"
PAIRS = [("review-implementation-readiness-31.json", "review-implementation-readiness-32.json"),
         ("review-attribute-model-89.json", "review-attribute-model-90.json"),
         ("review-primary-catalogue-261.json", "review-primary-catalogue-262.json"),
         ("review-complete-skill-design-236.json", "review-complete-skill-design-237.json"),
         ("remaining-individual-skill-review-counts-130.json", "remaining-individual-skill-review-counts-131.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """START-B01 · Bastion — profil de départ proposé

Vous encaissez les coups et ouvrez un passage au cœur de la mêlée.

Attributs fixes
Force 7 · Agilité 5 · Vitalité 7
Perception 5 · Intelligence 4 · Volonté 6

Compétences déjà apprises
- Frappe de rupture, rang 1 : +40 % de dégâts d'arme de mêlée ; 6 énergie ; recharge de 3 tours. Arme de mêlée active requise.
- Vitalité améliorée, rang 1 : +10 % de PV maximum.

Implant initial
- Bonus : +1 point d'armure.
- Malus : −10 % d'énergie maximum.
Cette paire peut être remplacée plus tard par un implant trouvé. Les compétences apprises restent acquises.

Au niveau 1, avant les bonus d'équipement : 41 / 41 PV et 27 / 27 énergie. Ces valeurs incluent le passif et le malus de l'implant.

Le matériel envisagé reste une hache, une tenue d'assaut et des soins. Les modèles exacts, leurs chiffres et quantités seront présentés avec l'équipement de départ. Les nouvelles plages de dégâts ne sont pas inventées dans cette fiche.

Le nom, l'orientation et la silhouette déjà validés restent conservés. Le texte de présentation, la répartition d'attributs et les chiffres de l'implant restent proposés, en attente de validation."""


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
        assert new["pending_contagion_retention_targeting"] is None
        assert new["pending_class_start_Bastion"] == ref(PROPOSAL)
        assert "class_start_Bastion" not in new.get("active_rule_overrides", {})
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load(SOURCE)["proposed_rule"]
    draft = load(PROPOSAL)
    assert draft["mechanic_approval"] == "pending_user_review"
    assert sum(draft["proposed_profile"]["attributes"].values()) == 34
    assert (30 + 4 * (7 - 5)) * 110 // 100 == draft["derived_preview"]["maximum_HP"] == 41
    assert (30 + 2 * (4 - 5) + 2 * (6 - 5)) * 90 // 100 == draft["derived_preview"]["maximum_energy"] == 27
    curves = load("skill-rank-curves-provisional-02.json")
    b01 = next(s for s in curves["skills"] if s["skill_id"] == "B01")["rank_snapshots"][0]["rank_values"]
    b03 = next(s for s in curves["skills"] if s["skill_id"] == "B03")["rank_snapshots"][0]["rank_values"]
    assert b01["damage_coefficient_percent"] == 140 and b01["energy"] == 6 and b01["cooldown_turns"] == 3
    assert b03["maximum_HP_bonus_percent"] == 10
    assert load("remaining-individual-skill-review-counts-131.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    return len(checked)


def main():
    source = load(SOURCE)
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-CONTAGION03-ACCEPT", "source_proposal": ref(SOURCE),
        "user_acceptance": "oui ca me va :)", "approved_rule": deepcopy(source["proposed_rule"]),
        "approval_boundary": "displayed_AF06_retention_rounding_existing_defense_modes_application_checks_grid_range_direct_wall_block_ties_and_death_order_continuity_accepted;new_rank_curves_RP_native_feedback_and_other_skills_not_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="shown_Contagion_retention_targeting_completion_accepted;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="shown_design_explanation_accepted;approved_AF06_name_and_RP_preserved")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "START-B01", "revision": 1, "title": "Bastion — profil de départ",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref("class-approval.json"), ref("progression-decisions.json"), ref("class-drone-corrections.json"),
                         ref("review-derived-stat-HP01-02.json"), ref("review-derived-stat-E01-02.json"),
                         ref("review-maximum-reserve-modifiers-02.json"), ref("skill-rank-curves-provisional-02.json")],
        "existing_design_source": "docs/REFONTE_AGENT_CRYOGENISE.md:five_class_profiles_34_points_and_initial_Bastion_skills",
        "scope": "one_class_start_RP_attribute_allocation_and_initial_implant_pair_numbers;no_new_skills_or_equipment_models",
        "already_accepted": "Bastion_name_melee_survival_direction_and_silhouette1;one_active_and_one_passive_learned_rank1;B01_identity_and_B03_initial10percent_HP;found_implant_replacement_without_skill_reset;level_gain5_attributes1_skill_point",
        "proposed_profile": {
            "class": "Bastion", "starting_level": 1,
            "roleplay": "Vous encaissez les coups et ouvrez un passage au cœur de la mêlée.",
            "attributes": {"Force": 7, "Agilité": 5, "Vitalité": 7, "Perception": 5, "Intelligence": 4, "Volonté": 6},
            "fixed_start_attribute_budget": 34,
            "learned_start_skills": [{"skill_id": "B01", "title": "Frappe de rupture", "learned_rank": 1},
                                    {"skill_id": "B03", "title": "Vitalité améliorée", "learned_rank": 1}],
            "initial_implant_pair": {"flat_armor_bonus": 1, "maximum_energy_penalty_percent": 10,
                                    "replacement": "existing_found_implant_replaces_pair_and_preserves_learned_skills;no_new_implant_name_or_loot_affix"}
        },
        "retained_rank1_skill_values": {"B01": {"weapon_damage_bonus_percentage_points": 40, "energy": 6, "cooldown_turns": 3,
                                                       "active_weapon_requirement": "melee"},
                                       "B03": {"maximum_HP_bonus_percent": 10}},
        "derived_preview": {"level": 1, "equipment_bonus_scope": "none_in_this_preview", "base_maximum_HP": 38,
                            "maximum_HP": 41, "current_HP": 41, "base_maximum_energy": 30, "maximum_energy": 27,
                            "current_energy": 27, "formula_scope": "accepted_attributes_reserve_modifier_order_and_final_floor;includes_B03_and_implant_only",
                            "not_final_equipped_profile": True},
        "equipment_direction": {"weapon": "hache", "clothing": "tenue d'assaut", "consumables": "soins",
                                "scope": "historical_role_direction_only;exact_models_values_counts_and_weapon_variance_pending_separate_review"},
        "visual_boundary": "existing_approved_class1_silhouette_unchanged;no_new_character_equipment_pixels_native_preview_layout_or_animation",
        "not_resolved_here": ["exact_starting_equipment_and_consumables", "four_other_class_profiles", "other_new_replacement_implant_pairs",
                              "remaining_older_partial_skill_receipts_and_final_tooltip_copy", "native_creation_UI_and_equipped_preview"],
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "mechanic_approval": "pending_user_review", "text_approval": "new_class_RP_pending;skill_names_and_existing_skill_RP_unchanged",
        "visual_approval": "existing_silhouette_assignment_retained;no_new_native_art_approval_requested", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data.update(date="2026-10-09", runtime_integration="not_integrated")
        data["previous_pending_contagion_retention_targeting"] = deepcopy(data["pending_contagion_retention_targeting"])
        data["pending_contagion_retention_targeting"] = None
        data["accepted_contagion_retention_targeting"] = ref(ACCEPTED)
        data["latest_contagion_retention_targeting_acceptance"] = ref(DECISION)
        data["pending_class_start_Bastion"] = ref(PROPOSAL)
        if old_name.startswith("review-implementation"):
            data.update(revision=32, status="shown_coupled_turn_interactions_resolved;Bastion_start_profile_proposed;older_receipts_equipment_and_native_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-CONTAGION03"] = {"source": ref(ACCEPTED),
                "rule": "same_retention_for_remaining_duration_and_stored_potency_existing_application_and_defense_modes;Chebyshev_direct_wall_block_yx_ties;existing_immediate_death_order",
                "unshown": "final_dynamic_AF06_tooltip_copy_and_native_feedback_under_READY07"}
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if s != "Contagion_retention_bindings_geometry_ties_and_existing_death_order_continuity_proposed_under_INTERACT_CONTAGION03"]
                    group["resolved"] = "shown_turn_status_arrival_command_and_Contagion_interactions_accepted;new_unshown_skill_specific_cases_not_inferred_approved"
                elif group["id"] == "READY04":
                    group["pending_Bastion_profile"] = ref(PROPOSAL)
            data["recommendation"] = "review_class_core_starts_then_exact_equipment_and_first_mission;reconcile_remaining_partial_receipts_without_reopening_accepted_parts"
        else:
            if old_name.startswith(("review-attribute", "review-primary", "review-complete")):
                data.setdefault("active_rule_overrides", {})["contagion_retention_targeting"] = {
                    **ref(ACCEPTED), "acceptance": ref(DECISION), "rule": deepcopy(source["proposed_rule"])}
            if old_name.startswith("review-attribute"):
                data.update(review_id="STAT-MODEL87", status="Contagion_completion_accepted;Bastion_start_profile_proposed;other_rules_preserved")
                pair(data, "previous_model", old_name)
            elif old_name.startswith(("review-primary", "review-complete")):
                data["batch_revision"] += 1
                pair(data, "previous_manifest", old_name)
                pair(data, "latest_decisions", DECISION)
                pair(data, "latest_proposal", PROPOSAL)
                data["latest_proposal_scope"] = "Bastion_start_RP_attributes_and_implant_pair;equipment_and_visuals_separate"
            else:
                pair(data, "previous_count", old_name)
                data["latest_feedback"] = "Contagion_completion_accepted;Bastion_start_profile_proposed"
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-32.json")
            if not old_name.startswith("review-attribute"):
                pair(data, "current_attribute_model", "review-attribute-model-90.json")
            data["next_review_kind"] = "Bastion_start_profile_START_B01"
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("bastion-start-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-CONTAGION03-START-B01-CHECK01",
        "scope": "design_reference_integrity_preserved_rules_and_derived_preview_arithmetic;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved", "Bastion_HP_energy_preview": "41_and27_checked",
        "CONTAGION03": "accepted", "START-B01": "proposal_only", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": checked, "errors": 0,
                      "Contagion_completion": "accepted", "Bastion_start": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
