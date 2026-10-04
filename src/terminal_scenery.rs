//! Small silhouettes viewed from above. Variants reuse authored footprints.
use super::{PixelGlyph, TerminalGlyphPalette, dim, draw_pixel_glyph_palette};
use crate::test_sector::Decor;
use macroquad::prelude::*;
use project_rl::world::GridPos;

pub(super) fn variant(kind: Decor, position: GridPos) -> Decor {
    if kind == Decor::ShopSign {
        return Decor::EntranceMat;
    }
    if !matches!(
        kind,
        Decor::Crate
            | Decor::Console
            | Decor::Pillar
            | Decor::Server
            | Decor::Bin
            | Decor::LampPost
            | Decor::TransitSign
    ) {
        return kind;
    }
    let mut hash = (position.x as u32).wrapping_mul(0x9e37_79b9)
        ^ (position.y as u32).wrapping_mul(0x85eb_ca6b);
    hash = (hash ^ (hash >> 16)).wrapping_mul(0x7feb_352d);
    match (kind, hash % 6) {
        (Decor::Crate, 0 | 1) => Decor::Barrel,
        (Decor::Crate, 2) => Decor::CargoPallet,
        (Decor::Crate, 3) => Decor::CableCoil,
        (Decor::Console, 0) | (Decor::Pillar, 0) => Decor::VentUnit,
        (Decor::Server, 0 | 1) => Decor::ElectricalCabinet,
        (Decor::Bin, 0 | 1) => Decor::Hydrant,
        (Decor::LampPost, 0 | 1) => Decor::RoadBollard,
        (Decor::TransitSign, 0 | 1) => Decor::StreetDrain,
        (Decor::TransitSign, _) => Decor::Street,
        _ => kind,
    }
}

