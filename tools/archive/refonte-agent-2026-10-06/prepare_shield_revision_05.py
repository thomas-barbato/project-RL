"""Prepare a new immutable review; only EQ-S02 artwork changes."""
from pathlib import Path
import json

BASE = Path(__file__).resolve().parent
OUT = BASE / "shield-review-05"
OLD = BASE / "shield-review-04"
OUT.mkdir(exist_ok=True)
request = json.loads((BASE / "edit-request-EQ-S02-05.json").read_text(encoding="utf-8"))
prompts = json.loads((OLD / "prompts.json").read_text(encoding="utf-8"))
prompts["revision"] = 5
prompts["revised_candidates_against_last_user_review"] = ["EQ-S02"]
for entry in prompts["assets"]:
    cid = entry["candidate_id"]
    entry["source_path"] = str(OLD / "sources" / f"{cid}.png")
    if cid == "EQ-S02":
        entry.update(source_path=request["source_path"], prompt=request["prompt"],
                     source_revision=5, edit_references=[request["edit_reference"]])
(OUT / "prompts.json").write_text(json.dumps(prompts, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
source = (BASE / "build_shield_review_04.py").read_text(encoding="utf-8")
replacements = {
    'OUT = BASE / "shield-review-04"': 'OUT = BASE / "shield-review-05"',
    '"shield-gallery-04.html"': '"shield-gallery-05.html"',
    '"revision":4': '"revision":5',
    'native_readability_review;EQ-S02_and_EQ-S04_edges_strengthened;EQ-S06_side_rebuilt_as_integrated_metal;pending_user_pixel_approval': 'native_readability_review;EQ-S02_lower_steel_frame_thickened_to_match_upper;other_five_unchanged;pending_user_pixel_approval',
    'shield-review-03/review-shields-SHIELD-VIS01-03.json': 'shield-review-04/review-shields-SHIELD-VIS01-04.json',
    'shield-review-02/review-shields-SHIELD-VIS01-02.json': 'shield-review-04/review-shields-SHIELD-VIS01-04.json',
    'EQ-S02_EQ-S04_EQ-S06_pending_revised_visual_review;other_three_unchanged;no_blanket_acceptance_inferred': 'EQ-S02_pending_revised_visual_review;other_five_unchanged_from_revision_4;no_additional_acceptance_inferred',
    '"review-shields-SHIELD-VIS01-04.json"': '"review-shields-SHIELD-VIS01-05.json"',
}
for old, new in replacements.items():
    assert old in source, old
    source = source.replace(old, new)
(BASE / "build_shield_review_05.py").write_text(source, encoding="utf-8", newline="\n")
print("Revision 5 prepared; EQ-S02 is the only revised source.")
