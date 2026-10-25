//! Installation data live beside the executable; Cargo launches keep using the checkout.
//! An explicit root is strict: missing packaged data must never fall back to authoring files.
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File as DiskFile,
    io::Read,
    path::{Component, Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
};

pub const MANIFEST: &str = "runtime-assets.json";
pub const ARCHIVE: &str = "data.rlpak";

fn select_root(explicit: Option<PathBuf>, executable: &Path, development: &Path) -> PathBuf {
    if let Some(path) = explicit {
        return path;
    }
    let directory = executable.parent().unwrap_or_else(|| Path::new("."));
    if !directory.join(MANIFEST).is_file() && executable.starts_with(development.join("target")) {
        development.to_owned()
    } else {
        directory.to_owned()
    }
}

pub fn root() -> &'static Path {
    static ROOT: OnceLock<PathBuf> = OnceLock::new();
    ROOT.get_or_init(|| {
        select_root(
            std::env::var_os("PROJECT_RL_DATA_DIR").map(PathBuf::from),
            &std::env::current_exe().expect("executable location"),
            Path::new(env!("CARGO_MANIFEST_DIR")),
        )
    })
}

pub fn relative_path(path: &str) -> Result<&Path, String> {
    let value = Path::new(path);
    if path.contains(['\\', ':'])
        || path.split('/').any(|part| matches!(part, "" | "." | ".."))
        || value
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err(format!("Invalid data path: {path}"));
    }
    Ok(value)
}

#[derive(Deserialize)]
pub struct Manifest {
    pub format_version: u32,
    pub files: Vec<File>,
    #[serde(default)]
    pub aliases: BTreeMap<String, String>,
    #[serde(default)]
    pub archive: Option<Archive>,
}

#[derive(Deserialize)]
pub struct Archive {
    pub path: String,
    pub bytes: u64,
}

#[derive(Deserialize)]
pub struct File {
    pub path: String,
    pub bytes: u64,
}

pub fn manifest_at(root: &Path) -> Result<Manifest, String> {
    let bytes = std::fs::read(root.join(MANIFEST)).map_err(|e| e.to_string())?;
    let manifest: Manifest = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if !matches!(manifest.format_version, 1 | 2)
        || (manifest.format_version == 2) != manifest.archive.is_some()
    {
        return Err("Unsupported data manifest".into());
    }
    let mut paths = BTreeSet::new();
    for file in &manifest.files {
        relative_path(&file.path)?;
        if !paths.insert(file.path.as_str()) {
            return Err(format!("Duplicate packaged path: {}", file.path));
        }
    }
    for (alias, target) in &manifest.aliases {
        relative_path(alias)?;
        relative_path(target)?;
        if paths.contains(alias.as_str()) || !paths.contains(target.as_str()) {
            return Err(format!("Alias has no packaged target: {alias}"));
        }
    }
    if let Some(archive) = &manifest.archive {
        if archive.path != ARCHIVE {
            return Err("Unsupported data archive path".into());
        }
    }
    Ok(manifest)
}

/// Reads development files or one indexed archive, without extracting to disk.
/// A packaged installation never falls back to loose files or authoring data.
pub struct DataSource {
    root: PathBuf,
    manifest: Option<Manifest>,
    archive: Option<Mutex<zip::ZipArchive<DiskFile>>>,
    sizes: BTreeMap<String, u64>,
}

