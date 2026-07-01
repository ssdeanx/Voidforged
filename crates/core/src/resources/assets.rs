//! Game asset handles resource.

use bevy::prelude::*;

/// Holds handles to loaded game assets.
#[derive(Resource, Debug, Clone)]
pub struct GameAssets {
    pub player_mesh: Handle<Mesh>,
    pub player_material: Handle<StandardMaterial>,
    pub enemy_meshes: Vec<Handle<Mesh>>,
    pub enemy_materials: Vec<Handle<StandardMaterial>>,
    pub projectile_mesh: Handle<Mesh>,
    pub projectile_material: Handle<StandardMaterial>,
    pub gem_mesh: Handle<Mesh>,
    pub gem_material: Handle<StandardMaterial>,
    pub health_pickup_mesh: Handle<Mesh>,
    pub health_pickup_material: Handle<StandardMaterial>,
    pub gold_pickup_mesh: Handle<Mesh>,
    pub gold_pickup_material: Handle<StandardMaterial>,
    pub floor_mesh: Handle<Mesh>,
    pub floor_material: Handle<StandardMaterial>,
    pub tile_mesh: Handle<Mesh>,
    pub tile_material: Handle<StandardMaterial>,
    pub tile_material_alt: Handle<StandardMaterial>,
    pub wall_mesh: Handle<Mesh>,
    pub wall_material: Handle<StandardMaterial>,
    pub shadow_mesh: Handle<Mesh>,
    pub shadow_material: Handle<StandardMaterial>,
    pub environment_meshes: Vec<Handle<Mesh>>,
}
