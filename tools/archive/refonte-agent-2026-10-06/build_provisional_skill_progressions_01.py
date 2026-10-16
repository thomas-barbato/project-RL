"""Consolidate delegated rank numbers. This does not edit or run the game."""
from __future__ import annotations

from copy import deepcopy
from fractions import Fraction
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
PREFIX = "artifacts/refonte-agent-2026-10-06/"
AUTHORITY = "skill-rank-balancing-delegation-decisions.json"
LEVEL_AUTHORITY = "current-level-skill-power-acceptance-decisions.json"
LEVEL_PROFILE = "review-current-level-skill-power-01.json"
OUT = "skill-rank-curves-provisional-01.json"
CHECKS = "skill-rank-curves-validation-01.json"
POLICIES: dict[str, dict] = {}


def load(name):
    return json.loads((BASE / name).read_text(encoding="utf-8"))


def reference(name):
    source = (BASE / name).read_text(encoding="utf-8")
    return {"reference": PREFIX + name,
            "reference_sha256": hashlib.sha256(source.encode("utf-8")).hexdigest()}


def save(name, data):
    path = BASE / name
    assert not path.exists(), f"Preserve historical review: {path}"
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n",
                    encoding="utf-8", newline="\n")


def constant(value):
    return {"kind": "constant", "value": value}


def linear(base, gain, *, cap=None, floor=None, direction="increase"):
    data = {"kind": "linear", "base": base, "gain_per_rank": gain,
            "direction": direction}
    if cap is not None:
        data["maximum"] = cap
    if floor is not None:
        data["minimum"] = floor
    return data


def stepped(base, every, gain=1, *, offset=0, cap=None):
    data = {"kind": "stepped", "base": base, "every_ranks": every,
            "gain_per_step": gain, "rank_offset": offset, "direction": "increase"}
    if cap is not None:
        data["maximum"] = cap
    return data


def scaled_damage(base, gain, numerator, denominator):
    return {"kind": "scaled_linear_damage", "base": base, "gain_per_rank": gain,
            "numerator": numerator, "denominator": denominator,
            "direction": "increase", "rounding": "final_damage_floor_after_modifiers"}


def decay(numerator, denominator):
    return {"kind": "exponential_ratio", "numerator": numerator,
            "denominator": denominator, "direction": "decrease"}


def put(identifier, **params):
    assert identifier not in POLICIES
    POLICIES[identifier] = {key: value if isinstance(value, dict) else constant(value)
                            for key, value in params.items()}


def cost(base, every=3):
    return stepped(base, every)


# Direct weapon coefficients become percentage points above ordinary 100% in
# the accepted additive damage pool; these are not a second final multiplier.
put("B01", damage_coefficient_percent=linear(140, 10), energy=cost(6), cooldown_turns=3, targets=1)
put("ME02", damage_coefficient_percent=linear(80, 5), energy=cost(8, 4), cooldown_turns=3, sector_cells=3)
put("ME03", damage_coefficient_percent=linear(100, 8), armor_ignored_percent=linear(20, 3, cap=79), energy=cost(8), cooldown_turns=4)
put("ME04", damage_coefficient_percent=linear(90, 6), push_cells=stepped(1, 5, cap=3), energy=cost(7, 4), cooldown_turns=3, collision_percent_of_actual_initial_damage=25)
put("ME05", damage_coefficient_percent=linear(110, 8), maximum_target_distance_cells=stepped(3, 4, cap=6), minimum_target_distance_cells=2, energy=cost(10), cooldown_turns=5)
put("ME06", damage_coefficient_percent=linear(60, 5), energy=cost(12), cooldown_turns=6, adjacent_cells=8, bleed_percent_of_actual_hit_per_turn=10, bleed_duration_turns=3)

# Stronger finite learned-rank gains; defenses retain strictly sub-immunity
# ceilings. These are numeric changes only, with all original effect scopes.
put("B03", maximum_HP_bonus_percent=linear(10, 4))
put("B02", direct_damage_reduction_percent=linear(25, 3, cap=74), duration_turns=4, energy=cost(10), cooldown_turns=8)
put("SU03", reflected_direct_melee_damage_percent=linear(10, 3, cap=49), duration_turns=5, energy=cost(12), cooldown_turns=10)
put("SU04", heal_percent_of_max_HP_per_turn=linear(4, 1, cap=19), maximum_channel_turns=3, energy_per_turn=cost(4, 4), cooldown_turns=8)
put("SU05", maximum_cleansed_statuses=stepped(1, 5), energy=linear(10, -1, floor=1, direction="decrease"), cooldown_turns=6)
put("SU06", cooldown_turns=linear(100, -4, floor=30, direction="decrease"), HP_after_eligible_fatal_hit=1, energy=0)
put("BD06", energy_return_percent_of_maximum=linear(2, 2), minimum_interval_turns=3)