impl DataSource {
    pub fn open(root: &Path) -> Result<Self, String> {
        let manifest = if root
            .join(MANIFEST)
            .try_exists()
            .map_err(|e| e.to_string())?
        {
            Some(manifest_at(root)?)
        } else {
            if root.join(ARCHIVE).exists() {
                return Err("Data archive has no manifest".into());
            }
            None
        };
        let sizes: BTreeMap<_, _> = manifest
            .iter()
            .flat_map(|m| &m.files)
            .map(|f| (f.path.clone(), f.bytes))
            .collect();
        let archive = if let Some(description) = manifest.as_ref().and_then(|m| m.archive.as_ref())
        {
            let file = DiskFile::open(root.join(&description.path))
                .map_err(|e| format!("{ARCHIVE}: {e}"))?;
            if file.metadata().map_err(|e| e.to_string())?.len() != description.bytes {
                return Err("Data archive size has changed".into());
            }
            let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
            if archive.len() != sizes.len() {
                return Err("Data archive inventory has changed".into());
            }
            for index in 0..archive.len() {
                let entry = archive.by_index(index).map_err(|e| e.to_string())?;
                relative_path(entry.name())?;
                if sizes.get(entry.name()) != Some(&entry.size())
                    || entry.unix_mode().is_some_and(|m| m & 0o170000 == 0o120000)
                {
                    return Err(format!("Invalid packaged entry: {}", entry.name()));
                }
            }
            Some(Mutex::new(archive))
        } else {
            None
        };
        Ok(Self {
            root: root.to_owned(),
            manifest,
            archive,
            sizes,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn is_packed(&self) -> bool {
        self.archive.is_some()
    }

    pub fn paths(&self) -> impl Iterator<Item = &str> {
        self.sizes.keys().map(String::as_str)
    }

    fn target<'a>(&'a self, path: &'a str) -> Result<&'a str, String> {
        relative_path(path)?;
        let target = self
            .manifest
            .as_ref()
            .and_then(|m| m.aliases.get(path))
            .map_or(path, String::as_str);
        if self.manifest.is_some() && !self.sizes.contains_key(target) {
            return Err(format!("Resource absent from package: {path}"));
        }
        Ok(target)
    }

    pub fn size(&self, path: &str) -> Result<u64, String> {
        let target = self.target(path)?;
        if let Some(size) = self.sizes.get(target) {
            return Ok(*size);
        }
        std::fs::metadata(self.root.join(target))
            .map(|m| m.len())
            .map_err(|e| e.to_string())
    }

    pub fn read(&self, path: &str) -> Result<Vec<u8>, String> {
        self.read_limited(path, u64::MAX)
    }

    pub fn read_limited(&self, path: &str, maximum: u64) -> Result<Vec<u8>, String> {
        let target = self.target(path)?;
        let expected = self.size(path)?;
        if expected > maximum {
            return Err(format!("Resource too large: {path} ({expected} bytes)"));
        }
        let mut bytes = Vec::new();
        if let Some(archive) = &self.archive {
            let mut archive = archive.lock().map_err(|e| e.to_string())?;
            let entry = archive
                .by_name(target)
                .map_err(|e| format!("{path}: {e}"))?;
            entry
                .take(expected.saturating_add(1))
                .read_to_end(&mut bytes)
                .map_err(|e| format!("{path}: {e}"))?;
        } else {
            DiskFile::open(self.root.join(target))
                .map_err(|e| format!("{path}: {e}"))?
                .take(expected.saturating_add(1))
                .read_to_end(&mut bytes)
                .map_err(|e| format!("{path}: {e}"))?;
        }
        if bytes.len() as u64 != expected {
            return Err(format!("Resource size has changed: {path}"));
        }
        Ok(bytes)
    }
}

pub fn source() -> Result<&'static Arc<DataSource>, String> {
    static SOURCE: OnceLock<Result<Arc<DataSource>, String>> = OnceLock::new();
    SOURCE
        .get_or_init(|| DataSource::open(root()).map(Arc::new))
        .as_ref()
        .map_err(Clone::clone)
}

pub fn read(path: &str) -> Result<Vec<u8>, String> {
    source()?.read(path)
}

pub fn read_to_string(path: &str) -> Result<String, String> {
    String::from_utf8(read(path)?).map_err(|e| format!("{path}: {e}"))
}

pub fn load_content(version: &semver::Version) -> Result<crate::content::LoadedContent, String> {
    crate::content::ContentLoader::load_installation(Arc::clone(source()?), version)
        .map_err(|e| e.to_string())
}

pub fn verify() -> Result<usize, String> {
    let data = source()?;
    let manifest = data
        .manifest
        .as_ref()
        .ok_or("Installation has no data manifest")?;
    for file in &manifest.files {
        data.read(&file.path)?;
    }
    load_content(&semver::Version::new(0, 1, 0))?;
    Ok(manifest.files.len())
}

#[cfg(test)]
#[path = "data_paths_tests.rs"]
mod archive_tests;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn explicit_root_never_falls_back_to_the_checkout() {
        assert_eq!(
            select_root(
                Some("missing-data".into()),
                Path::new("some/game.exe"),
                Path::new("checkout")
            ),
            PathBuf::from("missing-data")
        );
        assert_eq!(
            select_root(
                None,
                Path::new("checkout/target/debug/game.exe"),
                Path::new("checkout")
            ),
            PathBuf::from("checkout")
        );
        assert_eq!(
            select_root(None, Path::new("portable/game.exe"), Path::new("checkout")),
            PathBuf::from("portable")
        );
    }
    #[test]
    fn installation_paths_cannot_escape_the_data_directory() {
        for path in ["", "../source.png", "assets/../../file", "/outside"] {
            assert!(relative_path(path).is_err(), "{path}");
        }
        assert!(relative_path("assets/maps/camp/floor.png").is_ok());
        #[cfg(windows)]
        for path in [
            "C:/outside",
            "C:outside",
            "\\\\server\\share",
            "assets\\..\\outside",
        ] {
            assert!(relative_path(path).is_err());
        }
    }
}
