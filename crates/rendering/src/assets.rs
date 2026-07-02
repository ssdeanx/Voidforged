//! Placeholder sprite asset generation + sprite texture loading.
//!
//! Creates flat quads with distinct shapes, colors, and emissive glows.
//! Player gets class-tinted materials; enemies get improved silhouettes.
//! Also loads sprite textures from `assets/textures/sprites/` for world entities.

use bevy::prelude::*;
use ir_core::*;
use crate::proc_meshes;
use std::collections::HashMap;

/// Holds handles to every sprite texture loaded from `assets/textures/sprites/`.
#[derive(Resource, Debug, Clone, Default)]
pub struct GameSpriteAssets {
    pub sprites: HashMap<&'static str, Handle<Image>>,
}

impl GameSpriteAssets {
    pub fn get(&self, key: &str) -> Option<Handle<Image>> {
        self.sprites.get(key).cloned()
    }
}

/// Startup system: loads all world sprite textures from `assets/textures/sprites/`.
pub fn load_game_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut sprites = HashMap::new();
    let entries: Vec<(&str, &str)> = vec![
        ("npc_quest", "textures/sprites/npc_quest.png"),
        ("npc_vendor", "textures/sprites/npc_vendor.png"),
        ("obj_altar", "textures/sprites/obj_altar.png"),
        ("obj_loot", "textures/sprites/obj_loot.png"),
        ("obj_chest", "textures/sprites/obj_chest.png"),
        ("enemy_grunt", "textures/sprites/enemy_grunt.png"),
        ("enemy_ranged", "textures/sprites/enemy_ranged.png"),
        ("enemy_elite", "textures/sprites/enemy_elite.png"),
    ];
    for (id, path) in entries {
        sprites.insert(id, asset_server.load(path));
    }
    commands.insert_resource(GameSpriteAssets { sprites });
}

