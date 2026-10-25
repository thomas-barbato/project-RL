import json
from pathlib import Path
import struct
import tempfile
import unittest
import zlib
import zipfile
import package_release as release


def png():
    def chunk(kind, data):
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data))
    return release.PNG + chunk(b"IHDR", struct.pack(">IIBBBBB", 1, 1, 8, 6, 0, 0, 0)) + chunk(b"IDAT", zlib.compress(b"\0\xff\x80\0\xff", 1)) + chunk(b"IEND", b"")


class PackageTests(unittest.TestCase):
    def test_png_roundtrip_keeps_filtered_pixels_and_ancillary_chunks(self):
        original = png()
        output = release.recompress(original)
        scanlines = lambda blob: zlib.decompress(b"".join(data for kind, data, _ in release.chunks(blob) if kind == b"IDAT"))
        self.assertEqual(scanlines(original), scanlines(output))
        self.assertLessEqual(len(output), len(original))
        self.assertEqual([(k,d) for k,d,_ in release.chunks(original) if k != b"IDAT"], [(k,d) for k,d,_ in release.chunks(output) if k != b"IDAT"])

    def test_package_shares_duplicates_and_detects_missing_or_changed_files(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for name in ["assets/a.png", "assets/b.png"]:
                (root / name).parent.mkdir(exist_ok=True)
                (root / name).write_bytes(png())
            exe = root / "game.exe"
            exe.write_bytes(b"test executable")
            plan = root / "plan.json"
            plan.write_text(json.dumps({"format_version":1, "manifests":[], "files":["assets/a.png","assets/b.png"], "directories":[]}))
            output = root / "output"
            result = release.package(output, exe, root, plan)
            self.assertEqual(result["aliases"], {"assets/b.png":"assets/a.png"})
            self.assertEqual(result["format_version"], 2)
            self.assertEqual(sorted(p.name for p in output.iterdir()), ["Jouer.cmd", "data.rlpak", "mods", "project-rl.exe", "runtime-assets.json"])
            with zipfile.ZipFile(output / release.ARCHIVE) as archive:
                self.assertEqual(archive.namelist(), ["assets/a.png"])
                self.assertEqual(archive.read("assets/a.png"), release.recompress(png()))
            self.assertFalse((output / "assets/b.png").exists())
            release.verify(output)
            # Rebuilding reuses the same managed output, without accumulating copies.
            release.package(output, exe, root, plan)
            extra = output / "player-notes.txt"
            extra.write_text("keep me")
            with self.assertRaises(ValueError): release.package(output, exe, root, plan)
            self.assertEqual(extra.read_text(), "keep me")
            extra.unlink()
            (output / release.ARCHIVE).write_bytes(b"changed")
            with self.assertRaises(ValueError): release.verify(output)
            with self.assertRaises(ValueError): release.package(output, exe, root, plan)

    def test_reproducible_rebuild_adds_assets_and_preserves_old_package_on_bad_input(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            assets = root / "assets"
            assets.mkdir()
            original = png()
            (assets / "a.png").write_bytes(original)
            exe = root / "game.exe"
            exe.write_bytes(b"test executable")
            plan = root / "plan.json"
            plan.write_text(json.dumps({"format_version":1, "manifests":[], "files":[], "directories":["assets"]}))
            output = root / "output"
            first = release.package(output, exe, root, plan)
            archive = (output / release.ARCHIVE).read_bytes()
            release.package(output, exe, root, plan)
            self.assertEqual(archive, (output / release.ARCHIVE).read_bytes())
            (assets / "new-sound.wav").write_bytes(b"placeholder audio bytes" * 100)
            second = release.package(output, exe, root, plan)
            self.assertEqual(second["statistics"]["logical_input_files"], 2)
            self.assertNotEqual(first["archive"]["sha256"], second["archive"]["sha256"])
            self.assertEqual((assets / "a.png").read_bytes(), original)
            (assets / "broken.png").write_bytes(b"not a PNG")
            with self.assertRaises(ValueError): release.package(output, exe, root, plan)
            self.assertEqual(release.verify(output)["archive"], second["archive"])
            self.assertEqual(list(root.glob(".rl-package-*")), [])

    def test_migration_replaces_only_verified_legacy_files(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "assets").mkdir()
            (root / "assets/a.png").write_bytes(png())
            exe = root / "game.exe"
            exe.write_bytes(b"test executable")
            plan = root / "plan.json"
            plan.write_text(json.dumps({"format_version":1, "manifests":[], "files":["assets/a.png"], "directories":[]}))
            output = root / "output"
            manifest = release.package(output, exe, root, plan)
            with zipfile.ZipFile(output / release.ARCHIVE) as archive:
                blob = archive.read("assets/a.png")
            (output / "assets").mkdir()
            (output / "assets/a.png").write_bytes(blob)
            (output / release.ARCHIVE).unlink()
            manifest["format_version"] = 1
            del manifest["archive"]
            (output / "runtime-assets.json").write_text(json.dumps(manifest))
            release.package(output, exe, root, plan)
            self.assertFalse((output / "assets").exists())
            self.assertEqual(release.verify(output)["format_version"], 2)

    def test_inventory_forbids_source_art_and_traversal(self):
        for name in ["../private", "C:/private", "assets\\..\\private", "/private", "assets//a", "./a", "assets/./a"]:
            with self.assertRaises(ValueError): release.safe_path(name)
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            plan = root / "plan.json"
            plan.write_text(json.dumps({"format_version":1, "manifests":[], "files":["assets/source-art/master.png"], "directories":[]}))
            with self.assertRaises(ValueError): release.inventory(root, plan)


if __name__ == "__main__":
    unittest.main()
