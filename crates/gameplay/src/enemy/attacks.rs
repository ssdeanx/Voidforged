//! Enemy melee and ranged attack systems with telegraphing.
use bevy::prelude::*;
use ir_core::*;

pub fn enemy_melee_attack(
    mut commands: Commands, time: Res<Time>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<(Entity, &Enemy, &Transform, &mut AttackCooldown, &CombatStats)>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let (player_entity, player_pos) = match player_query.get_single() { Ok(p) => (p.0, p.1.translation), Err(_) => return };
    for (enemy_entity, enemy, transform, mut cooldown, stats) in enemy_query.iter_mut() {
        if enemy.variant == EnemyVariant::Ranged || enemy.variant == EnemyVariant::Caster || enemy.variant == EnemyVariant::Healer { continue; }
        let dist = transform.translation.distance(player_pos);
        let melee_range = match enemy.variant {
            EnemyVariant::Boss => 3.5, EnemyVariant::Charger => 3.0, EnemyVariant::Brute => 3.5,
            EnemyVariant::Elite => 3.0, _ => 2.0,
        };
        if cooldown.windup > 0.0 {
            cooldown.windup -= time.delta_secs();
            if cooldown.windup <= 0.0 {
                let base_dmg = match enemy.variant {
                    EnemyVariant::Grunt => 8.0, EnemyVariant::Charger => 18.0,
                    EnemyVariant::Elite => 25.0, EnemyVariant::Boss => 45.0,
                    EnemyVariant::Brute => 30.0, EnemyVariant::Assassin => 20.0,
                    _ => 10.0,
                };
                let dmg = base_dmg + stats.damage_bonus;
                let hit_pos = player_pos + Vec3::Y * 1.0;
                damage_events.send(DamageEvent { target: player_entity, source: enemy_entity, amount: dmg, is_critical: false, damage_type: DamageType::Physical, hit_position: Some(hit_pos) });
                if enemy.variant == EnemyVariant::Charger {
                    let kb_dir = (player_pos - transform.translation).normalize_or_zero() * Vec3::new(1.0, 0.0, 1.0);
                    if kb_dir.length_squared() > 0.1 { commands.entity(player_entity).insert(Knockback::new(kb_dir * 15.0, 4.0)); }
                    commands.entity(player_entity).insert(Stun::new(0.5));
                }
                let interval = match enemy.variant {
                    EnemyVariant::Boss => 1.5, EnemyVariant::Charger => 2.5,
                    EnemyVariant::Elite => 1.2, EnemyVariant::Brute => 1.8,
                    _ => 1.0,
                };
                cooldown.timer = interval; cooldown.windup = 0.0;
            }
            continue;
        }
        cooldown.timer = (cooldown.timer - time.delta_secs()).max(0.0);
        if dist < melee_range && cooldown.timer <= 0.0 {
            cooldown.windup = match enemy.variant {
                EnemyVariant::Charger => 0.6, EnemyVariant::Boss => 0.8,
                EnemyVariant::Elite => 0.4, EnemyVariant::Brute => 0.7,
                _ => 0.3,
            };
            commands.spawn((TelegraphIndicator::new(cooldown.windup, enemy_entity), Transform::from_translation(transform.translation + Vec3::Y * 0.1)));
        }
    }
}

pub fn enemy_ranged_attack(
    mut commands: Commands, time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(Entity, &Enemy, &Transform, &mut AttackCooldown)>,
) {
    let player_pos = match player_query.get_single() { Ok(t) => t.translation, Err(_) => return };
    for (enemy_entity, enemy, transform, mut cooldown) in enemy_query.iter_mut() {
        if enemy.variant != EnemyVariant::Ranged && enemy.variant != EnemyVariant::Caster { continue; }
        let dist = transform.translation.distance(player_pos);
        let flee_dist = 4.0;
        if cooldown.windup > 0.0 {
            cooldown.windup -= time.delta_secs();
            if cooldown.windup <= 0.0 {
                let direction = (player_pos - transform.translation).normalize_or_zero();
                commands.spawn(ProjectileBundle::new(
                    8.0, 10.0, 3.0, direction, transform.translation + Vec3::Y * 0.5, ProjectileOwner::Enemy,
                )).insert(EnemyProjectileMarker);
                cooldown.timer = 2.5; cooldown.windup = 0.0;
            }
            continue;
        }
        cooldown.timer = (cooldown.timer - time.delta_secs()).max(0.0);
        if dist < 22.0 && dist > flee_dist && cooldown.timer <= 0.0 {
            cooldown.windup = 0.5;
            commands.spawn((TelegraphIndicator::new(cooldown.windup, enemy_entity), Transform::from_translation(transform.translation + Vec3::Y * 0.1)));
        }
    }
}