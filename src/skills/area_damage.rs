//! AOE-FALLOFF01: accepted centred damage profile shared by the refonte and lab.
use crate::combat::DamageImpact;
use crate::skills::spatial::{Area, affected_cells};
use crate::world::{GridPos, Map};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CentredDamageBinding {
    pub damage_key: String,
    pub extent_key: String,
    pub extent_divisor: u16,
    pub shape: CentredDamageShape,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CentredDamageShape {
    Square,
    Disc,
    Diamond,
}

impl CentredDamageBinding {
    pub fn area(&self, extent: u16) -> Area {
        let radius = extent / self.extent_divisor.max(1);
        match self.shape {
            CentredDamageShape::Square => Area::Square { radius },
            CentredDamageShape::Disc => Area::Disc { radius },
            CentredDamageShape::Diamond => Area::Diamond { radius },
        }
    }
}

/// Exact square retention, also usable before the single final spell rounding.
pub fn square_retention(centre: GridPos, target: GridPos, radius: u16) -> Option<(u64, u64)> {
    let distance = centre.x.abs_diff(target.x).max(centre.y.abs_diff(target.y));
    if distance > u32::from(radius) {
        return None;
    }
    if radius == 0 {
        return Some((1, 1));
    }
    let diameter = u64::from(radius) * 2;
    Some((diameter - u64::from(distance), diameter))
}

/// Disc distance retains nine fractional places; no damage amount is rounded here.
pub fn retention(centre: GridPos, target: GridPos, area: Area) -> Option<(u64, u64)> {
    match area {
        Area::Square { radius } => square_retention(centre, target, radius),
        Area::Disc { radius } => {
            let dx = u128::from(centre.x.abs_diff(target.x));
            let dy = u128::from(centre.y.abs_diff(target.y));
            let squared = dx * dx + dy * dy;
            if squared > u128::from(radius).pow(2) {
                return None;
            }
            if radius == 0 {
                return Some((1, 1));
            }
            let scale = 1_000_000_000_u128;
            let distance = (squared * scale * scale).isqrt();
            let diameter = 2 * u128::from(radius) * scale;
            Some(((diameter - distance) as u64, diameter as u64))
        }
        Area::Diamond { radius } => {
            let distance =
                u64::from(centre.x.abs_diff(target.x)) + u64::from(centre.y.abs_diff(target.y));
            if distance > u64::from(radius) {
                return None;
            }
            if radius == 0 {
                return Some((1, 1));
            }
            let diameter = u64::from(radius) * 2;
            Some((diameter - distance, diameter))
        }
        Area::Point => (centre == target).then_some((1, 1)),
        _ => None,
    }
}

/// Integer damage before defenses: full at centre, half at nominal outer radius.
/// Distance uses the geometry's metric. Obstacles never renormalize the radius.
/// Directional areas and ongoing statuses are deliberately separate applications.
pub fn damage_at(amount: u16, centre: GridPos, target: GridPos, area: Area) -> Option<u16> {
    let dx = (i64::from(centre.x) - i64::from(target.x)).unsigned_abs();
    let dy = (i64::from(centre.y) - i64::from(target.y)).unsigned_abs();
    let (radius, distance, squared) = match area {
        Area::Point => return (dx == 0 && dy == 0).then_some(amount),
        Area::Square { radius } => {
            let (numerator, denominator) = square_retention(centre, target, radius)?;
            return Some((u64::from(amount) * numerator / denominator) as u16);
        }
        Area::Diamond { radius } => (radius, dx + dy, false),
        Area::Disc { radius } => {
            if dx > u64::from(radius) || dy > u64::from(radius) {
                return None;
            }
            (radius, dx * dx + dy * dy, true)
        }
        _ => return None,
    };
    let limit = if squared {
        u64::from(radius).pow(2)
    } else {
        u64::from(radius)
    };
    if distance > limit {
        return None;
    }
    if radius == 0 {
        return Some(amount);
    }
    let diameter = u64::from(radius) * 2;
    let loss = if squared {
        // floor(A - A*sqrt(d)/(2R)) = A - ceil(sqrt(A*A*d)/(2R)).
        // Inside the disc, A*A*d fits u64 even at u16::MAX. No float rounding.
        let product = u64::from(amount).pow(2) * distance;
        let root = product.isqrt();
        (root + u64::from(root * root < product)).div_ceil(diameter)
    } else {
        (u64::from(amount) * distance).div_ceil(diameter)
    };
    Some(amount.saturating_sub(loss as u16))
}

/// Preserve element families and penetration; reduce each family exactly once.
pub fn impact_at(
    impact: DamageImpact,
    centre: GridPos,
    target: GridPos,
    area: Area,
) -> Option<DamageImpact> {
    damage_at(0, centre, target, area)?;
    Some(impact.map_amounts(|amount| damage_at(amount, centre, target, area).unwrap()))
}

#[derive(Clone, Copy, Debug)]
pub struct CentredDamageCell {
    pub position: GridPos,
    pub impact: DamageImpact,
}

/// Final spatial recipients before actor eligibility and ordinary defense resolution.
pub fn damage_cells(
    map: &Map,
    centre: GridPos,
    area: Area,
    impact: DamageImpact,
) -> Vec<CentredDamageCell> {
    affected_cells(map, centre, area, (1, 0))
        .into_iter()
        .filter_map(|position| {
            impact_at(impact, centre, position, area)
                .map(|impact| CentredDamageCell { position, impact })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::{DamageComponent, DamagePacket, DamageType};
    #[test]
    fn square_shells_match_approved_three_and_five_cell_widths() {
        let c = GridPos::new(4, 4);
        let a = Area::Square { radius: 1 };
        assert_eq!(damage_at(25, c, c, a), Some(25));
        assert_eq!(damage_at(25, c, GridPos::new(3, 3), a), Some(12));
        assert_eq!(damage_at(25, c, GridPos::new(4, 3), a), Some(12));
        assert_eq!(damage_at(25, c, GridPos::new(4, 2), a), None);
        let a = Area::Square { radius: 2 };
        assert_eq!(damage_at(100, c, GridPos::new(3, 3), a), Some(75));
        assert_eq!(damage_at(100, c, GridPos::new(2, 4), a), Some(50));
    }
    #[test]
    fn round_and_diamond_metrics_keep_their_own_geometry() {
        let c = GridPos::new(0, 0);
        assert_eq!(
            damage_at(100, c, GridPos::new(1, 1), Area::Disc { radius: 2 }),
            Some(64)
        );
        assert_eq!(
            damage_at(100, c, GridPos::new(2, 0), Area::Disc { radius: 2 }),
            Some(50)
        );
        assert_eq!(
            damage_at(100, c, GridPos::new(1, 1), Area::Diamond { radius: 2 }),
            Some(50)
        );
        assert_eq!(
            damage_at(
                u16::MAX,
                c,
                GridPos::new(i32::from(u16::MAX), 0),
                Area::Disc { radius: u16::MAX }
            ),
            Some(32767)
        );
        assert_eq!(damage_at(100, c, c, Area::Point), Some(100));
        assert_eq!(
            damage_at(100, c, GridPos::new(1, 0), Area::Square { radius: 0 }),
            None
        );
        assert_eq!(damage_at(100, c, c, Area::Cone { length: 3 }), None);
    }
    #[test]
    fn clipping_removes_cells_without_boosting_remaining_damage() {
        let map = Map::from_ascii(".......\n.......\n....#..\n....#..\n....#..\n.......\n.......")
            .unwrap();
        let centre = GridPos::new(3, 3);
        let impact = DamageImpact::single(DamagePacket::new(100, DamageType::Thermal, 0));
        let cells = damage_cells(&map, centre, Area::Square { radius: 2 }, impact);
        assert!(!cells.iter().any(|c| c.position == GridPos::new(5, 3)));
        assert_eq!(
            cells
                .iter()
                .find(|c| c.position == GridPos::new(2, 3))
                .unwrap()
                .impact
                .raw_total(),
            75
        );
        assert_eq!(
            cells
                .iter()
                .find(|c| c.position == GridPos::new(1, 3))
                .unwrap()
                .impact
                .raw_total(),
            50
        );
    }
    #[test]
    fn mixed_impacts_preserve_defense_metadata_and_do_not_force_one_damage() {
        let impact = DamageImpact::mixed(
            [
                DamageComponent::new(25, DamageType::Kinetic),
                DamageComponent::new(1, DamageType::Thermal),
            ],
            3,
            [(DamageType::Thermal, 7)],
        )
        .unwrap();
        let out = impact_at(
            impact,
            GridPos::new(0, 0),
            GridPos::new(1, 1),
            Area::Square { radius: 1 },
        )
        .unwrap();
        assert_eq!(out.raw_amount(DamageType::Kinetic), 12);
        assert_eq!(out.raw_amount(DamageType::Thermal), 0);
        assert_eq!(out.armor_penetration(), 3);
        assert_eq!(out.resistance_penetration(DamageType::Thermal), 7);
        assert_eq!(impact.raw_total(), 26);
    }
    #[test]
    fn retention_precedes_ordinary_armor_resolution() {
        use crate::combat::{
            ArmorProfile, ArmorRules, DamageRules, ResistanceProfile,
            resolve_damage_impact_with_armor,
        };
        let raw = DamageImpact::single(DamagePacket::new(20, DamageType::Kinetic, 0));
        let edge = impact_at(
            raw,
            GridPos::new(0, 0),
            GridPos::new(1, 0),
            Area::Square { radius: 1 },
        )
        .unwrap();
        let resolved = resolve_damage_impact_with_armor(
            edge,
            ResistanceProfile::default(),
            ArmorProfile::new(2, 0, 0, 0),
            DamageRules::default(),
            ArmorRules::default(),
        );
        assert_eq!(edge.raw_total(), 10);
        assert_eq!(resolved.amount(), 8);
    }
    #[test]
    fn a_rounded_away_primary_component_does_not_hide_remaining_damage() {
        let impact = DamageImpact::mixed(
            [
                DamageComponent::new(1, DamageType::Thermal),
                DamageComponent::new(25, DamageType::Kinetic),
            ],
            3,
            [],
        )
        .unwrap();
        let out = impact_at(
            impact,
            GridPos::new(0, 0),
            GridPos::new(1, 0),
            Area::Square { radius: 1 },
        )
        .unwrap();
        assert_eq!(out.raw_total(), 12);
        assert_eq!(out.primary_component().amount, 12);
        assert_eq!(out.primary_damage_type(), DamageType::Kinetic);
    }
}
