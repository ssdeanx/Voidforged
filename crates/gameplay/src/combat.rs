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

/// Projectile overlap detection using proper distance checks.
/// Also applies Frozen/Stun effects from special projectiles.
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
        // Use proper overlap: projectile hit radius + enemy hitbox radius (~0.5 units)
        let hit_radius = if is_magic.is_some() { 1.2 } else { 0.8 };
        for (enemy_entity, enemy_transform, _stats) in enemies.iter() {
            let dist = proj_transform
                .translation
                .distance(enemy_transform.translation);
            if dist < hit_radius {
                let hit_pos = enemy_transform.translation + Vec3::Y * 1.0;
                // Determine damage type from projectile context
                let damage_type = if is_magic.is_some() {
                    DamageType::Magic
                } else {
                    DamageType::Physical
                };
                damage_events.send(DamageEvent {
                    target: enemy_entity,
                    source: proj_entity,
                    amount: projectile.damage,
                    is_critical: false,
                    damage_type: damage_type.clone(),
                });
                dmg_num_events.send(DamageNumberEvent {
                    position: hit_pos,
                    amount: projectile.damage as i32,
                    is_crit: false,
                    damage_type: damage_type.clone(),
                });
                // Impact color based on damage type
                let impact_color = match damage_type {
                    DamageType::Physical => Some(Vec4::new(1.0, 1.0, 1.0, 1.0)),
                    DamageType::Magic => Some(Vec4::new(0.8, 0.4, 1.0, 1.0)),
                    DamageType::True => Some(Vec4::new(1.0, 0.6, 0.0, 1.0)),
                };
                impact_events.send(SpawnImpactEvent {
                    position: hit_pos,
                    color: impact_color,
                });
                // Apply knockback to enemy
                let kb_dir = (enemy_transform.translation - proj_transform.translation)
                    .normalize_or_zero()
                    * Vec3::new(1.0, 0.0, 1.0);
                if kb_dir.length_squared() > 0.1 {
                    commands.entity(enemy_entity).insert(Knockback::new(
                        kb_dir * 8.0,
                        6.0,
                    ));
                }
                // Apply frozen effect from magic projectiles (frostbolt)
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
                damage_type: DamageType::Physical,
            });
            commands.entity(proj_entity).despawn();
        }
    }
}

// ============================================================================
// Damage Pipeline — applies armor, dodge, crit, lifesteal, shield block,
// stagger/hit-stun, knockback, and hit-stop
// ============================================================================

/// Damage reduction applied by shields and defensive buffs.
///
/// Multiplier applied after armor in the damage pipeline.
/// A value of `0.4` means 40% of incoming damage is negated.
#[derive(Component, Clone)]
pub struct DamageReduction {
    /// Damage reduction multiplier (0.0–1.0). 1.0 = full immunity.
    pub pct: f32,
}

