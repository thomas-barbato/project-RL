//! Local library, recovery and retained versions. No campaign saves are touched.
use super::{
    author::Stamp,
    editor::Editor,
    scene::{Document, Scene},
};
use std::path::{Path, PathBuf};

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("artifacts/map-editor")
}
pub fn new_map_path(folder: &Path) -> PathBuf {
    for index in 1.. {
        let name = if index == 1 {
            "nouvelle-carte.json".to_owned()
        } else {
            format!("nouvelle-carte-{index}.json")
        };
        let path = folder.join(name);
        if !path.exists() {
            return path;
        }
    }
    unreachable!()
}
pub fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let data = serde_json::to_vec_pretty(value).map_err(|e| e.to_string())?;
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    let temporary = path.with_extension(format!("{}.{}.tmp", std::process::id(), stamp));
    std::fs::write(&temporary, data).map_err(|e| e.to_string())?;
    if let Err(e) = std::fs::rename(&temporary, path) {
        let _ = std::fs::remove_file(&temporary);
        return Err(e.to_string());
    }
    Ok(())
}
pub fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let size = std::fs::metadata(path).map_err(|e| e.to_string())?.len();
    if size > 32 * 1024 * 1024 {
        return Err("Fichier trop volumineux pour l'éditeur.".into());
    }
    serde_json::from_slice(&std::fs::read(path).map_err(|e| e.to_string())?).map_err(|error| {
        eprintln!("Fichier JSON invalide ({}) : {error}", path.display());
        "Ce fichier est invalide ou incomplet. Choisis une carte JSON enregistrée par l'éditeur."
            .into()
    })
}
pub fn documents(directory: &Path) -> Vec<PathBuf> {
    let mut out: Vec<_> = std::fs::read_dir(directory)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().is_some_and(|e| e == "json"))
        .collect();
    out.sort();
    out
}
pub fn versions(path: &Path) -> PathBuf {
    path.parent()
        .unwrap_or(Path::new("."))
        .join(".versions")
        .join(path.file_stem().unwrap_or_default())
}
pub fn save_map(path: &Path, doc: &Document) -> Result<(), String> {
    if path.is_file() {
        let folder = versions(path);
        std::fs::create_dir_all(&folder).map_err(|e| e.to_string())?;
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_nanos();
        std::fs::copy(path, folder.join(format!("version-{stamp}.json")))
            .map_err(|e| e.to_string())?;
        let retained = documents(&folder);
        for previous in retained.iter().take(retained.len().saturating_sub(5)) {
            std::fs::remove_file(previous).map_err(|e| e.to_string())?;
        }
    }
    write_json(path, doc)
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Recovery {
    pub path: PathBuf,
    pub document: Document,
}
pub fn recover_path() -> PathBuf {
    root().join("recovery/derniere-session.json")
}
pub fn dismiss_recovery() -> Result<(), String> {
    let source = recover_path();
    if !source.exists() {
        return Ok(());
    }
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    std::fs::rename(
        &source,
        source.with_file_name(format!("session-ignoree-{time}.json")),
    )
    .map_err(|e| e.to_string())
}
pub fn load_recovery() -> Option<Recovery> {
    let recovery: Recovery = read_json(&recover_path()).ok()?;
    Scene::from_document(recovery.document.clone()).ok()?;
    if read_json::<Document>(&recovery.path).ok().as_ref() == Some(&recovery.document) {
        return None;
    }
    Some(recovery)
}
#[derive(Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Preferences {
    pub favorites: std::collections::BTreeSet<String>,
    pub recent: Vec<String>,
    pub layers: super::author::Layers,
    pub last_map: Option<PathBuf>,
}
pub fn load_preferences(editor: &mut Editor) {
    if let Ok(p) = read_json::<Preferences>(&root().join("preferences.json")) {
        editor.favorites = p.favorites;
        editor.recent = p.recent;
        editor.layers = p.layers;
        if let Some(path) = p.last_map.filter(|path| path.is_file()) {
            editor.path = path;
        }
    }
}
pub fn save_preferences(editor: &Editor) -> Result<(), String> {
    write_json(
        &root().join("preferences.json"),
        &Preferences {
            favorites: editor.favorites.clone(),
            recent: editor.recent.clone(),
            layers: editor.layers.clone(),
            last_map: Some(editor.path.clone()),
        },
    )
}
pub fn save_stamp(stamp: &Stamp) -> Result<PathBuf, String> {
    validate_stamp(stamp)?;
    let slug: String = stamp
        .name
        .chars()
        .take(60)
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect();
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    let path = root()
        .join("ensembles")
        .join(format!("ensemble-{slug}-{time}.json"));
    write_json(&path, stamp)?;
    Ok(path)
}
pub fn validate_stamp(stamp: &Stamp) -> Result<(), String> {
    if !(1..=super::scene::MAX_SIDE).contains(&stamp.width)
        || !(1..=super::scene::MAX_SIDE).contains(&stamp.height)
        || stamp.floors.as_ref().is_some_and(|floors| {
            floors.len() != (stamp.width * stamp.height) as usize
                || floors
                    .iter()
                    .flatten()
                    .any(|i| !super::catalog::is_floor(*i))
        })
        || stamp.props.len() > 65_536
        || stamp.structures.len() > 65_536
        || stamp.markers.len() > 4096
        || stamp.name.chars().count() > 80
        || stamp
            .backgrounds
            .keys()
            .any(|i| *i >= (stamp.width * stamp.height) as usize)
    {
        return Err("Ensemble invalide : dimensions ou catalogue incorrects.".into());
    }
    super::paint::validate(&stamp.paint, stamp.width, stamp.height)?;
    let inside = |p: project_rl::world::GridPos| {
        p.x >= 0 && p.y >= 0 && p.x < stamp.width && p.y < stamp.height
    };
    if stamp.props.iter().any(|p| {
        p.rotation > 3
            || (p.furniture && p.sprite >= super::catalog::FURNITURE_COUNT)
            || (!p.furniture && ![14, 15].contains(&p.sprite))
            || !(1..=8).contains(&p.details.width)
            || !(1..=8).contains(&p.details.height)
            || !(25..=200).contains(&p.details.scale)
            || !(-31..=31).contains(&p.details.offset_x)
            || !(-31..=31).contains(&p.details.offset_y)
            || !p.cells().all(inside)
    }) || stamp
        .structures
        .iter()
        .any(|p| !inside(p.pos) || p.rotation > 3 || p.style >= super::catalog::WALL_STYLE_COUNT)
        || stamp.markers.iter().any(|m| {
            m.width < 1
                || m.count > 64
                || m.name.chars().count() > 80
                || m.target.chars().count() > 240
                || m.height < 1
                || m.width > stamp.width
                || m.height > stamp.height
                || !inside(m.pos)
                || !inside(project_rl::world::GridPos::new(
                    m.pos.x + m.width - 1,
                    m.pos.y + m.height - 1,
                ))
        })
    {
        return Err("Un élément de l'ensemble dépasse son cadre.".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn saved_versions_retain_five_previous_maps_and_roundtrip() {
        let folder = std::env::temp_dir().join(format!("rl-editor-history-{}", std::process::id()));
        let file = folder.join("test.json");
        let mut doc = Scene::empty(8, 8).unwrap().document;
        for i in 0..7 {
            doc.floors[1] = Some(i);
            save_map(&file, &doc).unwrap();
        }
        assert_eq!(read_json::<Document>(&file).unwrap(), doc);
        assert_eq!(documents(&versions(&file)).len(), 5);
        // Each retained version must also load as a valid scene.
        for saved in documents(&versions(&file)) {
            Scene::from_document(read_json(&saved).unwrap()).unwrap();
        }
        let resolved = folder.canonicalize().unwrap();
        assert_eq!(
            resolved.parent().unwrap(),
            std::env::temp_dir().canonicalize().unwrap()
        );
        std::fs::remove_dir_all(resolved).unwrap();
    }
    #[test]
    fn malformed_prefab_is_rejected_before_transforming_or_pasting() {
        let doc = Scene::empty(4, 4).unwrap().document;
        let mut stamp = Stamp::take(
            &doc,
            super::super::author::Region {
                x: 0,
                y: 0,
                w: 4,
                h: 4,
            },
            &Default::default(),
        );
        stamp.floors = Some(vec![Some(6)]);
        assert!(validate_stamp(&stamp).is_err());
    }
}
