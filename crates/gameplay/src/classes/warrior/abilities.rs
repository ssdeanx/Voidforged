//! Warrior ability implementations.
//! Primary: Melee Cleave, Secondary: Shield Block, Cast: Charge,
//! Utility (spec): War Cry (Berserker) / Taunt (Protector),
//! Ultimate (spec): Berserker Rage (Berserker) / Shield Wall (Protector).

use bevy::prelude::*;
use ir_core::*;

// ── Primary: Melee Cleave ──────────────────────────────────────────

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
            dmg, attacker, DamageType::Physical, 0.15,
            ProjectileOwner::Player, 2.0,
        ).with_hit_reaction(0.15, 0.15, 0.05),
        Transform {
            translation: transform.translation,
            rotation: transform.rotation,
            ..default()
        },
    ));
}

// ── Secondary: Shield Block ─────────────────────────────────────────

/// Temporary damage reduction buff applied by Shield Block.
#[derive(Component)]
pub struct ShieldBlock {
    pub remaining: f32,
    pub reduction: f32,
}

/// Applies the Shield Block buff to the player.
pub fn secondary_shield_block(commands: &mut Commands, player: Entity) {
    commands.entity(player).insert(ShieldBlock { remaining: 3.0, reduction: 0.4 });
}

/// Ticks ShieldBlock buff — removes when expired.
pub fn tick_shield_block(time: Res<Time>, mut query: Query<&mut ShieldBlock>) {
    for mut block in query.iter_mut() {
        block.remaining -= time.delta_secs();
    }
}

/// Removes expired ShieldBlock.
pub fn cleanup_shield_block(mut commands: Commands, query: Query<(Entity, &ShieldBlock)>) {
    for (entity, block) in query.iter() {
        if block.remaining <= 0.0 {
            commands.entity(entity).remove::<ShieldBlock>();
        }
    }
}

// ── Cast: Charge ───────────────────────────────────────────────────

#[derive(Component)]
pub struct Charging {
    pub target: Vec3,
    pub speed: f32,
}

/// Spawns a Charge marker toward cursor position.
pub fn cast_charge(commands: &mut Commands, _transform: &Transform, _stats: &CombatStats, cursor: &CursorWorldPos) {
    commands.spawn((Charging { target: cursor.0, speed: 30.0 }, Lifetime { remaining: 0.5 }));
}

/// Moves the player toward the charge target each frame.
pub fn apply_charge_movement(
    time: Res<Time>,
    charge_query: Query<(&Charging, &Lifetime)>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Charging>)>,
) {
    let charge = match charge_query.get_single() { Ok((c, _)) => c, Err(_) => return };
    let mut player_transform = match player_query.get_single_mut() { Ok(t) => t, Err(_) => return };
    let to_target = charge.target - player_transform.translation;
    if to_target.length() < 1.0 { return; }
    let dir = to_target.normalize_or_zero();
    player_transform.translation += dir * charge.speed * time.delta_secs();
}

// ── Spec-based Utilities ───────────────────────────────────────────

/// War Cry (Berserker spec) — party buff, grants bonus Rage.
pub fn utility_war_cry(commands: &mut Commands, entity: Entity, stats: &CombatStats) {
    info!("Warrior (Berserker): War Cry — +{} damage buff", stats.damage_bonus * 0.2);
    commands.entity(entity).insert(BuffComponent {
        id: "war_cry".into(),
        remaining: 8.0,
        stat_boost: stats.damage_bonus * 0.2,
    });
}

/// Taunt (Protector spec) — forces nearby enemies to attack the warrior.
pub fn utility_taunt(commands: &mut Commands, entity: Entity, _stats: &CombatStats, enemies: &Query<(Entity, &Transform), With<Enemy>>) {
    info!("Warrior (Protector): Taunt — {} enemies taunted", enemies.iter().count());
    commands.entity(entity).insert(BuffComponent {
        id: "taunt".into(),
        remaining: 5.0,
        stat_boost: 0.0,
    });
}

// ── Spec-based Ultimates ───────────────────────────────────────────

/// Berserker Rage (Berserker spec) — attack speed and damage buff.
pub fn ultimate_berserker_rage(commands: &mut Commands, entity: Entity, _stats: &CombatStats) {
    info!("Warrior (Berserker): Berserker Rage — +50% attack speed for 10s");
    commands.entity(entity).insert(BuffComponent {
        id: "berserker_rage".into(),
        remaining: 10.0,
        stat_boost: 0.5,
    });
}

/// Shield Wall (Protector spec) — massive damage reduction.
pub fn ultimate_shield_wall(commands: &mut Commands, entity: Entity, _stats: &CombatStats) {
    info!("Warrior (Protector): Shield Wall — 80% damage reduction for 8s");
    commands.entity(entity).insert(ShieldBlock { remaining: 8.0, reduction: 0.8 });
}
