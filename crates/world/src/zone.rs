//! Zone definitions for the open world.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A named zone in the open world.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ZoneId {
    Grasslands,
    Desert,
    Forest,
    Tundra,
    Swamp,
    Void, // between-zone transition
}

impl ZoneId {
    /// Short display name (enum variant).
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

    /// Fancy zone label as shown on loading screens.
    pub fn zone_label(&self) -> &str {
        match self {
            ZoneId::Grasslands => "The Greatholm",
            ZoneId::Desert => "Scorched Reach",
            ZoneId::Forest => "Wealdwood",
            ZoneId::Tundra => "Frostguard Wastes",
            ZoneId::Swamp => "Mire of Sorrows",
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

    /// Returns (min_tier, max_tier) for enemies in this zone.
    pub fn enemy_tier_range(&self) -> (u32, u32) {
        match self {
            ZoneId::Grasslands => (0, 1),
            ZoneId::Desert => (2, 3),
            ZoneId::Forest => (1, 2),
            ZoneId::Tundra => (3, 4),
            ZoneId::Swamp => (2, 3),
            ZoneId::Void => (0, 0),
        }
    }

    /// Returns enemy composition as (variant, weight) pairs.
    /// Higher weight = more likely to spawn.
    pub fn enemy_composition(&self) -> Vec<(ir_core::EnemyVariant, u32)> {
        use ir_core::EnemyVariant::*;
        match self {
            ZoneId::Grasslands => vec![(Grunt, 10)],
            ZoneId::Desert => vec![(Ranged, 4), (Charger, 6)],
            ZoneId::Forest => vec![(Grunt, 7), (Elite, 3)],
            ZoneId::Tundra => vec![(Grunt, 2), (Ranged, 3), (Elite, 5)],
            ZoneId::Swamp => vec![(Charger, 5), (Ranged, 5)],
            ZoneId::Void => vec![],
        }
    }

    /// Returns zone-appropriate enemy material colors per variant.
    /// Indexed by variant enum order: Grunt, Ranged, Charger, Elite, Boss.
    pub fn enemy_colors(&self) -> [Color; 5] {
        match self {
            ZoneId::Grasslands => [
                Color::srgb(0.6, 0.15, 0.15), // Grunt — blood red
                Color::srgb(0.7, 0.3, 0.15),  // Ranged — rust
                Color::srgb(0.65, 0.1, 0.1),  // Charger — dark red
                Color::srgb(0.5, 0.05, 0.05), // Elite — very dark red
                Color::srgb(1.0, 0.15, 0.15), // Boss — intense red
            ],
            ZoneId::Desert => [
                Color::srgb(0.6, 0.4, 0.2),   // Grunt — sandy brown
                Color::srgb(0.9, 0.5, 0.1),   // Ranged — amber
                Color::srgb(0.8, 0.3, 0.05),  // Charger — burnt orange
                Color::srgb(0.4, 0.2, 0.1),   // Elite — dark leather
                Color::srgb(1.0, 0.5, 0.0),   // Boss — blazing orange
            ],
            ZoneId::Forest => [
                Color::srgb(0.2, 0.5, 0.15),  // Grunt — forest green
                Color::srgb(0.1, 0.4, 0.2),   // Ranged — deep green
                Color::srgb(0.15, 0.45, 0.1), // Charger — moss
                Color::srgb(0.05, 0.3, 0.1),  // Elite — dark forest
                Color::srgb(0.0, 0.6, 0.15),  // Boss — emerald
            ],
            ZoneId::Tundra => [
                Color::srgb(0.5, 0.5, 0.6),   // Grunt — frost grey
                Color::srgb(0.3, 0.5, 0.7),   // Ranged — ice blue
                Color::srgb(0.4, 0.4, 0.55),  // Charger — storm grey
                Color::srgb(0.2, 0.3, 0.6),   // Elite — deep frost
                Color::srgb(0.6, 0.7, 1.0),   // Boss — glacial blue
            ],
            ZoneId::Swamp => [
                Color::srgb(0.3, 0.35, 0.2),  // Grunt — murky green
                Color::srgb(0.4, 0.6, 0.2),   // Ranged — toxic green
                Color::srgb(0.25, 0.3, 0.15), // Charger — bog brown
                Color::srgb(0.5, 0.2, 0.3),   // Elite — dark purple-black
                Color::srgb(0.6, 0.8, 0.1),   // Boss — sickly neon
            ],
            ZoneId::Void => [
                Color::srgb(0.0, 0.0, 0.0),
                Color::srgb(0.0, 0.0, 0.0),
                Color::srgb(0.0, 0.0, 0.0),
                Color::srgb(0.0, 0.0, 0.0),
                Color::srgb(0.0, 0.0, 0.0),
            ],
        }
    }

    /// Returns an HP multiplier based on zone difficulty.
    pub fn enemy_hp_multiplier(&self) -> f32 {
        match self {
            ZoneId::Grasslands => 1.0,
            ZoneId::Desert => 1.3,
            ZoneId::Forest => 1.15,
            ZoneId::Tundra => 1.5,
            ZoneId::Swamp => 1.3,
            ZoneId::Void => 1.0,
        }
    }

    /// Returns zone-specific ground patch colors for ambient variation.
    pub fn ground_patch_colors(&self) -> Vec<Color> {
        match self {
            ZoneId::Grasslands => vec![
                Color::srgb(0.15, 0.55, 0.12),
                Color::srgb(0.25, 0.45, 0.18),
                Color::srgb(0.12, 0.6, 0.08),
                Color::srgb(0.3, 0.5, 0.1),
            ],
            ZoneId::Desert => vec![
                Color::srgb(0.75, 0.55, 0.2),
                Color::srgb(0.6, 0.5, 0.35),
                Color::srgb(0.65, 0.45, 0.25),
                Color::srgb(0.5, 0.4, 0.3),
            ],
            ZoneId::Forest => vec![
                Color::srgb(0.08, 0.3, 0.08),
                Color::srgb(0.15, 0.25, 0.05),
                Color::srgb(0.05, 0.35, 0.12),
                Color::srgb(0.12, 0.28, 0.06),
            ],
            ZoneId::Tundra => vec![
                Color::srgb(0.5, 0.5, 0.6),
                Color::srgb(0.65, 0.6, 0.75),
                Color::srgb(0.55, 0.55, 0.65),
                Color::srgb(0.7, 0.65, 0.8),
            ],
            ZoneId::Swamp => vec![
                Color::srgb(0.2, 0.25, 0.08),
                Color::srgb(0.3, 0.2, 0.1),
                Color::srgb(0.15, 0.28, 0.05),
                Color::srgb(0.25, 0.22, 0.12),
            ],
            ZoneId::Void => vec![],
        }
    }

    /// Decoration definitions for this zone: (mesh_shape, color, name).
    /// Used for zone-specific world objects.
    pub fn decor_definitions(&self) -> Vec<DecorDef> {
        let mut defs = Vec::new();
        match self {
            ZoneId::Grasslands => {
                defs.push(DecorDef::new("rock", Cuboid::new(0.3, 0.15, 0.3), Color::srgb(0.35, 0.3, 0.25)));
                defs.push(DecorDef::new("bush", Cuboid::new(0.5, 0.4, 0.5), Color::srgb(0.14, 0.35, 0.1)));
                defs.push(DecorDef::new("grass_clump", Cuboid::new(0.15, 0.2, 0.15), Color::srgb(0.1, 0.25, 0.07)));
                defs.push(DecorDef::new("flower_red", Cuboid::new(0.1, 0.08, 0.1), Color::srgb(0.9, 0.1, 0.1)));
                defs.push(DecorDef::new("flower_yellow", Cuboid::new(0.1, 0.08, 0.1), Color::srgb(0.9, 0.8, 0.1)));
                defs.push(DecorDef::new("wheat", Cuboid::new(0.08, 0.35, 0.08), Color::srgb(0.6, 0.5, 0.15)));
                defs.push(DecorDef::new("stump", Cuboid::new(0.25, 0.1, 0.25), Color::srgb(0.3, 0.2, 0.12)));
                defs.push(DecorDef::new("tall_grass", Cuboid::new(0.04, 0.3, 0.04), Color::srgb(0.12, 0.35, 0.08)));
            }
            ZoneId::Desert => {
                defs.push(DecorDef::new("cactus", Cuboid::new(0.2, 0.5, 0.2), Color::srgb(0.2, 0.45, 0.15)));
                defs.push(DecorDef::new("bone", Cuboid::new(0.08, 0.3, 0.08), Color::srgb(0.8, 0.75, 0.65)));
                defs.push(DecorDef::new("rock_desert", Cuboid::new(0.35, 0.15, 0.3), Color::srgb(0.5, 0.4, 0.25)));
                defs.push(DecorDef::new("dried_grass", Cuboid::new(0.1, 0.15, 0.1), Color::srgb(0.5, 0.4, 0.15)));
                defs.push(DecorDef::new("skull", Cuboid::new(0.15, 0.12, 0.15), Color::srgb(0.75, 0.7, 0.6)));
                defs.push(DecorDef::new("pillar", Cuboid::new(0.15, 0.5, 0.15), Color::srgb(0.55, 0.45, 0.3)));
                defs.push(DecorDef::new("stone_ruin", Cuboid::new(0.3, 0.08, 0.2), Color::srgb(0.5, 0.45, 0.35)));
                defs.push(DecorDef::new("agave", Cuboid::new(0.25, 0.2, 0.25), Color::srgb(0.25, 0.4, 0.15)));
            }
            ZoneId::Forest => {
                defs.push(DecorDef::new("mushroom_red", Cuboid::new(0.12, 0.15, 0.12), Color::srgb(0.8, 0.15, 0.15)));
                defs.push(DecorDef::new("mushroom_brown", Cuboid::new(0.1, 0.1, 0.1), Color::srgb(0.5, 0.3, 0.15)));
                defs.push(DecorDef::new("log", Cuboid::new(0.4, 0.12, 0.15), Color::srgb(0.3, 0.18, 0.08)));
                defs.push(DecorDef::new("fern", Cuboid::new(0.2, 0.25, 0.2), Color::srgb(0.1, 0.4, 0.08)));
                defs.push(DecorDef::new("mossy_rock", Cuboid::new(0.3, 0.12, 0.3), Color::srgb(0.2, 0.35, 0.15)));
                defs.push(DecorDef::new("fallen_branch", Cuboid::new(0.3, 0.04, 0.04), Color::srgb(0.25, 0.15, 0.08)));
                defs.push(DecorDef::new("thick_bush", Cuboid::new(0.6, 0.4, 0.6), Color::srgb(0.08, 0.3, 0.06)));
                defs.push(DecorDef::new("forest_flower", Cuboid::new(0.08, 0.06, 0.08), Color::srgb(0.5, 0.2, 0.7)));
            }
            ZoneId::Tundra => {
                defs.push(DecorDef::new("ice_crystal", Cuboid::new(0.08, 0.4, 0.08), Color::srgb(0.6, 0.7, 1.0)));
                defs.push(DecorDef::new("snow_mound", Cuboid::new(0.35, 0.15, 0.35), Color::srgb(0.85, 0.85, 0.9)));
                defs.push(DecorDef::new("frost_rock", Cuboid::new(0.3, 0.12, 0.3), Color::srgb(0.55, 0.55, 0.65)));
                defs.push(DecorDef::new("frozen_tree", Cuboid::new(0.15, 0.5, 0.15), Color::srgb(0.3, 0.25, 0.25)));
                defs.push(DecorDef::new("snow_drift", Cuboid::new(0.4, 0.05, 0.3), Color::srgb(0.9, 0.9, 0.92)));
                defs.push(DecorDef::new("icicle", Cuboid::new(0.04, 0.3, 0.04), Color::srgb(0.7, 0.8, 1.0)));
                defs.push(DecorDef::new("frost_pillar", Cuboid::new(0.15, 0.45, 0.15), Color::srgb(0.5, 0.6, 0.8)));
                defs.push(DecorDef::new("permafrost_chunk", Cuboid::new(0.25, 0.08, 0.2), Color::srgb(0.4, 0.4, 0.5)));
            }
            ZoneId::Swamp => {
                defs.push(DecorDef::new("dead_tree", Cuboid::new(0.12, 0.55, 0.12), Color::srgb(0.2, 0.15, 0.08)));
                defs.push(DecorDef::new("swamp_mushroom", Cuboid::new(0.15, 0.12, 0.15), Color::srgb(0.6, 0.2, 0.4)));
                defs.push(DecorDef::new("reed", Cuboid::new(0.04, 0.35, 0.04), Color::srgb(0.35, 0.35, 0.12)));
                defs.push(DecorDef::new("rotten_log", Cuboid::new(0.35, 0.1, 0.12), Color::srgb(0.15, 0.12, 0.05)));
                defs.push(DecorDef::new("lily_pad", Cuboid::new(0.2, 0.02, 0.2), Color::srgb(0.15, 0.4, 0.12)));
                defs.push(DecorDef::new("willow_wisp", Cuboid::new(0.06, 0.06, 0.06), Color::srgb(0.5, 0.9, 0.3)));
                defs.push(DecorDef::new("swamp_rock", Cuboid::new(0.3, 0.1, 0.25), Color::srgb(0.2, 0.22, 0.15)));
                defs.push(DecorDef::new("bog_flower", Cuboid::new(0.08, 0.1, 0.08), Color::srgb(0.7, 0.3, 0.5)));
            }
            ZoneId::Void => {}
        }
        defs
    }
}

/// Definition for a decoration type generated during world creation.
#[derive(Debug, Clone)]
pub struct DecorDef {
    pub name: &'static str,
    pub shape: Cuboid,
    pub color: Color,
}

impl DecorDef {
    pub fn new(name: &'static str, shape: impl Into<Cuboid>, color: Color) -> Self {
        Self { name, shape: shape.into(), color }
    }
}

/// Position in the world grid.
#[derive(Debug, Clone, Copy, Component, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorldPos(pub i32, pub i32);

/// A dungeon entrance on the world map.
#[derive(Debug, Clone, Component)]
pub struct DungeonEntrance {
    pub name: String,
    pub dungeon_tier: u32,
    pub depth: u32, // number of rooms
}

/// The full definition of a zone on the map.
#[derive(Debug, Clone)]
pub struct ZoneDef {
    pub id: ZoneId,
    pub label: &'static str,
    pub tile_w: usize,
    pub tile_h: usize,
    /// Grid offset in world coords (each tile is 2 units)
    pub offset_x: i32,
    pub offset_z: i32,
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
        ZoneDef {
            id: ZoneId::Tundra,
            label: "Frostguard Wastes",
            tile_w: 20,
            tile_h: 20,
            offset_x: 0,
            offset_z: 25,
            dungeon_entrances: vec![
                (10, 10, DungeonEntrance {
                    name: "Crystal Depths".into(),
                    dungeon_tier: 4,
                    depth: 10,
                }),
            ],
        },
        ZoneDef {
            id: ZoneId::Swamp,
            label: "Mire of Sorrows",
            tile_w: 20,
            tile_h: 20,
            offset_x: -25,
            offset_z: 0,
            dungeon_entrances: vec![
                (5, 15, DungeonEntrance {
                    name: "Sunken Catacombs".into(),
                    dungeon_tier: 3,
                    depth: 8,
                }),
            ],
        },
    ]
}
