//! Warrior auto-attack — melee weapon swing.
//! Triggers automatically when an enemy is within melee range (3.5 units).
//! Deals weapon-based physical damage scaled by `CombatStats.damage_bonus`.

use bevy::prelude::*;
use ir_core::*;
use crate::classes::auto_attack::*;

/// System: warrior melee auto-attack. Runs every frame.
/// Finds nearest enemy within 3.5 units and performs a weapon swing.
pub fn warrior_auto_attack(
    time: Res<Time>,
    mut player_query: Query<(Entity, &Transform, &CombatStats, &mut AutoAttackCooldown), (With<Player>, With<PlayerClass>)>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut commands: Commands,
) {
    let Ok((player_entity, player_tf, stats, mut cd)) = player_query.get_single_mut() else { return };
    cd.tick(time.delta_secs());
    if !cd.ready() { return; }

    let Some((_nearest_entity, _nearest_pos)) = nearest_enemy(
        player_tf.translation, 3.5, &enemies
    ) else { return };

    cd.reset();

    // Melee swing — cone hitbox in facing direction
    let dmg = (6.0 + stats.damage_bonus).max(1.0);
    commands.spawn((
        DamageHitbox::new(
            HitboxShape::Cone { range: 3.5, half_angle: 0.6 },
            dmg, player_entity, DamageType::Physical, 0.12,
            ProjectileOwner::Player, 1.0,
        ).with_hit_reaction(0.08, 0.15, 0.05),
        Transform {
            translation: player_tf.translation,
            rotation: player_tf.rotation,
            ..default()
        },
    ));
}
