//! Procedural placeholder asset generation.
//! Creates colored meshes from Bevy primitives — no 3D model files needed.

use bevy::prelude::*;
use ir_core::*;

/// Creates and injects procedural placeholder meshes & materials into GameAssets.
pub fn generate_placeholder_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // --- Player ---
    let player_mesh = meshes.add(Cuboid::new(0.6, 1.0, 0.6));
    let player_material = materials.add(Color::srgb(0.2, 0.6, 1.0));

    // --- Enemies ---
    let grunt_mesh = meshes.add(Cuboid::new(0.5, 0.8, 0.5));
    let ranged_mesh = meshes.add(Cylinder::new(0.3, 0.8));
    let charger_mesh = meshes.add(Cone::new(0.4, 0.9));
    let elite_mesh = meshes.add(Cuboid::new(0.7, 1.1, 0.7));
    let boss_mesh = meshes.add(Sphere::new(1.0));

    let grunt_mat = materials.add(Color::srgb(0.8, 0.2, 0.2));
    let ranged_mat = materials.add(Color::srgb(0.9, 0.5, 0.1));
    let charger_mat = materials.add(Color::srgb(0.9, 0.9, 0.2));
    let elite_mat = materials.add(Color::srgb(0.6, 0.1, 0.8));
    let boss_mat = materials.add(Color::srgb(1.0, 0.3, 0.3));

    // --- Projectile ---
    let projectile_mesh = meshes.add(Sphere::new(0.15));
    let projectile_mat = materials.add(Color::srgb(0.0, 0.8, 1.0));

    // --- XP Gem ---
    let gem_mesh = meshes.add(Cuboid::new(0.2, 0.2, 0.2));
    let gem_mat = materials.add(Color::srgb(0.3, 1.0, 0.3));

    // --- Environment (floor tile) ---
    let floor_mesh = meshes.add(Plane3d::default().mesh().size(40.0, 40.0));
    let floor_mat = materials.add(Color::srgb(0.15, 0.15, 0.18));

    commands.insert_resource(GameAssets {
        player_mesh,
        player_material,
        enemy_meshes: vec![grunt_mesh, ranged_mesh, charger_mesh, elite_mesh, boss_mesh],
        enemy_materials: vec![grunt_mat, ranged_mat, charger_mat, elite_mat, boss_mat],
        projectile_mesh,
        projectile_material: projectile_mat,
        gem_mesh,
        gem_material: gem_mat,
        floor_mesh,
        floor_material: floor_mat,
        environment_meshes: vec![],
    });
}
