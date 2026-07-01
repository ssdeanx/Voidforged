//! Placeholder sprite asset generation.
//! Creates flat quads with colored materials for Hades-style 2D sprites in 3D.
//! Each sprite type gets distinct shape + color + emissive glow.

use bevy::prelude::*;
use ir_core::*;

/// Creates and injects procedural sprite meshes & materials into GameAssets.
pub fn generate_placeholder_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Each sprite is a flat quad for Hades-style 2D-in-3D look.

    // --- Player sprite: tall figure with emissive blue glow ---
    let player_mesh = meshes.add(Rectangle::new(0.8, 1.4));
    let player_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.5, 1.0),
        emissive: LinearRgba::rgb(0.05, 0.2, 0.6),
        perceptual_roughness: 0.3,
        metallic: 0.1,
        ..default()
    });

    // --- Wall mesh ---
    let wall_mesh = meshes.add(Rectangle::new(32.0, 0.5));
    let wall_material = materials.add(Color::srgb(0.3, 0.15, 0.15));

    // --- Enemy sprites: distinct shapes + colors per variant ---
    let enemy_meshes = vec![
        meshes.add(Rectangle::new(0.7, 0.9)),   // Grunt — short, wide
        meshes.add(Rectangle::new(0.5, 1.2)),   // Ranged — tall, thin
        meshes.add(Rectangle::new(0.9, 0.8)),   // Charger — wide, low
        meshes.add(Rectangle::new(0.8, 1.1)),   // Elite — medium, imposing
        meshes.add(Rectangle::new(1.8, 1.8)),   // Boss — large square
    ];
    let enemy_materials = vec![
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.15, 0.15),
            emissive: LinearRgba::rgb(0.3, 0.02, 0.02),
            perceptual_roughness: 0.7,
            ..default()
        }), // Grunt
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.5, 0.1),
            emissive: LinearRgba::rgb(0.3, 0.1, 0.0),
            perceptual_roughness: 0.6,
            ..default()
        }), // Ranged
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.9, 0.2),
            emissive: LinearRgba::rgb(0.3, 0.25, 0.0),
            perceptual_roughness: 0.5,
            ..default()
        }), // Charger
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.1, 0.8),
            emissive: LinearRgba::rgb(0.2, 0.02, 0.35),
            perceptual_roughness: 0.4,
            metallic: 0.3,
            ..default()
        }), // Elite
        materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.2, 0.2),
            emissive: LinearRgba::rgb(0.6, 0.05, 0.05),
            perceptual_roughness: 0.3,
            metallic: 0.2,
            ..default()
        }), // Boss
    ];

    // --- Projectile (small circle) ---
    let projectile_mesh = meshes.add(Circle::new(0.15));
    let projectile_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.8, 1.0),
        emissive: LinearRgba::rgb(0.0, 0.3, 0.5),
        perceptual_roughness: 0.1,
        ..default()
    });

    // --- XP Gem ---
    let gem_mesh = meshes.add(Circle::new(0.18));
    let gem_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 1.0, 0.2),
        emissive: LinearRgba::rgb(0.05, 0.4, 0.05),
        ..default()
    });

    // --- Health Pickup ---
    let health_mesh = meshes.add(Rectangle::new(0.35, 0.35));
    let health_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 1.0, 0.15),
        emissive: LinearRgba::rgb(0.0, 0.35, 0.05),
        ..default()
    });

    // --- Gold Pickup ---
    let gold_mesh = meshes.add(Circle::new(0.18));
    let gold_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.85, 0.0),
        emissive: LinearRgba::rgb(0.3, 0.2, 0.0),
        metallic: 0.5,
        ..default()
    });

    // --- Shadow sprite ---
    let shadow_mesh = meshes.add(Rectangle::new(0.6, 0.6));
    let shadow_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 0.0, 0.0, 0.3),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    // --- Ground plane ---
    let floor_mesh = meshes.add(Plane3d::default().mesh().size(40.0, 40.0));
    let floor_material = materials.add(Color::srgb(0.25, 0.25, 0.28));

    // --- Ground tiles ---
    let tile_mesh = meshes.add(Cuboid::new(1.8, 0.05, 1.8));
    let tile_material = materials.add(Color::srgb(0.2, 0.2, 0.23));
    let tile_material_alt = materials.add(Color::srgb(0.18, 0.18, 0.21));

    commands.insert_resource(GameAssets {
        player_mesh,
        player_material,
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
        wall_mesh,
        wall_material,
        shadow_mesh,
        shadow_material,
        environment_meshes: vec![],
    });
}