pub(super) fn glyph(kind: Decor) -> Option<(&'static PixelGlyph, [[u8; 3]; 3])> {
    let metal = [[132, 157, 163], [48, 66, 72], [184, 209, 204]];
    let wood = [[156, 132, 93], [65, 62, 47], [192, 171, 117]];
    let green = [[77, 126, 79], [31, 68, 46], [133, 165, 94]];
    Some(match kind {
        Decor::UrbanTree | Decor::Tree => (
            &[
                "...++...", ".+####+.", "+###*##+", "+#*####+", "+####*#+", ".+####+.", "..+##+..",
                "...++...",
            ],
            green,
        ),
        Decor::Bench => (
            &[
                "........", ".######.", ".++++++.", ".######.", ".******.", ".#....#.", "........",
                "........",
            ],
            wood,
        ),
        Decor::Planter => (
            &[
                "........", ".######.", ".#++++#.", ".#+**+#.", ".#**++#.", ".#++++#.", ".######.",
                "........",
            ],
            [[121, 134, 130], [41, 78, 51], [107, 162, 90]],
        ),
        Decor::LampPost => (
            &[
                "........", "...##...", "..#++#..", "..#**#..", "..#**#..", "..#++#..", "...##...",
                "........",
            ],
            [[118, 137, 143], [37, 51, 57], [230, 206, 130]],
        ),
        Decor::Bin => (
            &[
                "........", "..####..", ".#++++#.", ".#+##+#.", ".#+##+#.", ".#++++#.", "..####..",
                "........",
            ],
            [[113, 137, 128], [40, 64, 59], [161, 177, 154]],
        ),
        Decor::ShopCounter | Decor::ClinicCounter => (
            &[
                "........", ".######.", ".#****#.", ".######.", ".++++++.", ".+##++#.", ".++++++.",
                "........",
            ],
            wood,
        ),
        Decor::ShopShelf => (
            &[
                "........", ".######.", ".#*+#*#.", ".######.", ".#*#++#.", ".######.", ".++++++.",
                "........",
            ],
            wood,
        ),
        Decor::DisplayCase => (
            &[
                "........", ".######.", ".#*+++#.", ".#++++#.", ".#++*+#.", ".#++++#.", ".######.",
                "........",
            ],
            [[127, 169, 177], [42, 72, 82], [174, 211, 207]],
        ),
        Decor::VendingMachine => (
            &[
                "........", ".######.", ".#++++#.", ".#*+++#.", ".#++++#.", ".###**#.", ".######.",
                "........",
            ],
            [[128, 150, 161], [42, 62, 76], [111, 186, 191]],
        ),
        Decor::CafeTable => (
            &[
                "...##...", "..++++..", "#+####+#", "+#****#+", "+#****#+", "#+####+#", "..++++..",
                "...##...",
            ],
            [[164, 130, 107], [68, 65, 57], [185, 157, 117]],
        ),
        Decor::EntranceMat => (
            &[
                "........", ".######.", ".#++++#.", ".#****#.", ".#++++#.", ".#****#.", ".######.",
                "........",
            ],
            [[106, 117, 109], [34, 43, 38], [141, 143, 113]],
        ),
        Decor::Barrel => (
            &[
                "........", "..####..", ".#++++#.", ".#+*++#.", ".#++++#.", ".#++++#.", "..####..",
                "........",
            ],
            [[151, 112, 77], [65, 58, 44], [195, 164, 108]],
        ),
        Decor::CargoPallet => (
            &[
                ".######.", ".++++++.", ".#**##+.", ".#+###+.", ".###**#.", ".+###+#.", ".++++++.",
                ".######.",
            ],
            wood,
        ),
        Decor::VentUnit => (
            &[
                "........", ".######.", ".#+##+#.", ".#++*##.", ".##*++#.", ".#+##+#.", ".######.",
                "........",
            ],
            metal,
        ),
        Decor::ElectricalCabinet => (
            &[
                "........", ".######.", ".#++++#.", ".#++*+#.", ".#+*++#.", ".#++*+#.", ".######.",
                "........",
            ],
            [[138, 149, 141], [48, 62, 59], [202, 178, 102]],
        ),
        Decor::Hydrant => (
            &[
                "........", "...##...", "..#**#..", ".##++##.", ".##++##.", "..#++#..", "...##...",
                "........",
            ],
            [[163, 99, 81], [74, 49, 44], [196, 150, 111]],
        ),
        Decor::RoadBollard => (
            &[
                "........", "...++...", "..+##+..", "..#**#..", "..#**#..", "..+##+..", "...++...",
                "........",
            ],
            [[147, 143, 122], [46, 54, 55], [186, 179, 135]],
        ),
        Decor::StreetDrain => (
            &[
                "........", ".######.", ".#++++#.", ".#.#.##.", ".##.#.#.", ".#++++#.", ".######.",
                "........",
            ],
            [[102, 117, 119], [16, 24, 27], [146, 155, 149]],
        ),
        Decor::CableCoil => (
            &[
                "........", "..####..", ".#++++#.", ".#+##+#.", ".#+#++#.", ".#++###.", "..##..#.",
                "......#.",
            ],
            [[137, 126, 91], [37, 44, 44], [178, 157, 101]],
        ),
        Decor::Server => (
            &[
                ".######.", ".#++++#.", ".#+##+#.", ".#+##+#.", ".#++++#.", ".######.", ".##**##.",
                "........",
            ],
            [[117, 154, 163], [36, 58, 69], [101, 194, 177]],
        ),
        Decor::Crate => (
            &[
                "........", ".######.", ".#*+++#.", ".#+##+#.", ".#+##+#.", ".#+++*#.", ".######.",
                "........",
            ],
            wood,
        ),
        Decor::Console => (
            &[
                "........", ".######.", ".#****#.", ".#*++*#.", ".######.", ".#+**+#.", ".######.",
                "........",
            ],
            [[122, 156, 165], [37, 56, 65], [107, 174, 186]],
        ),
        Decor::Coolant => (
            &[
                "..####..", ".#++++#.", "#+****+#", "#+*++*+#", "#+*++*+#", "#+****+#", ".#++++#.",
                "..####..",
            ],
            [[109, 157, 165], [34, 61, 73], [143, 180, 178]],
        ),
        Decor::Pillar => (
            &[
                "........", ".######.", ".#****#.", ".#*++*#.", ".#*++*#.", ".#****#.", ".######.",
                "........",
            ],
            [[132, 153, 155], [54, 72, 76], [163, 183, 179]],
        ),
        Decor::ClinicBed => (
            &[
                ".######.", ".#****#.", ".######.", ".#++++#.", ".#++++#.", ".#++++#.", ".######.",
                "........",
            ],
            [[125, 162, 151], [39, 71, 64], [180, 206, 187]],
        ),
        _ => return None,
    })
}

