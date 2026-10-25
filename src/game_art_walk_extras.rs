//! Additional whole-figure candidates, visible only inside the isolated review.
use super::*;
use project_rl::asset_cache::ByteCache;
use serde::Deserialize;
use std::{cell::RefCell, sync::OnceLock};
#[derive(Deserialize)]
struct Entry {
    id: String,
    file: String,
    foot: [u16; 2],
    vertical_offset: i16,
    #[serde(default)]
    left_row: u8,
}
fn entries() -> &'static [Entry] {
    static E: OnceLock<Vec<Entry>> = OnceLock::new();
    E.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../assets/animations/walk-08/extras/manifest.json"
        ))
        .unwrap()
    })
}
fn id(v: Visual) -> Option<&'static str> {
    Some(match v {
        Visual::AgentForm(0) => "FORM-PREDATOR",
        Visual::AgentForm(1) => "FORM-COLOSSUS",
        Visual::AgentForm(2) => "FORM-ARMOR",
        Visual::AgentForm(3) => "FORM-POSSESSED",
        Visual::AgentForm(4) => "FORM-ABYSS",
        Visual::AgentForm(5) => "FORM-CLOSED",
        Visual::DroneScout => "DRONE-SCOUT",
        Visual::DroneTracker => "DRONE-TRACKER",
        Visual::DroneGuardian => "DRONE-GUARDIAN",
        _ => return None,
    })
}
thread_local! {static CACHE:RefCell<ByteCache<String,Texture2D>>=const{RefCell::new(ByteCache::new(16*1024*1024))};}
fn source(entry: &Entry, pose: Pose) -> Rect {
    let column = if pose.frame == 8 { 0 } else { pose.frame + 1 };
    let row = if pose.facing < 0 {
        entry.left_row
    } else {
        1 - entry.left_row
    };
    Rect::new(column as f32 * 128., row as f32 * 128., 128., 128.)
}

pub(super) fn draw(cell: Rect, visual: Visual, opacity: f32, pose: Option<Pose>) -> bool {
    if !super::corrections_enabled() {
        return false;
    }
    let Some(p) =
        pose.filter(|p| [-1, 1].contains(&p.facing) && [0, 1, 2, 3, 4, 5, 8].contains(&p.frame))
    else {
        return false;
    };
    let Some(entry) = id(visual).and_then(|id| entries().iter().find(|e| e.id == id)) else {
        return false;
    };
    CACHE.with_borrow_mut(|cache| {
        if cache.get(&entry.id).is_none() {
            let Some(image) = project_rl::data_paths::read(&entry.file)
                .ok()
                .and_then(|b| Image::from_file_with_format(&b, Some(ImageFormat::Png)).ok())
            else {
                return false;
            };
            let texture = Texture2D::from_image(&image);
            texture.set_filter(FilterMode::Nearest);
            cache.insert(entry.id.clone(), texture, image.bytes.len());
        }
        let scale = cell.w / 64.;
        crate::world_readability::texture(
            cache.get(&entry.id).unwrap(),
            cell.center().x - entry.foot[0] as f32 * scale,
            cell.bottom() + (entry.vertical_offset as f32 - entry.foot[1] as f32) * scale,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                source: Some(source(entry, p)),
                dest_size: Some(vec2(128. * scale, 128. * scale)),
                ..Default::default()
            },
        );
        true
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predator_uses_its_left_facing_authored_row_for_leftward_steps() {
        let predator = entries().iter().find(|e| e.id == "FORM-PREDATOR").unwrap();
        // The actual source sheet has right-facing figures on top and left
        // facing figures below. Keep each whole figure and its phase order.
        for frame in [8, 0, 1, 2, 3, 4, 5] {
            let left = source(predator, Pose { facing: -1, frame });
            let right = source(predator, Pose { facing: 1, frame });
            assert_eq!(left.y, 128.);
            assert_eq!(right.y, 0.);
            assert_eq!(left.x, right.x);
            assert_eq!(
                left.x,
                if frame == 8 {
                    0.
                } else {
                    f32::from(frame + 1) * 128.
                }
            );
        }
        assert!(
            entries()
                .iter()
                .filter(|e| e.id != "FORM-PREDATOR")
                .all(|e| e.left_row == 0)
        );
    }
}
