"""Inventory reviewed disposable outputs and verify the untouched project files.

This script does not delete files. Execute only the exact targets in plan.json
after the release build succeeds. Working art, runtime data and pending reviews
are protected independently of their age.
"""
from pathlib import Path
import argparse
import hashlib
import json
import os
import stat
import subprocess

ROOT = Path(__file__).resolve().parents[2]
REPORT = ROOT / "docs/maintenance/nettoyage-2026-10-30"
PAGE_ROOT = ROOT / "artifacts/vertical-walk-review-18/current"
RETIRED = {
    "roster-C02": ["contact-native64.png"],
    "equipment-C03": ["mixed-marche-natif64.png", "pistols-marche-natif64.png"],
    "headgear-C04": ["contact-unhelmeted-native64.png"],
    "size-C05": ["all-bodies-before-after-native64.png", "all-bodies-native64.webp"],
    "extension-C06": ["MON-022-native-comparison.png", "MON-141-native-comparison.png"],
    "pistol-C07": ["comparatif-natif-C07.png"],
    "knife-C08": ["comparatif-natif-C08.png"],
    "pistol-C09": ["comparatif-natif-C09.png"],
    "armor-C10": ["C10-native64.png"],
}


def write(name, value):
    (REPORT / name).write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def regular(p):
    info = p.lstat()
    if info.st_file_attributes & stat.FILE_ATTRIBUTE_REPARSE_POINT:
        raise RuntimeError(f"Linked path requires separate review: {p}")
    return info


def files(p):
    regular(p)
    if p.is_file():
        return [p]
    result = []
    for current, directories, names in os.walk(p, followlinks=False):
        for name in directories:
            regular(Path(current) / name)
        for name in names:
            q = Path(current) / name
            regular(q)
            result.append(q)
    return sorted(result)


def row(p):
    digest = hashlib.sha256()
    with p.open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(block)
    return {"path": p.relative_to(ROOT).as_posix() if p.is_relative_to(ROOT) else str(p),
            "bytes": regular(p).st_size, "sha256": digest.hexdigest()}


def plan():
    REPORT.mkdir(exist_ok=True)
    if (REPORT / "preservation-before.json").exists():
        raise RuntimeError("Do not overwrite the baseline")
    targets = []

    def add(p, reason):
        p = ROOT / p
        if not p.exists():
            return
        p.resolve(strict=True).relative_to(ROOT)
        if p == ROOT or p == ROOT / "artifacts" or p.is_relative_to(ROOT / "artifacts/map-editor"):
            raise RuntimeError(f"Protected target {p}")
        rows = [row(q) for q in files(p)]
        targets.append({"path": p.relative_to(ROOT).as_posix(), "reason": reason,
                        "files": rows, "bytes": sum(r["bytes"] for r in rows)})

    for name, keep in RETIRED.items():
        directory = PAGE_ROOT / name
        for filename in keep:
            assert (directory / filename).is_file(), filename
        for p in directory.iterdir():
            if p.name not in {*keep, "apercu.html"}:
                add(p, "Completed historical preview superseded by integrated C11/C12; native reference and source artwork retained")
    add(PAGE_ROOT / "extension-catalog", "Obsolete C06 browser catalogue; approved sources and runtime bank retained")
    for base in ["integration/archive-normal", "integration/normal", "integration-C12/normal"]:
        directory = ROOT / "artifacts/vertical-walk-review-18" / base
        keep = {"arcaniste-armor1-W002.png", "bastion-armor2-W066-H120.png",
                "metamorphe-armor3-W088.png", "traqueuse-armor1-C11-104-70-H0.png"} if base == "integration-C12/normal" else set()
        for p in directory.rglob("*.png"):
            if p.name not in keep and not (base == "integration-C12/normal" and p.name == "cold-start.png"):
                add(p, "Repeated integrated-render capture; complete checks and representative final captures retained")
    for p in (PAGE_ROOT / "continuity-CONT01").glob("*.png"):
        if p.name != "comparatif-native64.png":
            add(p, "Duplicate fullscreen movement samples; native comparison and all 36 checks retained")
    add("artifacts/dual-texture-fix-16", "Withdrawn DUAL16 experiment; investigation and compact reference evidence retained in docs/reviews/dual-walk-15")
    arm = ROOT / "artifacts/walk-arm-projection-14/r01"
    keep_arm = {"apercu.html", "review.json", "Bastion-axe-natif64.gif", "Bastion-dual-natif64.gif", "Bastion-rifle-natif64.gif",
                "Arcaniste-axe-comparatif.png", "Bastion-dual-comparatif.png", "Metamorphe-rifle-comparatif.png", "Traqueuse-axe-comparatif.png"}
    for p in arm.iterdir():
        if p.name not in keep_arm:
            add(p, "Redundant accepted ARM14 animation export; representative grips, decisions and all source art retained")
    ui = ROOT / "artifacts/character-creation-review-02/current"
    for name in ["normal-check", "after-arcaniste-skill", "polish-terminal", "polish-reduced", "polish-02B", "polish-small",
                 "modern-final", "modern-final-reduced"]:
        add(ui / name, "Superseded UI prototype or duplicate capture; accepted 02A and pending final 02B preserved")
    for p in (ROOT / "tools").rglob("__pycache__"):
        add(p, "Regenerable Python bytecode cache")
    add("target/flycheck0", "Completed editor compiler diagnostics")
    removed = {r["path"] for t in targets for r in t["files"]}
    assert len(removed) == sum(len(t["files"]) for t in targets), "Overlapping targets"
    modified = {"docs/GESTION_FICHIERS_GENERES.md", "artifacts/walk-arm-projection-14/r01/apercu.html",
                "artifacts/vertical-walk-review-18/current/apercu.html"}
    modified.update(f"artifacts/vertical-walk-review-18/current/{name}/apercu.html" for name in RETIRED)
    protected = []
    for name in ["assets", "src", "content", "mods", "vendor", "tests", "examples", "tools", "docs", "dist", "artifacts", ".vscode", "config", "saves", "savegames"]:
        p = ROOT / name
        if p.exists():
            protected.extend(files(p))
    protected.extend(p for p in ROOT.iterdir() if p.is_file())
    protected.extend(p for p in (ROOT / ".git").glob("*") if p.name in {"HEAD", "index", "packed-refs"})
    if (ROOT / ".git/refs").exists():
        protected.extend(files(ROOT / ".git/refs"))
    user = Path(os.environ["APPDATA"]) / "ProjectRL"
    if user.exists():
        protected.extend(files(user))
    baseline = [row(p) for p in sorted(set(protected)) if not p.is_relative_to(REPORT)
                and (not p.is_relative_to(ROOT) or p.relative_to(ROOT).as_posix() not in removed | modified)]
    write("preservation-before.json", baseline)
    caches = {}
    for profile in ["debug", "loading"]:
        p = ROOT / "target" / profile
        entries = files(p) if p.exists() else []
        caches[profile] = {"files": len(entries), "bytes": sum(q.stat().st_size for q in entries)}
    write("cache-before.json", caches)
    write("plan.json", {"status": "ready_after_release_build", "root": str(ROOT), "targets": targets,
                       "files": len(removed), "bytes": sum(t["bytes"] for t in targets),
                       "protected_files": len(baseline), "deliberately_modified_pages": sorted(modified),
                       "cache_policy": "Cargo owns Debug/loading cleanup; keep Release and installed distribution",
                       "pending_reviews_preserved": ["CREATION-UI02-B", "DUAL17-C02", "mouse route preview"],
                       "rebuild": "cargo build --locked --release --bin project-rl"})
    (REPORT / "git-status-before.txt").write_text(subprocess.check_output(["git", "status", "--short"], cwd=ROOT, encoding="utf-8"), encoding="utf-8")
    print(json.dumps({"targets": len(targets), "files": len(removed), "MB": round(sum(t["bytes"] for t in targets)/1e6, 2),
                      "protected_files": len(baseline), "caches": caches}))


