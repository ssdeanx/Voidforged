//! Equipment slot and item category enums.

use serde::{Deserialize, Serialize};

/// High-level category of an item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemCategory {
    Weapon,
    Armor,
    Accessory,
    Consumable,
    Material,
    Quest,
}

/// Which equipment slot an item occupies. None for non-equippable items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipSlot {
    MainHand,
    OffHand,
    Helmet,
    Chest,
    Boots,
    Ring,
    Amulet,
    Trinket,
}

impl EquipSlot {
    /// Human-readable name for UI.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::MainHand => "Main Hand",
            Self::OffHand => "Off Hand",
            Self::Helmet => "Helmet",
            Self::Chest => "Chest",
            Self::Boots => "Boots",
            Self::Ring => "Ring",
            Self::Amulet => "Amulet",
            Self::Trinket => "Trinket",
        }
    }
}
