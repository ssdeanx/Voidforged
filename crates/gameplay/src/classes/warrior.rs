//! Warrior class — melee cleave, shield block, charge, combat roll.
//! Resource: Rage (generated on dealing/taking damage, 5 Rage per event).

use bevy::prelude::*;
use ir_core::*;

/// Resource config
pub fn resource_config() -> ClassResource {
    ClassResource::new(100.0, 2.0)
}

/// Temporary damage reduction buff.
#[derive(Component)]
pub struct ShieldBlock {
    pub remaining: f32,
    pub reduction: f32,
}

// ── Primary: Melee Cleave ─────────────────────────────────────────────────

/// Spawns a hitbox in a cone in front of the player.
pub fn primary_melee_cleave(
    commands: &mut Commands,
    attacker: Entity,
    transform: &Transform,
    stats: &CombatStats,
    _enemies: &Query<(Entity, &Transform), With<Enemy>>,
    _damage_events: &mut EventWriter<DamageEvent>,
    _dmg_num_events: &mut EventWriter<DamageNumberEvent>,
    _impact_events: &mut EventWriter<SpawnImpactEvent>,
) {
    let dmg = (14.0 + stats.damage_bonus).max(1.0);
    commands.spawn((
        DamageHitbox::new(
            HitboxShape::Cone { range: 3.5, half_angle: 0.8 },
            dmg,
            attacker,
            DamageType::Physical,
            0.15, // lifetime (seconds) — quick swing
            ProjectileOwner::Player,
            2.0,  // knockback
        )
        .with_hit_reaction(0.15, 0.15, 0.05),
        Transform {
            translation: transform.translation,
            rotation: transform.rotation,
            ..default()
        },
    ));
}

// ── Secondary: Shield Block ───────────────────────────────────────────────

pub fn secondary_shield_block(
    commands: &mut Commands,
    player: Entity,
) {
    commands.entity(player).insert(ShieldBlock {
        remaining: 3.0,
        reduction: 0.4,
    });
}

// ── Cast: Charge ──────────────────────────────────────────────────────────

#[derive(Component)]
pub struct Charging {
    pub target: Vec3,
    pub speed: f32,
}

pub fn cast_charge(
    commands: &mut Commands,
    _transform: &Transform,
    _stats: &CombatStats,
    cursor: &CursorWorldPos,
) {
    commands.spawn((
        Charging { target: cursor.0, speed: 30.0 },
        Lifetime { remaining: 0.5 },
    ));
}

pub fn apply_charge_movement(
    time: Res<Time>,
    charge_query: Query<(&Charging, &Lifetime)>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Charging>)>,
) {
    let charge = match charge_query.get_single() {
        Ok((c, _)) => c,
        Err(_) => return,
    };
    let mut player_transform = match player_query.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };
    let to_target = charge.target - player_transform.translation;
    if to_target.length() < 1.0 { return; }
    let dir = to_target.normalize_or_zero();
    player_transform.translation += dir * charge.speed * time.delta_secs();
}

/// Ticks ShieldBlock buff — removes when expired.
pub fn tick_shield_block(
    time: Res<Time>,
    mut query: Query<&mut ShieldBlock>,
) {
    for mut block in query.iter_mut() {
        block.remaining -= time.delta_secs();
    }
}

/// Removes expired ShieldBlock.
pub fn cleanup_shield_block(
    mut commands: Commands,
    query: Query<(Entity, &ShieldBlock)>,
) {
    for (entity, block) in query.iter() {
        if block.remaining <= 0.0 {
            commands.entity(entity).remove::<ShieldBlock>();
        }
    }
}

// ── Rage Generation ──────────────────────────────────────────────────────

/// Generates rage when the warrior deals damage (from hitbox processing).
/// Reads DamageEvent and grants 5 Rage per hit.
pub fn warrior_rage_on_damage(
    mut damage_events: EventReader<DamageEvent>,
    mut player_query: Query<&mut ClassResource, (With<Player>, With<PlayerClass>)>,
) {
    let Ok(mut resource) = player_query.get_single_mut() else { return };
    // Only generate rage for Warrior class
    // (we check the resource type — Warrior has max=100 and regen=2)
    if (resource.max - 100.0).abs() > 0.1 || (resource.regen_rate - 2.0).abs() > 0.1 {
        return;
    }
    for event in damage_events.read() {
        if event.damage_type == DamageType::Physical || event.damage_type == DamageType::True {
            resource.current = (resource.current + 5.0).min(resource.max);
        }
    }
}

/// Generates rage when the warrior takes damage.
/// Reads a separate event stream and grants 5 Rage.
/// This is called from the damage pipeline when the warrior is the target.
pub fn warrior_rage_on_take_damage(
    mut damage_events: EventReader<DamageEvent>,
    mut player_query: Query<(Entity, &mut ClassResource), (With<Player>, With<PlayerClass>)>,
) {
    let Ok((player_entity, mut resource)) = player_query.get_single_mut() else { return };
    if (resource.max - 100.0).abs() > 0.1 || (resource.regen_rate - 2.0).abs() > 0.1 {
        return;
    }
    for event in damage_events.read() {
        if event.target == player_entity {
            resource.current = (resource.current + 5.0).min(resource.max);
        }
    }
}

// ── Dash: Combat Roll ─────────────────────────────────────────────────────

pub fn dash_combat_roll() {
    // Standard dodge — handled by dash_ability system
}
