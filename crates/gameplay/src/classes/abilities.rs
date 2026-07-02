//! Central ability dispatching + shared ability infrastructure.
//!
//! Each ability slot (primary/secondary/cast/dash) has one dispatch system
//! that reads the player's class and calls the appropriate module function.
//! Cooldowns live on the `AbilityCooldowns` component. Resource costs come
//! from `CharacterClass::resource_costs()`.

use bevy::prelude::*;
use ir_core::*;
use crate::classes::{hunter, mage, paladin, rogue, warrior};

/// Passive regeneration of the per-class resource (Rage, Holy Power, etc.).
///
/// Adds `regen_rate * delta_secs` to the resource, capped at `max`.
pub fn class_resource_regen(
    time: Res<Time>,
    mut query: Query<&mut ClassResource, With<Player>>,
) {
    for mut resource in query.iter_mut() {
        resource.current = (resource.current + resource.regen_rate * time.delta_secs()).min(resource.max);
    }
}

// ============================================================================
// Helper: base cooldown for each slot (before attack_speed scaling)
// ============================================================================

fn slot_base_cooldown(slot: &str) -> f32 {
    match slot {
        "primary"   => 0.0,    // primary fires every frame while held (cooldown per player_attack)
        "secondary" => 1.0,    // 1.0 s base
        "cast"      => 3.0,    // 3.0 s base
        _           => 0.0,
    }
}

/// Scale a cooldown by the player's attack speed bonus.
/// Higher attack_speed_bonus ⇒ shorter cooldown.
fn scaled_cooldown(base: f32, stats: &CombatStats) -> f32 {
    if base <= 0.0 { return 0.0; }
    let speed_mult = 1.0 + stats.attack_speed_bonus;
    if speed_mult <= 0.0 { return base; }
    base / speed_mult
}

// ── Primary Attack ───────────────────────────────────────────────────────

/// Dispatches the player's primary attack based on their class.
///
/// Checks resource costs and routing to the appropriate class module
/// (melee cleave for Warrior, righteous strike for Paladin, etc.).
pub fn primary_attack(
    mut commands: Commands,
    input: Res<PlayerInput>,
    cursor: Res<CursorWorldPos>,
    mut player_query: Query<(
        Entity, &Transform, &PlayerClass, &CombatStats, &mut ClassResource,
        &mut AbilityCooldowns,
    ), With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
) {
    if !input.primary_attack { return; }
    let Ok((entity, transform, class, stats, mut resource, mut cooldowns)) = player_query.get_single_mut() else { return; };

    // Primary attack cooldown is scaled by attack speed
    let base_cd = slot_base_cooldown("primary");
    let effective_cd = if base_cd > 0.0 { scaled_cooldown(base_cd, stats) } else { 0.0 };
    if cooldowns.primary > 0.0 { return; }
    cooldowns.primary = effective_cd;

    let cost = class.0.resource_costs().0;
    if cost > 0.0 && !resource.can_afford(cost) { return; }
    if cost > 0.0 { resource.spend_resource(cost); }

    match class.0 {
        CharacterClass::Warrior => warrior::primary_melee_cleave(
            &mut commands, entity, transform, stats, &enemies,
            &mut damage_events, &mut dmg_num_events, &mut impact_events,
        ),
        CharacterClass::Paladin => paladin::primary_righteous_strike(
            &mut commands, entity, transform, stats, &enemies,
            &mut damage_events, &mut dmg_num_events, &mut impact_events,
        ),
        CharacterClass::Rogue => rogue::primary_backstab(
            &mut commands, entity, transform, stats, &enemies,
            &mut damage_events, &mut dmg_num_events, &mut impact_events,
        ),
        CharacterClass::Hunter => hunter::primary_aimed_shot(
            &mut commands, transform, stats, &cursor,
            &mut damage_events, &mut dmg_num_events, &mut impact_events,
        ),
        CharacterClass::Mage => mage::primary_fireball(
            &mut commands, transform, stats, &cursor,
            &mut damage_events, &mut dmg_num_events, &mut impact_events,
        ),
    }
}

// ── Secondary Attack ─────────────────────────────────────────────────────

