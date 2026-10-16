"""Prepare delegated class starts for review; never edit runtime content or pixels."""
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
DECISION = "class-start-delegation-decisions.json"
ACCEPTED = "review-class-start-bastion-START-B01-02.json"
BATCH = "review-class-starts-START-ALL01-01.json"
DOCUMENT = "review-class-starts-START-ALL01-01.md"
PAIRS = [("review-implementation-readiness-32.json", "review-implementation-readiness-33.json"),
         ("review-attribute-model-90.json", "review-attribute-model-91.json"),
         ("review-primary-catalogue-262.json", "review-primary-catalogue-263.json"),
         ("review-complete-skill-design-237.json", "review-complete-skill-design-238.json"),
         ("remaining-individual-skill-review-counts-131.json", "remaining-individual-skill-review-counts-132.json")]
NEW_FILES = [DECISION, ACCEPTED, BATCH] + [new for _, new in PAIRS]
ATTRIBUTES = ["Force", "Agilité", "Vitalité", "Perception", "Intelligence", "Volonté"]


def file_ref(relative):
    text = (ROOT / relative).read_text(encoding="utf-8")
    return {"reference": relative, "reference_sha256": hashlib.sha256(text.encode("utf-8")).hexdigest()}


def integer_floor(value):
    return value.numerator // value.denominator


def profile(review_id, identity, proposed_name, roleplay, stats, active, passive, implant, weapon, armor):
    attrs = dict(zip(ATTRIBUTES, stats))
    hp_base = 30 + 4 * (attrs["Vitalité"] - 5)
    energy_base = 30 + 2 * (attrs["Intelligence"] - 5) + 2 * (attrs["Volonté"] - 5)
    hp_percent = (10 if passive == "B03" else 0) - implant.get("maximum_HP_penalty_percent", 0)
    energy_percent = -implant.get("maximum_energy_penalty_percent", 0)
    hp = hp_base * (100 + hp_percent) // 100
    energy = energy_base * (100 + energy_percent) // 100
    return {
        "review_id": review_id, "approved_class_identity": identity, "proposed_display_name": proposed_name,
        "name_status": "proposed_awaiting_explicit_acceptance" if identity != proposed_name else "previously_accepted",
        "roleplay": roleplay, "starting_level": 1, "attributes": attrs,
        "fixed_start_attribute_budget": 34,
        "learned_start_skills": [{"skill_id": active, "learned_rank": 1, "kind": "active"},
                                 {"skill_id": passive, "learned_rank": 1, "kind": "passive"}],
        "initial_implant_pair": implant,
        "starting_equipment": {"active_weapon": weapon, "body_armor": armor,
                               "other_equipment_slots": "no_additional_start_bonus_in_this_draft"},
        "derived_level1_reserves": {"base_maximum_HP": hp_base, "maximum_HP": hp, "current_HP": hp,
                                    "base_maximum_energy": energy_base, "maximum_energy": energy, "current_energy": energy,
                                    "equipment_HP_or_energy_bonuses": 0},
        "core_approval": "accepted_as_displayed_under_START_B01" if review_id == "START-B01" else "prepared_under_class_delegation",
        "new_player_copy": "presented_for_review;not_yet_explicitly_accepted" if review_id != "START-B01" else "accepted_START_B01_RP_preserved",
        "equipment_balance": "specific_level1_proposal_for_review;not_a_general_weapon_progression_curve",
        "native_creation_preview": "existing_numbered_class_silhouette_preserved;equipped_composition_pending_native_review",
        "runtime_integration": "not_integrated"
    }


