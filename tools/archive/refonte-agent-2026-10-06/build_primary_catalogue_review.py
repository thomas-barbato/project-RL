"""Save pending design reviews; no game code or accepted artwork changes."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import re

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent


def read(path):
    return (ROOT / path).read_text(encoding="utf-8")


def digest(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def save(name, payload):
    target = OUT / name
    assert not target.exists(), f"Preserve prior review: {name}"
    target.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


DECISIONS = "artifacts/refonte-agent-2026-10-06/primary-catalogue-and-ui-decisions.json"
BASE = {
    "date": "2026-10-06", "runtime_integration": "not_integrated",
    "latest_decisions": DECISIONS, "latest_decisions_sha256": digest(read(DECISIONS)),
    "text_approval": "pending_user_review", "visual_approval": "pending_preview_and_user_review",
}


def blocks(source, previous=()):
    previous = {row["id"]: row for row in previous}
    matches = list(re.finditer(r"^### ([A-Z]+\d{2}) (.+)$", source, re.M))
    result = []
    for index, match in enumerate(matches):
        identifier, title = match.groups()
        end = matches[index + 1].start() if index + 1 < len(matches) else len(source)
        body = re.split(r"^## ", source[match.end():end], maxsplit=1, flags=re.M)[0].strip()
        text = "\n".join(line for line in body.splitlines() if line.startswith(">"))
        mechanic = "\n".join(line for line in body.splitlines() if not line.startswith(">"))
        assert text, identifier
        hashes = {"block_sha256": digest(body), "text_sha256": digest(text), "mechanic_sha256": digest(mechanic)}
        old = previous.get(identifier)
        row = deepcopy(old) if old else {**BASE, "id": identifier, "role": "interface", "revision": 0}
        if not old or old["block_sha256"] != hashes["block_sha256"] or old["title"] != title:
            row["revision"] += 1
        row.update(title=title, **hashes)
        if not old or old["text_sha256"] != hashes["text_sha256"]:
            row["text_approval"] = "pending_user_review"
        if not old or old["mechanic_sha256"] != hashes["mechanic_sha256"]:
            row["mechanic_approval"] = "requested_structure_and_specific_effects_accepted_other_details_pending"
        result.append(row)
    return result


def main():
    decisions = json.loads(read(DECISIONS))
    tree_old = json.loads(read("artifacts/refonte-agent-2026-10-06/review-trees-03.json"))
    secondary_old = json.loads(read("artifacts/refonte-agent-2026-10-06/review-secondary-skills-01.json"))
    for name, old in [("review-trees-revision-03.md", tree_old), ("review-secondary-skills-revision-01.md", secondary_old)]:
        assert digest((OUT / name).read_text(encoding="utf-8")) == old["document_sha256"], name

    secondary_path = "docs/REVUE_COMPETENCES_SECONDAIRES_AGENT.md"
    secondary = deepcopy(secondary_old)
    secondary.update(BASE, batch_revision=2, document_sha256=digest(read(secondary_path)))
    secondary["previous_manifest"] = "artifacts/refonte-agent-2026-10-06/review-secondary-skills-01.json"
    secondary["previous_document_snapshot"] = "artifacts/refonte-agent-2026-10-06/review-secondary-skills-revision-01.md"
    secondary["blocks"] = blocks(read(secondary_path), secondary_old["blocks"])
    assert len(secondary["blocks"]) == 17
    old_rows = {row["id"]: row for row in secondary_old["blocks"]}
    changed = [row["id"] for row in secondary["blocks"] if row["id"] not in old_rows or row["block_sha256"] != old_rows[row["id"]]["block_sha256"]]
    assert changed == ["SP01", "SP02"], changed
    secondary["accepted_requirements"]["shared_cell_inspection"] = decisions["accepted_ui_requirements"]["shared_mine_cell_tooltip"]
    secondary["accepted_requirements"]["previews"]["placed_mine_requires_no_other_inspection_target_on_cell"] = False
    secondary["accepted_requirements"]["previews"]["direct_mine_preview_when_alone"] = True
    secondary["change"] = "Replace shared-cell automatic inspection priority with player selection; preserve mine-alone preview"
    save("review-secondary-skills-02.json", secondary)

    tree_path = "docs/REVUE_ARBRES_COMPETENCES_AGENT.md"
    tree = {**BASE, "batch_revision": 4, "document": tree_path, "document_sha256": digest(read(tree_path)),
            "previous_manifest": "artifacts/refonte-agent-2026-10-06/review-trees-03.json",
            "previous_document_snapshot": "artifacts/refonte-agent-2026-10-06/review-trees-revision-03.md",
            "status": "fourteen_branches_accepted_counts_and_tiers_under_review",
            "accepted_requirements": decisions["accepted_primary_requirements"],
            "historical_grid_under_reconsideration": tree_old["accepted_requirements"],
            "proposed_catalogue_counts": decisions["proposed_catalogue_counts"]}
    save("review-trees-04.json", tree)

    catalogue_path = "docs/PROPOSITION_COMPETENCES_PRINCIPALES_AGENT.md"
    source = read(catalogue_path)
    sections = list(re.finditer(r"^## (\d+) — (.+)$", source, re.M))
    branches = []
    identifiers = []
    for index, match in enumerate(sections):
        end = sections[index + 1].start() if index + 1 < len(sections) else len(source)
        rows = []
        for line in source[match.end():end].splitlines():
            if not re.match(r"^\| [A-Z]+\d{2} \|", line):
                continue
            identifier, title, role, effect, visual = [cell.strip() for cell in line.strip("|").split("|")]
            assert role in ("A", "P") and all([identifier, title, effect, visual])
            identifiers.append(identifier)
            rows.append({"id": identifier, "title": title, "role": "active" if role == "A" else "passive",
                         "effect_proposal": effect, "visual_proposal": visual,
                         "row_sha256": digest(line), "text_approval": "pending_user_review",
                         "mechanic_approval": "proposal_pending_except_explicitly_recorded_prior_initial_effects",
                         "visual_approval": "pending_preview_and_user_review", "runtime_integration": "not_integrated"})
        assert len(rows) == (8 if match.group(2) == "Drones" else 6)
        branches.append({"number": int(match.group(1)), "name": match.group(2), "candidates": rows})
    assert len(branches) == 14 and len(identifiers) == 86 and len(set(identifiers)) == 86
    assert [row["name"] for row in branches] == decisions["accepted_primary_requirements"]["branch_names"]
    save("review-primary-catalogue-01.json", {**BASE, "batch_revision": 1, "document": catalogue_path,
         "document_sha256": digest(source), "candidate_count": 86, "branches": branches,
         "selection_approval": "pending_user_review", "learned_rank_cap": 10, "cost_per_rank": 1,
         "D59_alias": "O01; one skill node, not two skills",
         "M02_variant": "proposed form extension only; previous lot-two record and numbers remain unchanged",
         "approved_existing_artwork": "preserved; skill visual effects pending native preview",
         "prior_initial_effects_only": {"B01": "+40% melee damage", "B02": "25% direct damage reduction",
             "B03": "+10% maximum HP", "O03": "+25% drone damage", "T01": "instant, accuracy multiplied by 1.15",
             "T03": "+10% ranged weapon and ranged attack skill damage", "A03": "20% lower magic costs",
             "M03": "+10% damage while transformed"}})

    hud_path = "docs/REVUE_INTERFACE_EFFETS_AGENT.md"
    hud = blocks(read(hud_path))
    assert len(hud) == 1 and hud[0]["id"] == "HUD01"
    save("review-interface-effects-01.json", {**BASE, "batch_revision": 1, "document": hud_path,
         "document_sha256": digest(read(hud_path)), "blocks": hud,
         "accepted_requirements": decisions["accepted_ui_requirements"]["future_effect_panel"]})
    print(json.dumps({"branches": len(branches), "candidates": len(identifiers), "secondary_review_blocks": 17,
                      "changed_secondary_blocks": changed, "future_hud_review_blocks": 1}, ensure_ascii=False))


if __name__ == "__main__":
    main()
