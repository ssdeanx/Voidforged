//! Game asset handles resource — stores loaded mesh and material handles.

use bevy::prelude::*;

/// Holds handles to all loaded game assets.
///
/// Populated during the loading screen by [`CorePlugin`](crate::plugin::CorePlugin).
/// Every gameplay system that needs to spawn visual entities reads from this resource.
#[derive(Resource, Debug, Clone)]
pub struct GameAssets {
    /// Shared player character mesh.
    pub player_mesh: Handle<Mesh>,
    /// Default player material (used when no class-specific material applies).
    pub player_material: Handle<StandardMaterial>,
    /// Class-specific player materials in order: [Warrior, Paladin, Rogue, Hunter, Mage].
    pub class_materials: Vec<Handle<StandardMaterial>>,
    /// Meshes for each enemy variant type.
    pub enemy_meshes: Vec<Handle<Mesh>>,
    /// Materials for each enemy variant type.
    pub enemy_materials: Vec<Handle<StandardMaterial>>,
    /// Shared projectile mesh (sphere, arrow, bolt, etc.).
    pub projectile_mesh: Handle<Mesh>,
    /// Shared projectile material.
    pub projectile_material: Handle<StandardMaterial>,
    /// XP gem pickup mesh.
    pub gem_mesh: Handle<Mesh>,
    /// XP gem pickup material.
    pub gem_material: Handle<StandardMaterial>,
    /// Health pickup mesh.
    pub health_pickup_mesh: Handle<Mesh>,
    /// Health pickup material.
    pub health_pickup_material: Handle<StandardMaterial>,
    /// Gold pickup mesh.
    pub gold_pickup_mesh: Handle<Mesh>,
    /// Gold pickup material.
    pub gold_pickup_material: Handle<StandardMaterial>,
    /// Floor plane mesh.
    pub floor_mesh: Handle<Mesh>,
    /// Floor material.
    pub floor_material: Handle<StandardMaterial>,
    /// Dungeon tile mesh.
    pub tile_mesh: Handle<Mesh>,
    /// Primary tile material.
    pub tile_material: Handle<StandardMaterial>,
    /// Alternate tile material (variation for visual diversity).
    pub tile_material_alt: Handle<StandardMaterial>,
    /// Wall mesh for dungeon rooms.
    pub wall_mesh: Handle<Mesh>,
    /// Wall material.
    pub wall_material: Handle<StandardMaterial>,
    /// Shadow / decal mesh cast under entities.
    pub shadow_mesh: Handle<Mesh>,
    /// Shadow decal material.
    pub shadow_material: Handle<StandardMaterial>,
    /// Procedural bush mesh (sphere cluster).
    pub bush_mesh: Handle<Mesh>,
    /// Procedural tree mesh (cylinder trunk + cone canopy).
    pub tree_mesh: Handle<Mesh>,
    /// Procedural rock mesh (displaced sphere).
    pub rock_mesh: Handle<Mesh>,
    /// Procedural grass blade mesh (thin tall quad).
    pub grass_blade_mesh: Handle<Mesh>,
    /// Procedural flower mesh (sphere on stalk).
    pub flower_mesh: Handle<Mesh>,
    /// Procedural cactus mesh (cylinder with arms).
    pub cactus_mesh: Handle<Mesh>,
    /// Procedural mushroom mesh (cone on cylinder).
    pub mushroom_mesh: Handle<Mesh>,
    /// Procedural crystal mesh (elongated octahedron).
    pub crystal_mesh: Handle<Mesh>,
    /// Procedural pillar mesh (fluted cylinder).
    pub pillar_mesh: Handle<Mesh>,
}