# Actor level/attributes and actual gear remain the source of drone base stats.
# Do not import superseded rank-only HP/damage formulas from old cards.
for identifier, energy in [("D56", 10), ("D57", 10), ("D58", 12), ("D59", 12), ("D66", 14)]:
    params = dict(maximum_HP_rank_bonus_percent=linear(0, 10), energy=cost(energy), cooldown_turns=6)
    if identifier in {"D58", "D59", "D66"}:
        params["damage_rank_bonus_percent"] = linear(0, 10)
    if identifier == "D56":
        params["vision_radius_cells"] = stepped(4, 3, cap=8)
    if identifier == "D57":
        params["cargo_rank_bonus_kg"] = linear(0, 8)
    if identifier == "D66":
        params["protection_percent"] = linear(5, 2, cap=29)
        params["protection_radius_cells"] = 2
    put(identifier, **params)
put("DP01", repair_percent_of_drone_maximum_HP=linear(2, 1, cap=100))
put("DP02", incoming_damage_reduction_percent=linear(10, 2, cap=49))
put("O03", drone_damage_bonus_percent=linear(25, 5))
put("DR01", raw_robot_heal_HP=linear(12, 3), energy=cost(10, 4), range_cells=4, cooldown_turns=4)

for identifier, base_duration, energy, cooldown in [
        ("O02", 3, 8, 5), ("AF02", 3, 9, 5), ("AF03", 2, 12, 8),
        ("AF04", 3, 10, 6), ("AF05", 2, 10, 7)]:
    params = dict(duration_rank_turns=stepped(base_duration, 2),
                  application_power_rank_base=linear(100, 15),
                  energy=cost(energy), cooldown_turns=cooldown, range_cells=5)
    if identifier == "O02":
        params["direct_damage_reduction_percent"] = linear(15, 3, cap=49)
    if identifier == "AF02":
        params["physical_armor_reduction_percent"] = linear(20, 3, cap=59)
    put(identifier, **params)
put("AF06", transmitted_potency_retained_percent=linear(50, 3, cap=89), range_from_victim_cells=stepped(2, 5, cap=4))

put("DI01", damage_coefficient_percent=linear(110, 6), maximum_targets=stepped(2, 5, cap=5), damage_retained_per_traversal_percent=75, energy=cost(8), cooldown_turns=4)
put("DI02", damage_coefficient_per_projectile_percent=linear(60, 3), native_sequence_count=2, accuracy_multiplier_percent=90, energy=cost(10), cooldown_turns=4)
put("DI03", damage_coefficient_percent=linear(80, 5), maximum_targets=stepped(3, 5, cap=6), range_skill_limit_cells=stepped(3, 4, cap=6), cone_angle_degrees=90, energy=cost(12), cooldown_turns=6)
put("DI04", damage_coefficient_percent=100, accuracy_bonus_percent=linear(10, 3), armor_ignored_percent=linear(20, 3, cap=59), critical_chance_bonus_points=linear(5, 2, cap=24), energy=cost(8), cooldown_turns=4)
put("DI05", damage_coefficient_percent=linear(100, 4), maximum_pull_cells=stepped(1, 5, cap=3), energy=cost(10), cooldown_turns=6)
put("DI07", damage_coefficient_percent=linear(120, 5), life_return_percent_of_actual_primary_HP_removed=linear(10, 4, cap=100), heal_cap_percent_of_player_maximum_HP_per_use=10, energy=12, cooldown_turns=8)

put("T02", movement_cells=1, evasion_bonus_units=linear(15, 3), duration_turns=stepped(2, 5), energy=cost(8), cooldown_turns=6)
put("ES03", duration_turns=linear(3, 1), energy=cost(8), cooldown_turns=6, maximum_lures=1, impacts_to_destroy=1, range_cells=3)
put("ES05", movement_cells=2, evasion_bonus_units=linear(10, 2), duration_turns=2, energy=cost(10), cooldown_turns=8)
put("ES06", projectile_evasion_bonus_units=linear(12, 3))

