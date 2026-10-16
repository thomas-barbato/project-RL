"""Record three core approvals, universal ammo direction, and review starter gear."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
DECISION = "class-three-core-starts-acceptance-decisions.json"
AMMO = "universal-ammunition-and-matter-removal-decisions.json"
GEAR = "review-starting-equipment-START-GEAR01-01.json"
BATCH = "review-class-starts-START-ALL01-03.json"
DOCUMENT = "review-class-starts-START-ALL01-03.md"
CARDS = [("START-T01", "review-class-start-traqueur-START-T01-01.json", "review-class-start-traqueur-START-T01-02.json"),
         ("START-A01", "review-class-start-arcaniste-START-A01-01.json", "review-class-start-arcaniste-START-A01-02.json"),
         ("START-M01", "review-class-start-metamorphe-START-M01-01.json", "review-class-start-metamorphe-START-M01-02.json")]
PAIRS = [("review-implementation-readiness-34.json", "review-implementation-readiness-35.json"),
         ("review-attribute-model-92.json", "review-attribute-model-93.json"),
         ("review-primary-catalogue-264.json", "review-primary-catalogue-265.json"),
         ("review-complete-skill-design-239.json", "review-complete-skill-design-240.json"),
         ("remaining-individual-skill-review-counts-133.json", "remaining-individual-skill-review-counts-134.json")]
NEW_FILES = [DECISION, AMMO] + [v[2] for v in CARDS] + [GEAR, BATCH] + [v[1] for v in PAIRS]

COPY = """**START-GEAR01 · Matériel de départ — proposition**

*L’organisation vous remet un équipement standard et quelques réserves pour votre première mission.*

Matériel commun de **niveau 1**. Les plages ci-dessous sont les dégâts de base des armes ; vos attributs, compétences et implants s’appliquent ensuite.

**Bastion**

- Hachette : **6–10 dégâts physiques**, portée **1 case**, précision **−12**.
- Tenue d’assaut : **3 points d’armure**, **−5 points d’esquive**.
- Avec son implant, le Bastion commence à **4 points d’armure**. Frappe de rupture affiche **8–14 dégâts physiques** avec cette hachette, avant les défenses et les coups critiques.

**Technomancien**

- Pistolet : **3–5 dégâts physiques**, portée **5 cases**, précision **+5**.
- Tenue technique : **2 points d’armure**.
- **40 munitions** communes.

**Traqueur**

- Fusil de patrouille : **3–5 dégâts physiques**, portée **7 cases**, précision **+5**.
- Tenue de traque : **1 point d’armure**.
- **40 munitions** communes.

**Arcaniste**

- Couteau : **2–4 dégâts physiques**, portée **1 case**, précision **+10**.
- Tenue rituelle : **1 point d’armure**.

**Métamorphe**

- Couteau : **2–4 dégâts physiques**, portée **1 case**, précision **+10**.
- Tenue métamorphe : **2 points d’armure**.

**Réserves communes aux cinq classes**

- **2 Doses de soin** : chacune restaure **12 PV**.
- **1 Cellule d’énergie** : restaure **12 énergie**.

Ces objets sont consommés à l’utilisation ; la récupération reste limitée à votre maximum.

Le pistolet et le fusil de départ consomment **1 munition par projectile**. Les armes lourdes utiliseront le même type de munition ; leur consommation sera définie avec leurs modèles. Les compétences de mines et de grenades conservent leurs coûts en énergie validés.

