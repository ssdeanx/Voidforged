//! Central ability dispatching + shared ability infrastructure.
//!
//! Each ability slot (primary/secondary/cast/dash) has one dispatch system
//! that reads the player's class and calls the appropriate module function.
//! Cooldowns live on the `AbilityCooldowns` component. Resource costs come
//! from `CharacterClass::resource_costs()`.

use bevy::prelude::*;
use ir_core::*;
use crate::classes::{hunter, mage, paladin, rogue, warrior};

<<<<<<< HEAD
/// Passive regeneration of the per-class resource (Rage, Holy Power, etc.).
pub fn class_resource_regen(
    time: Res<Time>,
    mut query: Query<&mut ClassResource, With<Player>>,
) {
=======
// ============================================================================
// Class Resource Component
// ============================================================================

/// Per-class resource bar component (Rage, Energy, Mana, Focus, Holy Power).
///
/// Each class has its own resource type with different max capacity and
/// regeneration rates. Ability costs are deducted from this resource.
#[derive(Component, Debug, Clone)]
pub struct ClassResource {
    /// Current resource amount.
    pub current: f32,
    /// Maximum resource capacity.
    pub max: f32,
    /// Resource regenerated per second.
    pub regen_rate: f32,
}

impl ClassResource {
    /// Creates a new class resource starting at full capacity.
    pub fn new(max: f32, regen_rate: f32) -> Self {
        Self { current: max, max, regen_rate }
    }
    /// Returns `true` if the resource has at least `amount` available.
    pub fn has(&self, amount: f32) -> bool { self.current >= amount }
    /// Spends resource without checking — clamps to zero.
    pub fn spend(&mut self, amount: f32) { self.current = (self.current - amount).max(0.0); }
    /// Returns the current fraction (0.0–1.0) of resource remaining.
    pub fn fraction(&self) -> f32 { if self.max > 0.0 { self.current / self.max } else { 0.0 } }
    /// Returns `true` if enough resource is available for the given cost.
    pub fn can_afford(&self, amount: f32) -> bool { self.current >= amount }
    /// Tries to spend `amount`; returns `true` and deducts on success.
    pub fn spend_resource(&mut self, amount: f32) -> bool {
        if self.current >= amount { self.current = (self.current - amount).max(0.0); true } else { false }
    }
}

/// Regenerates the player's class resource each frame over time.
///
/// Adds `regen_rate * delta_secs` to the resource, capped at `max`.
pub fn class_resource_regen(time: Res<Time>, mut query: Query<&mut ClassResource, With<Player>>) {
>>>>>>> origin/master
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

/// Handles the player's dash (Shift key) — grants brief invulnerability.
///
/// Applies a burst of velocity in the movement direction (or toward cursor
/// if no direction is pressed). During the dash the player is invulnerable
/// for 0.2 seconds. Hunter's dash reverses direction (disengage).
pub fn dash_ability(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<PlayerInput>,
    cursor: Res<CursorWorldPos>,
    mut player_query: Query<(
        Entity, &Transform, &PlayerClass, &CombatStats,
        &mut DashCooldown, &mut Health, &mut Velocity, &mut Stamina,
    ), With<Player>>,
) {
    let Ok((_player_entity, transform, class, stats, mut dash, mut health, mut velocity, mut stamina)) = player_query.get_single_mut() else { return; };
    let cd_reduction = stats.dash_cooldown_reduction;
    let base_cd = (1.0 - cd_reduction).max(0.2);
    if dash.timer > 0.0 { dash.timer = (dash.timer - time.delta_secs()).max(0.0); }
    if dash.active {
        dash.duration -= time.delta_secs();
        if dash.duration > (0.25 - 0.2) {
            health.invulnerable_until = time.elapsed_secs() as f32 + 0.2;
        }
        if dash.duration <= 0.0 {
            dash.active = false; dash.duration = 0.25;
            health.invulnerable_until = 0.0; velocity.0 = Vec3::ZERO;
        }
        return;
    }
    if !input.dodge || dash.timer > 0.0 { return; }
    // Stamina cost gate: dodge costs 20 stamina
    if !stamina.has(20.0) { return; }
    stamina.spend(20.0);

    dash.active = true; dash.timer = base_cd; dash.duration = 0.25;
    health.invulnerable_until = time.elapsed_secs() as f32 + 0.2;
    let move_dir = input.direction;
    let dash_dir = if move_dir.length_squared() > 0.0 {
        // Dodge in input direction (keyboard-relative, converted to world)
        Vec3::new(move_dir.x, 0.0, move_dir.y).normalize()
    } else {
        // No input direction: dodge AWAY from cursor (dodge backward relative to aim)
        let away = (transform.translation - cursor.0).normalize_or_zero();
        if away.length_squared() > 0.1 { away } else { Vec3::new(0.0, 0.0, 1.0) }
    };
    // Hunter dodge: always away from cursor (disengage)
    let final_dir = if class.0 == CharacterClass::Hunter {
        let away = (transform.translation - cursor.0).normalize_or_zero();
        if away.length_squared() > 0.1 { away } else { dash_dir }
    } else {
        dash_dir
    };
    velocity.0 = final_dir * 25.0;
    for i in 0..3 {
        let offset = Vec3::new((i as f32 - 1.0) * 0.3, 0.0, 0.0);
        commands.spawn((
            DashTrail,
            Transform::from_translation(transform.translation + offset),
            Lifetime { remaining: 0.3 },
        ));
    }
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