def make_profiles():
    # Separate proposed level-1 entries: existing model identities/statistics are not overwritten.
    weapons = {
        "hachette": {"draft_id": "start:hachette", "display_name": "Hachette", "level": 1,
                     "damage_minimum": 6, "damage_maximum": 10, "damage_family": "physical", "physical_subtype": "kinetic",
                     "range_cells": 1, "accuracy_modifier": -12, "reference_model": "core:melee_hachette",
                     "reference_content": file_ref("content/core/weapons/melee_hachette.json5")},
        "pistolet": {"draft_id": "start:pistolet", "display_name": "Pistolet", "level": 1,
                     "damage_minimum": 3, "damage_maximum": 5, "damage_family": "physical", "physical_subtype": "piercing",
                     "range_cells": 5, "accuracy_modifier": 5, "reference_model": "core:modele_cogneur_p01",
                     "reference_content": file_ref("content/core/weapons/modele_cogneur_p01.json5")},
        "fusil": {"draft_id": "start:fusil", "display_name": "Fusil de patrouille", "level": 1,
                  "damage_minimum": 3, "damage_maximum": 5, "damage_family": "physical", "physical_subtype": "piercing",
                  "range_cells": 7, "accuracy_modifier": 5, "reference_model": "core:fusil_de_patrouille",
                  "reference_content": file_ref("content/core/weapons/fusil_de_patrouille.json5")},
        "couteau": {"draft_id": "start:couteau", "display_name": "Couteau", "level": 1,
                    "damage_minimum": 2, "damage_maximum": 4, "damage_family": "physical", "physical_subtype": "piercing",
                    "range_cells": 1, "accuracy_modifier": 10, "reference_model": "core:melee_couteau_de_camp",
                    "reference_content": file_ref("content/core/weapons/melee_couteau_de_camp.json5")}
    }
    def armor(identity, title, armor_value, evasion_penalty=0):
        return {"draft_id": identity, "display_name": title, "armor": armor_value,
                "evasion_penalty": evasion_penalty, "other_stat_or_skill_bonus": None,
                "art_assignment": "approved_class_silhouette_only;new_equipped_preview_not_approved"}
    common_implant_rule = {"replacement": "found_implant_replaces_entire_pair;learned_skills_retained",
                          "resource_changes": "ordinary_maximum_change_keeps_absolute_current_resource_then_clamps;no_heal"}
    def implant(**values):
        return common_implant_rule | values
    return [
        profile("START-B01", "Bastion", "Bastion", "Vous encaissez les coups et ouvrez un passage au cœur de la mêlée.",
                [7, 5, 7, 5, 4, 6], "B01", "B03", implant(flat_armor_bonus=1, maximum_energy_penalty_percent=10),
                deepcopy(weapons["hachette"]), armor("start:tenue_assaut", "Tenue d’assaut", 3, 5)),
        profile("START-O01", "Opérateur", "Technomancien", "Vous déployez vos drones et infiltrez les défenses ennemies par la technologie et l’occulte.",
                [4, 5, 5, 6, 8, 6], "D59", "O03",
                implant(drone_maximum_HP_bonus_percent=15, techno_occult_weakening_duration_bonus_turns=1,
                        weapon_damage_penalty_percent=10),
                deepcopy(weapons["pistolet"]), armor("start:tenue_technique", "Tenue technique", 2)),
        profile("START-T01", "Traqueur", "Traqueur", "Vous traquez vos cibles à distance et échappez à leurs ripostes.",
                [4, 8, 5, 7, 4, 6], "T01", "T03",
                implant(ranged_weapon_damage_bonus_percent=10, flat_evasion_bonus=2, melee_weapon_damage_penalty_percent=10),
                deepcopy(weapons["fusil"]), armor("start:tenue_traque", "Tenue de traque", 1)),
        profile("START-A01", "Arcaniste", "Arcaniste", "Vous canalisez les forces occultes pour consumer les menaces qui vous barrent la route.",
                [4, 5, 6, 6, 5, 8], "FE01", "A03",
                implant(magic_damage_bonus_percent=15, maximum_HP_penalty_percent=10),
                deepcopy(weapons["couteau"]), armor("start:tenue_rituelle", "Tenue rituelle", 1)),
        profile("START-M01", "Métamorphe", "Métamorphe", "Vous changez de forme pour libérer la violence enfouie dans votre corps.",
                [6, 6, 6, 5, 4, 7], "M01", "M03",
                implant(transformation_duration_bonus_turns=2, human_skill_energy_cost_increase_percent=10),
                deepcopy(weapons["couteau"]), armor("start:tenue_metamorphe", "Tenue métamorphe", 2))
    ]