# Revised accepted high-frequency elemental-status direction is respected.
put("FE01", raw_fire_damage=linear(8, 3), raw_burn_damage_per_turn=linear(2, 1), burn_duration_turns=stepped(3, 5), ordinary_burn_chance_percent=100, range_cells=5, energy=cost(8), cooldown_turns=3)
put("FE02", raw_fire_damage_per_target_per_turn=linear(6, 2), raw_burn_damage_per_turn=linear(1, 1), burn_chance_percent=linear(10, 3, cap=100), burn_duration_turns=3, cone_range_cells=stepped(3, 5, cap=5), cone_angle_degrees=90, maximum_duration_turns=3, energy_per_turn=cost(6, 4), cooldown_turns=6)
put("FE03", raw_fire_damage_per_target_per_turn=linear(5, 2), raw_burn_damage_per_turn=linear(1, 1), burn_chance_percent=linear(15, 3, cap=100), burn_duration_turns=3, maximum_wall_length_cells=stepped(3, 5, 2, cap=7), duration_turns=stepped(3, 4), placement_range_cells=4, maximum_walls=1, energy=cost(12), cooldown_turns=8)
put("FE04", raw_center_fire_damage=linear(14, 4), maximum_edge_reduction_percent=10, burn_chance_percent=linear(35, 3, cap=100), raw_burn_damage_per_turn=linear(1, 1), burn_duration_turns=3, radius_cells=1, range_cells=5, preparation_delay_complete_turns=1, energy=cost(12), cooldown_turns=6)
put("FE05", raw_explosion_fire_damage=linear(18, 4), burn_chance_percent=linear(30, 3, cap=100), raw_burn_damage_per_turn=linear(1, 1), burn_duration_turns=3, radius_cells=3, range_cells=6, energy=cost(18), cooldown_turns=8)
put("FE06", remaining_burn_conversion_percent=linear(100, 4), fresh_burn_chance_percent=linear(35, 3, cap=100), raw_fresh_burn_damage_per_turn=linear(1, 1), fresh_burn_duration_turns=3, range_cells=5, energy=cost(6), cooldown_turns=3)

put("FR01", raw_cold_damage=scaled_damage(8, 3, 85, 100), freeze_skill_chance_factor_percent=linear(40, 5, cap=100), application_power_rank_base=linear(100, 10), freeze_duration_turns=5, range_cells=5, energy=cost(6), cooldown_turns=2)
put("FR02", raw_cold_damage_per_target=linear(4, 2), freeze_skill_chance_factor_percent=linear(30, 5, cap=100), application_power_rank_base=linear(100, 10), freeze_duration_turns=stepped(2, 5), radius_cells=2, energy=cost(10), cooldown_turns=6)
put("FR03", raw_HP_per_block=linear(10, 3), freeze_skill_chance_factor_percent=linear(35, 5, cap=100), application_power_rank_base=linear(100, 10), freeze_duration_turns=2, maximum_wall_length_cells=stepped(3, 5, 2, cap=7), wall_duration_turns=stepped(3, 5), maximum_walls=1, range_cells=4, energy=cost(12), cooldown_turns=8)
put("FR04", raw_cold_damage_per_target=linear(4, 2), freeze_skill_chance_factor_percent=linear(25, 5, cap=100), application_power_rank_base=linear(100, 10), freeze_duration_turns=3, wave_progress_cells_per_turn=1, wave_width_cells=3, wave_thickness_cells=1, maximum_wave_travel_cells=5, energy=cost(14), cooldown_turns=8)
put("FR05", raw_cold_damage_per_impact=scaled_damage(6, 2, 9, 10), freeze_skill_chance_factor_percent=linear(30, 5, cap=100), application_power_rank_base=linear(100, 10), freeze_duration_turns=3, impacts_per_turn=stepped(2, 5, cap=5), duration_turns=stepped(3, 5), radius_cells=2, range_cells=5, first_impact_delay_complete_turns=1, energy=cost(16), cooldown_turns=10)
put("FR06", raw_cold_damage_on_frozen_source=linear(10, 3), splash_percent_of_source_damage=50, radius_cells=1, energy=cost(17), cooldown_turns=8)

put("A01", raw_occult_damage=linear(8, 3), range_cells=5, energy=cost(6), cooldown_turns=2)
put("A02", raw_occult_damage_per_target=linear(6, 2), zone_width_cells=3, zone_height_cells=3, range_cells=4, energy=cost(10), cooldown_turns=4)
put("A03", magical_energy_cost_reduction_percent=linear(20, 3, cap=49), positive_payable_cost_floor=1)
put("OC04", raw_HP_per_ritual_segment=linear(10, 2), duration_turns=stepped(2, 5), segment_count=8, center_range_cells=4, energy=cost(14), cooldown_turns=10)
put("OC05", raw_occult_damage_per_resolution=linear(6, 2), life_return_percent_of_actual_HP_removed=linear(10, 2, cap=24), maximum_resolutions=3, range_cells=5, energy_per_resolution=cost(4, 4), cooldown_turns=6)
put("OC07", origin_HP_and_damage_power_rank_percent=linear(45, 5), raised_duration_turns=stepped(6, 2), maximum_raised_allies=stepped(5, 1, offset=-9), range_cells=5, energy=cost(18, 4), cooldown_turns=8)
put("HY05", raw_occult_damage=linear(10, 2), energy_return_percent_of_actual_HP_removed=linear(25, 3, cap=75), range_cells=5, energy=2, cooldown_turns=6)

