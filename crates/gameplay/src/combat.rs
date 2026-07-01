//! Combat — projectile movement, hitbox processing, damage pipeline, and loot spawning.
//! Systems run in order: move_projectiles → projectile_hit → projectile_hit_player
//! → apply_damage → handle_death → process_hitboxes

use bevy::prelude::*;
use ir_core::*;
use rand::Rng;

// ── Projectile movement ────────────────────────────────────────────────

/// Moves projectiles and checks lifetime.
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

/// Player projectiles hit enemies.
pub fn projectile_hit(
    mut commands: Commands,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
    projectiles: Query<(Entity, &Projectile, &Transform)>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (proj_entity, projectile, proj_transform) in projectiles.iter() {
        if projectile.owner != ProjectileOwner::Player {
            continue;
        }
        for (enemy_entity, enemy_transform) in enemies.iter() {
            let dist = proj_transform
                .translation
                .distance(enemy_transform.translation);
            if dist < 1.0 {
                let hit_pos = enemy_transform.translation + Vec3::Y * 1.0;
                damage_events.send(DamageEvent {
                    target: enemy_entity,
                    source: proj_entity,
                    amount: projectile.damage,
                    is_critical: false,
                    damage_type: DamageType::Physical,
                });
                dmg_num_events.send(DamageNumberEvent {
                    position: hit_pos,
                    amount: projectile.damage as i32,
                    is_crit: false,
                });
                impact_events.send(SpawnImpactEvent {
                    position: hit_pos,
                    color: None,
                });
                if !projectile.piercing {
                    commands.entity(proj_entity).despawn();
                }
                break;
            }
        }
    }
}

/// Enemy projectiles hit the player.
pub fn projectile_hit_player(
    mut commands: Commands,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    projectiles: Query<(Entity, &Projectile, &Transform)>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    let Ok((player_entity, player_transform)) = player_query.get_single() else {
        return;
    };

    for (proj_entity, projectile, proj_transform) in projectiles.iter() {
        if projectile.owner != ProjectileOwner::Enemy {
            continue;
        }
        let dist = proj_transform
            .translation
            .distance(player_transform.translation);
        if dist < 0.8 {
            damage_events.send(DamageEvent {
                target: player_entity,
                source: proj_entity,
                amount: projectile.damage,
                is_critical: false,
                damage_type: DamageType::Physical,
            });
            dmg_num_events.send(DamageNumberEvent {
                position: player_transform.translation + Vec3::Y * 1.0,
                amount: projectile.damage as i32,
                is_crit: false,
            });
            commands.entity(proj_entity).despawn();
        }
    }
}

// ============================================================================
// Damage Pipeline — applies armor, dodge, crit, lifesteal, shield block
// ============================================================================

#[derive(Component, Clone)]
pub struct DamageReduction {
    pub pct: f32, // 0.0–1.0 damage reduction multiplier
}

/// Applies damage events to health — respects armor, dodge, crit, lifesteal, and buffs.
pub fn apply_damage(
    time: Res<Time>,
    mut commands: Commands,
    mut shake: ResMut<ScreenShake>,
    mut damage_events: EventReader<DamageEvent>,
    mut death_events: EventWriter<DeathEvent>,
    mut player_query: Query<(Entity, &CombatStats), With<Player>>,
    mut enemy_query: Query<&CombatStats, (With<Enemy>, Without<Player>)>,
    mut health_query: Query<&mut Health>,
    shield_query: Query<&DamageReduction>,
) {
    let player_data = player_query.get_single().ok();
    let (player_entity, player_stats) = player_data
        .map(|(e, s)| (Some(e), s.clone()))
        .unwrap_or((None, CombatStats::default()));

    let mut rng = rand::thread_rng();
    let mut dead_this_frame: Vec<Entity> = Vec::new();

    for event in damage_events.read() {
        let target = event.target;
        if dead_this_frame.contains(&target) {
            continue;
        }
        let mut amount = event.amount;
        let source = event.source;
        let is_player_target = Some(target) == player_entity;

        // ── Dodge check (player only for now) ───────────────────
        if is_player_target && rng.gen::<f32>() < player_stats.dodge_chance {
            continue; // attack missed
        }

        // ── Crit roll for player damage ─────────────────────────
        let is_crit = if Some(source) == player_entity {
            rng.gen::<f32>() < player_stats.crit_chance
        } else {
            event.is_critical
        };
        if is_crit {
            amount *= player_stats.crit_multiplier;
        }

        // ── Armor mitigation ────────────────────────────────────
        let target_stats = if is_player_target {
            Some(&player_stats)
        } else {
            enemy_query.get(target).ok()
        };

        if let Some(stats) = target_stats {
            let armor = stats.armor;
            let penetration = if is_player_target {
                0.0 // enemies don't have armor pen
            } else {
                player_stats.armor_penetration
            };
            let effective_armor = (armor - penetration).max(0.0);
            let reduction = effective_armor / (effective_armor + 100.0);
            amount *= 1.0 - reduction;
        }

        // ── Damage reduction buffs (ShieldBlock, etc.) ──────────
        if let Ok(red) = shield_query.get(target) {
            amount *= 1.0 - red.pct;
        }

        let raw_dmg = amount.max(1.0);

        // ── Screen shake on player hit ──────────────────────────
        if is_player_target {
            shake.trauma = (shake.trauma + raw_dmg / 100.0).min(1.0);
        }

        // ── Apply damage ────────────────────────────────────────
        let is_dead = health_query
            .get_mut(target)
            .ok()
            .map(|mut health| {
                let died = health.take_damage(raw_dmg, time.elapsed_secs() as f32);
                died && !health.is_alive()
            })
            .unwrap_or(false);

        // ── Lifesteal for player ────────────────────────────────
        if Some(source) == player_entity && player_stats.lifesteal > 0.0 {
            if let Some(player_entity) = player_entity {
                if let Ok(mut player_health) = health_query.get_mut(player_entity) {
                    player_health.heal(raw_dmg * player_stats.lifesteal);
                }
            }
        }

        if is_dead {
            dead_this_frame.push(target);
            death_events.send(DeathEvent {
                entity: target,
                killer: Some(source),
                enemy_variant: None,
            });
        }
    }
}