def make_document(profiles):
    rows = ["# START-ALL01 — Départ des cinq classes", "", "9 octobre 2026 — dossier de conception, avant intégration.", "",
            "Le Bastion est validé. Les autres départs sont préparés selon votre délégation ; les chiffres restent provisoires pour l’équilibrage. Le nouveau nom **Technomancien**, les nouveaux textes et les compositions équipées sont présentés pour revue. L’Opérateur commence avec l’Artilleur selon votre dernière correction.", "",
            "Chaque classe commence au niveau 1 avec **34 points d’attributs fixes**, une compétence active et un passif déjà appris au **rang 1**. Aucun point libre supplémentaire n’est ajouté au départ. Tous les arbres restent accessibles à toutes les classes. Les gains aux niveaux restent de cinq points d’attributs et un point de compétence.", "",
            "Les armes et tenues de ce dossier sont des propositions de matériel commun de niveau 1, sans affixe ni bonus de compétence. Les plages de dégâts concernent ces quatre armes seulement ; les modèles existants ne sont pas modifiés. Les tirages suivent la distribution entière uniforme déjà validée. Les PV et l’énergie commencent au maximum.", "",
            "**Équipement commun proposé :** deux Doses de soin, chacune restaurant **12 PV**, et une Cellule d’énergie restaurant **12 énergie**, dans la limite du maximum. Ce sont des objets consommables. Leurs noms, descriptifs et icônes restent à valider ; les objets actuels Patch de réparation et Batterie chargée ne sont pas renommés ni réinterprétés dans le code.", "",
            "Les ressources matérielles pour les armes à distance et leurs chargeurs seront fixés avec la première mission et le système de munitions. Le dossier ne transforme pas les tirs ordinaires en pouvoirs consommant de l’énergie.", ""]
    skills = {
        "START-B01": ["**Frappe de rupture — rang 1 :** +40 % de dégâts d’arme de mêlée ; **6 énergie** ; recharge **3 tours**. Arme de mêlée active requise.",
                       "**Vitalité améliorée — rang 1 :** **+10 % de PV maximum**."],
        "START-O01": ["**Invocation de l’Artilleur — rang 1 :** **12 énergie** ; recharge **6 tours**. Le drone suit votre niveau et utilise son propre profil d’attributs.",
                       "**Algorithmes de combat améliorés — rang 1 :** **+25 % de dégâts des drones**."],
        "START-T01": ["**Marquage balistique — rang 1 :** **+15 % de précision** contre la cible marquée ; durée **4 tours** ; portée **6 cases** ; **6 énergie** ; recharge **3 tours**. La prochaine attaque à distance compatible consomme la marque. Arme à distance active requise.",
                       "**Maîtrise des attaques à distance — rang 1 :** **+10 % de dégâts des armes et compétences d’attaque à distance**."],
        "START-A01": ["**Braise — rang 1 :** **9 dégâts de feu** avant les défenses ; portée **5 cases** ; **7 énergie** après Canalisation ; recharge **3 tours**. La Brûlure conserve les règles validées.",
                       "**Canalisation — rang 1 :** **−20 % au coût en énergie des pouvoirs magiques**."],
        "START-M01": ["**Mue prédatrice — rang 1 :** durée **14 tours** avec l’implant ; **14 énergie** avec le malus humain ; recharge **10 tours**. Les actions propres à la forme restent celles validées.",
                       "**Puissance métamorphe — rang 1 :** **+10 % de dégâts lorsque vous êtes transformé**."]
    }
    implant_copy = {
        "START-B01": "+1 point d’armure ; −10 % d’énergie maximum.",
        "START-O01": "+15 % de PV maximum aux drones ; +1 tour aux affaiblissements techno-occultes ; −10 % de dégâts de vos armes.",
        "START-T01": "+10 % de dégâts de vos armes à distance ; +2 points d’esquive ; −10 % de dégâts de vos armes de mêlée.",
        "START-A01": "+15 % de dégâts des pouvoirs magiques ; −10 % de PV maximum.",
        "START-M01": "+2 tours aux transformations ; +10 % au coût en énergie des compétences sous forme humaine. Le coût d’entrée en transformation reçoit ce malus."
    }
    for p in profiles:
        rid = p["review_id"]
        name = p["proposed_display_name"]
        if name != p["approved_class_identity"]:
            name += " — nom proposé pour l’Opérateur"
        rows += [f"## {rid} · {name}", "", f"*{p['roleplay']}*", "", "**Attributs fixes**", "",
                 " · ".join(f"{k} **{v}**" for k, v in p["attributes"].items()), "", "**Compétences initiales**", ""]
        rows += ["- " + value for value in skills[rid]]
        rows += ["", "**Implant initial**", "", implant_copy[rid], "",
                 "Un implant trouvé peut remplacer cette paire. Les compétences apprises restent acquises.", "",
                 "**Équipement proposé**", ""]
        w = p["starting_equipment"]["active_weapon"]
        a = p["starting_equipment"]["body_armor"]
        case_label = "case" if w["range_cells"] == 1 else "cases"
        armor_label = "point" if a["armor"] == 1 else "points"
        rows += [f"- Arme active : {w['display_name']} — **{w['damage_minimum']}–{w['damage_maximum']} dégâts physiques de base**, portée **{w['range_cells']} {case_label}**, précision **{w['accuracy_modifier']:+d}**.",
                 f"- {a['display_name']} : **{a['armor']} {armor_label} d’armure**" + (f", **−{a['evasion_penalty']} points d’esquive**." if a["evasion_penalty"] else "."),
                 "- Les consommables communs proposés ci-dessus.", ""]
        reserves = p["derived_level1_reserves"]
        rows += [f"**Départ : {reserves['current_HP']} / {reserves['maximum_HP']} PV · {reserves['current_energy']} / {reserves['maximum_energy']} énergie.**", ""]
    rows += ["## Conditions de conception", "",
             "Les bonus et malus d’implant concernent le personnage ou les drones indiqués, sans verrouiller les arbres. Les pénalités de dégâts proposées multiplient les dégâts compatibles par 0,90 après les bonus positifs ; elles ne retirent pas dix points à la somme des bonus. Aucun arrondi intermédiaire n’est ajouté.", "",
             "Pour l’implant du Technomancien, le tour supplémentaire concerne Interférence, Faille d’armure, Silence, Rupture de régénération et Désorientation lors d’un lancement compatible. Il ne prolonge pas implicitement les poisons, brûlures, saignements ou gels. Il ne modifie pas rétroactivement les effets déjà lancés.", "",
             "Les classifications d’armes lourdes, d’effets indirects et d’actions hybrides encore non revues ne sont pas résolues par une fiche de classe. Les bonus et contraintes des compétences gardent leurs règles de revue courantes, qui priment sur les anciens exemples.", "",
             "Le Métamorphe reçoit **M01**, la Mue prédatrice liée au Prédateur 62, et non **TR61**, le Colosse biomécanique. La durée d’entrée conserve le bonus de Volonté déjà défini ; avec Volonté 7, il n’ajoute aucun tour. Douze tours de base plus deux de l’implant donnent quatorze tours. L’énergie est arrondie une seule fois à l’entier supérieur : 12 × 1,10 donne 14.", "",
             "Le drone initial du Technomancien est **D59**, l’Artilleur. La réponse précédente en faveur de D56 est remplacée par votre correction. L’Éclaireur conserve sa place de drone de reconnaissance accessible par apprentissage ; aucun seuil de déblocage n’est réintroduit. La classe change de nom dans cette proposition seulement ; les identifiants de revue START-O01, D59 et O03 restent stables.", "",
             "Les cinq silhouettes déjà acceptées restent affectées aux mêmes classes. Les tenues de départ de ce dossier ont leurs propres valeurs proposées ; les statistiques des anciens plastrons, robes ou carapaces ne sont pas copiées à partir de leurs dessins. Les nouvelles compositions équipées de création seront montrées à taille native avant intégration.", "",
             "**Reste à revoir avec vous :** le nouveau nom et les nouveaux textes, le matériel proposé, les munitions de départ et l’aperçu équipé de création. Les options d’implants trouvés relèvent du catalogue de loot. Ce dossier ne constitue ni un test d’équilibrage ni une intégration dans le jeu.", ""]
    return "\n".join(rows)


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
            assert new["active_rule_overrides"][key] == value, key
        assert new["pending_class_start_Bastion"] is None
        assert new["runtime_integration"] == "not_integrated"
    batch = load(BATCH)
    profiles = batch["profiles"]
    assert len(profiles) == 5
    assert len({p["review_id"] for p in profiles}) == 5
    assert [(p["derived_level1_reserves"]["maximum_HP"], p["derived_level1_reserves"]["maximum_energy"]) for p in profiles] == [(41, 27), (30, 38), (30, 30), (30, 36), (34, 32)]
    curves = {s["skill_id"]: s for s in load("skill-rank-curves-provisional-02.json")["skills"]}
    for p in profiles:
        assert sum(p["attributes"].values()) == 34
        assert [s["kind"] for s in p["learned_start_skills"]] == ["active", "passive"]
        for s in p["learned_start_skills"]:
            assert s["learned_rank"] == 1 and s["skill_id"] in curves
    assert profiles[1]["learned_start_skills"][0]["skill_id"] == "D59"
    assert profiles[4]["learned_start_skills"][0]["skill_id"] == "M01"
    b01 = curves["B01"]["rank_snapshots"][0]["rank_values"]
    assert b01["damage_coefficient_percent"] == 140 and b01["energy"] == 6
    assert integer_floor(Fraction(8 * 118, 100)) == batch["derived_preview_checks"]["Arcaniste_Braise_direct_predefense_damage"] == 9
    assert -(-8 * 80 // 100) == batch["derived_preview_checks"]["Arcaniste_Braise_energy"] == 7
    assert -(-12 * 110 // 100) == batch["derived_preview_checks"]["Metamorphe_Mue_energy"] == 14
    assert 12 + 2 == batch["derived_preview_checks"]["Metamorphe_Mue_duration"] == 14
    assert load(ACCEPTED)["accepted_profile"] == load("review-class-start-bastion-START-B01-01.json")["proposed_profile"]
    assert batch["rename"]["text_approval"] == "proposed_not_yet_explicitly_accepted"
    assert batch["equipment_approval"] == "presented_proposal_not_yet_explicitly_accepted"
    assert "| ---" not in (BASE / DOCUMENT).read_text(encoding="utf-8")
    assert load("remaining-individual-skill-review-counts-132.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    return checked


def main():
    save(DECISION, {
        "date": "2026-10-09", "review_id": "START-DELEGATE01", "source_Bastion_proposal": ref("review-class-start-bastion-START-B01-01.json"),
        "user_message": "oui ca me va :) , je te laisse faire pour les classes. si tu as des questions tu peux me les poser.",
        "Bastion_approval": "displayed_START_B01_RP_attributes_initial_implant_pair_and_reserve_preview_accepted;exact_equipment_was_not_presented",
        "authority": "prepare_all_five_start_profiles_and_provisional_balance_under_class_delegation;preserve_existing_skill_rules_and_review_new_copy_equipment_and_native_appearance_before_integration",
        "earlier_async_answer": "Éclaireur — reconnaissance et analyse des cibles",
        "latest_user_correction": "pardon je change de réponse pour l'éclaireur et d'ailleurs c'est pas un super nom pour un droniste. On va changer le nom de cette classe. Met lui un drone d'attaque",
        "corrected_start_direction": "attack_drone_replaces_earlier_scout_start;choose_D59_Artilleur_under_class_delegation;no_new_active_skill_added",
        "rename_direction": "rename_Operator_class;Technomancien_is_new_candidate_pending_explicit_name_acceptance;not_a_rename_of_D56_reconnaissance_drone",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(load("review-class-start-bastion-START-B01-01.json"))
    accepted.update(revision=2, status="shown_Bastion_core_start_accepted;exact_equipment_separate;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="shown_Bastion_RP_accepted;existing_skill_copy_preserved")
    accepted["accepted_profile"] = accepted.pop("proposed_profile")
    accepted["previous_proposal"] = ref("review-class-start-bastion-START-B01-01.json")
    accepted["acceptance"] = ref(DECISION)
    accepted["not_resolved_here"] = ["exact_equipment_and_consumables_under_START_ALL01", "native_equipped_creation_preview", "replacement_implant_loot_options", "older_partial_skill_receipts"]
    save(ACCEPTED, accepted)
    profiles = make_profiles()
    doc = make_document(profiles)
    assert not (BASE / DOCUMENT).exists()
    (BASE / DOCUMENT).write_text(doc, encoding="utf-8", newline="\n")
    save(BATCH, {
        "date": "2026-10-09", "review_id": "START-ALL01", "revision": 1,
        "status": "five_starts_prepared_under_delegation;Bastion_core_accepted;Operator_attack_drone_correction_applied_to_design;new_name_copy_and_equipment_presented_for_review",
        "authority": ref(DECISION), "accepted_Bastion": ref(ACCEPTED),
        "dependencies": [ref("class-approval.json"), ref("class-candidates.json"), ref("class-drone-corrections.json"),
                         ref("skill-rank-curves-provisional-02.json"), ref("review-attribute-model-90.json")],
        "profiles": profiles,
        "rename": {"review_id": "CLASS-NAME-O02", "approved_old_name": "Opérateur", "proposed_new_name": "Technomancien",
                   "stable_profile_id": "START-O01", "text_approval": "proposed_not_yet_explicitly_accepted",
                   "no_drone_name_changed": True},
        "shared_proposed_consumables": [{"draft_id": "start:dose_soin", "display_name": "Dose de soin", "quantity": 2, "restores_HP": 12},
                                        {"draft_id": "start:cellule_energie", "display_name": "Cellule d’énergie", "quantity": 1, "restores_energy": 12}],
        "common_equipment_policy": "level1_common_no_affixes_or_skill_bonuses;own_proposed_entries_not_overwriting_existing_model_IDs;integer_uniform_damage_draw_uses_existing_variance_rule",
        "derived_preview_checks": {"Arcaniste_Braise_direct_predefense_damage": 9, "Arcaniste_Braise_energy": 7,
                                   "Metamorphe_Mue_duration": 14, "Metamorphe_Mue_energy": 14},
        "implant_conditions": {
            "damage_penalties": "relative0.90_multiplier_after_compatible_positive_bonus_pool;no_intermediate_rounding;only_compatible_player_weapon_damage",
            "O00_extended_effects": ["Interférence", "Faille d’armure", "Silence", "Rupture de régénération", "Désorientation"],
            "O00_duration_snapshot": "one_extra_turn_at_new_compatible_cast;no_retroactive_extension_of_active_effects;not_Burn_Poison_Bleed_or_Gel_by_extension",
            "M00": "temporary_form_duration_plus2_at_entry;all_human_skill_energy_costs_times1.10_including_form_entry;ceil_once_after_all_compatible_cost_modifiers;normal_weapon_ammo_and_energy_recovery_unchanged",
            "classification_boundary": "does_not_resolve_pending_heavy_weapon_indirect_hybrid_or_other_unshown_skill_eligibility"
        },
        "player_review_document": ref(DOCUMENT),
        "numeric_authority": "provisional_class_balance_prepared_under_explicit_delegation;not_final_playtest_balance",
        "new_text_approval": "presented_not_yet_explicitly_accepted;Bastion_shown_RP_already_accepted",
        "equipment_approval": "presented_proposal_not_yet_explicitly_accepted",
        "visual_approval": "approved_class_silhouettes_retained;no_new_native_armed_character_preview_approval_inferred",
        "remaining": ["new_class_name_and_player_copy_review", "specific_start_equipment_review", "first_mission_ammunition_supply", "native_equipped_character_creation_preview", "found_replacement_implant_loot_catalogue"],
        "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data.update(date="2026-10-09", runtime_integration="not_integrated")
        data["previous_pending_class_start_Bastion"] = deepcopy(data["pending_class_start_Bastion"])
        data["pending_class_start_Bastion"] = None
        data["accepted_class_start_Bastion"] = ref(ACCEPTED)
        data["class_start_delegation"] = ref(DECISION)
        data["prepared_five_class_starts"] = ref(BATCH)
        data["pending_class_name_and_start_equipment_review"] = ref(BATCH)
        if old_name.startswith("review-implementation"):
            data.update(revision=33, status="Bastion_core_start_accepted;five_class_starts_prepared_under_delegation;new_name_copy_equipment_and_native_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY04":
                    group.pop("pending_Bastion_profile", None)
                    group["accepted_Bastion_profile"] = ref(ACCEPTED)
                    group["prepared_five_profiles"] = ref(BATCH)
                    group["remaining"] = ["proposed_Operator_rename_Technomancien_and_new_class_player_copy_explicit_review", "specific_start_weapon_armor_and_consumable_proposal_review", "native_character_creation_layout_and_equipped_preview", "first_mission_ammunition_supply_and_replacement_implant_loot_options"]
                    group["resolved"] = "Bastion_core_accepted;five_attribute_arrays_and_exact_active_passive_binding_prepared_under_class_delegation;latest_attack_drone_correction_replaces_scout_start"
            data["recommendation"] = "review_prepared_class_name_copy_equipment_and_native_creation;then_initial_mission_and_loot_catalogue;preserve_completed_interaction_rules"
        else:
            if old_name.startswith(("review-attribute", "review-primary", "review-complete")):
                data.setdefault("active_rule_overrides", {})["class_start_Bastion"] = {**ref(ACCEPTED), "acceptance": ref(DECISION), "rule": deepcopy(accepted["accepted_profile"])}
            if old_name.startswith("review-attribute"):
                data.update(review_id="STAT-MODEL88", status="Bastion_core_accepted;five_class_starts_prepared_under_delegation;other_rules_preserved")
                pair(data, "previous_model", old_name)
            elif old_name.startswith(("review-primary", "review-complete")):
                data["batch_revision"] += 1
                pair(data, "previous_manifest", old_name)
                pair(data, "latest_decisions", DECISION)
                pair(data, "latest_proposal", BATCH)
                data["latest_proposal_scope"] = "five_class_starts_and_specific_level1_equipment_under_delegation;new_name_player_copy_and_native_review_separate"
            else:
                pair(data, "previous_count", old_name)
                data["latest_feedback"] = "Bastion_accepted;five_class_starts_prepared;Operator_attack_drone_correction;Technomancien_name_proposed"
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-33.json")
            if not old_name.startswith("review-attribute"):
                pair(data, "current_attribute_model", "review-attribute-model-91.json")
            data["next_review_kind"] = "START_ALL01_name_player_copy_specific_equipment_and_native_creation"
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("class-starts-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "START-ALL01-CHECK01",
        "scope": "design_only_reference_integrity_prior_rule_preservation_five_profile_arithmetic_and_correct_skill_IDs;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
        "class_count": 5, "attribute_total_per_class": 34, "gift_learned_rank": 1,
        "HP_energy_pairs": [[41, 27], [30, 38], [30, 30], [30, 36], [34, 32]],
        "Operator_initial_active": "D59_Artilleur", "Metamorphe_initial_active": "M01_Predateur62_not_TR61",
        "accepted_fields_prior_overrides_catalogues_and_curves": "preserved",
        "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": checked, "errors": 0, "classes": 5, "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
