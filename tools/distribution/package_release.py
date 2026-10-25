"""Build a portable game with a reproducible, directly readable data.rlpak archive.

Original artwork is never rewritten. PNGs are deduplicated by content, then
recompressed losslessly. Unknown/missing runtime paths fail the build.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import os
import shutil
import struct
import subprocess
import tempfile
import zlib
import zipfile
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[2]
PLAN = Path(__file__).with_name("runtime-assets.json")
PATH_KEYS = {"floor", "structure", "file", "background", "car_file", "correction", "irradiance", "surface_wear"}
PNG = b"\x89PNG\r\n\x1a\n"
ARCHIVE = "data.rlpak"


def safe_path(name: str) -> str:
    path = PurePosixPath(name)
    if (not name or "\\" in name or ":" in name or path.is_absolute()
            or any(part in {"", ".", ".."} for part in name.split("/"))):
        raise ValueError(f"Unsafe runtime path: {name}")
    return path.as_posix()


def inventory(root: Path = ROOT, plan: Path = PLAN) -> list[str]:
    spec = json.loads(plan.read_text(encoding="utf-8"))
    if spec["format_version"] != 1:
        raise ValueError("Unsupported inventory version")
    result = set()

    def add(name: str) -> None:
        name = safe_path(name)
        if name.startswith(("assets/source-art/", "assets/prototypes/", "artifacts/", "docs/")):
            raise ValueError(f"Authoring dependency in runtime inventory: {name}")
        path = (root / name).resolve(strict=True)
        if not path.is_file() or not path.is_relative_to(root.resolve()):
            raise ValueError(f"Runtime path escapes project: {name}")
        result.add(name)

    def visit(value, parent: Path) -> None:
        if isinstance(value, dict):
            for key, item in value.items():
                if isinstance(item, (dict, list)):
                    visit(item, parent)
                elif key in PATH_KEYS and isinstance(item, str):
                    if not item:
                        continue
                    candidate = root / safe_path(item)
                    if not candidate.is_file():
                        candidate = parent / safe_path(item)
                    add(candidate.resolve().relative_to(root.resolve()).as_posix())
        elif isinstance(value, list):
            for item in value:
                visit(item, parent)

    for name in spec["manifests"]:
        add(name)
        visit(json.loads((root / name).read_text(encoding="utf-8")), (root / name).parent)
    for name in spec["files"]:
        add(name)
    for name in spec["directories"]:
        directory = root / safe_path(name)
        if not directory.is_dir():
            raise ValueError(f"Missing runtime directory: {name}")
        for path in sorted(directory.rglob("*")):
            if path.is_file():
                if path.suffix in {".pyc", ".tmp", ".bak"} or "__pycache__" in path.parts:
                    continue
                add(path.relative_to(root).as_posix())
    return sorted(result)


def chunks(blob: bytes):
    if not blob.startswith(PNG):
        raise ValueError("Not a PNG")
    at = len(PNG)
    while at < len(blob):
        size = struct.unpack_from(">I", blob, at)[0]
        kind = blob[at + 4:at + 8]
        end = at + size + 12
        if end > len(blob):
            raise ValueError("Truncated PNG")
        data = blob[at + 8:end - 4]
        if zlib.crc32(kind + data) != struct.unpack_from(">I", blob, end - 4)[0]:
            raise ValueError("Invalid PNG CRC")
        yield kind, data, blob[at:end]
        at = end
        if kind == b"IEND":
            if at != len(blob):
                raise ValueError("Trailing PNG data")
            return
    raise ValueError("Missing PNG IEND")


def recompress(blob: bytes) -> bytes:
    parts = list(chunks(blob))
    filtered = zlib.decompress(b"".join(data for kind, data, _ in parts if kind == b"IDAT"))
    encoded = zlib.compress(filtered, 9)
    if zlib.decompress(encoded) != filtered:
        raise ValueError("Lossless PNG verification failed")
    idat = struct.pack(">I", len(encoded)) + b"IDAT" + encoded + struct.pack(">I", zlib.crc32(b"IDAT" + encoded))
    output = bytearray(PNG)
    written = False
    for kind, _, raw in parts:
        if kind == b"IDAT":
            if not written:
                output.extend(idat)
                written = True
        else:
            output.extend(raw)
    return bytes(output) if len(output) < len(blob) else blob


def verify(directory: Path) -> dict:
    manifest = json.loads((directory / "runtime-assets.json").read_text(encoding="utf-8"))
    if manifest["format_version"] not in {1, 2}:
        raise ValueError("Unsupported package manifest")
    paths = set()
    archive = None
    try:
        if manifest["format_version"] == 2:
            description = manifest["archive"]
            if description["path"] != ARCHIVE:
                raise ValueError("Unsupported archive path")
            blob = (directory / ARCHIVE).read_bytes()
            if len(blob) != description["bytes"] or hashlib.sha256(blob).hexdigest() != description["sha256"]:
                raise ValueError("Archive integrity failed")
            archive = zipfile.ZipFile(directory / ARCHIVE)
            names = archive.namelist()
            if len(names) != len(set(names)) or set(names) != {row["path"] for row in manifest["files"]}:
                raise ValueError("Archive inventory mismatch")
        for row in manifest["files"]:
            name = safe_path(row["path"])
            if name in paths:
                raise ValueError(f"Duplicate packaged file: {name}")
            if archive is not None:
                info = archive.getinfo(name)
                if info.file_size != row["bytes"] or (info.external_attr >> 16) & 0o170000 == 0o120000:
                    raise ValueError(f"Invalid archive entry: {name}")
                blob = archive.read(name)
            else:
                blob = (directory / name).read_bytes()
            if len(blob) != row["bytes"] or hashlib.sha256(blob).hexdigest() != row["sha256"]:
                raise ValueError(f"Package integrity failed: {name}")
            if name.endswith(".png"):
                list(chunks(blob))
            paths.add(name)
    finally:
        if archive is not None:
            archive.close()
    for alias, target in manifest["aliases"].items():
        safe_path(alias)
        if alias in paths or target not in paths:
            raise ValueError(f"Missing alias target: {alias}")
    exe = manifest["executable"]
    blob = (directory / safe_path(exe["path"])).read_bytes()
    if hashlib.sha256(blob).hexdigest() != exe["sha256"]:
        raise ValueError("Executable integrity failed")
    return manifest


def package(output: Path, executable: Path, root: Path = ROOT, plan: Path = PLAN, *, check_runtime: bool = False) -> dict:
    names = inventory(root, plan)
    previous = None
    if output.is_symlink() or output.is_junction():
        raise ValueError(f"Linked distribution directory: {output}")
    if output.exists():
        # Check links before following any managed path during verification.
        for path in output.rglob("*"):
            if path.is_symlink() or path.is_junction():
                raise ValueError(f"Linked item in distribution: {path}")
        previous = verify(output)
        managed = physical_files(previous)
        for path in output.rglob("*"):
            if path.is_file() and path.relative_to(output).as_posix() not in managed:
                raise ValueError(f"Unmanaged file preserved; choose an empty output instead: {path}")
    if not executable.is_file():
        raise ValueError(f"Build Release first: {executable}")
    output.parent.mkdir(parents=True, exist_ok=True)
    # All inputs and the complete new package are checked before publishing.
    with tempfile.TemporaryDirectory(prefix=".rl-package-", dir=output.parent) as temporary:
        staging = Path(temporary)
        manifest = build_archive(staging, executable, names, root)
        verify(staging)
        if check_runtime:
            environment = os.environ.copy()
            environment["PROJECT_RL_DATA_DIR"] = str(staging.resolve())
            subprocess.run([str((staging / manifest["executable"]["path"]).resolve()), "--verify-assets"],
                           cwd=staging, env=environment, check=True)
        output.mkdir(exist_ok=True)
        current = physical_files(manifest)
        # Replace the executable first: a running Windows game must block the
        # rebuild before its data archive is changed.
        for name in [manifest["executable"]["path"]] + sorted(current - {"runtime-assets.json", manifest["executable"]["path"]}):
            (staging / name).replace(output / name)
        if previous is not None:
            for name in sorted(physical_files(previous) - current):
                obsolete = output / safe_path(name)
                if not obsolete.resolve().is_relative_to(output.resolve()):
                    raise ValueError(f"Obsolete file escapes distribution: {obsolete}")
                obsolete.unlink()
                parent = obsolete.parent
                while parent != output and not any(parent.iterdir()):
                    parent.rmdir()
                    parent = parent.parent
        (output / "mods").mkdir(exist_ok=True)
        (staging / "runtime-assets.json").replace(output / "runtime-assets.json")
    verify(output)
    return manifest


def physical_files(manifest: dict) -> set[str]:
    data = {manifest["archive"]["path"]} if manifest["format_version"] == 2 else {row["path"] for row in manifest["files"]}
    return data | {manifest["executable"]["path"], "runtime-assets.json", "Jouer.cmd"}


def build_archive(output: Path, executable: Path, names: list[str], root: Path) -> dict:
    files, aliases, canonical = [], {}, {}
    original_bytes = recompression_saved = dedup_saved = 0
    with zipfile.ZipFile(output / ARCHIVE, "w") as archive:
        for name in names:
            blob = (root / name).read_bytes()
            original_bytes += len(blob)
            original_sha = hashlib.sha256(blob).hexdigest()
            if name.endswith(".png") and original_sha in canonical:
                aliases[name] = canonical[original_sha]
                dedup_saved += len(blob)
                continue
            if name.endswith(".png"):
                canonical[original_sha] = name
                packed = recompress(blob)
            else:
                packed = blob
            recompression_saved += len(blob) - len(packed)
            # Fixed metadata and sorted paths make identical inputs reproducible.
            entry = zipfile.ZipInfo(name, date_time=(1980, 1, 1, 0, 0, 0))
            entry.create_system = 3
            entry.external_attr = 0o100644 << 16
            method = zipfile.ZIP_DEFLATED if len(zlib.compress(packed, 9)) < len(packed) else zipfile.ZIP_STORED
            archive.writestr(entry, packed, compress_type=method, compresslevel=9)
            files.append({"path": name, "bytes": len(packed), "sha256": hashlib.sha256(packed).hexdigest(), "source_sha256": original_sha})
    exe_name = "project-rl.exe" if executable.suffix == ".exe" else "project-rl"
    shutil.copyfile(executable, output / exe_name)
    (output / "Jouer.cmd").write_text('@echo off\ncd /d "%~dp0"\n"%~dp0project-rl.exe" %*\n', encoding="ascii")
    exe_blob = executable.read_bytes()
    manifest = {
        "format_version": 2, "files": files, "aliases": aliases,
        "archive": {"path": ARCHIVE, "format": "zip", "bytes": (output / ARCHIVE).stat().st_size,
                    "sha256": hashlib.sha256((output / ARCHIVE).read_bytes()).hexdigest()},
        "executable": {"path": exe_name, "bytes": len(exe_blob), "sha256": hashlib.sha256(exe_blob).hexdigest()},
        "statistics": {"logical_input_files": len(names), "original_data_bytes": original_bytes,
                       "deduplicated_bytes": dedup_saved, "lossless_compression_saved_bytes": recompression_saved,
                       "unpacked_data_bytes": sum(row["bytes"] for row in files),
                       "packaged_data_bytes": (output / ARCHIVE).stat().st_size},
    }
    (output / "runtime-assets.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    return manifest


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=ROOT / "dist/Project-RL")
    parser.add_argument("--executable", type=Path, default=ROOT / "target/release/project-rl.exe")
    parser.add_argument("--verify", type=Path)
    parser.add_argument("--build", action="store_true")
    args = parser.parse_args()
    if args.verify:
        result = verify(args.verify)
    else:
        if args.build:
            subprocess.run(["cargo", "build", "--locked", "--release", "--bin", "project-rl"], cwd=ROOT, check=True)
        result = package(args.output.absolute(), args.executable.resolve(), check_runtime=args.build)
    print(json.dumps(result["statistics"], indent=2))


if __name__ == "__main__":
    main()
