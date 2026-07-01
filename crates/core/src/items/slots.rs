//! Equipment slot and item category enums.

use serde::{Deserialize, Serialize};

/// High-level category of an item.
///
/// Determines which inventory tab and equip rules apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemCategory {
    /// Melee or ranged weapons.
    Weapon,
    /// Body armour, helmets, boots.
    Armor,
    /// Rings, amulets, trinkets.
    Accessory,
    /// Potions, scrolls, one-time-use items.
    Consumable,
    /// Crafting components and upgrade materials.
    Material,
    /// Story-related quest items (cannot be sold or dropped).
    Quest,
}

/// Which equipment slot an item occupies.
///
/// `None` is used for non-equippable items (consumables, materials, quest items).
/// Each slot can hold at most one item at a time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipSlot {
    /// Primary weapon hand.
    MainHand,
    /// Off-hand / shield / secondary weapon.
    OffHand,
    /// Head armour slot.
    Helmet,
    /// Chest / body armour slot.
    Chest,
    /// Footwear slot.
    Boots,
    /// Ring accessory slot (two can be worn).
    Ring,
    /// Necklace accessory slot.
    Amulet,
    /// Miscellaneous trinket slot.
    Trinket,
}

impl EquipSlot {
    /// Human-readable name for UI tooltips and panels.
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
