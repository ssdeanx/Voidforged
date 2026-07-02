//! Loot drop system — spawns XP gems, health pickups, and gold based on
//! enemy variant and wave tier.

use bevy::prelude::*;
use ir_core::*;
use rand::Rng;

/// Spawns loot drops at a position based on enemy variant and wave tier.
/// Can drop XP gems, health pickups, and gold.
pub fn spawn_loot(
    commands: &mut Commands,
    assets: &GameAssets,
    position: Vec3,
    enemy_variant: &EnemyVariant,
    wave_tier: u32,
) {
    let mut rng = rand::thread_rng();
    let origin = position + Vec3::Y * 0.3;

    // Always drop XP gems
    let xp_base = match enemy_variant {
        EnemyVariant::Grunt => 10,
        EnemyVariant::Ranged => 15,
        EnemyVariant::Charger => 20,
        EnemyVariant::Elite => 50,
        EnemyVariant::Boss => 200,
        EnemyVariant::Caster => 18,
        EnemyVariant::Healer => 25,
        EnemyVariant::Summoner => 35,
        EnemyVariant::Assassin => 22,
        EnemyVariant::Brute => 40,
    };
    let xp_value = (xp_base as f64 * (1.0 + wave_tier as f64 * 0.15)) as u64;
    commands.spawn((
        ExperienceGemBundle::new(xp_value, origin),
        Mesh3d(assets.gem_mesh.clone()),
        MeshMaterial3d(assets.gem_material.clone()),
    ));

    // Health drop (25% chance from grunts, 40% from elites, 100% from bosses)
    let health_chance = match enemy_variant {
        EnemyVariant::Grunt => 0.25,
        EnemyVariant::Ranged => 0.20,
        EnemyVariant::Charger => 0.15,
        EnemyVariant::Elite => 0.40,
        EnemyVariant::Boss => 1.0,
        _ => 0.15,
    };
    if rng.gen::<f32>() < health_chance {
        let offset = Vec3::new(rng.gen::<f32>() - 0.5, 0.0, rng.gen::<f32>() - 0.5);
        commands.spawn((
            Pickup { kind: PickupKind::Health },
            Transform::from_translation(origin + offset * 0.5),
            Mesh3d(assets.health_pickup_mesh.clone()),
            MeshMaterial3d(assets.health_pickup_material.clone()),
            RoomEntity,
        ));
    }

    // Gold drop (10% from most, 30% from elites)
    let gold_chance = match enemy_variant {
        EnemyVariant::Elite => 0.30,
        EnemyVariant::Boss => 0.50,
        _ => 0.10,
    };
    if rng.gen::<f32>() < gold_chance {
        let offset = Vec3::new(rng.gen::<f32>() - 0.5, 0.0, rng.gen::<f32>() - 0.5);
        commands.spawn((
            Pickup { kind: PickupKind::Gold },
            Transform::from_translation(origin + offset * 0.5),
            Mesh3d(assets.gold_pickup_mesh.clone()),
            MeshMaterial3d(assets.gold_pickup_material.clone()),
            RoomEntity,
        ));
    }

    // Elites and bosses drop extra XP gems
    if matches!(enemy_variant, EnemyVariant::Elite | EnemyVariant::Boss) {
        for _ in 0..rng.gen_range(1..=3) {
            let offset = Vec3::new(
                rng.gen::<f32>() - 0.5,
                0.0,
                rng.gen::<f32>() - 0.5,
            );
            commands.spawn(ExperienceGemBundle::new(
                xp_value / 2,
                origin + offset,
            ));
        }
    }
}
