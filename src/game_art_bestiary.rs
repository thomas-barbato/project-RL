//! Approved native actor bank, including authored corpses and separate shadows.
//! Review numbers identify art only; no new simulation identities are created.
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub(super) struct BankEntry {
    pub number: u8,
    pub side: u16,
    pub index: u16,
    pub bleeds: bool,
    pub live_shadow: [u8; 3],
    pub dead_shadow: [u8; 3],
}

include!("../assets/sprites/bestiary/catalog.rs");

struct BankTextures {
    living: [Texture2D; 2],
    corpses: [Texture2D; 2],
}
thread_local! {
    static TEXTURES: BankTextures = BankTextures {
        living: [
            super::texture(include_bytes!("../assets/sprites/bestiary/living-32.png"), FilterMode::Nearest),
            super::texture(include_bytes!("../assets/sprites/bestiary/living-64.png"), FilterMode::Nearest),
        ],
        corpses: [
            super::texture(include_bytes!("../assets/sprites/bestiary/corpse-32.png"), FilterMode::Nearest),
            super::texture(include_bytes!("../assets/sprites/bestiary/corpse-64.png"), FilterMode::Nearest),
        ],
    };
}

pub(super) fn prewarm() {
    TEXTURES.with(|_| {});
}

pub(super) fn entry(number: u8) -> Option<&'static BankEntry> {
    ENTRIES.iter().find(|entry| entry.number == number)
}

pub(super) fn legacy_corpse(actor: u8) -> Option<u8> {
    LEGACY_CORPSES
        .get(usize::from(actor))
        .copied()
        .filter(|n| *n != 0)
}

fn world_rect(rect: Rect, side: u16) -> Rect {
    let scale = f32::from(side) / 32.0;
    let size = rect.size() * scale;
    // Larger visuals extend above and beside the owning cell, with the same feet.
    Rect::new(
        rect.x + (rect.w - size.x) * 0.5,
        rect.y + rect.h - size.y,
        size.x,
        size.y,
    )
}

pub(super) fn draw(rect: Rect, number: u8, corpse: bool, opacity: f32, world: bool) {
    let Some(entry) = entry(number) else { return };
    let rect = if world {
        world_rect(rect, entry.side)
    } else {
        rect
    };
    super::draw_contact_shadow(
        rect,
        if corpse {
            entry.dead_shadow
        } else {
            entry.live_shadow
        },
        entry.side,
        opacity,
    );
    TEXTURES.with(|textures| {
        let family = usize::from(entry.side == 64);
        let texture = if corpse {
            &textures.corpses[family]
        } else {
            &textures.living[family]
        };
        super::draw_cell(texture, rect, entry.index, entry.side, opacity);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_legacy_actor_has_an_authored_corpse_without_renumbering() {
        for actor in 3..=25 {
            let number = legacy_corpse(actor).unwrap();
            assert_eq!(number, actor + 30);
            assert_eq!(entry(number).unwrap().side, 32);
        }
        assert!(legacy_corpse(0).is_none());
        assert!(legacy_corpse(26).is_none());
        assert!(legacy_corpse(255).is_none());
    }

    #[test]
    fn retired_guardian_is_absent_and_mechanical_bodies_are_dry() {
        assert_eq!(ENTRIES.len(), 65);
        assert!(entry(60).is_none());
        assert!(entry(0).is_none());
        assert!(entry(67).is_none());
        for number in [21, 22, 23, 24, 42, 43, 56, 57, 58, 59, 66] {
            assert!(!entry(number).unwrap().bleeds, "{number}");
        }
        for number in [1, 13, 25, 61, 62, 63, 64, 65] {
            assert!(entry(number).unwrap().bleeds, "{number}");
        }
        for entry in ENTRIES {
            for [x, y, width] in [entry.live_shadow, entry.dead_shadow] {
                assert!(u16::from(x) < entry.side && u16::from(y) < entry.side);
                assert!(width > 0 && u16::from(width) < entry.side);
            }
        }
    }

    #[test]
    fn bosses_double_the_visual_footprint_and_preserve_the_cell_ground_anchor() {
        let cell = Rect::new(100.0, 200.0, 32.0, 32.0);
        assert_eq!(world_rect(cell, 32), cell);
        let boss = world_rect(cell, 64);
        assert_eq!(boss, Rect::new(84.0, 168.0, 64.0, 64.0));
        assert_eq!(boss.bottom(), cell.bottom());
        assert_eq!(boss.center().x, cell.center().x);
        for number in 29..=32 {
            assert_eq!(entry(number).unwrap().side, 64);
        }
    }
}
