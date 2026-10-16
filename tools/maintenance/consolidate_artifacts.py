"""Plan and preserve source inputs before the separately reviewed PowerShell cleanup.

This tool never deletes files. Reparse points are never traversed. Historical
artwork, decisions and scripts get durable homes; disposable outputs stay in
artifacts until compilation and native diagnostics have passed.
"""

from __future__ import annotations

import argparse
import collections
import hashlib
import json
import os
from pathlib import Path
import re
import shutil

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / "artifacts"
REPORT = ROOT / "docs/maintenance/nettoyage-2026-10-16"
PLAN = REPORT / "plan.json"
IMAGE_EXTENSIONS = {".png", ".webp", ".jpg", ".jpeg", ".gif", ".svg", ".ttf"}
CODE_EXTENSIONS = {".rs", ".py", ".ps1", ".cjs", ".js", ".css", ".h", ".cc", ".java", ".cmd", ".toml", ".lock", ".patch", ".diff"}
DOCUMENT_EXTENSIONS = {".json", ".json5", ".md", ".txt"}
GENERATED_DIRECTORIES = {"portraits", "normal", "native32", "display64", "conditioned", "palettes", "audit", "native", "raw-qa", "raw-bar-qa", "apercu-animations-data", "apercus", "__pycache__"}
CAPTURE_NAME = re.compile(r"(?:cold-start|screenshot|screen-shot|capture|contact-sheet|comparaison|comparison|preview-board|native-board|references-page|runtime-before|runtime-after)", re.I)
DOCUMENT_NAME = re.compile(r"(?:accept|decision|validat|review|proposal|proposition|description|manifest|catalog|rules|regle|prompt|register|policy|style|summary|resume|bilan|verification|checks|integration-status)", re.I)
INCLUDE = re.compile(r'include_(?:bytes|str)!\s*\(\s*"([^"\n]+)"')


def walk(directory: Path):
    if not directory.exists():
        return
    stack = [directory]
    while stack:
        current = stack.pop()
        for entry in sorted(os.scandir(current), key=lambda e: e.name):
            path = Path(entry.path)
            if entry.is_symlink() or path.is_junction():
                continue
            if entry.is_dir(follow_symlinks=False):
                stack.append(path)
            elif entry.is_file(follow_symlinks=False):
                yield path