def pages():
    for name, references in RETIRED.items():
        links = ''.join(f'<p><a href="{filename}">{filename}</a></p>' for filename in references)
        (PAGE_ROOT / name / "apercu.html").write_text(
            '<!doctype html><meta charset="utf-8"><title>Référence historique conservée</title>'
            '<style>body{background:#131c25;color:#dbe7ed;font:16px system-ui;max-width:900px;margin:3rem auto}a{color:#8dd3ee}</style>'
            f'<h1>Référence historique · {name}</h1><p>Les exports de cette étape terminée ont été nettoyés. Les sources, validations et références ci-dessous sont conservées.</p>'
            '<p><a href="../knife-contact-C11/apercu.html">Prises C11 intégrées</a> · <a href="../grips-C12/apercu.html">Armes à deux mains C12 intégrées</a></p>' + links, encoding="utf-8")
    (PAGE_ROOT / "apercu.html").write_text(
        '<!doctype html><meta charset="utf-8"><meta http-equiv="refresh" content="0;url=grips-C12/apercu.html">'
        '<a href="grips-C12/apercu.html">Animations intégrées C11/C12</a>', encoding="utf-8")
    arm = ROOT / "artifacts/walk-arm-projection-14/r01"
    items = [p.name for p in arm.iterdir() if p.name in {"Bastion-axe-natif64.gif", "Bastion-dual-natif64.gif", "Bastion-rifle-natif64.gif",
              "Arcaniste-axe-comparatif.png", "Bastion-dual-comparatif.png", "Metamorphe-rifle-comparatif.png", "Traqueuse-axe-comparatif.png"}]
    (arm / "apercu.html").write_text('<!doctype html><meta charset="utf-8"><title>ARM14 · références acceptées</title>'
        '<style>body{background:#131c25;color:#dbe7ed;font:16px system-ui;margin:2rem}img{image-rendering:pixelated;max-width:100%}</style>'
        '<h1>ARM14 · références acceptées et intégrées</h1><p>Exports répétés nettoyés le 30 octobre 2026.</p>'
        + ''.join(f'<p>{n}</p><img src="{n}" alt="{n}">' for n in items), encoding="utf-8")
    print("Historical preview pages now point to preserved references and current integration.")


def verify():
    before = json.loads((REPORT / "preservation-before.json").read_text(encoding="utf-8"))
    changed = []
    for r in before:
        p = ROOT / r["path"]
        if not p.is_file() or row(p) != r:
            changed.append(r["path"])
    plan = json.loads((REPORT / "plan.json").read_text(encoding="utf-8"))
    remaining = [t["path"] for t in plan["targets"] if (ROOT / t["path"]).exists()]
    result = {"status": "passed" if not changed and not remaining else "failed", "protected_files_verified": len(before),
              "changed": changed, "remaining_cleanup_targets": remaining}
    write("preservation-check.json", result)
    print(json.dumps(result))
    assert not changed and not remaining


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=["plan", "pages", "verify"])
    {"plan": plan, "pages": pages, "verify": verify}[parser.parse_args().action]()