for identifier, energy in [("M01", 12), ("TR61", 14), ("TR63", 14), ("TR64", 16), ("TR65", 14)]:
    params = dict(duration_rank_turns=linear(12, 1), energy=cost(energy), cooldown_turns=10)
    if identifier == "M01":
        params.update(raw_claw_damage=linear(10, 3), raw_laceration_damage_per_target=linear(10, 3), bleed_chance_percent=linear(20, 3, cap=100), bleed_percent_of_actual_hit_per_turn=10, bleed_duration_turns=3, claw_energy=0, claw_range_cells=1, leap_cells=2, leap_energy=4, leap_cooldown_turns=3, laceration_cells=3, laceration_energy=6, laceration_cooldown_turns=3)
    elif identifier == "TR61":
        params.update(raw_heavy_strike_damage=linear(14, 4), raw_shockwave_damage=linear(6, 2), crushing_damage_coefficient_percent=150, crushing_armor_bypass_percent=linear(30, 2, cap=100), stun_chance_percent=linear(20, 3, cap=100), stun_duration_turns=1, armor_bonus_percent=linear(10, 3), maximum_HP_bonus_percent=linear(20, 3), movement_time_increase_percent=25, strike_energy=0, strike_range_cells=1, shockwave_radius_cells=1, shockwave_energy=6, shockwave_cooldown_turns=4, crushing_energy=8, crushing_cooldown_turns=4)
    elif identifier == "TR63":
        params.update(raw_heurt_damage=linear(10, 3), direct_damage_reduction_percent=linear(10, 2, cap=34), reserve_cap_percent_of_maximum_HP=linear(10, 2, cap=29), raw_discharge_damage_per_target=linear(5, 2), heurt_energy=0, heurt_range_cells=1, discharge_radius_cells=1, discharge_energy=6, discharge_cooldown_turns=4, closure_additional_reduction_percent=30, closure_duration_turns=3, closure_energy=5, closure_cooldown_turns=6)
    elif identifier == "TR64":
        params.update(raw_corporeal_bolt_damage=linear(8, 3), raw_pulse_damage_per_target=linear(5, 2), raw_drain_damage=linear(10, 4), drain_life_return_percent=linear(10, 3, cap=75), bolt_range_cells=4, bolt_energy=0, pulse_radius_cells=2, pulse_energy=6, pulse_cooldown_turns=4, drain_range_cells=4, drain_energy=8, drain_cooldown_turns=4)
    else:
        params.update(raw_reach_attack_damage=linear(10, 3), raw_line_strike_damage_per_target=linear(14, 4), reach_attack_range_cells=2, reach_attack_energy=0, line_strike_length_cells=4, line_strike_energy=6, line_strike_cooldown_turns=4, shadow_step_range_cells=3, shadow_step_energy=5, shadow_step_cooldown_turns=3)
    put(identifier, **params)
put("M03", transformed_damage_bonus_percent=linear(10, 5))
put("M02", raw_occult_damage=linear(10, 4), extension_turns_per_eligible_hit=stepped(1, 10, cap=2), maximum_added_turns=stepped(2, 5, cap=6), range_cells=3, energy=cost(6), cooldown_turns=2)
put("HY07", raw_fire_explosion_damage=linear(12, 4), radius_cells=2, command_range_cells=6, energy=6, cooldown_turns=4)
put("HY04", maximum_drone_distance_cells=stepped(4, 3, cap=8), energy=linear(10, -1, floor=1, direction="decrease"), cooldown_turns=stepped(8, 2, -1))
POLICIES["HY04"]["cooldown_turns"].update(minimum=4, direction="decrease")
put("HY06", duration_rank_turns=stepped(2, 2), application_power_rank_base=linear(100, 15), maximum_controlled_enemies=1, range_cells=4, energy=cost(16), cooldown_turns=10)

put("BM01", raw_added_fire_damage=linear(3, 1), duration_turns=stepped(4, 5), energy=cost(8), cooldown_turns=6)
put("BM02", life_return_percent_of_actual_eligible_HP_removed=linear(10, 3, cap=100), heal_cap_percent_of_maximum_HP_per_turn=10, duration_turns=4, energy=cost(10), cooldown_turns=12)
put("BM03", damage_bonus_percent_per_charge=linear(5, 2), maximum_charges=3)
put("BM05", reflected_direct_magic_damage_percent=linear(10, 3, cap=100), duration_turns=stepped(4, 5), energy=cost(10), cooldown_turns=8)
put("T01", accuracy_bonus_percent=linear(15, 3), duration_turns=stepped(4, 5), energy=cost(6), cooldown_turns=3, range_cells=6)
put("T03", ranged_attack_damage_bonus_percent=linear(10, 5))
put("BD04", accuracy_bonus_percent_per_stationary_turn=linear(5, 2), maximum_stationary_charges=3)

