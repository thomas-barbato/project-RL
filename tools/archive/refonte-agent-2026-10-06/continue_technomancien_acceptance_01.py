"""Accept only the displayed Technomancien start and present three remaining cores."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
DECISION = "class-start-technomancien-acceptance-decisions.json"
ACCEPTED = "review-class-start-technomancien-START-O01-01.json"
OLD_BATCH = "review-class-starts-START-ALL01-01.json"
BATCH = "review-class-starts-START-ALL01-02.json"
DOCUMENT = "review-class-starts-START-ALL01-02.md"
PROPOSALS = {
    "START-T01": "review-class-start-traqueur-START-T01-01.json",
    "START-A01": "review-class-start-arcaniste-START-A01-01.json",
    "START-M01": "review-class-start-metamorphe-START-M01-01.json"
}
PAIRS = [("review-implementation-readiness-33.json", "review-implementation-readiness-34.json"),
         ("review-attribute-model-91.json", "review-attribute-model-92.json"),
         ("review-primary-catalogue-263.json", "review-primary-catalogue-264.json"),
         ("review-complete-skill-design-238.json", "review-complete-skill-design-239.json"),
         ("remaining-individual-skill-review-counts-132.json", "remaining-individual-skill-review-counts-133.json")]
NEW_FILES = [DECISION, ACCEPTED] + list(PROPOSALS.values()) + [BATCH] + [new for _, new in PAIRS]
SHOWN_TECHNO = """Pour remplacer Opérateur, je propose Technomancien.

Vous déployez vos drones et infiltrez les défenses ennemies par la technologie et l’occulte.

Son départ proposé :
- Actif : Invocation de l’Artilleur, rang 1, un drone d’attaque.
- Passif : Algorithmes de combat améliorés, +25 % de dégâts des drones.
- Implant : +15 % de PV aux drones, +1 tour aux affaiblissements techno-occultes ; −10 % de dégâts des armes.
- Réserves : 30 PV et 38 énergie.
- Équipement : pistolet et tenue technique.

Les cinq départs sont préparés dans le dossier des classes, encore au stade de conception."""

COPIES = {
    "START-T01": """**START-T01 · Traqueur — proposition**

*Vous traquez vos cibles à distance et échappez à leurs ripostes.*

Force **4** · Agilité **8** · Vitalité **5** · Perception **7** · Intelligence **4** · Volonté **6**.

- **Marquage balistique — rang 1 :** précision **×1,15** contre la cible marquée ; durée maximale **4 tours** ; portée **6 cases** ; **6 énergie** ; recharge **3 tours**. Votre prochaine attaque à distance contre cette cible consomme la marque. Arme à distance équipée et active requise.
- **Maîtrise des attaques à distance — rang 1 :** **+10 % de dégâts des armes et compétences d’attaque à distance**.
- **Implant :** **+10 % de dégâts des armes à distance**, **+2 points d’esquive** ; **−10 % de dégâts des armes de mêlée**.
- **Départ : 30 / 30 PV · 30 / 30 énergie.** Matériel prévu : fusil et tenue légère.""",
    "START-A01": """**START-A01 · Arcaniste — proposition**

*Vous canalisez les forces occultes pour consumer les menaces qui vous barrent la route.*

Force **4** · Agilité **5** · Vitalité **6** · Perception **6** · Intelligence **5** · Volonté **8**.

- **Braise — rang 1 :** **9 dégâts de feu** avant les défenses ; portée **5 cases** ; **7 énergie** avec Canalisation ; recharge **3 tours**. Elle applique aussi sa Brûlure selon les règles déjà validées.
- **Canalisation — rang 1 :** **−20 % au coût en énergie des pouvoirs magiques**.
- **Implant :** **+15 % de dégâts des pouvoirs magiques** ; **−10 % de PV maximum**.
- **Départ : 30 / 30 PV · 36 / 36 énergie.** Matériel prévu : couteau et tenue rituelle.""",
    "START-M01": """**START-M01 · Métamorphe — proposition**