/// Dispatches the player's secondary attack based on their class.
///
/// Has an independent 1-second cooldown. Routes to shield block,
/// holy light, poison blade, multi-shot, or frostbolt per class.
pub fn secondary_attack(
    mut commands: Commands,
    input: Res<PlayerInput>,
    cursor: Res<CursorWorldPos>,
    mut player_query: Query<(
        Entity, &Transform, &PlayerClass, &CombatStats, &mut ClassResource,
        &mut AbilityCooldowns,
    ), With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
) {
    let Ok((entity, transform, class, stats, mut resource, mut cooldowns)) = player_query.get_single_mut() else { return; };
    if !input.secondary_attack { return; }

    let base_cd = slot_base_cooldown("secondary");
    let effective_cd = scaled_cooldown(base_cd, stats);
    if cooldowns.secondary > 0.0 { return; }
    cooldowns.secondary = effective_cd;

    let cost = class.0.resource_costs().1;
    if cost > 0.0 && !resource.can_afford(cost) { return; }
    if cost > 0.0 { resource.spend_resource(cost); }

    match class.0 {
        CharacterClass::Warrior => warrior::secondary_shield_block(&mut commands, entity),
        CharacterClass::Paladin => paladin::secondary_holy_light(&mut commands, entity),
        CharacterClass::Rogue => rogue::secondary_poison_blade(
            &mut commands, entity, transform, stats, &enemies,
            &mut damage_events, &mut dmg_num_events, &mut impact_events,
        ),
        CharacterClass::Hunter => hunter::secondary_multi_shot(&mut commands, transform, stats, &cursor),
        CharacterClass::Mage => mage::secondary_frostbolt(&mut commands, transform, stats, &cursor),
    }
}

// ── Cast Ability ─────────────────────────────────────────────────────────

/// Dispatches the player's cast (Q key) ability based on their class.
///
/// Has an independent 3-second cooldown. Routes to charge, consecration,
/// vanish, trap, or arcane blast per class.
pub fn cast_ability(
    mut commands: Commands,
    input: Res<PlayerInput>,
    cursor: Res<CursorWorldPos>,
    mut player_query: Query<(
        Entity, &Transform, &PlayerClass, &CombatStats, &mut ClassResource,
        &mut AbilityCooldowns,
    ), With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
) {
    let Ok((entity, transform, class, stats, mut resource, mut cooldowns)) = player_query.get_single_mut() else { return; };
    if !input.cast { return; }

    let base_cd = slot_base_cooldown("cast");
    let effective_cd = scaled_cooldown(base_cd, stats);
    if cooldowns.cast > 0.0 { return; }
    cooldowns.cast = effective_cd;

    let cost = class.0.resource_costs().2;
    if cost > 0.0 && !resource.can_afford(cost) { return; }
    if cost > 0.0 { resource.spend_resource(cost); }

    match class.0 {
        CharacterClass::Warrior => warrior::cast_charge(&mut commands, transform, stats, &cursor),
        CharacterClass::Paladin => paladin::cast_consecration(&mut commands, transform, stats),
        CharacterClass::Rogue => rogue::cast_vanish(&mut commands, entity),
        CharacterClass::Hunter => hunter::cast_trap(&mut commands, transform),
        CharacterClass::Mage => mage::cast_arcane_blast(
            &mut commands, transform, stats, &cursor, &enemies,
            &mut damage_events, &mut dmg_num_events, &mut impact_events,
        ),
    }
}

// ── Dash Ability ─────────────────────────────────────────────────────────

/// Helper: class color as Vec4 for particle bursts.
fn class_color_v4(class: CharacterClass) -> Vec4 {
    match class {
        CharacterClass::Warrior => Vec4::new(0.78, 0.61, 0.43, 1.0),
        CharacterClass::Paladin => Vec4::new(0.96, 0.55, 0.73, 1.0),
        CharacterClass::Rogue => Vec4::new(1.00, 0.96, 0.41, 1.0),
        CharacterClass::Hunter => Vec4::new(0.67, 0.83, 0.45, 1.0),
        CharacterClass::Mage => Vec4::new(0.41, 0.80, 0.94, 1.0),
    }
}

/// Spawns a semi-transparent copy of the player mesh as an afterimage trail.
fn spawn_dash_afterimage(
    commands: &mut Commands,
    assets: &GameAssets,
    materials: &mut Assets<StandardMaterial>,
    class: &PlayerClass,
    transform: &Transform,
) {
    let class_idx = match class.0 {
        CharacterClass::Warrior => 0,
        CharacterClass::Paladin => 1,
        CharacterClass::Rogue => 2,
        CharacterClass::Hunter => 3,
        CharacterClass::Mage => 4,
    };
    let player_mat = assets
        .class_materials
        .get(class_idx)
        .cloned()
        .unwrap_or_else(|| assets.player_material.clone());

    if let Some(base_mat) = materials.get(&player_mat) {
        let mut trail_mat = base_mat.clone();
        let srgb = trail_mat.base_color.to_srgba();
        trail_mat.base_color = Color::srgba(srgb.red, srgb.green, srgb.blue, 0.3);
        trail_mat.alpha_mode = AlphaMode::Blend;

        commands.spawn((
            Mesh3d(assets.player_mesh.clone()),
            MeshMaterial3d(materials.add(trail_mat)),
            Transform::from_translation(transform.translation),
            GlobalTransform::default(),
            Lifetime { remaining: 0.3 },
            TrailSegment,
        ));
    }
}