put("HL01", damage_coefficient_percent=linear(100, 10), maximum_distinct_observed_targets=3, energy=18, cooldown_turns=5)
put("HL02", damage_coefficient_percent=linear(80, 10), ordinary_stun_chance_percent=linear(20, 4, cap=95), stun_duration_turns=2, stun_radius_cells=1, energy=14, cooldown_turns=6)
put("HL03", damage_coefficient_percent=linear(130, 10), maximum_firing_turns=4, aim_zone_radius_cells=2, energy_per_firing_turn=8, cooldown_after_end_turns=8)
put("HL04", heavy_weapon_damage_bonus_percent=linear(10, 10))
put("HL05", added_explosion_radius_cells=stepped(1, 4, offset=1))
put("HL06", damage_coefficient_percent=linear(100, 10), life_return_percent_of_actual_primary_HP_removed=linear(10, 5, cap=100), heal_cap_percent_of_player_maximum_HP_per_use=linear(5, 1, cap=25), energy=16, cooldown_turns=10)

# Secondary active effects remain rank-constant and bought once. Rank-positive
# item grants still permit use, without inventing new active-level bonuses.
put("SF00", activation_energy=0, stationary_energy=0, base_step_energy=11, cooldown_turns=0, minimum_detection_cells=1, step_noise_reduction_percent=50)
put("SF01", energy_per_successful_stealth_step=linear(10, -1, floor=1, direction="decrease"))
put("SF02", detection_range_multiplier=decay(4, 5), minimum_detection_cells=1)
put("SM00", raw_fire_explosion_damage=12, radius_cells=1, placement_range_cells=1, arming_delay_turns=1, base_maximum_mines=1, energy=10, cooldown_turns=1)
put("SM01", maximum_simultaneous_mines=stepped(1, 3, offset=3))
put("SM02", variant_damage_bonus_percent=linear(0, 5), raw_poison_damage_per_turn=linear(6, 2), raw_poison_explosion_damage=12, poison_duration_turns=stepped(3, 5))
put("SM03", variant_damage_bonus_percent=linear(0, 5), raw_cold_explosion_damage=10, freeze_duration_turns=stepped(3, 5), freeze_skill_chance_factor_percent=linear(30, 5, cap=100), application_power_rank_base=linear(100, 10))
put("SM04", added_explosion_radius_cells=stepped(0, 4, offset=4))
put("SM05", zone_duration_turns=linear(3, 1), raw_fire_zone_damage_per_turn=3, raw_poison_cloud_damage_per_turn=3, first_effect_delay_turns=1)
put("SG00", raw_physical_explosion_damage=8, base_throw_range_cells=4, radius_cells=1, energy=8, cooldown_turns=2)
put("SG01", accuracy_reduction_percent=30, blindness_duration_turns=linear(2, 1), application_power_rank_base=linear(100, 10))
put("SG02", open_terrain_sound_reach_cells=linear(3, 1), emission_turns=2)
put("SG03", throw_range_bonus_cells=stepped(1, 3))
put("SG04", grenade_damage_bonus_percent=linear(10, 10))


