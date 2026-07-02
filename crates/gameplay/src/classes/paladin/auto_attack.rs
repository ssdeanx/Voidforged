//! Paladin auto-attack — basic melee weapon swing, identical to warrior pattern.
//! Triggers within 3.5 units, deals physical weapon damage.

use bevy::prelude::*;
use ir_core::*;
use crate::classes::auto_attack::*;

pub fn paladin_auto_attack(
    time: Res<Time>,
    mut player_query: Query<(Entity, &Transform, &CombatStats, &mut AutoAttackCooldown), (With<Player>, With<PlayerClass>)>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut commands: Commands,
) {
    let Ok((player_entity, player_tf, stats, mut cd)) = player_query.get_single_mut() else { return };
    cd.tick(time.delta_secs());
    if !cd.ready() { return; }
    let Some((_, _)) = nearest_enemy(player_tf.translation, 3.5, &enemies) else { return };
    cd.reset();
    let dmg = (5.0 + stats.damage_bonus).max(1.0);
    commands.spawn((
        DamageHitbox::new(
            HitboxShape::Cone { range: 3.5, half_angle: 0.5 },
            dmg, player_entity, DamageType::Physical, 0.12,
            ProjectileOwner::Player, 1.0,
        ),
        Transform { translation: player_tf.translation, rotation: player_tf.rotation, ..default() },
    ));
}
