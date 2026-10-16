"""Reconcile the revised review while preserving the original approval record."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import re

ROOT = Path(__file__).resolve().parents[2]
DIRECTORY = Path(__file__).resolve().parent
DOCUMENT = ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md"
PREVIOUS = DIRECTORY / "review-lot-01.json"
SNAPSHOT = DIRECTORY / "review-lot-01-revision-01.md"
DECISIONS = DIRECTORY / "progression-decisions.json"
OUTPUT = DIRECTORY / "review-lot-01-revision-02.json"


def digest(value):
    return hashlib.sha256(value.encode("utf-8")).hexdigest()


def main():
    assert not OUTPUT.exists(), "Preserve this revision; create a later revision for further changes."
    source = DOCUMENT.read_text(encoding="utf-8")
    previous = json.loads(PREVIOUS.read_text(encoding="utf-8"))
    assert digest(SNAPSHOT.read_text(encoding="utf-8")) == previous["document_sha256"]
    decisions_source = DECISIONS.read_text(encoding="utf-8")
    decisions = json.loads(decisions_source)
    accepted = decisions["accepted_requirements"]
    previous_records = {record["id"]: record for record in previous["blocks"]}
    requirements = {
        "P01": ["attribute_points_per_level", "attribute_gameplay_cap",
                "skill_points_per_level", "starting_active_skills",
                "starting_active_is_first_in_class", "starting_passive_skills"],
        "P02": ["cost_per_learned_skill_level", "learned_skill_level_cap",
                "equipment_can_exceed_learned_cap"],
        "P03": ["equipment_can_exceed_learned_cap", "skill_bonuses_in_equipment_loot",
                "boosted_skill_level_uses_different_display_color"],
        "B03": ["bastion_passive_initial_max_hp_percent",
                "bastion_passive_replaces_interrupt_resistance"],
    }
    matches = list(re.finditer(r"^### ([NSEPB]\d{2}) (.+)$", source, re.MULTILINE))
    expected = ["N01", "N02", "S01", "S02", "S03", "S04", "S05", "S06",
                "E01", "E02", "E03", "P01", "P02", "P03",
                "B00", "B01", "B02", "B03"]
    assert [match.group(1) for match in matches] == expected
    records = []
    for index, match in enumerate(matches):
        identifier, title = match.group(1), match.group(2)
        end = matches[index + 1].start() if index + 1 < len(matches) else len(source)
        block = source[match.end():end]
        block = re.split(r"^## ", block, maxsplit=1, flags=re.MULTILINE)[0].strip()
        copy = "\n".join(line for line in block.splitlines() if line.startswith(">"))
        assert copy, identifier
        narrative = identifier.startswith("N")
        mechanic = "\n".join(line for line in block.splitlines() if not line.startswith(">"))
        old = previous_records.get(identifier)
        hashes = {"block_sha256": digest(block), "text_sha256": digest(copy),
                  "mechanic_sha256": None if narrative else digest(mechanic)}
        unchanged = old and old["title"] == title and old["block_sha256"] == hashes["block_sha256"]
        record = deepcopy(old) if old else {"id": identifier, "revision": 0}
        record.update({"title": title, **hashes, "runtime_integration": "not_integrated"})
        if not unchanged:
            record["revision"] += 1
        if not old or old["text_sha256"] != hashes["text_sha256"]:
            record["text_approval"] = "pending_user_review"
        if not old or old["mechanic_sha256"] != hashes["mechanic_sha256"]:
            record["mechanic_approval"] = "not_applicable" if narrative else "pending_user_review"
        record.setdefault("visual_approval", "not_proposed_in_this_batch")
        if identifier in requirements:
            record["accepted_user_requirements"] = {
                key: accepted[key] for key in requirements[identifier]
            }
            record["mechanic_approval"] = "user_requirements_recorded_details_pending_review"
        if identifier == "P03":
            record["visual_approval"] = "color_change_required_palette_and_preview_pending_review"
        records.append(record)
    payload = {
        "date": "2026-10-06", "batch_revision": 2,
        "document": DOCUMENT.relative_to(ROOT).as_posix(),
        "document_sha256": digest(source),
        "status": "user_requirements_recorded_additional_proposals_pending_review",
        "previous_manifest": PREVIOUS.relative_to(ROOT).as_posix(),
        "previous_document_snapshot": SNAPSHOT.relative_to(ROOT).as_posix(),
        "decisions": DECISIONS.relative_to(ROOT).as_posix(),
        "decisions_sha256": digest(decisions_source),
        "user_instruction": previous["user_instruction"],
        "resolution_conventions_approval": "pending_user_review",
        "blocks": records,
    }
    OUTPUT.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"{len(records)} review blocks; user requirements recorded; added details await review.")


if __name__ == "__main__":
    main()
