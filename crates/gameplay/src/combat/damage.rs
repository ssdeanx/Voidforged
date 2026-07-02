//! Damage pipeline — armor, crit, dodge, lifesteal, death events.
use bevy::prelude::*;
use ir_core::*;
use rand::Rng;

#[derive(Component, Clone)]
pub struct DamageReduction {
    pub pct: f32,
}

pub fn apply_damage(
    time: Res<Time>,
    mut commands: Commands,
    mut shake: ResMut<ScreenShake>,
    mut damage_events: EventReader<DamageEvent>,
    mut death_events: EventWriter<DeathEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
    mut hit_dir_events: EventWriter<HitDirectionEvent>,
    player_query: Query<(Entity, &CombatStats), With<Player>>,
    enemy_query: Query<&CombatStats, (With<Enemy>, Without<Player>)>,
    mut health_query: Query<&mut Health>,
    shield_query: Query<&DamageReduction>,
    projectile_query: Query<&Projectile>,
) {
    let player_data = player_query.get_single().ok();
    let (player_entity, player_stats) = player_data
        .map(|(e, s)| (Some(e), s.clone()))
        .unwrap_or((None, CombatStats::default()));
    let mut rng = rand::thread_rng();
    let mut dead_this_frame: Vec<Entity> = Vec::new();

    for event in damage_events.read() {
        let target = event.target;
        if dead_this_frame.contains(&target) { continue; }
        let mut amount = event.amount;
        let source = event.source;
        let is_player_target = Some(target) == player_entity;
        let hit_pos = event.hit_position;

        if is_player_target && rng.gen::<f32>() < player_stats.dodge_chance { continue; }

        let is_crit = if Some(source) == player_entity {
            rng.gen::<f32>() < player_stats.crit_chance
        } else { event.is_critical };
        if is_crit { amount *= player_stats.crit_multiplier; }

        let target_stats = if is_player_target { Some(&player_stats) }
            else { enemy_query.get(target).ok() };

        if let Some(stats) = target_stats {
            let effective_armor = (stats.armor - if is_player_target { 0.0 } else { player_stats.armor_penetration }).max(0.0);
            amount *= 1.0 - effective_armor / (effective_armor + 100.0);
        }

        if let Ok(red) = shield_query.get(target) { amount *= 1.0 - red.pct; }

        let raw_dmg = amount.max(1.0);
        let is_dead = health_query.get_mut(target).ok()
            .map(|mut health| { let died = health.take_damage(raw_dmg, time.elapsed_secs() as f32); died && !health.is_alive() })
            .unwrap_or(false);

        if Some(source) == player_entity && player_stats.lifesteal > 0.0 {
            if let Some(pe) = player_entity {
                if let Ok(mut ph) = health_query.get_mut(pe) { ph.heal(raw_dmg * player_stats.lifesteal); }
            }
        }

        if is_player_target {
            shake.trauma = (shake.trauma + if is_crit { 0.3 } else { (raw_dmg / 50.0).min(1.0) }).min(1.0);
        } else if is_crit { shake.trauma = (shake.trauma + 0.15).min(1.0); }

        if is_player_target {
            if let Some(pos) = hit_pos {
                let dir = pos.normalize_or_zero();
                if dir.length_squared() > 0.1 { hit_dir_events.send(HitDirectionEvent { direction: dir }); }
            }
        }

        let hit_stop_duration = if is_dead { 0.2 } else if is_crit { 0.15 } else { 0.08 };
        commands.entity(target).insert(HitStop::new(hit_stop_duration));
        if Some(source) == player_entity { commands.entity(source).insert(HitStop::new(hit_stop_duration)); }
        else if let Ok(proj) = projectile_query.get(source) {
            if proj.owner == ProjectileOwner::Player {
                if let Some(pe) = player_entity { commands.entity(pe).insert(HitStop::new(hit_stop_duration)); }
            }
        }

        if !is_dead {
            let stun_duration = (raw_dmg / 50.0).min(0.3);
            if stun_duration > 0.05 { commands.entity(target).insert(HitStun::new(stun_duration)); }
            commands.entity(target).insert(HitFlash::new(0.15));
        }

        if is_crit {
            if let Some(pos) = hit_pos {
                impact_events.send(SpawnImpactEvent { position: pos, color: Some(Vec4::new(1.0, 0.7, 0.0, 1.0)) });
                dmg_num_events.send(DamageNumberEvent {
                    position: pos, amount: raw_dmg as i32, is_crit: true,
                    damage_type: if is_player_target { DamageType::Physical } else { event.damage_type.clone() },
                });
            }
        }

        if let Ok(health) = health_query.get(target) {
            if raw_dmg > health.max * 0.3 && !is_dead { commands.entity(target).insert(Stun::new(0.5)); }
        }

        if is_dead {
            dead_this_frame.push(target);
            death_events.send(DeathEvent { entity: target, killer: Some(source), enemy_variant: None });
        }
    }
}

pub fn handle_death(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    mut death_effect_events: EventWriter<SpawnDeathEffectEvent>,
    enemy_query: Query<(&Enemy, &Transform)>,
    assets: Res<GameAssets>,
) {
    for event in death_events.read() {
        if let Ok((enemy, transform)) = enemy_query.get(event.entity) {
            let pos = transform.translation + Vec3::Y * 0.5;
            death_effect_events.send(SpawnDeathEffectEvent { position: pos, enemy_variant: enemy.variant.clone() });
            ir_procedural::loot::spawn_loot(&mut commands, &assets, pos, &enemy.variant, enemy.tier);
            let current_scale = transform.scale.x;
            commands.entity(event.entity).insert((
                DeathAnimation::new(0.5, current_scale),
                Stun::new(0.5),
            ));
        }
    }
}