/// Creates and injects procedural sprite meshes & materials into GameAssets.
pub fn generate_placeholder_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ── Player sprite: class-colored tall figure with emissive glow ───
    let player_mesh = meshes.add(Rectangle::new(0.8, 1.4)); // tall humanoid
    let player_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.5, 1.0), // default blue
        emissive: LinearRgba::rgb(0.05, 0.2, 0.6),
        perceptual_roughness: 0.3,
        metallic: 0.1,
        ..default()
    });

    // ── Class-specific player materials ──────────────────────────────
    let class_materials = vec![
        // Warrior — red with fiery glow
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.77, 0.12, 0.23),
            emissive: LinearRgba::rgb(0.4, 0.02, 0.02),
            perceptual_roughness: 0.3,
            metallic: 0.3,
            ..default()
        }),
        // Paladin — golden with holy glow
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.96, 0.75, 0.35),
            emissive: LinearRgba::rgb(0.3, 0.2, 0.0),
            perceptual_roughness: 0.2,
            metallic: 0.5,
            ..default()
        }),
        // Rogue — dark with yellow trim
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.25, 0.2),
            emissive: LinearRgba::rgb(0.2, 0.18, 0.0),
            perceptual_roughness: 0.4,
            metallic: 0.1,
            ..default()
        }),
        // Hunter — forest green
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.55, 0.2),
            emissive: LinearRgba::rgb(0.05, 0.25, 0.05),
            perceptual_roughness: 0.5,
            metallic: 0.1,
            ..default()
        }),
        // Mage — arcane blue/purple
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.4, 0.9),
            emissive: LinearRgba::rgb(0.1, 0.1, 0.45),
            perceptual_roughness: 0.2,
            metallic: 0.2,
            ..default()
        }),
    ];

    // ── Enemy sprites: distinct shapes + colors per variant ──────────
    let enemy_meshes = vec![
        meshes.add(Rectangle::new(0.7, 0.9)),   // Grunt — short, wide
        meshes.add(Rectangle::new(0.5, 1.2)),   // Ranged — tall, thin
        meshes.add(Rectangle::new(0.9, 0.8)),   // Charger — wide, low
        meshes.add(Rectangle::new(0.8, 1.1)),   // Elite — medium, imposing
        meshes.add(Rectangle::new(2.0, 2.0)),   // Boss — large square w/ crown shape
    ];
    let enemy_materials = vec![
        // Grunt — blood red
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.15, 0.15),
            emissive: LinearRgba::rgb(0.3, 0.02, 0.02),
            perceptual_roughness: 0.7,
            ..default()
        }),
        // Ranged — orange/amber
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.5, 0.1),
            emissive: LinearRgba::rgb(0.3, 0.1, 0.0),
            perceptual_roughness: 0.6,
            ..default()
        }),
        // Charger — bright yellow/green
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.9, 0.2),
            emissive: LinearRgba::rgb(0.3, 0.25, 0.0),
            perceptual_roughness: 0.5,
            ..default()
        }),
        // Elite — dark purple with glow
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.1, 0.8),
            emissive: LinearRgba::rgb(0.2, 0.02, 0.35),
            perceptual_roughness: 0.4,
            metallic: 0.3,
            ..default()
        }),
        // Boss — intense red with high emissive
        materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.15, 0.15),
            emissive: LinearRgba::rgb(0.7, 0.03, 0.03),
            perceptual_roughness: 0.3,
            metallic: 0.2,
            ..default()
        }),
    ];

    // ── Projectile (small circle) ────────────────────────────────────
    let projectile_mesh = meshes.add(Circle::new(0.15));
    let projectile_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.8, 1.0),
        emissive: LinearRgba::rgb(0.0, 0.3, 0.5),
        perceptual_roughness: 0.1,
        ..default()
    });

    // ── XP Gem ───────────────────────────────────────────────────────
    let gem_mesh = meshes.add(Circle::new(0.2));
    let gem_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 1.0, 0.2),
        emissive: LinearRgba::rgb(0.05, 0.5, 0.05),
        ..default()
    });

    // ── Health Pickup ────────────────────────────────────────────────
    let health_mesh = meshes.add(Rectangle::new(0.35, 0.35));
    let health_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 1.0, 0.15),
        emissive: LinearRgba::rgb(0.0, 0.4, 0.05),
        ..default()
    });

    // ── Gold Pickup ──────────────────────────────────────────────────
    let gold_mesh = meshes.add(Circle::new(0.2));
    let gold_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.85, 0.0),
        emissive: LinearRgba::rgb(0.35, 0.25, 0.0),
        metallic: 0.5,
        ..default()
    });

    // ── Shadow sprite ────────────────────────────────────────────────
    let shadow_mesh = meshes.add(Rectangle::new(0.6, 0.6));
    let shadow_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 0.0, 0.0, 0.3),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // ── Ground plane ─────────────────────────────────────────────────
    let floor_mesh = meshes.add(Plane3d::default().mesh().size(40.0, 40.0));
    let floor_material = materials.add(Color::srgb(0.25, 0.25, 0.28));

    // ── Ground tiles ─────────────────────────────────────────────────
    let tile_mesh = meshes.add(Cuboid::new(1.8, 0.05, 1.8));
    let tile_material = materials.add(Color::srgb(0.2, 0.2, 0.23));
    let tile_material_alt = materials.add(Color::srgb(0.18, 0.18, 0.21));

    // ── Procedural environment meshes ────────────────────────────────
    let bush_mesh = meshes.add(proc_meshes::make_bush(0.6));
    let tree_mesh = meshes.add(proc_meshes::make_tree(0.6, 0.4));
    let rock_mesh = meshes.add(proc_meshes::make_rock(0.4, 0));
    let grass_blade_mesh = meshes.add(proc_meshes::make_grass_blade(0.35));
    let flower_mesh = meshes.add(proc_meshes::make_flower(0.08));
    let cactus_mesh = meshes.add(proc_meshes::make_cactus(0.7));
    let mushroom_mesh = meshes.add(proc_meshes::make_mushroom(0.15));
    let crystal_mesh = meshes.add(proc_meshes::make_crystal(0.5));
    let pillar_mesh = meshes.add(proc_meshes::make_pillar(0.6, 0.15));

    commands.insert_resource(GameAssets {
        player_mesh,
        player_material,
        class_materials, // new: class-specific player materials
        enemy_meshes,
        enemy_materials,
        projectile_mesh,
        projectile_material,
        gem_mesh,
        gem_material,
        health_pickup_mesh: health_mesh,
        health_pickup_material: health_material,
        gold_pickup_mesh: gold_mesh,
        gold_pickup_material: gold_material,
        floor_mesh,
        floor_material,
        tile_mesh,
        tile_material,
        tile_material_alt,
        wall_mesh: meshes.add(Rectangle::new(32.0, 0.5)),
        wall_material: materials.add(Color::srgb(0.3, 0.15, 0.15)),
        shadow_mesh,
        shadow_material,
        // Procedural environment meshes
        bush_mesh,
        tree_mesh,
        rock_mesh,
        grass_blade_mesh,
        flower_mesh,
        cactus_mesh,
        mushroom_mesh,
        crystal_mesh,
        pillar_mesh,
    });
}