pub(super) fn draw(rect: Rect, kind: Decor, visible: bool) -> bool {
    let Some((pattern, colors)) = glyph(kind) else {
        return false;
    };
    if rect.w >= 16.0 {
        let center = rect.center() + rect.size() * 0.035;
        for (radius, alpha) in [(0.42, 0.08), (0.36, 0.14), (0.30, 0.20)] {
            draw_ellipse(
                center.x,
                center.y,
                rect.w * radius,
                rect.h * radius * 0.85,
                0.0,
                Color::new(0.0, 0.0, 0.0, alpha * if visible { 1.0 } else { 0.5 }),
            );
        }
    }
    let colors = colors.map(|[r, g, b]| dim(Color::from_rgba(r, g, b, 255), visible));
    draw_pixel_glyph_palette(
        rect,
        pattern,
        TerminalGlyphPalette::new(colors[0], Some(colors[1]), Some(colors[2])),
    );
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_saved_floor_symbols_are_removed_without_refreshing_hidden_terrain() {
        use super::super::{KnownTile, TerminalView};
        use crate::test_sector::SectorDecor;
        use project_rl::world::Terrain;
        let mut view = TerminalView {
            title: String::new(),
            decor: SectorDecor::default(),
            remembered: std::collections::BTreeMap::new(),
        };
        for (x, kind) in [(0, Decor::TransitSign), (1, Decor::ShopSign)] {
            view.remembered.insert(
                GridPos::new(x, 0),
                KnownTile {
                    terrain: Terrain::Floor,
                    decor: kind,
                },
            );
        }
        let bytes = bincode::serialize(&view).unwrap();
        let restored: TerminalView = bincode::deserialize(&bytes).unwrap();
        for x in 0..2 {
            let tile = restored.known(GridPos::new(x, 0)).unwrap();
            assert!(!matches!(tile.decor, Decor::TransitSign | Decor::ShopSign));
            assert_eq!(tile.terrain, Terrain::Floor);
            assert!(!tile.decor.blocks());
        }
        assert_eq!(restored.known(GridPos::new(3, 0)), None);
        assert_eq!(bincode::serialize(&restored).unwrap(), bytes);
    }

    #[test]
    fn scenery_variants_preserve_authored_obstacles_and_walkable_marks() {
        let mut observed = std::collections::BTreeSet::new();
        for kind in [
            Decor::Crate,
            Decor::Console,
            Decor::Pillar,
            Decor::Server,
            Decor::Bin,
            Decor::LampPost,
            Decor::TransitSign,
            Decor::ShopSign,
        ] {
            for y in -16..16 {
                for x in -16..16 {
                    let result = variant(kind, GridPos::new(x, y));
                    assert_eq!(result.blocks(), kind.blocks());
                    assert_eq!(variant(result, GridPos::new(x, y)), result);
                    if result != kind && result != Decor::Street {
                        observed.insert(result.label());
                    }
                }
            }
        }
        assert_eq!(observed.len(), 9);
    }

    #[test]
    fn every_new_scenery_has_a_readable_pattern_and_a_distinct_label() {
        let mut patterns = Vec::new();
        for kind in super::super::CITY_DECOR_KINDS {
            let (pattern, _) = glyph(kind).expect("catalogued scenery must render");
            assert!(
                pattern
                    .iter()
                    .all(|row| row.len() == 8 && row.bytes().all(|b| b".#+*".contains(&b)))
            );
            assert!(
                !patterns.contains(pattern),
                "duplicate silhouette: {kind:?}"
            );
            patterns.push(*pattern);
        }
    }
}