Ces tenues ne modifient pas les réserves de PV et d’énergie déjà validées. L’aperçu équipé de chaque classe sera présenté à taille native."""


def checksum(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def validate():
    errors, checked = [], 0
    def walk(value):
        nonlocal checked
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
                        elif source.startswith(("artifacts/", "docs/", "content/")):
                            errors.append(("missing", source))
                            continue
                    checked += 1
                    if checksum(content) != expected:
                        errors.append((key, source[:100]))
            for nested in value.values():
                walk(nested)
        elif isinstance(value, list):
            for nested in value:
                walk(nested)
    for name in NEW_FILES:
        walk(load(name))
    for rid, old_name, new_name in CARDS:
        old, new = load(old_name), load(new_name)
        assert new["accepted_core_profile"] == old["proposed_core_profile"]
        assert new["player_copy_markdown"] == old["player_copy_markdown"]
        assert new["text_approval"] == "shown_class_RP_and_displayed_copy_accepted;skill_receipts_preserved"
    for old_name, new_name in PAIRS:
        old, new = load(old_name), load(new_name)
        preserve(old, new)
        for key, value in old.get("active_rule_overrides", {}).items():
            assert new["active_rule_overrides"][key] == value, key
        assert new["pending_class_core_starts"] == {}
        assert new["pending_starting_equipment"] == ref(GEAR)
        assert new["runtime_integration"] == "not_integrated"
    old, batch = load("review-class-starts-START-ALL01-02.json"), load(BATCH)
    for p in old["profiles"]:
        updated = next(v for v in batch["profiles"] if v["review_id"] == p["review_id"])
        for key in ["attributes", "learned_start_skills", "initial_implant_pair", "derived_level1_reserves"]:
            assert updated[key] == p[key], (p["review_id"], key)
        assert sum(updated["attributes"].values()) == 34
    gear = load(GEAR)
    assert gear["mechanic_approval"] == "proposed_for_review"
    assert gear["player_copy_markdown"] == COPY
    assert [(v["class"], v["starting_ammunition_quantity"]) for v in gear["proposed_loadouts"]] == [("Bastion", 0), ("Technomancien", 40), ("Traqueur", 40), ("Arcaniste", 0), ("Métamorphe", 0)]
    h = gear["proposed_loadouts"][0]
    assert (h["active_weapon"]["damage_minimum"], h["active_weapon"]["damage_maximum"]) == (6, 10)
    assert 6 * 142 // 100 == h["derived_B01_preview"]["damage_minimum"] == 8
    assert 10 * 142 // 100 == h["derived_B01_preview"]["damage_maximum"] == 14
    assert h["armor"]["armor"] + 1 == h["derived_B01_preview"]["ordinary_total_armor"] == 4
    assert load(AMMO)["accepted_direction"]["all_ranged_weapon_families_share_one_ammunition_type"] is True
    assert load(AMMO)["accepted_direction"]["includes_heavy_ranged_weapons"] is True
    assert "| ---" not in (BASE / DOCUMENT).read_text(encoding="utf-8")
    assert load("remaining-individual-skill-review-counts-134.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    return checked


def main():
    accepted_refs = {rid: new for rid, _, new in CARDS}
    save(DECISION, {
        "date": "2026-10-09", "review_id": "START-THREE-ACCEPT01", "user_acceptance": "Je valide :)",
        "shown_cards": {rid: ref(old) for rid, old, _ in CARDS},
        "approved_scope": "displayed_Traqueur_Arcaniste_Metamorphe_RP_fixed_attributes_rank1_gifts_implant_numbers_reserves_skill_summary_and_equipment_direction",
        "unshown_not_implicitly_approved": ["specific_start_weapon_and_armor_values", "consumable_counts_values_names_and_icons", "native_equipped_preview", "unshown_implant_classification_and_penalty_pipeline", "unshown_skill_or_visual_cases"],
        "prior_numeric_authority": ref("class-start-delegation-decisions.json"), "runtime_integration": "not_integrated"
    })
    save(AMMO, {
        "date": "2026-10-09", "review_id": "AMMO-UNIVERSAL01",
        "question": "Pour les tirs ordinaires, veux-tu conserver la matière utilisée actuellement comme réserve de munitions, ou passer à des munitions ramassables par famille d’arme ? L’énergie des compétences reste une réserve séparée.",
        "user_answer": "Non en faite, on va supprimer \"la matière\" et mettre les mêmes munitions pour toutes les armes à distance (lourde ou non).",
        "accepted_direction": {"remove_weapon_matter_resource_and_its_player_facing_identity": True,
                               "all_ranged_weapon_families_share_one_ammunition_type": True,
                               "includes_heavy_ranged_weapons": True},
        "preserved": "common_power_energy_and_accepted_energy_only_mine_and_grenade_skills;ordinary_weapon_supply_not_automatically_converted_to_energy",
        "source_observation": "current_weapon_supplies.rs_uses_weapon_matter_inventory_stacks;equipment_upgrade.rs_uses_separate_credit_and_fragment_costs",
        "scope_correction": "prior_commentary_implied_upgrade_matter_use;verified_upgrade_credit_fragment_costs_are_distinct;no_new_currency_or_removal_of_all_crafting_items_inferred",
        "not_fixed_by_direction": ["per_model_ammunition_costs_and_burst_accounting", "starting_stock_quantity", "weight_stack_limits_pickup_and_loot_rates", "reload_magazine_rules", "familiar_and_monster_ammunition_management", "exact_UI_item_and_log_copy_or_native_icon"],
        "implementation_scope": "future_refonte_replaces_weapon_matter_item_supply_events_refusals_LOOT_and_UI_bindings;old_history_and_runtime_files_not_edited_in_this_design_step",
        "runtime_integration": "not_integrated"
    })
    for rid, old_name, new_name in CARDS:
        data = deepcopy(load(old_name))
        data.update(revision=2, status="displayed_class_core_accepted;equipment_values_and_native_review_separate",
                    mechanic_approval="accepted_as_displayed", text_approval="shown_class_RP_and_displayed_copy_accepted;skill_receipts_preserved")
        data["previous_proposal"] = ref(old_name)
        data["acceptance"] = ref(DECISION)
        data["accepted_core_profile"] = data.pop("proposed_core_profile")
        save(new_name, data)
    batch = deepcopy(load("review-class-starts-START-ALL01-02.json"))
    loadouts = []
    for p in batch["profiles"]:
        rid = p["review_id"]
        if rid in accepted_refs:
            p["previous_core_proposal"] = deepcopy(p.pop("current_core_proposal"))
            p["accepted_core_card"] = ref(accepted_refs[rid])
            p["core_approval"] = "displayed_class_core_accepted"
            p["new_player_copy"] = "shown_class_RP_accepted"
        loadout = {"review_id": "START-GEAR01-" + rid[6], "class": p["proposed_display_name"],
                   "active_weapon": deepcopy(p["starting_equipment"]["active_weapon"]),
                   "armor": deepcopy(p["starting_equipment"]["body_armor"]),
                   "starting_ammunition_quantity": 40 if rid in ["START-O01", "START-T01"] else 0,
                   "ammunition_type": "universal_for_ranged_weapons_including_heavy"}
        if rid == "START-B01":
            loadout["derived_B01_preview"] = {"effective_rank": 1, "Force": 7, "skill_extra_percent": 40,
                                            "Force_bonus_percent": 2, "damage_minimum": 8, "damage_maximum": 14,
                                            "ordinary_total_armor": 4, "noncritical_before_defenses": True}
        loadouts.append(loadout)
    save(GEAR, {
        "date": "2026-10-09", "review_id": "START-GEAR01", "revision": 1, "status": "specific_starting_gear_proposed;not_runtime_integrated",
        "source_profile_batch": ref("review-class-starts-START-ALL01-02.json"), "class_approval": ref(DECISION),
        "ammunition_direction": ref(AMMO), "proposed_loadouts": loadouts,
        "proposed_common_consumables": deepcopy(batch["shared_proposed_consumables"]),
        "proposed_start_weapon_ammo_cost": {"start:pistolet": 1, "start:fusil": 1, "unit": "per_projectile"},
        "consumable_effects": "consumed_on_use;restores_12_HP_or_energy_up_to_current_maximum;does_not_change_maximum",
        "equipment_scope": "common_level1_start_entries_no_affixes;own_draft_IDs_preserve_existing_models;no_general_weapon_level_damage_curve_created",
        "preserved_variance": "accepted_inclusive_integer_uniform_weapon_draw;explicit_weapon_ranges_only;no_new_spell_or_transformation_variance",
        "current_damage_preview_scope": "raw_weapon_ranges_identified_as_base;B01_current_range_uses_active_hachette_Force7_and_accepted_additive_skill_pool",
        "unshown_item_copy": "item_roleplay_tooltip_descriptions_not_authored_in_this_lot;shown_names_numeric_effects_and_grant_RP_presented",
        "visual_scope": "existing_approved_class_silhouettes_preserved;equipped_compositions_and_consumable_ammo_icons_pending_native_review",
        "player_copy_markdown": COPY, "player_copy_sha256": checksum(COPY),
        "mechanic_approval": "proposed_for_review", "text_approval": "new_item_names_and_grant_RP_proposed;approved_class_names_preserved",
        "visual_approval": "native_equipped_previews_pending;no_new_pixels_integrated", "runtime_integration": "not_integrated"
    })
    batch.update(revision=3, status="five_class_start_directions_prepared_and_displayed_cores_accepted;exact_equipment_proposed;universal_ammunition_direction_accepted")
    batch["previous_batch"] = ref("review-class-starts-START-ALL01-02.json")
    batch["latest_acceptance"] = ref(DECISION)
    batch["accepted_remaining_core_starts"] = {k: ref(v) for k, v in accepted_refs.items()}
    batch["previous_pending_core_starts"] = deepcopy(batch["pending_core_starts"])
    batch["pending_core_starts"] = {}
    batch["new_text_approval"] = "all_five_shown_class_RP_texts_accepted;new_equipment_grant_and_item_copy_under_START_GEAR01"
    batch["pending_starting_equipment"] = ref(GEAR)
    batch["accepted_ammunition_direction"] = ref(AMMO)
    batch["remaining"] = ["specific_start_equipment_and_consumable_review", "unshown_initial_implant_scope_and_penalty_pipeline", "ammo_model_costs_reload_and_actor_management", "native_equipped_character_creation_preview", "found_replacement_implant_loot_catalogue"]
    old_doc = (ROOT / load("review-class-starts-START-ALL01-02.json")["player_review_document"]["reference"]).read_text(encoding="utf-8")
    doc = "# START-ALL01 — Départs courants et équipement proposé\n\n9 octobre 2026 — conception avant intégration.\n\n"
    doc += "Les trois fiches de classe Traqueur, Arcaniste et Métamorphe sont acceptées telles qu’affichées. Le Bastion et la fiche présentée du Technomancien conservent leurs accords antérieurs. La répartition d’attributs du Technomancien reste préparée sous la délégation numérique. Les détails d’équipement, les conditions non affichées et les nouveaux rendus restent distincts.\n\n"
    doc += "**AMMO-UNIVERSAL01 — décision :** la matière des tirs sera supprimée. Toutes les armes à distance, lourdes comprises, partageront un type de munition. Cette décision ne fixe pas les consommations, stocks, rechargements ou approvisionnements des familiers. Les crédits et fragments de l’artisan sont distincts de l’ancienne réserve de tirs.\n\n"
    doc += COPY + "\n\n## Départs de classe acceptés\n\n"
    for rid, _, name in CARDS:
        doc += load(name)["player_copy_markdown"].replace("— proposition", "— départ accepté") + "\n\n"
    doc += "Les textes et valeurs du Bastion et du Technomancien sont conservés dans leurs fiches acceptées. Les références historiques restent intactes. Les conditions d’implant encore non montrées, le catalogue d’implants trouvés, les consommations d’armes lourdes, les profils de munitions des acteurs et les aperçus équipés ont leurs revues distinctes. Aucun code de jeu ni pixel n’est modifié ici.\n"
    assert not (BASE / DOCUMENT).exists()
    (BASE / DOCUMENT).write_text(doc, encoding="utf-8", newline="\n")
    batch["player_review_document"] = ref(DOCUMENT)
    save(BATCH, batch)
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data.update(date="2026-10-09", runtime_integration="not_integrated")
        data["previous_pending_class_core_starts"] = deepcopy(data["pending_class_core_starts"])
        data["pending_class_core_starts"] = {}
        data["accepted_remaining_class_core_starts"] = {k: ref(v) for k, v in accepted_refs.items()}
        data["latest_class_start_acceptance"] = ref(DECISION)
        data["prepared_five_class_starts"] = ref(BATCH)
        data["pending_class_name_and_start_equipment_review"] = ref(BATCH)
        data["pending_starting_equipment"] = ref(GEAR)
        data["accepted_universal_ammunition_direction"] = ref(AMMO)
        data["next_review_kind"] = "START_GEAR01_exact_initial_equipment_and_consumables"
        if old_name.startswith("review-implementation"):
            data.update(revision=35, status="three_remaining_class_cores_accepted;universal_ammo_direction_accepted;starting_equipment_proposed")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY03":
                    group["accepted_ammunition_direction"] = ref(AMMO)
                    group["remaining"].append("replace_weapon_matter_with_universal_ammunition;per_model_costs_reload_weight_loot_and_familiar_monster_supply_need_review")
                elif group["id"] == "READY04":
                    group["previous_pending_core_cards"] = deepcopy(group["pending_core_cards"])
                    group["pending_core_cards"] = {}
                    group["accepted_remaining_core_cards"] = {k: ref(v) for k, v in accepted_refs.items()}
                    group["prepared_five_profiles"] = ref(BATCH)
                    group["pending_equipment"] = ref(GEAR)
                    group["remaining"] = ["specific_start_weapon_armor_ammunition_stock_and_consumable_review", "unshown_initial_implant_scope_and_penalty_pipeline", "native_character_creation_layout_and_equipped_preview", "ammunition_costs_reload_actor_management_and_replacement_implant_loot"]
                    group["resolved"] = "displayed_core_cards_of_all_five_classes_accepted;Technomancien_attribute_vector_prepared_under_delegation;universal_ammunition_direction_accepted"
            data["recommendation"] = "review_START_GEAR01_then_unshown_implant_scope_ammunition_profiles_and_native_equipped_creation"
        else:
            if old_name.startswith(("review-attribute", "review-primary", "review-complete")):
                overrides = data.setdefault("active_rule_overrides", {})
                for rid, _, name in CARDS:
                    overrides["class_start_" + rid] = {**ref(name), "acceptance": ref(DECISION), "rule": deepcopy(load(name)["accepted_core_profile"])}
                overrides["universal_ammunition_direction"] = {**ref(AMMO), "rule": deepcopy(load(AMMO)["accepted_direction"]), "scope": "direction_only;unshown_cost_and_actor_supply_rules_not_created"}
            if old_name.startswith("review-attribute"):
                data.update(review_id="STAT-MODEL90", status="three_remaining_class_cores_accepted;universal_ammunition_direction;equipment_proposed;other_rules_preserved")
                pair(data, "previous_model", old_name)
            elif old_name.startswith(("review-primary", "review-complete")):
                data["batch_revision"] += 1
                pair(data, "previous_manifest", old_name)
                pair(data, "latest_decisions", DECISION)
                pair(data, "latest_proposal", GEAR)
                data["latest_proposal_scope"] = "specific_level1_starting_gear_consumables_and_ammo_stock;class_cores_and_universal_ammo_direction_accepted"
            else:
                pair(data, "previous_count", old_name)
                data["latest_feedback"] = "three_class_cores_accepted;universal_ammo_direction_recorded;starter_gear_proposed"
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-35.json")
            if not old_name.startswith("review-attribute"):
                pair(data, "current_attribute_model", "review-attribute-model-93.json")
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("class-cores-and-start-gear-validation-01.json", {
        "date": "2026-10-09", "review_id": "START-GEAR01-CHECK01", "files_checked": NEW_FILES,
        "scope": "design_reference_integrity_approval_scope_old_rule_preservation_and_B01_weapon_preview;not_runtime_or_balance_tests",
        "hash_references_checked": checked, "hash_errors": [], "class_core_profiles": "accepted_without_numeric_change",
        "prior_accepted_fields_and_overrides": "preserved", "universal_ammunition": "direction_accepted;stocks_and_costs_proposed",
        "Bastion_B01_preview": "8_to14_predefense_noncritical_and_armor4_checked", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "hash_checks": checked, "errors": 0, "accepted_cores": 3, "ammo_direction": "accepted", "equipment": "proposed", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