def evaluate(spec, rank):
    assert isinstance(rank, int) and rank >= 1
    kind = spec["kind"]
    if kind == "constant":
        value = Fraction(str(spec["value"]))
    elif kind == "linear":
        value = Fraction(str(spec["base"])) + Fraction(str(spec["gain_per_rank"])) * (rank - 1)
    elif kind == "stepped":
        count = max(0, (rank - 1 + spec["rank_offset"]) // spec["every_ranks"])
        value = Fraction(spec["base"] + spec["gain_per_step"] * count)
    elif kind == "scaled_linear_damage":
        value = Fraction((spec["base"] + spec["gain_per_rank"] * (rank - 1)) * spec["numerator"], spec["denominator"])
    elif kind == "exponential_ratio":
        value = Fraction(spec["numerator"], spec["denominator"]) ** rank
    else:
        raise AssertionError(kind)
    if "maximum" in spec:
        value = min(value, Fraction(spec["maximum"]))
    if "minimum" in spec:
        value = max(value, Fraction(spec["minimum"]))
    return value


def display(value):
    if value.denominator == 1:
        return value.numerator
    return round(float(value), 6)


def latest_card(identifier):
    matches = list(BASE.glob(f"review-individual-description-{identifier}-*.json"))
    if identifier == "B01":
        return "review-skill-progression-B01-PROG01-01.json"
    assert matches, identifier
    return max(matches, key=lambda p: int(p.stem.rsplit("-", 1)[-1])).name


def get_formulas(card):
    return {key: value for key, value in card.items()
            if "formula" in key and "approval" not in key}


def is_raw_damage_parameter(key):
    return key.startswith("raw_") and "damage" in key


def level_damage_reference(value, level):
    """Tentative numerical prototype, not an approved final level coefficient."""
    assert isinstance(level, int) and level >= 1
    return value * Fraction(level + 9, 10)


def main():
    catalogue = load("review-primary-catalogue-232.json")
    primary = [(row, branch["name"]) for branch in catalogue["branches"] for row in branch["candidates"]]
    primary_ids = {row["id"] for row, _ in primary}
    secondary_ids = {"SF00", "SF01", "SF02", "SM00", "SM01", "SM02", "SM03", "SM04", "SM05", "SG00", "SG01", "SG02", "SG03", "SG04"}
    expected = primary_ids | secondary_ids | {"DR01"}
    assert len(primary_ids) == 79 and len(secondary_ids) == 14
    assert set(POLICIES) == expected, (set(POLICIES) - expected, expected - set(POLICIES))
    assert not any(identifier.startswith("FO") for identifier in POLICIES)
    assert evaluate(POLICIES["SM01"]["maximum_simultaneous_mines"], 10) == 5
    assert evaluate(POLICIES["SM01"]["maximum_simultaneous_mines"], 13) == 6
    assert evaluate(POLICIES["SF01"]["energy_per_successful_stealth_step"], 10) == 1
    assert all(evaluate(POLICIES["OC07"]["maximum_raised_allies"], r) == 5 + max(0, r - 10) for r in range(1, 101))

    rows, validation_rows = [], []
    branch_by_id = {row["id"]: branch for row, branch in primary}
    title_by_id = {row["id"]: row["title"] for row, _ in primary}
    one_purchase = {"SF00", "SM00", "SG00"}
    discrete_only = {"SF02", "SM01", "SM04", "SG03", "HL05"}
    source_refs = []
    for identifier in sorted(POLICIES):
        filename = latest_card(identifier)
        card = load(filename)
        source_refs.append(reference(filename))
        specs = POLICIES[identifier]
        parameters = deepcopy(specs)
        for key, spec in parameters.items():
            spec["value_scope"] = "rank_component_only;apply_current_accepted_attributes_equipment_and_target_rules_separately"
            if is_raw_damage_parameter(key):
                spec["current_level_binding"] = "caster_current_level_damage_base;learning_level_never_used"
            is_energy_cost = "energy" in key and not any(token in key for token in ["return", "reduction"])
            spec["benefit"] = not is_energy_cost and "movement_time_increase" not in key
            if key == "cooldown_turns" and spec["kind"] != "constant":
                spec["benefit"] = True
        snapshots = []
        for rank in list(range(1, 21)) + [30, 50, 100]:
            values = {key: display(evaluate(spec, rank)) for key, spec in specs.items()}
            integer_damage = {key: int(evaluate(spec, rank)) for key, spec in specs.items()
                              if is_raw_damage_parameter(key)}
            snapshots.append({"effective_rank": rank, "rank_values": values,
                              "rank_values_scope": "raw_design_components;not_final_player_damage_display",
                              "neutral_level1_integer_damage_before_other_modifiers": integer_damage})
        changed = []
        for rank in range(1, 10):
            benefits = [key for key, spec in parameters.items() if spec["benefit"] and evaluate(spec, rank) != evaluate(spec, rank + 1)]
            cheaper = [key for key, spec in specs.items() if "energy" in key and evaluate(spec, rank + 1) < evaluate(spec, rank)]
            changed.append({"from_rank": rank, "to_rank": rank + 1, "improved_fields": benefits + cheaper})
        if identifier not in one_purchase | discrete_only:
            assert all(row["improved_fields"] for row in changed), (identifier, changed)
        for key, spec in specs.items():
            sample_ranks = list(range(1, 100))
            if spec["kind"] == "exponential_ratio":
                assert 0 < spec["numerator"] < spec["denominator"]
            else:
                sample_ranks += [1000, 1000000]
            for rank in sample_ranks:
                before, after = evaluate(spec, rank), evaluate(spec, rank + 1)
                assert before >= 0 and after >= 0, (identifier, key)
                if "energy" in key and not (spec["kind"] == "constant" and spec["value"] == 0):
                    assert before >= 1 and after >= 1, (identifier, key)
                if spec.get("direction") == "increase":
                    assert after >= before, (identifier, key)
                elif spec.get("direction") == "decrease":
                    assert after <= before, (identifier, key)
                if any(token in key for token in ["chance_percent", "chance_factor_percent", "reduction_percent", "armor_ignored_percent", "life_return_percent"]):
                    assert before <= 100, (identifier, key)
        # A single coefficient with 0 < numerator < denominator proves the
        # exponential tail monotone without allocating huge rational powers.
        title = title_by_id.get(identifier, card.get("title", identifier))
        inherited = {key: deepcopy(value) for key, value in card.items()
                     if key in {"conditions", "conditions_proposed", "target", "targeting", "active_weapon_requirement", "source_control_rules", "control_rules", "allied_targeting", "transformation_actions", "actions"}}
        rows.append({
            "skill_id": identifier, "title": title,
            "branch": branch_by_id.get(identifier, "Drones — catalogue allocation pending" if identifier == "DR01" else "Secondary"),
            "source_card": reference(filename),
            "rank_numbers_status": "provisional_under_explicit_user_delegation;not_individual_final_balance_approval",
            "learned_rank_cap": 1 if identifier in one_purchase else 10,
            "positive_effective_rank_domain": True,
            "equipment_grant_from_zero_learned": "accepted_player_GEAR_SK01;no_permanent_learning",
            "once_purchased_active_rank_scaling": "constant_effects_even_with_item_rank_bonus" if identifier in one_purchase else None,
            "parameters": parameters, "rank_snapshots": snapshots,
            "current_level_damage_parameters": [key for key in specs if is_raw_damage_parameter(key)],
            "original_formula_snapshot": get_formulas(card),
            "inherited_effect_rules_snapshot": inherited,
            "unchanged_rules_source": "source_card_and_current_global_active_overrides;old_skill_numeric_curves_only_superseded",
            "player_copy": "no_new_roleplay_or_name;dynamic_current_next_values_must_use_these_curves_and_current_accepted_modifier_pipeline",
            "provisional_native_numeric_profile": "rank_component_only;actual_drone_native_attacks_energy_resistance_profiles_and_unreviewed_attribute_links_not_invented",
            "runtime_integration": "not_integrated"
        })
        validation_rows.append({"skill_id": identifier, "learned_transitions": changed,
                                "discrete_only": identifier in discrete_only,
                                "once_purchased": identifier in one_purchase})
    system = {
        "date": "2026-10-09", "review_id": "SKILL-RANKS01", "revision": 1,
        "status": "all_retained_rank_curves_completed_under_explicit_numeric_delegation;provisional_balance;not_runtime_integrated",
        "authority": reference(AUTHORITY), "catalogue": reference("review-primary-catalogue-232.json"),
        "accepted_current_level_power_direction": reference(LEVEL_AUTHORITY),
        "current_level_numeric_prototype": reference(LEVEL_PROFILE),
        "level_numeric_coefficient_approval": "not_explicitly_accepted;separate_provisional_review_profile;no_runtime_integration",
        "attribute_model": reference("review-attribute-model-60.json"),
        "primary_count": 79, "secondary_count": 14, "accepted_repair_complement_count": 1, "total_skill_count": len(rows),
        "sources": source_refs,
        "global_constraints": {
            "learned_cap": 10, "secondary_active_learned_cap": 1,
            "no_unlock_tiers": True, "secondary_passive_purchase_requires_actual_learned_active": True,
            "equipped_bonus_rank_color_without_tooltip_breakdown": True,
            "base_parameters_before_attributes_gear_and_target_defenses": True,
            "raw_nonweapon_damage_current_level_binding": "accepted_direction;separate_tentative_level_curve;never_learning_level",
            "no_double_level_scaling": "weapon_native_base_drone_actor_base_raised_origin_and_actual_damage_derivatives_keep_their_own_sources",
            "damage_final_integer_floor": True, "healing_current_accepted_limits_and_rounding_preserved": True,
            "direct_weapon_skill_coefficients_use_accepted_additive_pool": True,
            "no_magic_or_Agility_damage_multiplier_restored_from_obsolete_cards": True,
            "drone_actor_level_and_attribute_profiles_preserved": True,
            "drone_rank_HP_damage_bonus_pool": "10_percentage_points_per_rank_after1;owner_Intelligence_and_actor_modifiers_applied_once",
            "Mule_capacity": "40+2*(effective_own_Force-5)+cargo_rank_bonus_kg",
            "raised_power_attribute_addition": "rank_power_percent_plus_preexisting_Willpower_term;exact_unreviewed_attribute_term_not_approved_by_rank_delegation",
            "rank_decrease_cleanup": "only_elements_invoked_by_affected_skill_with_ground_real_item_drops_under_INTERACT01",
            "same_status_duration_restart": True, "overlapping_zone_effects_cumulate": True,
            "boss_Gel_Burn_chance_multiplier": "1/2", "elite_control_multiplier": "3/4",
            "other_control_boss_immunity": True, "poison_importance_penalty": False,
            "zone_status_exposure_growth_and_exit_reset": True,
            "own_offensive_effects_do_not_damage_player_or_familiars": True,
            "negative_effect_scopes_and_conditions": "preserve_current_approved_skill_and_global_rules;no_new_control_or_status_behavior",
            "secondary_dependencies": {"SF01": "SF00", "SF02": "SF00", "SM01": "SM00", "SM02": "SM00", "SM03": "SM00", "SM04": "SM00", "SM05": "SM00", "SG01": "SG00", "SG02": "SG00", "SG03": "SG00", "SG04": "SG00"}
        },
        "conditional_parameters_and_bindings": {
            "statuses_and_controls": "rank_power_is_only_rank_part;chance_uses_current_accepted_resistances_and_importance_once;other_attribute_potency_links_remain_separate_review",
            "SF00": "accepted_existing_Agility10_detection_example_and_stealth_rules_preserved;no_new_unreviewed_attribute_curve",
            "SF02": "max1_case_after_existing_stealth_range_times_multiplier;grid_rounding_unchanged_pending_detail",
            "SM02": "variant_damage_bonus_applies_once_to_poison_explosion_and_Poisoned_damage_before_current_attribute_and_defense_pipeline",
            "SM03": "variant_damage_bonus_applies_once_to_cold_explosion;Gel_rules_preserved",
            "SM05": "radius_uses_current_SM04;poison_cloud_uses_current_SM02_bonus_once;cold_exposure_uses_current_SM03;rank_improves_zone_duration_only",
            "SG03": "bonus_added_to_SG00_base_throw_range;not_sound_radius_or_weapon_grenade_launcher_range",
            "SG02": "sound_intensity_uses_actual_sound_propagation_and_terrain_attenuation;not_circular_radius_approximation",
            "TR63": "stored_damage_reserve_added_once;closure_combines_remaining_damage_multiplicatively_as_previously_approved",
            "FR06": "retain_existing_source_and_splash_calculation_rounding_policy;numeric_splash_ratio50_only;no_new_Gel",
            "BM03": "current_charge_count_times_per_charge_bonus;maximum3",
            "BD04": "current_stationary_charge_count_times_bonus;maximum3",
            "HL01_HL02_HL03_HL06": "range_radius_delay_supply_and_native_burst_sequence_taken_from_active_heavy_weapon;rank_coefficient_is_added_pool_not_post_defense_multiplication",
            "HL05": "only_eligible_existing_heavy_explosions;do_not_give_area_effects_to_other_attacks",
            "healing": "missing_HP_actual_removed_primary_HP_and_existing_per_turn_per_use_caps_retained;not_new_spammable_heal"
        },
        "skills": rows,
        "remaining_outside_delegated_scope": [
            "exact_multi_source_status_strength_refresh_and_expiration_scheduler",
            "unshown_attribute_to_control_duration_technique_heal_or_transformation_power_links",
            "actual_native_drone_attack_energy_and_resistance_profiles",
            "DR01_final_tree_placement_and_older_partial_player_copy_receipts",
            "final_class_starts_loot_catalogue_monster_roster_and_first_mission",
            "native_UI_icons_art_animations_and_final_balance"
        ],
        "runtime_integration": "not_integrated"
    }
    braise = POLICIES["FE01"]["raw_fire_damage"]
    # No acquisition-level argument exists. Both learning histories use the
    # same current actor level, rank, equipment and attributes at evaluation.
    assert level_damage_reference(evaluate(braise, 10), 40) == Fraction(343, 2)
    assert level_damage_reference(evaluate(braise, 1), 40) == Fraction(196, 5)
    assert level_damage_reference(evaluate(braise, 10), 40) > level_damage_reference(evaluate(braise, 1), 40)
    for spec in [v for ps in POLICIES.values() for key, v in ps.items() if is_raw_damage_parameter(key)]:
        for rank in [1, 10, 12, 100]:
            value = evaluate(spec, rank)
            assert level_damage_reference(value, 1) == value
            assert level_damage_reference(value, 40) > level_damage_reference(value, 2)
    save(OUT, system)
    save(CHECKS, {"date": "2026-10-09", "review_id": "SKILL-RANKS01-CHECK01",
                  "scope": "design_arithmetic_and_coverage;not_game_engine_tests_or_playtest",
                  "curves": reference(OUT), "total_skills_checked": len(rows),
                  "primary": 79, "secondary": 14, "repair_complement": 1,
                  "constraints": "source identity coverage_no_removed_Foudre_content_positive_integer_costs_when_required_legal_probabilities_monotonic_curves_and_explicit_user_caps_checked",
                  "rank_snapshots": "1_to20_then30_50_100;formula_supports_positive_integer_equipment_rank_beyond10",
                  "current_level_checks": "level1_identity_monotone_tentative_growth_and_learning_history_invariance;same_current_level_rank_attributes_and_equipment_yield_identical_damage",
                  "level_coefficient_status": "tentative_reference_only;direction_accepted;not_final_balance_or_runtime",
                  "learned_transition_details": validation_rows,
                  "runtime_integration": "not_integrated"})
    print(json.dumps({"curves": OUT, "validation": CHECKS, "skills": len(rows),
                      "rank_authority": "explicit_user_delegation", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
