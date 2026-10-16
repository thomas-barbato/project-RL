"""Record unapproved review blocks without granting gameplay approval."""
from pathlib import Path
import hashlib
import json
import re

ROOT = Path(__file__).resolve().parents[2]
DOCUMENT = ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md"
OUTPUT = Path(__file__).resolve().parent / "review-lot-01.json"


def digest(value):
    return hashlib.sha256(value.encode("utf-8")).hexdigest()


def main():
    source = DOCUMENT.read_text(encoding="utf-8")
    matches = list(re.finditer(r"^### ([NSEPB]\d{2}) (.+)$", source, re.MULTILINE))
    identifiers = [match.group(1) for match in matches]
    expected = ["N01", "N02", "S01", "S02", "S03", "S04", "S05", "S06",
                "E01", "E02", "E03", "P01", "B00", "B01", "B02", "B03"]
    assert identifiers == expected, identifiers
    assert not OUTPUT.exists(), "Preserve existing approvals; reconcile revisions explicitly."
    records = []
    for index, match in enumerate(matches):
        end = matches[index + 1].start() if index + 1 < len(matches) else len(source)
        block = source[match.end():end]
        block = re.split(r"^## ", block, maxsplit=1, flags=re.MULTILINE)[0].strip()
        copy = "\n".join(line for line in block.splitlines() if line.startswith(">"))
        assert copy, match.group(1)
        is_narrative = match.group(1).startswith("N")
        mechanic = "\n".join(line for line in block.splitlines() if not line.startswith(">"))
        records.append({"id": match.group(1), "title": match.group(2), "revision": 1,
                        "block_sha256": digest(block), "text_sha256": digest(copy),
                        "mechanic_sha256": None if is_narrative else digest(mechanic),
                        "text_approval": "pending_user_review",
                        "mechanic_approval": "not_applicable" if is_narrative else "pending_user_review",
                        "visual_approval": "not_proposed_in_this_batch",
                        "runtime_integration": "not_integrated"})
    payload = {"date": "2026-10-06", "document": DOCUMENT.relative_to(ROOT).as_posix(),
               "document_sha256": digest(source), "status": "pending_user_review",
               "user_instruction": "tous les textes / effets devront etre validés par moi meme",
               "resolution_conventions_approval": "pending_user_review",
               "blocks": records}
    OUTPUT.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"{len(records)} review blocks; all new text and mechanics await user approval.")


if __name__ == "__main__":
    main()
