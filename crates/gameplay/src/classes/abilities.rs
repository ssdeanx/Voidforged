//! Central ability dispatching + shared ability infrastructure.
//!
//! Each ability slot (primary/secondary/cast/dash) has one dispatch system
//! that reads the player's class and calls the appropriate module function.

use bevy::prelude::*;
use ir_core::*;
use crate::classes::{warrior, paladin, rogue, hunter, mage};

// ============================================================================
// Class Resource Component
// ============================================================================

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
    pub fn has(&self, amount: f32) -> bool { self.current >= amount }
    pub fn spend(&mut self, amount: f32) { self.current = (self.current - amount).max(0.0); }
    pub fn fraction(&self) -> f32 { if self.max > 0.0 { self.current / self.max } else { 0.0 } }
    pub fn can_afford(&self, amount: f32) -> bool { self.current >= amount }
    pub fn spend_resource(&mut self, amount: f32) -> bool {
        if self.current >= amount { self.current = (self.current - amount).max(0.0); true } else { false }
    }
}

pub fn class_resource_regen(time: Res<Time>, mut query: Query<&mut ClassResource, With<Player>>) {
    for mut resource in query.iter_mut() {
        resource.current = (resource.current + resource.regen_rate * time.delta_secs()).min(resource.max);
    }
}

fn resource_cost(class: CharacterClass, slot: &str) -> f32 {
    match (class, slot) {
        (CharacterClass::Rogue, "primary") => 15.0,
        (CharacterClass::Hunter, "primary") => 10.0,
        (CharacterClass::Mage, "primary") => 20.0,
        (CharacterClass::Mage, "secondary") => 15.0,
        (CharacterClass::Mage, "cast") => 30.0,
        (CharacterClass::Rogue, "secondary") => 20.0,
        (CharacterClass::Hunter, "secondary") => 15.0,
        _ => 0.0,
    }
}

// ── Primary Attack ───────────────────────────────────────────────────────

pub fn primary_attack(
    mut commands: Commands,
    input: Res<PlayerInput>,
    cursor: Res<CursorWorldPos>,
    mut player_query: Query<(Entity, &Transform, &PlayerClass, &CombatStats, &mut ClassResource), With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
) {
    if !input.primary_attack { return; }
    let Ok((entity, transform, class, stats, mut resource)) = player_query.get_single_mut() else { return; };
    let cost = resource_cost(class.0, "primary");
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

pub fn secondary_attack(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<PlayerInput>,
    cursor: Res<CursorWorldPos>,
    mut player_query: Query<(Entity, &Transform, &PlayerClass, &CombatStats, &mut ClassResource), With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
    mut cooldown: Local<f32>,
) {
    *cooldown = (*cooldown - time.delta_secs()).max(0.0);
    if !input.secondary_attack || *cooldown > 0.0 { return; }
    let Ok((entity, transform, class, stats, mut resource)) = player_query.get_single_mut() else { return; };
    *cooldown = 1.0;
    let cost = resource_cost(class.0, "secondary");
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

pub fn cast_ability(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<PlayerInput>,
    cursor: Res<CursorWorldPos>,
    mut player_query: Query<(Entity, &Transform, &PlayerClass, &CombatStats, &mut ClassResource), With<Player>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut damage_events: EventWriter<DamageEvent>,
    mut dmg_num_events: EventWriter<DamageNumberEvent>,
    mut impact_events: EventWriter<SpawnImpactEvent>,
    mut cooldown: Local<f32>,
) {
    *cooldown = (*cooldown - time.delta_secs()).max(0.0);
    if !input.cast || *cooldown > 0.0 { return; }
    let Ok((entity, transform, class, stats, mut resource)) = player_query.get_single_mut() else { return; };
    *cooldown = 3.0;
    let cost = resource_cost(class.0, "cast");
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

pub fn dash_ability(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<PlayerInput>,
    cursor: Res<CursorWorldPos>,
    mut player_query: Query<(
        Entity, &Transform, &PlayerClass, &CombatStats,
        &mut DashCooldown, &mut Health, &mut Velocity,
    ), With<Player>>,
) {
    let Ok((player_entity, transform, class, stats, mut dash, mut health, mut velocity)) = player_query.get_single_mut() else { return; };
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
    dash.active = true; dash.timer = base_cd; dash.duration = 0.25;
    health.invulnerable_until = time.elapsed_secs() as f32 + 0.2;
    let move_dir = input.direction;
    let dash_dir = if move_dir.length_squared() > 0.0 {
        Vec3::new(move_dir.x, 0.0, move_dir.y).normalize()
    } else {
        let to_cursor = cursor.0 - transform.translation;
        Vec3::new(to_cursor.x, 0.0, to_cursor.z).normalize_or_zero()
    };
    velocity.0 = dash_dir * 25.0;
    for i in 0..3 {
        let offset = Vec3::new((i as f32 - 1.0) * 0.3, 0.0, 0.0);
        commands.spawn((
            DashTrail,
            Transform::from_translation(transform.translation + offset),
            Lifetime { remaining: 0.3 },
        ));
    }
    if class.0 == CharacterClass::Hunter {
        let away = (transform.translation - cursor.0).normalize_or_zero();
        if away.length_squared() > 0.1 { velocity.0 = away * 25.0; }
    }
}