def relative(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def digest(path: Path) -> str:
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def included_files():
    result = set()
    for source in (ROOT / "src").rglob("*.rs"):
        for name in INCLUDE.findall(source.read_text(encoding="utf-8")):
            path = (source.parent / name).resolve()
            if not path.is_file():
                raise RuntimeError(f"Missing compilation input: {path}")
            if path.is_relative_to(ART):
                result.add(relative(path))
    return result


def baseline(path: Path) -> bool:
    return any(part.startswith(("baseline", "compat-baseline", "controle-avant", "integration-avant")) or part in ("before", "avant") for part in path.relative_to(ART).parts)


def raw_capture(path: Path) -> bool:
    parts = set(path.relative_to(ART).parts[:-1])
    return bool(parts & {"raw-qa", "raw-bar-qa", "native", "apercus", "apercu-animations-data", "verification-02", "verification-03"}) or bool(CAPTURE_NAME.search(path.name))


def make_plan():
    if PLAN.exists() and json.loads(PLAN.read_text(encoding="utf-8"))["status"] == "completed":
        raise RuntimeError("Keep the completed migration registry; use a new dated report for another cleanup")
    files = list(walk(ART))
    includes = included_files()
    mappings = {}
    reasons = {}
    indexed_assets = {}
    # Prefer existing authoritative assets over duplicate copies in old reviews.
    for path in sorted(walk(ROOT / "assets")):
        if path.suffix.lower() in IMAGE_EXTENSIONS:
            indexed_assets.setdefault(digest(path), relative(path))

    def preserve(path: Path, destination: str, reason: str):
        name = relative(path)
        mappings[name] = destination
        reasons[name] = reason

    # First preserve every direct compilation input, including historical lab art.
    for name in sorted(includes):
        path = ROOT / name
        short = path.relative_to(ART).as_posix()
        if path.suffix.lower() in IMAGE_EXTENSIONS:
            target = "assets/animations/agent/" + short.removeprefix("refonte-agent-2026-10-06/")
        else:
            target = "docs/reviews/" + short
        preserve(path, target, "direct compilation input")

    # Preserve external source-art references declared by authoritative manifests.
    def manifest_value(value):
        if isinstance(value, dict):
            for key, child in value.items():
                manifest_value(key)
                manifest_value(child)
        elif isinstance(value, list):
            for child in value:
                manifest_value(child)
        elif isinstance(value, str) and value.replace("\\", "/").startswith("artifacts/"):
            path = ROOT / value.replace("\\", "/")
            if path.is_file() and not path.is_symlink() and not path.is_junction():
                name = relative(path)
                if name not in mappings:
                    home = "assets/source-art/" if path.suffix.lower() in IMAGE_EXTENSIONS else "docs/reviews/"
                    preserve(path, home + path.relative_to(ART).as_posix(), "authoritative asset manifest")

    for manifest in (ROOT / "assets").rglob("*.json"):
        manifest_value(json.loads(manifest.read_text(encoding="utf-8")))

    for path in files:
        name = relative(path)
        if name in mappings or name.startswith("artifacts/map-editor/"):
            continue
        if baseline(path):
            continue
        ext = path.suffix.lower()
        short = path.relative_to(ART).as_posix()
        if ext in IMAGE_EXTENSIONS:
            if raw_capture(path):
                continue
            parts = set(path.relative_to(ART).parts[:-1])
            # Masters and original drawings take precedence over generated sizes.
            source_art = bool(parts & {"sources", "masters", "originals"}) or bool(re.search(r"(?:source|master|original)", path.stem, re.I))
            if not source_art:
                if parts & GENERATED_DIRECTORIES:
                    continue
                # Sized review exports are rebuildable when a master exists.
                if re.search(r"-(?:24|32|64|128|192|256|512)$", path.stem):
                    stem = re.sub(r"-(?:24|32|64|128|192|256|512)$", "", path.stem)
                    if any((path.parent / (stem + suffix)).is_file() for suffix in ("-master.png", "-source.png")):
                        continue
                # Viewport-size images without source/master provenance are QA,
                # not original drawings. The original masters are kept above.
                if ext == ".png":
                    import struct
                    with path.open("rb") as stream:
                        header = stream.read(24)
                    if header[:8] == b"\x89PNG\r\n\x1a\n":
                        width, height = struct.unpack(">II", header[16:24])
                        if width > 1000 and height > 700:
                            continue
            sha = digest(path)
            target = indexed_assets.get(sha)
            if target is None:
                target = "assets/source-art/" + short
                indexed_assets[sha] = target
            preserve(path, target, "original or unique drawing; identical images deduplicated")
        elif ext in DOCUMENT_EXTENSIONS:
            # Keep decisions, design, prompts, manifests and concise validation.
            # Large scene dumps, cached editor trials and screen transcripts are disposable.
            if raw_capture(path) or path.stat().st_size > 2 * 1024 * 1024 or "disk-audit-2026-10-14" in path.parts:
                continue
            if re.search(r"(?:stdout|stderr|timings|capture|screenshots|inventory|cleanup-plan|protected-files|file-inventory|capture-associated)", path.name, re.I):
                continue
            if ext in (".json", ".json5") and not DOCUMENT_NAME.search(path.name) and path.parent != ART / "refonte-agent-2026-10-06":
                continue
            preserve(path, "docs/reviews/" + short, "decision, prompt, manifest or validation record")
        elif ext in CODE_EXTENSIONS:
            if "disk-audit-2026-10-14" not in path.parts and "__pycache__" not in path.parts:
                if path.name in ("player.js", "gallery-script.js"):
                    continue
                preserve(path, "tools/archive/" + short, "historical generation or review source")

    # Map PNG provenance names in manifests to any existing authoritative copy,
    # without retaining derived portraits or screen captures just for provenance.
    source_hashes = {name: digest(ROOT / name) for name in mappings}
    destinations = {}
    for name, destination in mappings.items():
        previous = destinations.setdefault(destination, source_hashes[name])
        if previous != source_hashes[name]:
            raise RuntimeError(f"Destination collision: {destination}")
        target = ROOT / destination
        if target.exists() and digest(target) != source_hashes[name]:
            raise RuntimeError(f"Existing destination differs: {destination}")

    to_delete = []
    kept = []
    for path in files:
        name = relative(path)
        if name.startswith("artifacts/map-editor/") or name == "artifacts/refonte-agent-2026-10-06/apercu-animations.html":
            kept.append(name)
        else:
            info = path.stat()
            to_delete.append({"path": name, "bytes": info.st_size, "mtime_ns": info.st_mtime_ns})
    target_files = []
    for path in walk(ROOT / "target"):
        name = relative(path)
        if name in ("target/debug/project-rl.exe", "target/release/project-rl.exe"):
            kept.append(name)
        else:
            info = path.stat()
            target_files.append({"path": name, "bytes": info.st_size, "mtime_ns": info.st_mtime_ns})
    destination_sizes = {destination: (ROOT / name).stat().st_size for name, destination in mappings.items()}
    report = {
        "status": "planned_no_deletion", "root": str(ROOT), "direct_inputs": sorted(includes),
        "mapping": mappings, "sha256": source_hashes, "reasons": reasons,
        "preserved_in_place": kept, "artifact_deletions": to_delete,
        "target_deletions_initial": target_files,
        "summary": {
            "migrated_paths": len(mappings), "unique_destinations": len(destinations),
            "new_destination_bytes": sum(destination_sizes[destination] for destination in destinations if not (ROOT / destination).exists()),
            "artifact_delete_files": len(to_delete), "artifact_delete_bytes": sum(row["bytes"] for row in to_delete),
            "target_delete_files": len(target_files), "target_delete_bytes": sum(row["bytes"] for row in target_files),
        },
    }
    REPORT.mkdir(parents=True, exist_ok=True)
    PLAN.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(report["summary"], ensure_ascii=False))


