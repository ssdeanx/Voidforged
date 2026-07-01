//! Item rarity — quality tiers with color coding and stat multipliers.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl ItemRarity {
    /// Display color for UI elements.
    pub fn color(&self) -> Color {
        match self {
            Self::Common => Color::srgb(0.7, 0.7, 0.7),
            Self::Uncommon => Color::srgb(0.3, 0.8, 0.3),
            Self::Rare => Color::srgb(0.3, 0.5, 1.0),
            Self::Epic => Color::srgb(0.7, 0.3, 0.9),
            Self::Legendary => Color::srgb(1.0, 0.6, 0.0),
        }
    }

    /// Name color string for UI labels.
    pub fn color_hex(&self) -> &'static str {
        match self {
            Self::Common => "#B3B3B3",
            Self::Uncommon => "#4DCC4D",
            Self::Rare => "#4D7FFF",
            Self::Epic => "#B34DE5",
            Self::Legendary => "#FF9900",
        }
    }

    /// Multiplier applied to base stats based on quality.
    pub fn stat_multiplier(&self) -> f32 {
        match self {
            Self::Common => 1.0,
            Self::Uncommon => 1.3,
            Self::Rare => 1.6,
            Self::Epic => 2.0,
            Self::Legendary => 2.5,
        }
    }

    /// Display label for tooltips.
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
