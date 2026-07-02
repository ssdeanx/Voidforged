//! Rogue auto-attack — quick melee stab, short range (2.5 units).

use bevy::prelude::*;
use ir_core::*;
use crate::classes::auto_attack::*;

pub fn rogue_auto_attack(
    time: Res<Time>,
    mut player_query: Query<(Entity, &Transform, &CombatStats, &mut AutoAttackCooldown), (With<Player>, With<PlayerClass>)>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut commands: Commands,
) {
    let Ok((player_entity, player_tf, stats, mut cd)) = player_query.get_single_mut() else { return };
    cd.tick(time.delta_secs());
    if !cd.ready() { return; }
    let Some((_, _)) = nearest_enemy(player_tf.translation, 2.5, &enemies) else { return };
    cd.reset();
    let dmg = (4.0 + stats.damage_bonus * 0.8).max(1.0);
    commands.spawn((
        DamageHitbox::new(
            HitboxShape::Cone { range: 2.5, half_angle: 0.3 },
            dmg, player_entity, DamageType::Physical, 0.08,
            ProjectileOwner::Player, 0.3,
        ),
        Transform { translation: player_tf.translation, rotation: player_tf.rotation, ..default() },
    ));
}
