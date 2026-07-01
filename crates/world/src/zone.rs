//! Zone definitions for the open world.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A named zone in the open world.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ZoneId {
    /// The starting zone — green grassy plains with scattered trees.
    Grasslands,
    /// An arid sandy desert region to the south of the Grasslands.
    Desert,
    /// A dense woodland to the east with darker, thicker ground cover.
    Forest,
    /// A frozen tundra region (defined but not placed on the current world grid).
    Tundra,
    /// A murky swamp region (defined but not placed on the current world grid).
    Swamp,
    /// Between-zone transition / void state — returned when the player is
    /// outside all defined zones.
    Void,
}

impl ZoneId {
    /// Returns the human-readable display name for this zone.
    pub fn display_name(&self) -> &str {
        match self {
            ZoneId::Grasslands => "Grasslands",
            ZoneId::Desert => "Desert",
            ZoneId::Forest => "Forest",
            ZoneId::Tundra => "Tundra",
            ZoneId::Swamp => "Swamp",
            ZoneId::Void => "---",
        }
    }

    /// Ground color for the zone.
    pub fn ground_color(&self) -> Color {
        match self {
            ZoneId::Grasslands => Color::srgb(0.2, 0.5, 0.15),
            ZoneId::Desert => Color::srgb(0.7, 0.6, 0.3),
            ZoneId::Forest => Color::srgb(0.1, 0.35, 0.1),
            ZoneId::Tundra => Color::srgb(0.6, 0.6, 0.7),
            ZoneId::Swamp => Color::srgb(0.25, 0.3, 0.15),
            ZoneId::Void => Color::srgb(0.0, 0.0, 0.0),
        }
    }

    /// Alternate tile color for checkerboard.
    pub fn ground_color_alt(&self) -> Color {
        match self {
            ZoneId::Grasslands => Color::srgb(0.25, 0.55, 0.2),
            ZoneId::Desert => Color::srgb(0.65, 0.55, 0.25),
            ZoneId::Forest => Color::srgb(0.15, 0.4, 0.15),
            ZoneId::Tundra => Color::srgb(0.55, 0.55, 0.65),
            ZoneId::Swamp => Color::srgb(0.2, 0.25, 0.1),
            ZoneId::Void => Color::srgb(0.05, 0.05, 0.05),
        }
    }
}

/// Position in the world grid.
#[derive(Debug, Clone, Copy, Component, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorldPos(
    /// X coordinate in the world tile grid.
    pub i32,
    /// Z coordinate in the world tile grid.
    pub i32,
);

/// A dungeon entrance on the world map.
#[derive(Debug, Clone, Component)]
pub struct DungeonEntrance {
    /// Display name shown when the player approaches the entrance.
    pub name: String,
    /// Difficulty tier of the dungeon (higher = stronger enemies and better rewards).
    pub dungeon_tier: u32,
    /// Number of rooms / floors to clear before reaching the exit.
    pub depth: u32,
}

/// The full definition of a zone on the map.
#[derive(Debug, Clone)]
pub struct ZoneDef {
    /// Unique identifier for this zone.
    pub id: ZoneId,
    /// Display label shown in the HUD when the player is in this zone.
    pub label: &'static str,
    /// Width of the zone in tiles.
    pub tile_w: usize,
    /// Height of the zone in tiles.
    pub tile_h: usize,
    /// Grid offset in world coords (each tile is 2 units).
    pub offset_x: i32,
    /// Z-axis offset corresponding to `offset_x`, in world coords.
    pub offset_z: i32,
    /// Dungeon entrance locations and definitions within this zone.
    /// Each entry is `(tile_x, tile_z, dungeon_definition)`.
    pub dungeon_entrances: Vec<(i32, i32, DungeonEntrance)>,
}

/// All zone definitions — placed on a larger world grid.
pub fn all_zones() -> Vec<ZoneDef> {
    vec![
        ZoneDef {
            id: ZoneId::Grasslands,
            label: "The Greatholm",
            tile_w: 20,
            tile_h: 20,
            offset_x: 0,
            offset_z: 0,
            dungeon_entrances: vec![
                (10, 3, DungeonEntrance {
                    name: "Rat King's Warren".into(),
                    dungeon_tier: 1,
                    depth: 5,
                }),
                (18, 15, DungeonEntrance {
                    name: "Forgotten Catacombs".into(),
                    dungeon_tier: 2,
                    depth: 7,
                }),
            ],
        },
        ZoneDef {
            id: ZoneId::Desert,
            label: "Scorched Reach",
            tile_w: 20,
            tile_h: 20,
            offset_x: 0,
            offset_z: -25,
            dungeon_entrances: vec![
                (5, 5, DungeonEntrance {
                    name: "Tomb of the Sun King".into(),
                    dungeon_tier: 3,
                    depth: 9,
                }),
            ],
        },
        ZoneDef {
            id: ZoneId::Forest,
            label: "Wealdwood",
            tile_w: 20,
            tile_h: 20,
            offset_x: 25,
            offset_z: 0,
            dungeon_entrances: vec![
                (3, 10, DungeonEntrance {
                    name: "The Hollow Tree".into(),
                    dungeon_tier: 2,
                    depth: 6,
                }),
            ],
        },
    ]
}