*Vous changez de forme pour libérer la violence enfouie dans votre corps.*

Force **6** · Agilité **6** · Vitalité **6** · Perception **5** · Intelligence **4** · Volonté **7**.

- **Mue prédatrice — rang 1 :** durée **14 tours** avec l’implant ; **14 énergie** avec le malus sous forme humaine ; recharge **10 tours**. Vous disposez des actions propres à la forme déjà validées.
- **Puissance métamorphe — rang 1 :** **+10 % de dégâts lorsque vous êtes transformé**.
- **Implant :** **+2 tours aux transformations** ; **+10 % au coût en énergie des compétences sous forme humaine**.
- **Départ : 34 / 34 PV · 32 / 32 énergie.** Matériel prévu : couteau et tenue métamorphe."""
}


def checksum(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def validate():
    checked, errors = 0, []
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
    for old_name, new_name in PAIRS:
        old, new = load(old_name), load(new_name)
        preserve(old, new)
        for key, value in old.get("active_rule_overrides", {}).items():
            assert new["active_rule_overrides"][key] == value, key
        assert new["accepted_class_start_Technomancien"] == ref(ACCEPTED)
        assert new["pending_class_core_starts"] == {k: ref(v) for k, v in PROPOSALS.items()}
        assert new["runtime_integration"] == "not_integrated"
    old, batch = load(OLD_BATCH), load(BATCH)
    for p in old["profiles"]:
        other = next(v for v in batch["profiles"] if v["review_id"] == p["review_id"])
        for key in ["attributes", "learned_start_skills", "initial_implant_pair", "starting_equipment", "derived_level1_reserves"]:
            assert other[key] == p[key], (p["review_id"], key)
        assert sum(other["attributes"].values()) == 34
    assert batch["rename"]["text_approval"] == "accepted_as_displayed_under_CLASS_NAME_O02"
    assert load(ACCEPTED)["approved_shown_fields"]["public_class_name"] == "Technomancien"
    assert load(ACCEPTED)["approved_shown_fields"]["starting_active"]["skill_id"] == "D59"
    curves = {s["skill_id"]: s for s in load("skill-rank-curves-provisional-02.json")["skills"]}
    for key, name in PROPOSALS.items():
        card = load(name)
        p = card["proposed_core_profile"]
        attrs = p["attributes"]
        base_hp = 30 + 4 * (attrs["Vitalité"] - 5)
        hp_percent = 100 - p["initial_implant_pair"].get("maximum_HP_penalty_percent", 0)
        hp = base_hp * hp_percent // 100
        energy = 30 + 2 * (attrs["Intelligence"] - 5) + 2 * (attrs["Volonté"] - 5)
        assert p["derived_level1_reserves"]["maximum_HP"] == hp
        assert p["derived_level1_reserves"]["maximum_energy"] == energy
        for gift in p["learned_start_skills"]:
            assert gift["learned_rank"] == 1 and gift["skill_id"] in curves
        assert card["mechanic_approval"] == "proposed_for_explicit_review;delegated_balance_not_reopened"
        assert card["player_copy_markdown"] == COPIES[key]
    t01 = curves["T01"]["rank_snapshots"][0]["rank_values"]
    assert (t01["accuracy_bonus_percent"], t01["duration_turns"], t01["range_cells"], t01["energy"], t01["cooldown_turns"]) == (15, 4, 6, 6, 3)
    assert 8 * 118 // 100 == 9
    assert -(-8 * 80 // 100) == 7
    assert -(-12 * 110 // 100) == 14
    assert load("remaining-individual-skill-review-counts-133.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    return checked


def main():
    batch = deepcopy(load(OLD_BATCH))
    techno = next(p for p in batch["profiles"] if p["review_id"] == "START-O01")
    approved = {
        "public_class_name": "Technomancien", "roleplay": techno["roleplay"],
        "starting_active": {"skill_id": "D59", "title": "Invocation de l’Artilleur", "learned_rank": 1},
        "starting_passive": {"skill_id": "O03", "title": "Algorithmes de combat améliorés", "learned_rank": 1, "drone_damage_bonus_percent": 25},
        "implant_values": {"drone_maximum_HP_bonus_percent": 15, "techno_occult_weakening_duration_bonus_turns": 1, "weapon_damage_penalty_percent": 10},
        "starting_reserves": {"maximum_HP": 30, "current_HP": 30, "maximum_energy": 38, "current_energy": 38},
        "equipment_direction": {"weapon": "pistolet", "clothing": "tenue technique"}
    }
    unshown = ["exact_initial_attribute_vector_prepared_under_numeric_class_delegation_not_displayed_in_last_message",
              "specific_weapon_and_armor_values_damage_ranges_and_other_equipment_slots", "shared_consumable_names_counts_values_and_icons",
              "precise_implant_effect_eligibility_and_relative_penalty_pipeline", "native_equipped_creation_preview_and_new_visuals",
              "other_class_RP_and_start_cards_not_approved_by_link_only"]
    save(DECISION, {
        "date": "2026-10-09", "review_id": "START-O01-ACCEPT01", "name_review_id": "CLASS-NAME-O02",
        "user_acceptance": "ca me va :)", "source_batch": ref(OLD_BATCH), "prior_class_authority": ref("class-start-delegation-decisions.json"),
        "shown_copy_markdown": SHOWN_TECHNO, "shown_copy_sha256": checksum(SHOWN_TECHNO),
        "approved_shown_fields": approved, "unshown_not_implicitly_approved": unshown,
        "stable_identity": "START-O01_remains_class_profile_anchor;Operator_name_is_historical_alias;D56_drone_name_unchanged",
        "runtime_integration": "not_integrated"
    })
    save(ACCEPTED, {
        "date": "2026-10-09", "review_id": "START-O01", "revision": 1, "title": "Technomancien — départ présenté accepté",
        "status": "shown_class_name_RP_initial_skills_implant_numbers_reserves_and_equipment_direction_accepted;detailed_equipment_and_unshown_conditions_separate",
        "acceptance": ref(DECISION), "source_batch": ref(OLD_BATCH), "approved_shown_fields": approved,
        "unshown_not_implicitly_approved": unshown,
        "text_approval": "class_name_and_exact_shown_RP_accepted;other_skill_and_item_copy_receipts_preserved",
        "mechanic_approval": "displayed_values_and_starting_D59_O03_binding_accepted;unshown_technical_conditions_not_added",
        "visual_approval": "existing_silhouette_assignment_preserved;new_equipped_composition_and_effects_pending_native_review",
        "runtime_integration": "not_integrated"
    })
    for rid, name in PROPOSALS.items():
        p = next(p for p in batch["profiles"] if p["review_id"] == rid)
        core = {k: deepcopy(p[k]) for k in ["roleplay", "starting_level", "attributes", "fixed_start_attribute_budget", "learned_start_skills", "initial_implant_pair", "derived_level1_reserves"]}
        save(name, {
            "date": "2026-10-09", "review_id": rid, "revision": 1, "title": p["proposed_display_name"] + " — départ proposé",
            "status": "class_core_proposed_for_review;not_runtime_integrated", "source_batch": ref(OLD_BATCH),
            "authority": ref("class-start-delegation-decisions.json"), "proposed_core_profile": core,
            "player_copy_markdown": COPIES[rid], "player_copy_sha256": checksum(COPIES[rid]),
            "copy_scope": "character_creation_start_summary;not_a_replacement_for_full_skill_current_next_rank_tooltips",
            "existing_skill_rules": "preserve_current_approved_skill_cards_and_global_interaction_overrides;no_new_cost_curve_status_or_visual_created",
            "equipment_scope": "weapon_and_clothing_role_only;exact_START_ALL01_equipment_values_remain_separate",
            "mechanic_approval": "proposed_for_explicit_review;delegated_balance_not_reopened", "text_approval": "new_class_RP_proposed;approved_skill_names_preserved",
            "visual_approval": "existing_class_silhouette_assignment_preserved;native_equipped_preview_pending",
            "runtime_integration": "not_integrated"
        })
    techno["historical_class_name"] = techno["approved_class_identity"]
    techno["approved_class_identity"] = "Technomancien"
    techno["name_status"] = "accepted_as_displayed_under_CLASS_NAME_O02"
    techno["core_approval"] = "shown_START_O01_name_RP_skills_implant_numbers_reserves_and_gear_direction_accepted;attributes_prepared_under_delegation;unshown_details_separate"
    techno["new_player_copy"] = "shown_class_RP_accepted"
    techno["acceptance"] = ref(ACCEPTED)
    for p in batch["profiles"]:
        if p["review_id"] in PROPOSALS:
            p["current_core_proposal"] = ref(PROPOSALS[p["review_id"]])
    batch.update(revision=2, status="Bastion_core_and_displayed_Technomancien_start_accepted;three_other_core_cards_presented;detailed_equipment_and_native_review_pending")
    batch["previous_batch"] = ref(OLD_BATCH)
    batch["latest_acceptance"] = ref(DECISION)
    batch["accepted_Technomancien"] = ref(ACCEPTED)
    batch["rename"]["text_approval"] = "accepted_as_displayed_under_CLASS_NAME_O02"
    batch["rename"]["approved_new_name"] = "Technomancien"
    batch["rename"]["acceptance"] = ref(DECISION)
    batch["pending_core_starts"] = {k: ref(v) for k, v in PROPOSALS.items()}
    batch["new_text_approval"] = "Bastion_and_Technomancien_class_RP_accepted;Traqueur_Arcaniste_Metamorphe_RP_presented_for_review"
    batch["equipment_approval"] = "Technomancien_pistol_and_technical_clothing_direction_accepted;exact_values_and_shared_consumables_still_proposed"
    batch["remaining"] = ["three_other_class_core_cards_review", "specific_start_equipment_review", "unshown_initial_implant_scope_and_penalty_pipeline", "first_mission_ammunition_supply", "native_equipped_character_creation_preview", "found_replacement_implant_loot_catalogue"]
    old_doc = load(OLD_BATCH)["player_review_document"]["reference"]
    doc = (ROOT / old_doc).read_text(encoding="utf-8")
    replacements = [
        ("Le Bastion est validé. Les autres départs sont préparés selon votre délégation ; les chiffres restent provisoires pour l’équilibrage. Le nouveau nom **Technomancien**, les nouveaux textes et les compositions équipées sont présentés pour revue. L’Opérateur commence avec l’Artilleur selon votre dernière correction.",
         "Le Bastion est validé. Le nom **Technomancien**, son texte de classe et son départ présenté avec l’Artilleur sont maintenant validés. Ses détails d’équipement et les conditions non affichées restent distincts. Les trois autres départs sont présentés pour revue selon votre délégation ; les chiffres restent provisoires pour l’équilibrage."),
        ("## START-O01 · Technomancien — nom proposé pour l’Opérateur", "## START-O01 · Technomancien — départ présenté validé"),
        ("La classe change de nom dans cette proposition seulement ; les identifiants de revue START-O01, D59 et O03 restent stables.",
         "Le nouveau nom Technomancien est validé ; les identifiants de revue START-O01, D59 et O03 restent stables. L’ancien nom Opérateur reste une référence historique."),
        ("**Reste à revoir avec vous :** le nouveau nom et les nouveaux textes, le matériel proposé, les munitions de départ et l’aperçu équipé de création.",
         "**Reste à revoir avec vous :** les trois autres fiches de départ, les détails non affichés des implants, le matériel proposé, les munitions de départ et l’aperçu équipé de création.")
    ]
    for old, new in replacements:
        assert doc.count(old) == 1, old
        doc = doc.replace(old, new)
    doc += "\n## Fiches de départ présentées dans ce lot\n\n" + "\n\n".join(COPIES.values()) + "\n"
    assert not (BASE / DOCUMENT).exists()
    (BASE / DOCUMENT).write_text(doc, encoding="utf-8", newline="\n")
    batch["player_review_document"] = ref(DOCUMENT)
    save(BATCH, batch)
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data.update(date="2026-10-09", runtime_integration="not_integrated")
        data["accepted_class_start_Technomancien"] = ref(ACCEPTED)
        data["latest_class_start_acceptance"] = ref(DECISION)
        data["previous_prepared_five_class_starts"] = deepcopy(data["prepared_five_class_starts"])
        data["prepared_five_class_starts"] = ref(BATCH)
        data["pending_class_name_and_start_equipment_review"] = ref(BATCH)
        data["class_name_review_status"] = "Technomancien_accepted;other_four_names_preserved"
        data["pending_class_core_starts"] = {k: ref(v) for k, v in PROPOSALS.items()}
        data["next_review_kind"] = "three_remaining_class_core_cards_START_T01_A01_M01"
        if old_name.startswith("review-implementation"):
            data.update(revision=34, status="Technomancien_name_and_shown_start_accepted;three_other_start_cores_presented;equipment_and_native_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY04":
                    group["accepted_Technomancien_start"] = ref(ACCEPTED)
                    group["prepared_five_profiles"] = ref(BATCH)
                    group["pending_core_cards"] = {k: ref(v) for k, v in PROPOSALS.items()}
                    group["remaining"] = ["three_remaining_class_core_cards_and_RP_explicit_review", "specific_start_weapon_armor_and_consumable_proposal_review", "unshown_initial_implant_scope_and_penalty_pipeline", "native_character_creation_layout_and_equipped_preview", "first_mission_ammunition_supply_and_replacement_implant_loot_options"]
                    group["resolved"] = "Bastion_core_and_displayed_Technomancien_name_RP_skills_implant_numbers_reserves_and_equipment_direction_accepted;initial_numeric_arrays_prepared_under_delegation"
            data["recommendation"] = "review_three_remaining_start_cores_then_detailed_equipment_and_native_creation;preserve_accepted_class_and_skill_rules"
        else:
            if old_name.startswith(("review-attribute", "review-primary", "review-complete")):
                data.setdefault("active_rule_overrides", {})["class_start_Technomancien"] = {**ref(ACCEPTED), "acceptance": ref(DECISION), "rule": deepcopy(approved), "scope": "only_fields_displayed_in_last_final;not_entire_linked_dossier"}
            if old_name.startswith("review-attribute"):
                data.update(review_id="STAT-MODEL89", status="Technomancien_displayed_start_accepted;three_remaining_core_starts_proposed;other_rules_preserved")
                pair(data, "previous_model", old_name)
            elif old_name.startswith(("review-primary", "review-complete")):
                data["batch_revision"] += 1
                pair(data, "previous_manifest", old_name)
                pair(data, "latest_decisions", DECISION)
                pair(data, "latest_proposal", BATCH)
                data["latest_proposal_scope"] = "three_remaining_class_core_starts;Technomancien_shown_fields_accepted;equipment_and_native_review_separate"
            else:
                pair(data, "previous_count", old_name)
                data["latest_feedback"] = "Technomancien_name_and_displayed_start_accepted;three_other_start_cards_presented"
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-34.json")
            if not old_name.startswith("review-attribute"):
                pair(data, "current_attribute_model", "review-attribute-model-92.json")
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("technomancien-and-class-core-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "START-O01-THREE-CORES-CHECK01", "files_checked": NEW_FILES,
        "scope": "design_reference_integrity_approval_boundaries_preserved_rules_and_profile_arithmetic;not_runtime_or_balance_tests",
        "hash_references_checked": checked, "hash_errors": [], "prior_accepted_fields_and_overrides": "preserved",
        "Technomancien_approval_scope": "displayed_name_RP_initial_D59_O03_implant_values_reserves_and_gear_direction_only",
        "other_core_cards": "three_presented_proposals;not_accepted", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "hash_checks": checked, "errors": 0, "accepted": "shown_Technomancien_fields", "proposed": 3, "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