/// Applies damage events to health — respects armor, dodge, crit, lifesteal, buffs.
/// Also applies hit-stun, knockback, and status effects.
pub fn apply_damage(
    time: Res<Time>,
    mut commands: Commands,
    mut shake: ResMut<ScreenShake>,
    mut damage_events: EventReader<DamageEvent>,
    mut death_events: EventWriter<DeathEvent>,
    player_query: Query<(Entity, &CombatStats), With<Player>>,
    enemy_query: Query<&CombatStats, (With<Enemy>, Without<Player>)>,
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

        // ── Screen shake on player hit (proportional to damage) ─
        if is_player_target {
            let trauma_add = (raw_dmg / 50.0).min(1.0);
            shake.trauma = (shake.trauma + trauma_add).min(1.0);
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

        // ── Hit-stun (stagger) on damage taken ──────────────────
        // Brief movement interrupt proportional to damage
        if !is_dead {
            let stun_duration = (raw_dmg / 50.0).min(0.3);
            if stun_duration > 0.05 {
                commands.entity(target).insert(HitStun::new(stun_duration));
            }
            // Small hit-stop for game feel
            commands.entity(target).insert(HitStop::new(stun_duration * 0.3));
            // Hit-flash for game feel (fixed 0.15s for projectile/non-hitbox damage)
            commands.entity(target).insert(HitFlash::new(0.15));
        }

        // ── Stun on heavy hits (damage > 30% of max HP estimate) ─
        if let Ok(health) = health_query.get(target) {
            if raw_dmg > health.max * 0.3 && !is_dead {
                commands.entity(target).insert(Stun::new(0.5));
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
// Death Events — spawns loot
// ============================================================================

/// Handles death events — spawns loot, despawns enemies, fires death particle event.
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
            // Fire death particle effect
            death_effect_events.send(SpawnDeathEffectEvent {
                position: pos,
                enemy_variant: enemy.variant.clone(),
            });
            ir_procedural::loot::spawn_loot(
                &mut commands,
                &assets,
                pos,
                &enemy.variant,
                enemy.tier,
            );
            commands.entity(event.entity).despawn();
        }
    }
}

// ============================================================================
// Hitbox Processing — with knockback, hit-stop, hit-flash
// ============================================================================

/// Processes all DamageHitbox entities — checks shape overlap with enemies,
/// sends DamageEvent on first hit, applies knockback, ticks lifetime, despawns.
pub fn process_hitboxes(
    mut commands: Commands,
    time: Res<Time>,
    mut hitboxes: Query<(Entity, &mut DamageHitbox, &Transform)>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
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

                // Knockback direction: from hitbox origin toward enemy, on XZ plane
                let kb_dir = (enemy_tf.translation - origin)
                    .normalize_or_zero()
                    * Vec3::new(1.0, 0.0, 1.0);
                if hitbox.knockback > 0.0 && kb_dir.length_squared() > 0.1 {
                    commands.entity(enemy_entity).insert(Knockback::new(
                        kb_dir * hitbox.knockback * 5.0,
                        6.0,
                    ));
                }

                // Hit-stun and hit-stop on the target
                if hitbox.hit_stun_duration > 0.0 {
                    commands.entity(enemy_entity).insert(HitStun::new(hitbox.hit_stun_duration));
                }
                if hitbox.hit_stop_duration > 0.0 {
                    commands.entity(enemy_entity).insert(HitStop::new(hitbox.hit_stop_duration));
                }
                // Hit-flash on the target
                if hitbox.hit_flash_duration > 0.0 {
                    commands.entity(enemy_entity).insert(HitFlash::new(hitbox.hit_flash_duration));
                }

                let damage_type = hitbox.damage_type.clone();
                let hit_pos = enemy_tf.translation + Vec3::Y * 1.0;

                damage_events.send(DamageEvent {
                    target: enemy_entity,
                    source: hitbox.source,
                    amount: hitbox.damage,
                    is_critical: false,
                    damage_type: damage_type.clone(),
                });

                dmg_num_events.send(DamageNumberEvent {
                    position: hit_pos,
                    amount: hitbox.damage as i32,
                    is_crit: false,
                    damage_type: damage_type.clone(),
                });

                // Colored impact effect
                let impact_color = match damage_type {
                    DamageType::Physical => Some(Vec4::new(1.0, 1.0, 1.0, 1.0)),
                    DamageType::Magic => Some(Vec4::new(0.8, 0.4, 1.0, 1.0)),
                    DamageType::True => Some(Vec4::new(1.0, 0.6, 0.0, 1.0)),
                };
                impact_events.send(SpawnImpactEvent {
                    position: hit_pos,
                    color: impact_color,
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
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
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

            // Knockback player
            let kb_dir = (player_tf.translation - origin)
                .normalize_or_zero()
                * Vec3::new(1.0, 0.0, 1.0);
            if hitbox.knockback > 0.0 && kb_dir.length_squared() > 0.1 {
                commands.entity(player_entity).insert(Knockback::new(
                    kb_dir * hitbox.knockback * 4.0,
                    5.0,
                ));
            }

            // Hit-stun on player
            if hitbox.hit_stun_duration > 0.0 {
                commands.entity(player_entity).insert(HitStun::new(hitbox.hit_stun_duration));
            }
            if hitbox.hit_stop_duration > 0.0 {
                commands.entity(player_entity).insert(HitStop::new(hitbox.hit_stop_duration));
            }

            damage_events.send(DamageEvent {
                target: player_entity,
                source: hitbox.source,
                amount: hitbox.damage,
                is_critical: false,
                damage_type: hitbox.damage_type.clone(),
            });

            dmg_num_events.send(DamageNumberEvent {
                position: player_tf.translation + Vec3::Y * 1.0,
                amount: hitbox.damage as i32,
                is_crit: false,
                damage_type: hitbox.damage_type.clone(),
            });
        }

        if hitbox.lifetime <= 0.0 {
            commands.entity(hitbox_entity).despawn();
        }
    }
}

// ============================================================================
// Knockback System
// ============================================================================

/// Applies knockback velocity and decays it with damping.
/// Writes into Velocity so knockback interacts with the friction/acceleration
/// model in player_movement, and with apply_enemy_velocity for enemies.
pub fn apply_knockback(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Velocity, &mut Knockback)>,
) {
    for (entity, mut velocity, mut knockback) in query.iter_mut() {
        // Add knockback to velocity
        velocity.0 += knockback.velocity * time.delta_secs();

        // Damping decay
        let damping_factor = (1.0 - knockback.damping * time.delta_secs()).max(0.0);
        knockback.velocity *= damping_factor;

        // Remove when negligible
        if knockback.velocity.length_squared() < 0.01 {
            commands.entity(entity).remove::<Knockback>();
        }
    }
}

