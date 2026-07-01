//! Item rarity — quality tiers with color coding and stat multipliers.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Quality tiers for items, from common to legendary.
///
/// Rarity affects the item's stat multiplier (higher rarity = stronger stats),
/// UI color coding, vendor price, and loot drop probability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemRarity {
    /// Grey — baseline quality, no stat multiplier.
    Common,
    /// Green — slightly better, 1.3× stat multiplier.
    Uncommon,
    /// Blue — good quality, 1.6× stat multiplier.
    Rare,
    /// Purple — excellent quality, 2.0× stat multiplier.
    Epic,
    /// Orange — best quality, 2.5× stat multiplier.
    Legendary,
}

impl ItemRarity {
    /// Display colour for UI tooltips and nameplates.
    pub fn color(&self) -> Color {
        match self {
            Self::Common => Color::srgb(0.7, 0.7, 0.7),
            Self::Uncommon => Color::srgb(0.3, 0.8, 0.3),
            Self::Rare => Color::srgb(0.3, 0.5, 1.0),
            Self::Epic => Color::srgb(0.7, 0.3, 0.9),
            Self::Legendary => Color::srgb(1.0, 0.6, 0.0),
        }
    }

    /// Hex colour string for UI labels (e.g. `"#B3B3B3"`).
    pub fn color_hex(&self) -> &'static str {
        match self {
            Self::Common => "#B3B3B3",
            Self::Uncommon => "#4DCC4D",
            Self::Rare => "#4D7FFF",
            Self::Epic => "#B34DE5",
            Self::Legendary => "#FF9900",
        }
    }

    /// Stat multiplier applied to base item stats based on rarity tier.
    ///
    /// Common: 1.0, Uncommon: 1.3, Rare: 1.6, Epic: 2.0, Legendary: 2.5.
    pub fn stat_multiplier(&self) -> f32 {
        match self {
            Self::Common => 1.0,
            Self::Uncommon => 1.3,
            Self::Rare => 1.6,
            Self::Epic => 2.0,
            Self::Legendary => 2.5,
        }
    }

    /// Human-readable label for UI tooltips.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::Epic => "Epic",
            Self::Legendary => "Legendary",
        }
    }
}
<<<<<<< HEAD

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rarity_count() {
        assert_eq!(ItemRarity::Common as u8, 0);
        assert_eq!(ItemRarity::Uncommon as u8, 1);
        assert_eq!(ItemRarity::Rare as u8, 2);
        assert_eq!(ItemRarity::Epic as u8, 3);
        assert_eq!(ItemRarity::Legendary as u8, 4);
    }

    #[test]
    fn test_rarity_color_not_black() {
        // All rarities should produce a non-zero color
        for rarity in &[ItemRarity::Common, ItemRarity::Uncommon, ItemRarity::Rare, ItemRarity::Epic, ItemRarity::Legendary] {
            let c = rarity.color().to_linear();
            let sum = c.red + c.green + c.blue;
            assert!(sum > 0.0, "color sum for {:?} should be > 0", rarity);
        }
    }

    #[test]
    fn test_rarity_hex_distinct() {
        let mut hexes = std::collections::HashSet::new();
        for rarity in &[ItemRarity::Common, ItemRarity::Uncommon, ItemRarity::Rare, ItemRarity::Epic, ItemRarity::Legendary] {
            assert!(hexes.insert(rarity.color_hex()), "duplicate hex for {:?}", rarity);
        }
    }

    #[test]
    fn test_stat_multiplier_increasing() {
        let rarities = [ItemRarity::Common, ItemRarity::Uncommon, ItemRarity::Rare, ItemRarity::Epic, ItemRarity::Legendary];
        for w in rarities.windows(2) {
            assert!(w[0].stat_multiplier() < w[1].stat_multiplier(),
                "{:?} multiplier ({}) should be < {:?} multiplier ({})",
                w[0], w[0].stat_multiplier(), w[1], w[1].stat_multiplier());
        }
    }

    #[test]
    fn test_stat_multiplier_specific_values() {
        assert!((ItemRarity::Common.stat_multiplier() - 1.0).abs() < f32::EPSILON);
        assert!((ItemRarity::Uncommon.stat_multiplier() - 1.3).abs() < f32::EPSILON);
        assert!((ItemRarity::Rare.stat_multiplier() - 1.6).abs() < f32::EPSILON);
        assert!((ItemRarity::Epic.stat_multiplier() - 2.0).abs() < f32::EPSILON);
        assert!((ItemRarity::Legendary.stat_multiplier() - 2.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_label_matches_enum_name() {
        assert_eq!(ItemRarity::Common.label(), "Common");
        assert_eq!(ItemRarity::Uncommon.label(), "Uncommon");
        assert_eq!(ItemRarity::Rare.label(), "Rare");
        assert_eq!(ItemRarity::Epic.label(), "Epic");
        assert_eq!(ItemRarity::Legendary.label(), "Legendary");
    }

    #[test]
    fn test_clone_and_copy() {
        let a = ItemRarity::Epic;
        let b = a; // Copy
        assert_eq!(a, b);
        let c = a.clone();
        assert_eq!(a, c);
    }
}
=======
>>>>>>> origin/master
