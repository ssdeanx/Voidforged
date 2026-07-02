//! Projectile movement, player-hit detection, enemy-hit detection.
use bevy::prelude::*;
use ir_core::*;

pub fn move_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile, &mut Transform, &Velocity)>,
) {
    for (entity, mut projectile, mut transform, velocity) in query.iter_mut() {
        projectile.lifetime -= time.delta_secs();
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        transform.translation += velocity.0 * time.delta_secs();
    }
}

pub fn projectile_hit(
    mut commands: Commands,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
    projectiles: Query<(Entity, &Projectile, &Transform, Option<&MagicProjectile>)>,
    enemies: Query<(Entity, &Transform, &CombatStats), With<Enemy>>,
) {
    for (proj_entity, projectile, proj_transform, is_magic) in projectiles.iter() {
        if projectile.owner != ProjectileOwner::Player {
            continue;
        }
        let hit_radius = if is_magic.is_some() { 1.2 } else { 0.8 };
        for (enemy_entity, enemy_transform, _stats) in enemies.iter() {
            let dist = proj_transform.translation.distance(enemy_transform.translation);
            if dist < hit_radius {
                let hit_pos = enemy_transform.translation + Vec3::Y * 1.0;
                let damage_type = if is_magic.is_some() { DamageType::Magic } else { DamageType::Physical };
                damage_events.send(DamageEvent {
                    target: enemy_entity,
                    source: proj_entity,
                    amount: projectile.damage,
                    is_critical: false,
                    damage_type: damage_type.clone(),
                    hit_position: Some(hit_pos),
                });
                dmg_num_events.send(DamageNumberEvent {
                    position: hit_pos,
                    amount: projectile.damage as i32,
                    is_crit: false,
                    damage_type: damage_type.clone(),
                });
                let impact_color = match damage_type {
                    DamageType::Physical => Some(Vec4::new(1.0, 1.0, 1.0, 1.0)),
                    DamageType::Magic => Some(Vec4::new(0.8, 0.4, 1.0, 1.0)),
                    DamageType::True => Some(Vec4::new(1.0, 0.6, 0.0, 1.0)),
                };
                impact_events.send(SpawnImpactEvent {
                    position: hit_pos,
                    color: impact_color,
                });
                let kb_dir = (enemy_transform.translation - proj_transform.translation)
                    .normalize_or_zero() * Vec3::new(1.0, 0.0, 1.0);
                if kb_dir.length_squared() > 0.1 {
                    commands.entity(enemy_entity).insert(Knockback::new(kb_dir * 8.0, 6.0));
                }
                if is_magic.is_some() {
                    commands.entity(enemy_entity).insert(Frozen::new(2.0));
                }
                if !projectile.piercing {
                    commands.entity(proj_entity).despawn();
                }
                break;
            }
        }
    }
}

pub fn projectile_hit_player(
    mut commands: Commands,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    projectiles: Query<(Entity, &Projectile, &Transform)>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    let Ok((player_entity, player_transform)) = player_query.get_single() else { return };
    for (proj_entity, projectile, proj_transform) in projectiles.iter() {
        if projectile.owner != ProjectileOwner::Enemy { continue; }
        let dist = proj_transform.translation.distance(player_transform.translation);
        if dist < 0.8 {
            let hit_pos = player_transform.translation + Vec3::Y * 1.0;
            damage_events.send(DamageEvent {
                target: player_entity, source: proj_entity,
                amount: projectile.damage, is_critical: false,
                damage_type: DamageType::Physical, hit_position: Some(hit_pos),
            });
            dmg_num_events.send(DamageNumberEvent {
                position: hit_pos, amount: projectile.damage as i32,
                is_crit: false, damage_type: DamageType::Physical,
            });
            commands.entity(proj_entity).despawn();
        }
    }
}