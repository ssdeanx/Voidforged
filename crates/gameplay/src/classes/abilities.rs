//! Central ability dispatching + shared ability infrastructure.
//!
//! Each ability slot (primary/secondary/cast/dash) has one dispatch system
//! that reads the player's class and calls the appropriate module function.
//! This keeps the per-class logic in isolated files while avoiding 20 separate systems.

use bevy::prelude::*;
use ir_core::*;
use crate::classes::{warrior, paladin, rogue, hunter, mage};

// ============================================================================
// Class Resource Component
// ============================================================================

/// Per-class resource meter (Rage, Energy, Mana, Focus, Holy Power).
#[derive(Component, Debug, Clone)]
pub struct ClassResource {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32,
}

impl ClassResource {
    pub fn new(max: f32, regen_rate: f32) -> Self {
        Self { current: max, max, regen_rate }
    }

    pub fn has(&self, amount: f32) -> bool {
        self.current >= amount
    }

    pub fn spend(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn fraction(&self) -> f32 {
        self.current / self.max
    }
}

/// Passive regen for all class resources each frame.
pub fn class_resource_regen(
    time: Res<Time>,
    mut query: Query<&mut ClassResource, With<Player>>,
) {
    for mut resource in query.iter_mut() {
        resource.current = (resource.current + resource.regen_rate * time.delta_secs())
            .min(resource.max);
    }
}

// ============================================================================
// Primary Attack — dispatches to class module
// ============================================================================

pub fn primary_attack(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<PlayerInput>,
    cursor: Res<CursorWorldPos>,
    player_query: Query<(Entity, &Transform, &PlayerClass, &CombatStats), With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
) {
    let Ok((entity, transform, class, stats)) = player_query.get_single() else { return };
    if !input.primary_attack { return; }

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

// ============================================================================
// Secondary Attack — dispatches to class module
// ============================================================================

pub fn secondary_attack(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<PlayerInput>,
    cursor: Res<CursorWorldPos>,
    player_query: Query<(Entity, &Transform, &PlayerClass, &CombatStats), With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
    mut cooldown: Local<f32>,
) {
    *cooldown = (*cooldown - time.delta_secs()).max(0.0);
    if !input.secondary_attack || *cooldown > 0.0 { return; }
    let Ok((entity, transform, class, stats)) = player_query.get_single() else { return };
    *cooldown = 1.0;

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

// ============================================================================
// Cast Ability — dispatches to class module
// ============================================================================

pub fn cast_ability(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<PlayerInput>,
    cursor: Res<CursorWorldPos>,
    player_query: Query<(Entity, &Transform, &PlayerClass, &CombatStats), With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
    mut cooldown: Local<f32>,
) {
    *cooldown = (*cooldown - time.delta_secs()).max(0.0);
    if !input.cast || *cooldown > 0.0 { return; }
    let Ok((_entity, transform, class, stats)) = player_query.get_single() else { return };
    *cooldown = 3.0;

    match class.0 {
        CharacterClass::Warrior => warrior::cast_charge(&mut commands, transform, stats, &cursor),
        CharacterClass::Paladin => paladin::cast_consecration(&mut commands, transform, stats),
        CharacterClass::Rogue => rogue::cast_vanish(&mut commands, _entity),
        CharacterClass::Hunter => hunter::cast_trap(&mut commands, transform),
        CharacterClass::Mage => mage::cast_arcane_blast(
            &mut commands, transform, stats, &cursor, &enemies,
            &mut damage_events, &mut dmg_num_events, &mut impact_events,
        ),
    }
}

// ============================================================================
// Dash Ability — handles dash state machine + dispatches class behavior
// ============================================================================

pub fn dash_ability(
    _commands: Commands,
    time: Res<Time>,
    input: Res<PlayerInput>,
    _cursor: Res<CursorWorldPos>,
    mut player_query: Query<(&Transform, &PlayerClass, &CombatStats, &mut DashCooldown, &mut Health), With<Player>>,
) {
    let Ok((_transform, class, stats, mut dash, mut health)) = player_query.get_single_mut() else {
        return;
    };

    let cd_reduction = stats.dash_cooldown_reduction;
    let base_cd = (1.0 - cd_reduction).max(0.2);

    if dash.timer > 0.0 {
        dash.timer = (dash.timer - time.delta_secs()).max(0.0);
    }

    // Handle active dash state
    if dash.active {
        dash.duration -= time.delta_secs();
        if dash.duration <= 0.0 {
            dash.active = false;
            dash.duration = 0.25;
            health.invulnerable_until = 0.0;
        }
        return;
    }

    if !input.dodge || dash.timer > 0.0 {
        return;
    }

    dash.active = true;
    dash.timer = base_cd;
    dash.duration = 0.25;
    health.invulnerable_until = time.elapsed_secs() as f32 + 0.3;

    match class.0 {
        CharacterClass::Warrior => {
            // Combat roll — no extra effect, standard dodge
        }
        CharacterClass::Paladin => {
            // Divine steed — handled by dash velocity multiplier in future
        }
        CharacterClass::Rogue => {
            // Shadowstep — standalone system
        }
        CharacterClass::Hunter => {
            // Disengage — standalone system
        }
        CharacterClass::Mage => {
            // Blink — standalone system
        }
    }
}