def preserve_plan():
    plan = json.loads(PLAN.read_text(encoding="utf-8"))
    for name, destination in plan["mapping"].items():
        path = ROOT / name
        if digest(path) != plan["sha256"][name]:
            raise RuntimeError(f"Source changed: {name}")
        target = ROOT / destination
        if not target.exists():
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(path, target)
        if digest(target) != plan["sha256"][name]:
            raise RuntimeError(f"Copy mismatch: {destination}")
    plan["status"] = "sources_copied_and_hash_verified_no_deletion"
    PLAN.write_text(json.dumps(plan, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"copied_or_verified": len(plan["mapping"]), "deletions": 0}))


def rewrite_includes():
    plan = json.loads(PLAN.read_text(encoding="utf-8"))
    changed = []
    for source in (ROOT / "src").rglob("*.rs"):
        text = source.read_text(encoding="utf-8")

        def replace(match):
            original = (source.parent / match.group(1)).resolve()
            name = relative(original)
            destination = plan["mapping"].get(name)
            if destination is None:
                return match.group(0)
            new_path = os.path.relpath(ROOT / destination, source.parent).replace("\\", "/")
            return match.group(0).replace(match.group(1), new_path)

        revised = INCLUDE.sub(replace, text)
        if revised != text:
            source.write_text(revised, encoding="utf-8", newline="\n")
            changed.append(relative(source))
    for name in included_files():
        raise RuntimeError(f"Compilation input still in artifacts: {name}")
    plan["rewritten_source_files"] = changed
    plan["status"] = "compile_inputs_relocated_no_deletion"
    PLAN.write_text(json.dumps(plan, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"source_files_updated": len(changed), "artifact_compile_inputs_remaining": 0}))


