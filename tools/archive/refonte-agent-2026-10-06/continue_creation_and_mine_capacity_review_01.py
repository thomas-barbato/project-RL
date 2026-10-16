"""Record accepted creation scope; prepare mine capacity loss for review only."""
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
DECISION = "invoked-creation-scope-acceptance-decisions.json"
ACCEPTED = "review-invoked-creation-scope-INTERACT-RANK02-02.json"
PROPOSAL = "review-mine-passive-capacity-loss-INTERACT-RANK03-01.json"
PAIRS = [("review-implementation-readiness-07.json", "review-implementation-readiness-08.json"),
         ("review-attribute-model-65.json", "review-attribute-model-66.json"),
         ("review-primary-catalogue-237.json", "review-primary-catalogue-238.json"),
         ("review-complete-skill-design-212.json", "review-complete-skill-design-213.json"),
         ("remaining-individual-skill-review-counts-106.json", "remaining-individual-skill-review-counts-107.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """Pour Réseau de mines, je propose que perdre des rangs du passif applique immédiatement la nouvelle limite. Seules les mines en trop disparaissent, en commençant par les plus anciennes, sans explosion ni remboursement d'énergie. Les mines en attente d'activation comptent aussi dans cette limite.

Exemple : avec neuf rangs appris et un rang d'équipement, vous pouvez avoir cinq mines. Vous retirez ce bonus : la limite passe à quatre. Si cinq mines sont présentes, la plus ancienne disparaît et les quatre autres restent.

Si c'est Pose de mine qui perd un rang, toutes les mines qu'elle a créées disparaissent, conformément à la règle déjà validée."""


def common(data):
    data["date"] = "2026-10-09"
    data["previous_pending_invoked_creation_scope"] = deepcopy(data["pending_invoked_creation_scope"])
    data["pending_invoked_creation_scope"] = None
    data["accepted_invoked_creation_scope"] = ref(ACCEPTED)
    data["latest_creation_scope_acceptance"] = ref(DECISION)
    data["pending_mine_passive_capacity_loss"] = ref(PROPOSAL)
    data["runtime_integration"] = "not_integrated"


def override(data):
    data.setdefault("active_rule_overrides", {})["invoked_creation_scope"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "included": "own_drones_raised_allies_placed_mines_spell_ground_zones_ice_wall_bloody_ritual_segments_and_persistent_map_decoys",
        "creator": "creating_active_skill_of_this_player;supporting_passive_is_not_creator",
        "rank_decrease": "remove_all_own_creations_of_affected_creator_skill_even_when_personally_learned;actual_familiar_items_drop_under_existing_rules",
        "ordinary_effects": "attached_buffs_debuffs_and_transformations_keep_existing_duration_power_snapshot_and_final_access_loss_rules",
        "passive_example": "DP02_Blindage_ameliore_rank_loss_retains_drones_and_updates_applicable_protection",
        "pending_cases": "cross_skill_capacity_or_variant_passives_predeath_marks_delayed_jobs_cleanup_event_order_and_native_feedback"
    }


def main():
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-RANK02-ACCEPT",
        "source_proposal": ref("review-invoked-creation-scope-INTERACT-RANK02-01.json"),
        "user_acceptance": "oui ca me va :)",
        "shown_explanation": "Créations invoquées : drones, monstres relevés, mines, zones, murs de glace, segments du cercle rituel et leurres persistants. Seules les créations de la compétence dont le rang effectif baisse disparaissent, même si elle reste apprise. Les objets portés par les familiers supprimés tombent au sol. Perdre +1 Mur de glace supprime les murs ; perdre un rang de Blindage amélioré conserve les drones avec une protection réduite. Les buffs, débuffs et transformations gardent leurs règles de durée et de perte d'accès.",
        "approved_classification": load("review-invoked-creation-scope-INTERACT-RANK02-01.json")["proposed_classification"],
        "approval_boundary": "shown_invoked_creation_classification_and_creator_vs_supporting_passive_distinction;capacity_variant_predeath_mark_delayed_job_and_event_order_cases_not_newly_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = load("review-invoked-creation-scope-INTERACT-RANK02-01.json")
    accepted.update(revision=2, status="creation_scope_explicitly_accepted;capacity_and_other_unshown_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="design_explanation_accepted;no_new_skill_RP_or_name")
    accepted["previous_proposal"] = ref("review-invoked-creation-scope-INTERACT-RANK02-01.json")
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_classification"] = accepted.pop("proposed_classification")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-RANK03", "revision": 1,
        "title": "Perte de rang de Réseau de mines",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref(ACCEPTED), ref("review-individual-description-SM01-02.json"),
                         ref("skill-rank-curves-provisional-02.json"), ref("review-equipment-grant-loss-effects-GEAR-SK02-03.json")],
        "scope": "SM01_supporting_capacity_passive_loss_when_SM00_creator_rank_and_access_remain_unchanged",
        "already_accepted": "SM01_is_not_mine_creator;SM00_rank_loss_removes_all_own_SM00_creations;new_placement_at_capacity_replaces_oldest_mine;one_mine_per_cell;learned_cap10_and_gear_growth_preserved",
        "proposed_rule": {
            "capacity": "use_new_current_SM01_passive_limit_immediately;when_passive_unavailable_use_existing_SM00_baseline_capacity",
            "overcapacity": "remove_only_excess_own_placed_mines_oldest_placement_first_until_count_fits;retain_mines_if_count_already_fits",
            "counted": "armed_and_not_yet_armed_placed_mines",
            "removal": "no_explosion_damage_or_energy_refund",
            "retained_mines": "ordinary_existing_activation_and_trigger_rules_preserved;no_recast_or_free_rearming",
            "creator_rank_loss": "accepted_all_creations_cleanup_takes_precedence_if_SM00_rank_also_decreases;no_capacity_trim_exception_to_creator_rule"
        },
        "shown_example": {"learned_SM01_rank": 9, "old_equipment_bonus": 1, "new_equipment_bonus": 0,
                          "old_effective_rank": 10, "new_effective_rank": 9, "old_capacity": 5, "new_capacity": 4,
                          "placed_mines_before": ["oldest", "second", "third", "fourth", "newest"],
                          "removed": ["oldest"], "retained": ["second", "third", "fourth", "newest"],
                          "source_of_numbers": "existing_delegated_SM01_curve;illustrative_inventory_not_real_save"},
        "shown_copy_markdown": COPY,
        "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["lost_mine_element_or_persistent_residue_variant_passive_and_existing_clouds",
                              "predeath_resurrection_marks_and_raised_birth_sampling", "exact_cleanup_and_equipment_transaction_event_order",
                              "same_age_technical_tiebreakers_and_native_removal_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_skill_RP_or_name",
        "visual_approval": "no_new_native_art_or_animation_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        common(data)
        if old_name.startswith("review-implementation"):
            data.update(revision=8, status="creation_classification_accepted;mine_passive_capacity_loss_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-RANK02"] = {
                "rule": "creator_active_skill_cleanup_covers_own_actors_and_persistent_map_creations;supporting_passives_not_creators;ordinary_effect_rules_preserved",
                "source": ref(ACCEPTED), "unshown": "capacity_variants_marks_and_event_order"
            }
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if s not in {
                        "invoked_persistent_creation_classification_proposed_under_INTERACT_RANK02",
                        "invocation_skill_rank_decrease_cleanup_now_accepted_for_only_its_elements;non_actor_classification_and_passive_capacity_dependency_pending"}]
                    group["remaining"].append("mine_passive_capacity_loss_proposed_under_INTERACT_RANK03;variant_passives_and_resurrection_marks_pending")
            data["remaining_after_user_directions"][0] = "partial_non_numeric_receipt_consolidation;creation_classification_now_accepted"
            data["recommendation"] = "review_mine_capacity_and_variant_passives_resurrection_marks_and_remaining_turn_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            override(data)
            data.update(review_id="STAT-MODEL63", status="creation_scope_accepted;mine_capacity_loss_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-08.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            override(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-66.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-08.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "SM01_capacity_passive_loss_and_oldest_excess_mine_cleanup_only;creation_classification_accepted"
            data["next_review_kind"] = "mine_passive_capacity_loss_INTERACT_RANK03"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-66.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-08.json")
            data["latest_feedback"] = "user_accepts_RANK02;mine_passive_capacity_loss_proposed"
            data["next_review_kind"] = "mine_passive_capacity_loss_INTERACT_RANK03"
        preserve(old, data)
        save(new_name, data)
    print(json.dumps({"accepted": "INTERACT-RANK02", "proposed": "INTERACT-RANK03", "runtime": "not_integrated"}))


def check():
    errors, checked = [], []
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
            for index, nested in enumerate(value):
                walk(nested, context + "/" + str(index))
    for name in NEW_FILES:
        walk(load(name), name)
    for old_name, new_name in PAIRS:
        old, new = load(old_name), load(new_name)
        preserve(old, new)
        for key, value in old.get("active_rule_overrides", {}).items():
            assert new["active_rule_overrides"][key] == value
        assert new["pending_invoked_creation_scope"] is None
        assert new["runtime_integration"] == "not_integrated"
    accepted, proposal = load(ACCEPTED), load(PROPOSAL)
    assert accepted["mechanic_approval"] == "accepted_as_displayed"
    assert accepted["accepted_classification"] == load("review-invoked-creation-scope-INTERACT-RANK02-01.json")["proposed_classification"]
    assert proposal["mechanic_approval"] == "pending_user_review"
    sm01 = next(s for s in load("skill-rank-curves-provisional-02.json")["skills"] if s["skill_id"] == "SM01")
    snapshots = sm01["rank_snapshots"]
    for rank, expected in [(9, 4), (10, 5)]:
        item = next(s for s in snapshots if s["effective_rank"] == rank)
        assert item["rank_values"]["maximum_simultaneous_mines"] == expected
    assert len(proposal["shown_example"]["retained"]) == proposal["shown_example"]["new_capacity"]
    assert load("remaining-individual-skill-review-counts-107.json")["remaining_first_pass_total"] == 0
    tail = (ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md").read_text(encoding="utf-8").splitlines()[-85:]
    assert not any(line.rstrip() != line for line in tail)
    assert not errors, errors
    save("creation-and-mine-capacity-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-RANK02-RANK03-CHECK01",
        "scope": "design_references_preserved_rules_and_existing_capacity_example;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "catalogues_old_accepted_fields_and_overrides": "preserved",
        "RANK02": "accepted", "RANK03": "proposal_only", "mine_capacity_example": "rank10_five_to_rank9_four_checked_against_delegated_snapshots",
        "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "creation_scope": "accepted", "mine_capacity_loss": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    check() if sys.argv[1:] == ["--check"] else main()
