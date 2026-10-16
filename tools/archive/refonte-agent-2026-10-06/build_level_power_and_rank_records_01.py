"""Design-only receipts/prototype; no runtime content integration."""
from fractions import Fraction
from pathlib import Path
import hashlib
import json

BASE = Path(__file__).resolve().parent
PREFIX = "artifacts/refonte-agent-2026-10-06/"


def ref(name):
    text = (BASE / name).read_text(encoding="utf-8")
    return {"reference": PREFIX + name,
            "reference_sha256": hashlib.sha256(text.encode("utf-8")).hexdigest()}


def save(name, data):
    path = BASE / name
    assert not path.exists(), name
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n",
                    encoding="utf-8", newline="\n")


def main():
    examples = []
    for level in [1, 2, 10, 40, 100]:
        for rank in [1, 2, 10, 12]:
            raw = Fraction(8 + 3 * (rank - 1)) * Fraction(level + 9, 10)
            examples.append({"skill_id": "FE01", "character_current_level": level,
                             "effective_rank": rank, "Volonte": 5,
                             "fire_damage_integer": int(raw),
                             "noncritical_before_defenses": True,
                             "profile": "neutral_attributes_no_equipment_or_other_modifiers;illustrative_not_real_save"})
    save("review-current-level-skill-power-01.json", {
        "date": "2026-10-09", "review_id": "SKILL-LEVEL01-NUM01", "revision": 1,
        "status": "accepted_current_level_direction_with_separate_tentative_numeric_reference;not_runtime_integrated",
        "accepted_direction": ref("current-level-skill-power-acceptance-decisions.json"),
        "rank_authority": ref("skill-rank-balancing-delegation-decisions.json"),
        "coefficient_approval": "proposed_reference_only;not_individually_accepted_or_finally_balanced",
        "tentative_damage_factor": {
            "formula": "1 + (current_caster_level - 1) / 10",
            "domain": "positive_integer_level;no_new_character_level_cap",
            "level1_factor": 1,
            "level1_identity": "retains_rank_damage_component_at_level1",
            "evaluation": "exact_rational_until_existing_final_damage_floor",
            "placement": "apply_once_to_explicit_raw_nonweapon_rank_damage_component_before_accepted_compatible_flat_and_percentage_modifiers;never_to_final_damage_or_derived_actual_damage",
            "history_input": "none;acquisition_level_is_not_a_damage_parameter"
        },
        "binding": "explicit_raw_damage_parameters_in_SKILL_RANKS01;no_implicit_scaling_of_all_numeric_fields",
        "excluded_sources": [
            "active_weapon_native_damage_and_weapon_skill_coefficients",
            "drone_level_own_attributes_gear_and_existing_invocation_rank_HP_damage_bonuses",
            "raised_origin_actor_statistics_and_mark_rank_power_percentage",
            "damage_derived_from_actual_initial_damage_including_collision_bleed_reflection_drain_and_stored_damage",
            "FE06_remaining_burn_conversion_and_FR06_splash_already_derived_from_scaled_source",
            "status_probability_duration_area_range_cooldown_and_energy_cost",
            "maximum_HP_percent_heals_and_other_existing_relative_bonuses",
            "barrier_HP_and_fixed_robot_healing_pending_separate_numeric_binding_review"
        ],
        "BM01_flat_damage_binding": "level_scales_only_conjured_raw_fire_addition_once_before_addition_to_weapon;no_level_multiplier_on_entire_weapon_attack",
        "SM00_secondary_once_binding": "rank_constant_active_damage_can_follow_current_caster_level_without_additional_active_purchases",
        "existing_effect_update_policy": "pending;new_activations_use_current_level;no_automatic_live_zone_damage_refresh_rule_inferred",
        "Braise_neutral_profile_examples": examples,
        "history_invariance_example": {
            "first_history": "learn_at_level2_then_rank10_then_level40",
            "second_history": "level40_then_learn_and_invest_to_rank10",
            "same_profile_result": "171_fire_damage_in_this_tentative_neutral_profile",
            "level40_rank1_result": "39_fire_damage_in_this_tentative_neutral_profile",
            "numbers_status": "tentative_reference;direction_not_final_coefficients_is_accepted"
        },
        "balance_boundary": "actual_enemy_HP_defenses_weapon_curves_and_resource_pressure_must_be_calibrated_together_later;no_claim_of_playtest_balance",
        "player_copy": "no_new_RP_name_or_UI_text_integrated;future_tooltip_displays_integer_current_damage_and_only_changed_next_rank_values",
        "runtime_integration": "not_integrated"
    })
    print("Created SKILL-LEVEL01-NUM01 tentative level-power reference.")


if __name__ == "__main__":
    main()