def rewrite_references():
    plan = json.loads(PLAN.read_text(encoding="utf-8"))
    mapping = plan["mapping"]
    reverse = {destination: name for name, destination in mapping.items()}
    path_pattern = re.compile(r'''artifacts(?:[/\\]+)[^"'`\s<>\[\]{}(),;]+''')
    changed = []
    candidates = list(walk(ROOT / "assets")) + list(walk(ROOT / "docs"))
    for path in candidates:
        if path.suffix.lower() not in DOCUMENT_EXTENSIONS or path.is_relative_to(REPORT):
            continue
        text = path.read_text(encoding="utf-8-sig")
        name = relative(path)
        original = ROOT / reverse.get(name, name)

        def new_reference(value):
            normalized = re.sub(r"\\+", "/", value)
            if normalized in mapping:
                return mapping[normalized]
            if value.endswith((".png", ".webp", ".json", ".json5", ".md", ".txt")):
                old_target = (original.parent / normalized).resolve()
                if old_target.is_relative_to(ART):
                    destination = mapping.get(relative(old_target))
                    if destination:
                        return os.path.relpath(ROOT / destination, path.parent).replace("\\", "/")
            return value

        revised = text
        if path.suffix.lower() == ".json":
            try:
                content = json.loads(text)
            except json.JSONDecodeError:
                continue  # Keep historical non-JSON excerpts unchanged.

            def visit(value):
                if isinstance(value, str):
                    return new_reference(value)
                if isinstance(value, list):
                    return [visit(v) for v in value]
                if isinstance(value, dict):
                    return {new_reference(k): visit(v) for k, v in value.items()}
                return value

            updated = visit(content)
            if updated != content:
                revised = json.dumps(updated, ensure_ascii=False, indent=2) + "\n"
        else:
            revised = path_pattern.sub(lambda m: mapping.get(re.sub(r"\\+", "/", m.group(0)), m.group(0)), text)
        if revised != text:
            path.write_text(revised, encoding="utf-8", newline="\n")
            changed.append(name)
    plan["rewritten_reference_files"] = changed
    PLAN.write_text(json.dumps(plan, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"reference_files_updated": len(changed)}))


def prepare_deletion():
    plan = json.loads(PLAN.read_text(encoding="utf-8"))
    validation = json.loads((REPORT / "validation.json").read_text(encoding="utf-8"))
    if validation["status"] != "passed" or included_files():
        raise RuntimeError("Successful build, tests and native verification required first")
    if not plan["status"] == "compile_inputs_relocated_no_deletion":
        raise RuntimeError("Preservation must be completed before cleanup")
    checked_images = set()
    for name, destination in plan["mapping"].items():
        path = ROOT / destination
        if not path.is_file():
            raise RuntimeError(f"Preserved file missing: {destination}")
        if path.suffix.lower() in IMAGE_EXTENSIONS and destination not in checked_images:
            if digest(path) != plan["sha256"][name]:
                raise RuntimeError(f"Preserved image changed: {destination}")
            checked_images.add(destination)
    preserved = set(plan["preserved_in_place"])
    hashes = {name: digest(ROOT / name) for name in preserved if (ROOT / name).is_file()}
    files = []
    for directory in (ART, ROOT / "target"):
        for path in walk(directory):
            name = relative(path)
            if name in preserved or name.startswith("artifacts/map-editor/"):
                continue
            info = path.stat()
            files.append({"path": name, "bytes": info.st_size, "mtime_ms": info.st_mtime_ns // 1_000_000})
    junctions = []
    stack = [ART]
    while stack:
        current = stack.pop()
        for entry in os.scandir(current):
            path = Path(entry.path)
            if entry.is_symlink() or path.is_junction():
                if not path.is_relative_to(ART / "map-editor"):
                    junctions.append(relative(path))
            elif entry.is_dir(follow_symlinks=False):
                stack.append(path)
    ready = {
        "status": "ready_after_validation", "root": str(ROOT),
        "files": files, "junctions": junctions,
        "preserved_in_place": sorted(preserved), "preserved_in_place_sha256": hashes,
        "unique_art_images_hash_verified": len(checked_images),
        "disk_free_before_deletion": shutil.disk_usage(ROOT).free,
        "logical_bytes_to_delete": sum(row["bytes"] for row in files),
    }
    (REPORT / "cleanup-ready.json").write_text(json.dumps(ready, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"files_ready": len(files), "bytes_ready": ready["logical_bytes_to_delete"], "junctions": len(junctions), "unique_images_hash_verified": len(checked_images)}))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("plan", "preserve", "rewrite-includes", "rewrite-references", "prepare-deletion"))
    action = parser.parse_args().action
    {"plan": make_plan, "preserve": preserve_plan, "rewrite-includes": rewrite_includes, "rewrite-references": rewrite_references, "prepare-deletion": prepare_deletion}[action]()
