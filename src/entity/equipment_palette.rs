/// Cosmetic paint/fabric palette of an equipment instance. It has no effect
/// on the definition, quality, bonuses, prices or simulation random stream.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum EquipmentPalette {
    #[default]
    Original,
    Red,
    Blue,
    Violet,
    Turquoise,
    Yellow,
    #[serde(alias = "Pink")]
    Gray,
    /// Asset-authored palettes, selected without globally recoloring materials.
    ReviewedVariant1,
    ReviewedVariant2,
}

impl EquipmentPalette {
    pub const ALL: [Self; 7] = [
        Self::Original,
        Self::Red,
        Self::Blue,
        Self::Violet,
        Self::Turquoise,
        Self::Yellow,
        Self::Gray,
    ];

    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Original => "D'origine",
            Self::Red => "Rouge",
            Self::Blue => "Bleu",
            Self::Violet => "Violet",
            Self::Turquoise => "Turquoise",
            Self::Yellow => "Jaune",
            Self::Gray => "Gris",
            Self::ReviewedVariant1 => "Variante 2",
            Self::ReviewedVariant2 => "Variante 3",
        }
    }

    /// Shadow, midtone and highlight of the approved six paint palettes.
    pub const fn ramp(self) -> Option<[[u8; 3]; 3]> {
        match self {
            Self::Original | Self::ReviewedVariant1 | Self::ReviewedVariant2 => None,
            Self::Red => Some([[101, 30, 56], [215, 71, 91], [255, 154, 139]]),
            Self::Blue => Some([[19, 40, 102], [52, 107, 221], [134, 195, 255]]),
            Self::Violet => Some([[61, 27, 97], [153, 81, 219], [217, 168, 255]]),
            Self::Turquoise => Some([[10, 73, 74], [19, 182, 182], [147, 249, 223]]),
            Self::Yellow => Some([[104, 80, 26], [225, 196, 48], [255, 242, 154]]),
            Self::Gray => Some([[38, 43, 48], [161, 171, 181], [232, 239, 245]]),
        }
    }
}

#[cfg(test)]
mod palette_compatibility_tests {
    use super::EquipmentPalette;

    #[test]
    fn legacy_pink_json_and_binary_position_resolve_to_gray() {
        let palette: EquipmentPalette = serde_json::from_str(r#""Pink""#).unwrap();
        assert_eq!(palette, EquipmentPalette::Gray);
        assert_eq!(EquipmentPalette::Gray.index(), 6);
        assert_eq!(serde_json::to_string(&palette).unwrap(), r#""Gray""#);
        let old_binary = 6_u32.to_le_bytes();
        assert_eq!(
            bincode::deserialize::<EquipmentPalette>(&old_binary).unwrap(),
            EquipmentPalette::Gray
        );
        assert_eq!(
            bincode::serialize(&EquipmentPalette::Gray).unwrap(),
            old_binary
        );
        for palette in EquipmentPalette::ALL {
            assert_eq!(
                bincode::deserialize::<EquipmentPalette>(&bincode::serialize(&palette).unwrap())
                    .unwrap(),
                palette
            );
        }
    }
}
