//! Enemy AI — movement with formation awareness, aggro ranges, boss phases.
use bevy::prelude::*;
use ir_core::*;
use rand::Rng;

fn aggro_range(variant: &EnemyVariant) -> (f32, f32) {
    match variant {
        EnemyVariant::Grunt => (15.0, 0.0),
        EnemyVariant::Ranged => (22.0, 4.0),
        EnemyVariant::Charger => (25.0, 2.0),
        EnemyVariant::Elite => (18.0, 0.0),
        EnemyVariant::Boss => (30.0, 0.0),
        EnemyVariant::Caster => (24.0, 5.0),
        EnemyVariant::Healer => (20.0, 2.0),
        EnemyVariant::Summoner => (22.0, 4.0),
        EnemyVariant::Assassin => (20.0, 0.0),
        EnemyVariant::Brute => (16.0, 0.0),
    }
}

pub fn enemy_ai(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Enemy, &mut Velocity, &Transform, &CombatStats, &AttackCooldown, Option<&Stun>)>,
    other_enemies: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let player_pos = match player_query.get_single() { Ok(t) => t.translation, Err(_) => return };
    let mut rng = rand::thread_rng();
    let enemy_positions: Vec<Vec3> = other_enemies.iter().map(|t| t.translation).collect();
    for (enemy, mut velocity, transform, stats, cooldown, stun) in enemy_query.iter_mut() {
        if cooldown.windup > 0.0 || stun.is_some() { velocity.0 = Vec3::ZERO; continue; }
        let to_player = player_pos - transform.translation;
        let dist = to_player.length();
        let (max_aggro, flee_dist) = aggro_range(&enemy.variant);
        if dist > max_aggro { velocity.0 = Vec3::ZERO; continue; }
        let dir_to_player = if dist > 0.1 { (to_player / dist) * Vec3::new(1.0, 0.0, 1.0) } else { Vec3::ZERO };
        let behavior = match enemy.variant {
            EnemyVariant::Grunt => {
                let mut avoid = Vec3::ZERO;
                for other_pos in enemy_positions.iter() {
                    let to_other = transform.translation - *other_pos;
                    let other_dist = to_other.length();
                    if other_dist < 2.0 && other_dist > 0.1 { avoid += to_other / other_dist; }
                }
                (dir_to_player + avoid * 0.3).normalize_or_zero() * stats.move_speed
            }
            EnemyVariant::Ranged => {
                if dist < flee_dist { -dir_to_player * stats.move_speed * 1.2 }
                else if dist < 18.0 {
                    let strafe = Vec3::new(-to_player.z, 0.0, to_player.x).normalize_or_zero();
                    let s = if (transform.translation.x + transform.translation.z) as i32 % 2 == 0 { strafe } else { -strafe };
                    (dir_to_player * 0.2 + s * 0.8) * stats.move_speed
                } else { dir_to_player * stats.move_speed * 0.5 }
            }
            EnemyVariant::Charger => {
                if dist < 20.0 && dist > 3.0 {
                    let strafe = Vec3::new(-to_player.z, 0.0, to_player.x).normalize_or_zero();
                    if dist > 10.0 { dir_to_player * stats.move_speed } else { strafe * stats.move_speed * 0.7 }
                } else if dist <= 3.0 { -dir_to_player * stats.move_speed * 0.3 } else { Vec3::ZERO }
            }
            EnemyVariant::Elite => {
                if dist < 18.0 {
                    let wobble = Vec3::new((rng.gen::<f32>() - 0.5) * 2.0, 0.0, (rng.gen::<f32>() - 0.5) * 2.0);
                    (dir_to_player + wobble * 0.15).normalize_or_zero() * stats.move_speed * (0.8 + rng.gen::<f32>() * 0.4)
                } else { dir_to_player * stats.move_speed }
            }
            EnemyVariant::Boss => dir_to_player * stats.move_speed * 0.5,
            EnemyVariant::Caster => {
                if dist < flee_dist { -dir_to_player * stats.move_speed }
                else { dir_to_player * stats.move_speed * 0.4 }
            }
            EnemyVariant::Healer => {
                if dist < 15.0 { -dir_to_player * stats.move_speed * 0.3 }
                else { Vec3::ZERO }
            }
            EnemyVariant::Summoner => {
                if dist < flee_dist { -dir_to_player * stats.move_speed * 0.6 }
                else if dist > 12.0 { (dir_to_player + Vec3::new(rng.gen::<f32>() - 0.5, 0.0, rng.gen::<f32>() - 0.5) * 0.2).normalize_or_zero() * stats.move_speed * 0.5 }
                else { Vec3::ZERO }
            }
            EnemyVariant::Assassin => dir_to_player * stats.move_speed * 1.3,
            EnemyVariant::Brute => dir_to_player * stats.move_speed * 0.7,
        };
        velocity.0 = Vec3::new(behavior.x, 0.0, behavior.z);
    }
}

pub fn boss_phase_ai(
    _time: Res<Time>,
    mut boss_query: Query<(Entity, &Enemy, &mut Velocity, &Transform, &Health, &CombatStats, &mut AttackCooldown)>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let (player_entity, player_pos) = match player_query.get_single() { Ok(p) => (p.0, p.1.translation), Err(_) => return };
    for (boss_entity, enemy, mut velocity, transform, health, stats, mut cooldown) in boss_query.iter_mut() {
        if enemy.variant != EnemyVariant::Boss { continue; }
        let hp_pct = health.fraction();
        let to_player = player_pos - transform.translation;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero() * Vec3::new(1.0, 0.0, 1.0);
        match hp_pct {
            p if p > 0.6 => { velocity.0 = dir * stats.move_speed * 0.5; }
            p if p > 0.3 => {
                velocity.0 = dir * stats.move_speed * 0.8;
                if cooldown.windup > 0.3 { cooldown.windup = 0.3; }
            }
            _ => {
                velocity.0 = dir * stats.move_speed * 1.1;
                if dist < 6.0 && cooldown.timer <= 0.0 {
                    damage_events.send(DamageEvent {
                        target: player_entity, source: boss_entity, amount: 15.0 + stats.damage_bonus * 0.5,
                        is_critical: false, damage_type: DamageType::Physical, hit_position: Some(player_pos + Vec3::Y * 1.0),
                    });
                    cooldown.timer = 2.0;
                }
            }
        }
    }
}