// ============================================================================
// Death Events — spawns old-style loot (legacy, kept for XP/gems)
// ============================================================================

/// Handles death events — spawns loot, despawns enemies.
/// Note: new item drops are handled by loot::spawn_loot_from_table.
pub fn handle_death(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    enemy_query: Query<(&Enemy, &Transform)>,
    assets: Res<GameAssets>,
) {
    for event in death_events.read() {
        if let Ok((enemy, transform)) = enemy_query.get(event.entity) {
            let pos = transform.translation + Vec3::Y * 0.5;
            ir_procedural::loot::spawn_loot(
                &mut commands,
                &assets,
                pos,
                &enemy.variant,
                enemy.tier,
            );
        }
    }
}

// ============================================================================
// Hitbox Processing
// ============================================================================

/// Processes all DamageHitbox entities — checks shape overlap with enemies,
/// sends DamageEvent on first hit, ticks lifetime, despawns expired.
pub fn process_hitboxes(
    mut commands: Commands,
    time: Res<Time>,
    mut hitboxes: Query<(Entity, &mut DamageHitbox, &Transform)>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for (hitbox_entity, mut hitbox, tf) in hitboxes.iter_mut() {
        hitbox.lifetime -= time.delta_secs();

        let origin = tf.translation;
        let facing = *tf.forward();

        for (enemy_entity, enemy_tf) in enemies.iter() {
            if hitbox.hit_enemies.contains(&enemy_entity) {
                continue;
            }

            let to_enemy = enemy_tf.translation - origin;
            let dist = to_enemy.length();
            let dir = to_enemy.normalize_or_zero();

            let hit = match hitbox.shape {
                HitboxShape::Cone { range, half_angle } => {
                    dist <= range && facing.dot(dir) >= half_angle.cos()
                }
                HitboxShape::Circle { radius } => dist <= radius,
                HitboxShape::Rect { width, length } => {
                    let dot = facing.dot(dir);
                    let lateral = (dir - facing * dot).length();
                    dist <= length && lateral * dist <= width * 0.5
                }
                HitboxShape::Point { range } => dist <= range,
            };

            if hit {
                hitbox.hit_enemies.push(enemy_entity);
                damage_events.send(DamageEvent {
                    target: enemy_entity,
                    source: hitbox.source,
                    amount: hitbox.damage,
                    is_critical: false,
                    damage_type: hitbox.damage_type.clone(),
                });
            }
        }

        if hitbox.lifetime <= 0.0 {
            commands.entity(hitbox_entity).despawn();
        }
    }
}

/// Processes EnemyHitbox entities — check overlap with player for damage.
pub fn process_enemy_hitboxes(
    mut commands: Commands,
    time: Res<Time>,
    mut hitboxes: Query<(Entity, &mut DamageHitbox, &Transform), With<EnemyHitbox>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let Ok((player_entity, player_tf)) = player_query.get_single() else {
        return;
    };

    for (hitbox_entity, mut hitbox, tf) in hitboxes.iter_mut() {
        hitbox.lifetime -= time.delta_secs();

        let origin = tf.translation;
        let to_player = player_tf.translation - origin;
        let dist = to_player.length();

        let hit = match hitbox.shape {
            HitboxShape::Circle { radius } => dist <= radius,
            HitboxShape::Point { range } => dist <= range,
            _ => dist <= 2.0,
        };

        if hit && !hitbox.hit_enemies.contains(&player_entity) {
            hitbox.hit_enemies.push(player_entity);
            damage_events.send(DamageEvent {
                target: player_entity,
                source: hitbox.source,
                amount: hitbox.damage,
                is_critical: false,
                damage_type: hitbox.damage_type.clone(),
            });
        }

        if hitbox.lifetime <= 0.0 {
            commands.entity(hitbox_entity).despawn();
        }
    }
}

// ============================================================================
// Stamina System
// ============================================================================

/// Sprint toggle state.
#[derive(Component, Default)]
pub struct Sprinting(pub bool);

/// Tick once per second to give a periodic stamina burst.
pub fn stamina_regen(
    time: Res<Time>,
    mut query: Query<&mut Stamina, With<Player>>,
) {
    for mut stamina in query.iter_mut() {
        stamina.current = (stamina.current + stamina.regen_rate * time.delta_secs()).min(stamina.max);
    }
}

/// Sprint costs stamina every frame while active.
pub fn sprint_stamina_drain(
    time: Res<Time>,
    input: Res<PlayerInput>,
    mut query: Query<(&mut Stamina, &mut Sprinting), With<Player>>,
) {
    let Ok((mut stamina, mut sprinting)) = query.get_single_mut() else { return };
    let wants_sprint = input.direction.length_squared() > 0.1
        && (input.dodge); // reuse dodge keybind for sprint too

    if wants_sprint && stamina.current > 0.0 {
        sprinting.0 = true;
        stamina.current = (stamina.current - 20.0 * time.delta_secs()).max(0.0);
        if stamina.current <= 0.0 {
            sprinting.0 = false;
        }
    } else {
        sprinting.0 = false;
    }
}
