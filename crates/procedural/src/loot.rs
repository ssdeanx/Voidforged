use bevy::prelude::*;
use ir_core::*;
use rand::Rng;

// Placeholder loot tables — will be expanded with proper item definitions.
pub fn spawn_loot(
    commands: &mut Commands,
    position: Vec3,
    enemy_variant: &EnemyVariant,
    wave_tier: u32,
) {
    let mut rng = rand::thread_rng();
    let xp_base = match enemy_variant {
        EnemyVariant::Grunt => 10,
        EnemyVariant::Ranged => 15,
        EnemyVariant::Charger => 20,
        EnemyVariant::Elite => 50,
        EnemyVariant::Boss => 200,
    };

    let xp_value = (xp_base as f64 * (1.0 + wave_tier as f64 * 0.15)) as u64;
    commands.spawn(ExperienceGemBundle::new(xp_value, position));

    // Elites and bosses drop extra loot
    if matches!(enemy_variant, EnemyVariant::Elite | EnemyVariant::Boss) {
        for _ in 0..rng.gen_range(1..=3) {
            let offset = Vec3::new(
                rng.gen::<f32>() - 0.5,
                0.0,
                rng.gen::<f32>() - 0.5,
            );
            commands.spawn(ExperienceGemBundle::new(
                xp_value / 2,
                position + offset,
            ));
        }
    }
}
