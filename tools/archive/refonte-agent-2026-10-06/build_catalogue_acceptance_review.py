"""Record the user's catalogue agreement and new review requirements."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import re

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent


def read(path):
    return (ROOT / path).read_text(encoding="utf-8")


def digest(source):
    return hashlib.sha256(source.encode("utf-8")).hexdigest()


def load(name):
    return json.loads((OUT / name).read_text(encoding="utf-8"))


def save(name, payload):
    target = OUT / name
    assert not target.exists(), f"Preserve existing review: {name}"
    target.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def main():
    old_catalogue = load("review-primary-catalogue-01.json")
    old_trees = load("review-trees-04.json")
    old_lot = load("review-lot-02-revision-03.json")
    for old, snapshot in [(old_catalogue, "review-primary-catalogue-revision-01.md"),
                          (old_trees, "review-trees-revision-04.md"),
                          (old_lot, "review-lot-02-revision-03.md")]:
        assert digest((OUT / snapshot).read_text(encoding="utf-8")) == old["document_sha256"]

    accepted_ids = [row["id"] for branch in old_catalogue["branches"] for row in branch["candidates"]]
    assert len(accepted_ids) == len(set(accepted_ids)) == 86
    decisions = {
        "date": "2026-10-06", "runtime_integration": "not_integrated",
        "user_agreement": "j'accepte ce que tu me propose",
        "scope": "86-entry list presented in the preceding response; final individual descriptions reserved for later review",
        "accepted_catalogue_ids": accepted_ids,
        "accepted_catalogue_counts": {"branches": 14, "normal_branch_skills": 6, "drone_skills": 8, "total_skills": 86},
        "accepted": ["presented names", "presented roles and effects", "presented visual intentions", "explicitly shown numbers and prior accepted decisions"],
        "M02": "direct damage and bounded transformation extension on a successful hit accepted; extension amount, limit and rank curve pending",
        "descriptions": {
            "user_requirement": "Texte descrptif roleplay, effets chiffrés de ce que fais la compétence par defaut et affiche aussi la valeur actuelle",
            "required_parts": ["roleplay description", "default numeric effects", "current numeric values"],
            "each_final_description_requires_user_review": True,
            "default_reference_definition": "clarification_pending_rank_one_or_current_learned_rank_before_bonuses"
        },
        "monsters": {
            "own_skills": True,
            "usable_skill_count_depends_on": ["role", "level", "importance"],
            "importance_definition": "to_be_defined_later_by_user",
            "numeric_budget_and_catalogue": "not_defined"
        },
        "pending": ["final roleplay descriptions", "unshown costs, durations, curves and restrictions", "tier arrangement and thresholds", "native visual render review", "exact default reference", "monster importance, catalogue and skill count rules"],
        "does_not_approve": "Unshown numbers or text in other review lots and new rendered assets"
    }
    decision_name = "catalogue-acceptance-and-description-decisions.json"
    save(decision_name, decisions)
    decision_path = f"artifacts/refonte-agent-2026-10-06/{decision_name}"
    base = {"date": "2026-10-06", "latest_decisions": decision_path,
            "latest_decisions_sha256": digest(read(decision_path)), "runtime_integration": "not_integrated"}

    catalogue = deepcopy(old_catalogue)
    catalogue.update(base, batch_revision=2, document_sha256=digest(read(catalogue["document"])),
                     previous_manifest="artifacts/refonte-agent-2026-10-06/review-primary-catalogue-01.json",
                     previous_document_snapshot="artifacts/refonte-agent-2026-10-06/review-primary-catalogue-revision-01.md",
                     selection_approval="accepted_user_catalogue_list", name_approval="accepted_as_presented",
                     visual_intention_approval="accepted_as_presented", text_approval="final_descriptions_pending_individual_user_review")
    lines = {line.split("|")[1].strip(): line for line in read(catalogue["document"]).splitlines()
             if re.match(r"^\| [A-Z]+\d{2} \|", line)}
    assert set(lines) == set(accepted_ids)
    for branch in catalogue["branches"]:
        for row in branch["candidates"]:
            identifier, title, role, effect, visual = [part.strip() for part in lines[row["id"]].strip("|").split("|")]
            assert effect == row["effect_proposal"] and visual == row["visual_proposal"]
            row.update(title=title, row_sha256=digest(lines[identifier]), name_approval="accepted_as_presented",
                       mechanic_approval="displayed_catalogue_effect_accepted_unshown_details_pending",
                       visual_intention_approval="accepted_as_presented",
                       text_approval="final_description_pending_individual_user_review")
    catalogue["M02_variant"] = "bounded form extension accepted; prior numeric lot remains unapproved where previously pending"
    save("review-primary-catalogue-02.json", catalogue)

    trees = deepcopy(old_trees)
    trees.update(base, batch_revision=5, document_sha256=digest(read(trees["document"])),
                 previous_manifest="artifacts/refonte-agent-2026-10-06/review-trees-04.json",
                 previous_document_snapshot="artifacts/refonte-agent-2026-10-06/review-trees-revision-04.md",
                 status="catalogue_accepted_tier_layout_and_thresholds_pending")
    trees.pop("proposed_catalogue_counts")
    trees["accepted_requirements"]["catalogue_counts"] = decisions["accepted_catalogue_counts"]
    trees["accepted_requirements"]["three_part_description"] = decisions["descriptions"]["required_parts"]
    save("review-trees-05.json", trees)

    lot = deepcopy(old_lot)
    lot.update(base, batch_revision=4, document_sha256=digest(read(lot["document"])),
               previous_manifest="artifacts/refonte-agent-2026-10-06/review-lot-02-revision-03.json",
               previous_document_snapshot="artifacts/refonte-agent-2026-10-06/review-lot-02-revision-03.md",
               status="M02_catalogue_effect_updated_other_unshown_details_remain_pending")
    source = read(lot["document"])
    matches = list(re.finditer(r"^### ([A-Z]\d{2}) (.+)$", source, re.M))
    bodies = {}
    for index, match in enumerate(matches):
        end = matches[index + 1].start() if index + 1 < len(matches) else len(source)
        bodies[match.group(1)] = re.split(r"^## ", source[match.end():end], maxsplit=1, flags=re.M)[0].strip()
    changed = []
    for row in lot["blocks"]:
        body = bodies[row["id"]]
        if digest(body) == row["block_sha256"]:
            continue
        changed.append(row["id"])
        text = "\n".join(line for line in body.splitlines() if line.startswith(">"))
        mechanic = "\n".join(line for line in body.splitlines() if not line.startswith(">"))
        assert digest(text) == row["text_sha256"]
        row.update(revision=row["revision"] + 1, block_sha256=digest(body), mechanic_sha256=digest(mechanic),
                   mechanic_approval="catalogue_damage_and_bounded_form_extension_accepted_numeric_details_pending",
                   accepted_catalogue_effect=decisions["M02"], name_approval="accepted_as_presented")
    assert changed == ["M02"], changed
    lot["catalogue_approval_reference"] = "artifacts/refonte-agent-2026-10-06/review-primary-catalogue-02.json"
    save("review-lot-02-revision-04.json", lot)

    for name, document, section_prefix, requirements in [
        ("review-description-format-01.json", "docs/REVUE_DESCRIPTIFS_COMPETENCES_AGENT.md", "CD", decisions["descriptions"]),
        ("review-monster-skills-01.json", "docs/REVUE_COMPETENCES_MONSTRES_AGENT.md", "MC", decisions["monsters"])
    ]:
        source = read(document)
        matches = list(re.finditer(rf"^### ({section_prefix}\d{{2}}) (.+)$", source, re.M))
        rows = []
        for index, match in enumerate(matches):
            end = matches[index + 1].start() if index + 1 < len(matches) else len(source)
            body = source[match.end():end].strip()
            rows.append({"id": match.group(1), "title": match.group(2), "revision": 1, "block_sha256": digest(body),
                         "mechanic_approval": "explicit_user_requirements_recorded_proposed_details_pending",
                         "runtime_integration": "not_integrated"})
        assert len(rows) == (3 if section_prefix == "CD" else 2)
        save(name, {**base, "batch_revision": 1, "document": document, "document_sha256": digest(source),
                    "blocks": rows, "accepted_requirements": requirements})
    print(json.dumps({"accepted_skills": 86, "updated_M02_blocks": changed,
                      "description_review_sections": 3, "monster_review_sections": 2}))


if __name__ == "__main__":
    main()
