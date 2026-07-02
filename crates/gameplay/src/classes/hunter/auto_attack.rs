//! Hunter auto-attack — ranged auto-shot toward mouse cursor.
//! Triggers when an enemy is within 8 units, fires a projectile toward cursor.

use bevy::prelude::*;
use ir_core::*;
use crate::classes::auto_attack::*;

pub fn hunter_auto_attack(
    time: Res<Time>,
    cursor: Res<CursorWorldPos>,
    mut player_query: Query<(&Transform, &CombatStats, &mut AutoAttackCooldown), (With<Player>, With<PlayerClass>)>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut commands: Commands,
) {
    let Ok((player_tf, stats, mut cd)) = player_query.get_single_mut() else { return };
    cd.tick(time.delta_secs());
    if !cd.ready() { return; }
    let Some((_, _)) = nearest_enemy(player_tf.translation, 8.0, &enemies) else { return };
    cd.reset();
    let dir = direction_to(player_tf.translation, cursor.0);
    if dir.length_squared() < 0.1 { return; }
    let dmg = (5.0 + stats.damage_bonus * 0.7).max(1.0);
    commands.spawn(ProjectileBundle::new(
        dmg, 16.0, 3.0, dir,
        player_tf.translation + Vec3::Y * 0.5, ProjectileOwner::Player,
    ));
}
