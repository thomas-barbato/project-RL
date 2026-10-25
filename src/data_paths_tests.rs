use super::*;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        static ID: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "project-rl-pack-{}-{}",
            std::process::id(),
            ID.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn pack(&self, files: &[(String, Vec<u8>)], aliases: serde_json::Value) {
        let archive = DiskFile::create(self.0.join(ARCHIVE)).unwrap();
        let mut writer = zip::ZipWriter::new(archive);
        for (path, bytes) in files {
            writer
                .start_file(
                    path,
                    zip::write::SimpleFileOptions::default()
                        .compression_method(zip::CompressionMethod::Deflated),
                )
                .unwrap();
            writer.write_all(bytes).unwrap();
        }
        writer.finish().unwrap();
        let rows: Vec<_> = files
            .iter()
            .map(|(path, bytes)| serde_json::json!({"path":path, "bytes":bytes.len()}))
            .collect();
        std::fs::write(
            self.0.join(MANIFEST),
            serde_json::to_vec(&serde_json::json!({
                "format_version":2, "files":rows, "aliases":aliases,
                "archive":{"path":ARCHIVE, "bytes":self.0.join(ARCHIVE).metadata().unwrap().len()}
            }))
            .unwrap(),
        )
        .unwrap();
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn packed_reads_resolve_aliases_and_never_fall_back_to_loose_files() {
    let fixture = Fixture::new();
    fixture.pack(
        &[("assets/a.txt".into(), b"archive bytes".to_vec())],
        serde_json::json!({"assets/b.txt":"assets/a.txt"}),
    );
    std::fs::create_dir(fixture.0.join("assets")).unwrap();
    std::fs::write(fixture.0.join("assets/a.txt"), b"loose override").unwrap();
    std::fs::write(fixture.0.join("assets/missing.txt"), b"loose fallback").unwrap();
    let source = DataSource::open(&fixture.0).unwrap();
    assert!(source.is_packed());
    assert_eq!(source.read("assets/b.txt").unwrap(), b"archive bytes");
    assert!(source.read("assets/missing.txt").is_err());
    assert!(source.read("../outside").is_err());
    assert!(source.read_limited("assets/a.txt", 3).is_err());
    drop(source);
    std::fs::remove_file(fixture.0.join(ARCHIVE)).unwrap();
    assert!(DataSource::open(&fixture.0).is_err());
}

#[test]
fn development_and_legacy_packages_remain_readable() {
    let fixture = Fixture::new();
    std::fs::write(fixture.0.join("a.txt"), b"development").unwrap();
    assert_eq!(
        DataSource::open(&fixture.0).unwrap().read("a.txt").unwrap(),
        b"development"
    );
    std::fs::write(fixture.0.join(MANIFEST), br#"{"format_version":1,"files":[{"path":"a.txt","bytes":11}],"aliases":{"b.txt":"a.txt"}}"#).unwrap();
    let source = DataSource::open(&fixture.0).unwrap();
    assert!(!source.is_packed());
    assert_eq!(source.read("b.txt").unwrap(), b"development");
}

#[test]
fn missing_manifest_corrupt_bytes_and_mismatched_inventory_are_rejected() {
    let fixture = Fixture::new();
    fixture.pack(
        &[("a.txt".into(), b"some compressed data".to_vec())],
        serde_json::json!({}),
    );
    let mut bytes = std::fs::read(fixture.0.join(ARCHIVE)).unwrap();
    let start = zip::ZipArchive::new(std::io::Cursor::new(&bytes))
        .unwrap()
        .by_name("a.txt")
        .unwrap()
        .data_start()
        .unwrap() as usize;
    bytes[start] ^= 0xff;
    std::fs::write(fixture.0.join(ARCHIVE), bytes).unwrap();
    assert!(DataSource::open(&fixture.0).unwrap().read("a.txt").is_err());
    std::fs::remove_file(fixture.0.join(MANIFEST)).unwrap();
    assert!(DataSource::open(&fixture.0).is_err());
    fixture.pack(&[("a.txt".into(), b"data".to_vec())], serde_json::json!({}));
    let mut manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(fixture.0.join(MANIFEST)).unwrap()).unwrap();
    manifest["files"][0]["path"] = "other.txt".into();
    std::fs::write(
        fixture.0.join(MANIFEST),
        serde_json::to_vec(&manifest).unwrap(),
    )
    .unwrap();
    assert!(DataSource::open(&fixture.0).is_err());
}

fn collect_files(root: &Path, directory: &Path, output: &mut Vec<(String, Vec<u8>)>) {
    for entry in std::fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_files(root, &path, output);
        } else {
            output.push((
                path.strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/"),
                std::fs::read(path).unwrap(),
            ));
        }
    }
}

#[test]
fn archived_catalogs_and_additional_loose_mods_preserve_all_game_rules() {
    let development = Path::new(env!("CARGO_MANIFEST_DIR"));
    let version = semver::Version::new(0, 1, 0);
    let expected = crate::content::ContentLoader::load(
        &[development.join("content"), development.join("mods")],
        &version,
    )
    .unwrap();
    let mut files = Vec::new();
    collect_files(development, &development.join("content"), &mut files);
    collect_files(development, &development.join("mods"), &mut files);
    files.sort_by(|a, b| a.0.cmp(&b.0));
    let packed = Fixture::new();
    packed.pack(&files, serde_json::json!({}));
    let actual = crate::content::ContentLoader::load_installation(
        Arc::new(DataSource::open(&packed.0).unwrap()),
        &version,
    )
    .unwrap();
    assert_eq!(expected, actual);
    let modded = Fixture::new();
    for (name, bytes) in files.iter().filter(|(name, _)| name.starts_with("mods/")) {
        let path = modded.0.join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, bytes).unwrap();
    }
    files.retain(|(name, _)| name.starts_with("content/"));
    modded.pack(&files, serde_json::json!({}));
    let actual = crate::content::ContentLoader::load_installation(
        Arc::new(DataSource::open(&modded.0).unwrap()),
        &version,
    )
    .unwrap();
    assert_eq!(expected, actual);
}