// ============================================================================
// Status Effect Systems
// ============================================================================

/// Ticks Frozen duration and removes expired.
pub fn tick_frozen(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Frozen)>,
) {
    for (entity, mut frozen) in query.iter_mut() {
        frozen.remaining -= time.delta_secs();
        if frozen.remaining <= 0.0 {
            commands.entity(entity).remove::<Frozen>();
        }
    }
}

/// Ticks Stun duration and removes expired.
pub fn tick_stun(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Stun)>,
) {
    for (entity, mut stun) in query.iter_mut() {
        stun.remaining -= time.delta_secs();
        if stun.remaining <= 0.0 {
            commands.entity(entity).remove::<Stun>();
        }
    }
}

/// Ticks HitStun (stagger) duration and removes expired.
pub fn tick_hit_stun(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut HitStun)>,
) {
    for (entity, mut hit_stun) in query.iter_mut() {
        hit_stun.remaining -= time.delta_secs();
        if hit_stun.remaining <= 0.0 {
            commands.entity(entity).remove::<HitStun>();
        }
    }
}

/// Ticks HitStop duration. While active, entity velocity is suppressed.
pub fn tick_hit_stop(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut HitStop)>,
) {
    for (entity, mut hit_stop) in query.iter_mut() {
        hit_stop.remaining -= time.delta_secs();
        if hit_stop.remaining <= 0.0 {
            commands.entity(entity).remove::<HitStop>();
        }
    }
}

/// Prevents movement when stunned or frozen (applied as velocity suppression).
/// This system zeroes out movement velocity for stunned entities.
pub fn apply_stun_movement_block(
    mut query: Query<&mut Velocity, (With<Stun>, Without<Player>)>,
) {
    for mut velocity in query.iter_mut() {
        velocity.0 = Vec3::ZERO;
    }
}

/// Ticks HitFlash duration and removes expired.
pub fn tick_hit_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut HitFlash)>,
) {
    for (entity, mut flash) in query.iter_mut() {
        flash.remaining -= time.delta_secs();
        if flash.remaining <= 0.0 {
            commands.entity(entity).remove::<HitFlash>();
        }
    }
}

// ============================================================================
// Stamina System
// ============================================================================

<<<<<<< HEAD
/// Regenerates stamina over time. Regen pauses for `stamina_lockout_timer`
/// seconds after any stamina spend (wow-style lockout).
=======
/// Sprint toggle state.
///
/// `true` when the player is currently sprinting (consuming stamina).
#[derive(Component, Default)]
pub struct Sprinting(pub bool);

/// Tick once per second to give a periodic stamina burst.
>>>>>>> origin/master
pub fn stamina_regen(
    time: Res<Time>,
    mut query: Query<&mut Stamina, With<Player>>,
) {
    for mut stamina in query.iter_mut() {
        // Tick lockout timer down
        if stamina.stamina_lockout_timer > 0.0 {
            stamina.stamina_lockout_timer = (stamina.stamina_lockout_timer - time.delta_secs()).max(0.0);
        }
        // Only regen when lockout has expired
        if stamina.stamina_lockout_timer <= 0.0 {
            stamina.current = (stamina.current + stamina.regen_rate * time.delta_secs()).min(stamina.max);
        }
    }
}