/// Handles the player's dash (Shift key) — grants brief invulnerability.
///
/// Dodges AWAY from the mouse cursor for combat evasion. Spawns afterimage
/// trail segments every 50ms during the dash, fires a particle burst and
/// screen shake on activation, and fires another burst on end.
/// Cooldown is tracked via [`AbilityCooldowns::dash`].
pub fn dash_ability(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<PlayerInput>,
    cursor: Res<CursorWorldPos>,
    assets: Res<GameAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut shake: ResMut<ScreenShake>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
    mut player_query: Query<(
        Entity,
        &Transform,
        &PlayerClass,
        &CombatStats,
        &mut DashCooldown,
        &mut Health,
        &mut Velocity,
        &mut Stamina,
        &mut AbilityCooldowns,
        &mut DashTrailTimer,
    ), With<Player>>,
) {
    let Ok((_player_entity, transform, class, stats, mut dash, mut health,
             mut velocity, mut stamina, mut cooldowns, mut trail_timer)) =
        player_query.get_single_mut()
    else {
        return;
    };

    // ── Active dash: tick duration, spawn afterimages, check end ──
    if dash.active {
        dash.duration -= time.delta_secs();

        // Invulnerability window (first 0.2 s of dash)
        if dash.duration > (0.25 - 0.2) {
            health.invulnerable_until = time.elapsed_secs() as f32 + 0.2;
        }

        // Afterimage trail: spawn transparent mesh copy every 50 ms
        trail_timer.0 -= time.delta_secs();
        if trail_timer.0 <= 0.0 {
            trail_timer.0 = 0.05;
            spawn_dash_afterimage(&mut commands, &assets, &mut materials, class, transform);
        }

        // Dash end
        if dash.duration <= 0.0 {
            dash.active = false;
            dash.duration = 0.25;
            health.invulnerable_until = 0.0;
            velocity.0 = Vec3::ZERO;

            // Particle burst at dash end (smaller)
            impact_events.send(SpawnImpactEvent {
                position: transform.translation,
                color: Some(class_color_v4(class.0)),
            });
        }
        return;
    }

    // ── Cooldown, input, and stamina checks ──
    if !input.dodge || cooldowns.dash > 0.0 {
        return;
    }
    if !stamina.has(20.0) {
        return;
    }
    stamina.spend(20.0);

    // ── Activation ──
    let cd_reduction = stats.dash_cooldown_reduction;
    let base_cd = (1.0 - cd_reduction).max(0.2);

    dash.active = true;
    cooldowns.dash = base_cd;
    dash.duration = 0.25;
    trail_timer.0 = 0.0; // first afterimage spawns immediately
    health.invulnerable_until = time.elapsed_secs() as f32 + 0.2;

    // ── Cursor-relative dash direction ──
    // Dodge AWAY from the cursor (evade what you're aiming at).
    let away = (transform.translation - cursor.0).normalize_or_zero();
    let dash_dir = if away.length_squared() > 0.1 {
        away
    } else {
        // Cursor is on or very close to the player; fall back to input direction.
        let move_dir = input.direction;
        if move_dir.length_squared() > 0.0 {
            Vec3::new(move_dir.x, 0.0, move_dir.y).normalize()
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        }
    };
    velocity.0 = dash_dir * 25.0;

    // ── Screen shake on dash start ──
    shake.trauma = (shake.trauma + 0.15).min(1.0);

    // ── Particle burst on dash start ──
    impact_events.send(SpawnImpactEvent {
        position: transform.translation,
        color: Some(class_color_v4(class.0)),
    });
}

/// Ticks all AbilityCooldowns on players each frame.
pub fn tick_ability_cooldowns(
    time: Res<Time>,
    mut query: Query<&mut AbilityCooldowns, With<Player>>,
) {
    let dt = time.delta_secs();
    for mut cooldowns in query.iter_mut() {
        cooldowns.tick(dt);
    }
}
