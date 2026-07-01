//! Convenience [`Bundle`]s for spawning complex entities with all required
//! components at once.

use bevy::prelude::*;
use crate::components::*;

/// Spawns a full player entity with all required components.
///
/// Attaches [`Player`], [`Health`], [`CombatStats`], [`Position`], [`Velocity`],
/// [`Team`], [`RenderInfo`], and [`RoomEntity`] in one call.
#[derive(Bundle)]
pub struct PlayerBundle {
    /// Player component — level, XP tracking.
    pub player: Player,
    /// Health pool — current and max HP.
    pub health: Health,
    /// Core combat stats — damage, speed, armour, etc.
    pub combat_stats: CombatStats,
    /// 3D world position.
    pub position: Position,
    /// 3D movement velocity.
    pub velocity: Velocity,
    /// Faction allegiance (Player, Enemy, Neutral).
    pub team: Team,
    /// Visual mesh and material handles.
    pub render_info: RenderInfo,
    /// Room-scoping marker for cleanup on room exit.
    pub room_entity: RoomEntity,
}

impl PlayerBundle {
    /// Creates a new player bundle at the given world position with default stats.
    pub fn new(position: Vec3) -> Self {
        Self {
            player: Player::default(),
            health: Health::new(100.0),
            combat_stats: CombatStats::default(),
            position: Position(position),
            velocity: Velocity(Vec3::ZERO),
            team: Team::Player,
            render_info: RenderInfo::default(),
            room_entity: RoomEntity,
        }
    }
}

/// Spawns an enemy entity with its components.
///
/// Stats scale with tier: higher tier enemies deal more damage and have more HP.
#[derive(Bundle)]
pub struct EnemyBundle {
    /// Enemy component — variant, tier, XP reward.
    pub enemy: Enemy,
    /// Health pool — scales with tier.
    pub health: Health,
    /// Combat stats — move speed, armour, etc.
    pub combat_stats: CombatStats,
    /// 3D world position.
    pub position: Position,
    /// 3D movement velocity.
    pub velocity: Velocity,
    /// Faction allegiance (always Enemy for enemies).
    pub team: Team,
    /// Visual mesh and material handles.
    pub render_info: RenderInfo,
    /// Room-scoping marker for cleanup on room exit.
    pub room_entity: RoomEntity,
    /// Per-enemy attack cooldown timer.
    pub attack_cooldown: AttackCooldown,
}

impl EnemyBundle {
    /// Creates a new enemy bundle at the given position with variant-based base stats.
    ///
    /// Tier acts as a difficulty multiplier: HP, XP, and damage scale by `tier * 1.15`.
    pub fn new(variant: EnemyVariant, tier: u32, position: Vec3) -> Self {
        let (hp, xp, speed) = match &variant {
            EnemyVariant::Grunt => (30.0, 10, 3.5),
            EnemyVariant::Ranged => (20.0, 15, 2.5),
            EnemyVariant::Charger => (50.0, 20, 7.0),
            EnemyVariant::Elite => (200.0, 50, 3.0),
            EnemyVariant::Boss => (1000.0, 200, 2.0),
        };
        let scale = tier as f32 * 1.15;
        Self {
            enemy: Enemy {
                variant,
                tier,
                xp_reward: (xp as f64 * scale as f64) as u64,
            },
            health: Health::new(hp * scale),
            combat_stats: CombatStats {
                move_speed: speed,
                ..Default::default()
            },
            position: Position(position),
            velocity: Velocity(Vec3::ZERO),
            team: Team::Enemy,
            render_info: RenderInfo::default(),
            room_entity: RoomEntity,
            attack_cooldown: AttackCooldown::default(),
        }
    }
}

/// Spawns a projectile entity with its components.
#[derive(Bundle)]
pub struct ProjectileBundle {
    /// Projectile component — damage, speed, lifetime, owner.
    pub projectile: Projectile,
    /// 3D world position.
    pub position: Position,
    /// 3D movement velocity (direction * speed).
    pub velocity: Velocity,
    /// Visual mesh and material handles.
    pub render_info: RenderInfo,
    /// Room-scoping marker for cleanup on room exit.
    pub room_entity: RoomEntity,
}

impl ProjectileBundle {
    /// Creates a new projectile bundle moving in the given direction.
    pub fn new(
        damage: f32,
        speed: f32,
        lifetime: f32,
        direction: Vec3,
        origin: Vec3,
        owner: ProjectileOwner,
    ) -> Self {
        Self {
            projectile: Projectile {
                damage,
                speed,
                lifetime,
                max_lifetime: lifetime,
                piercing: false,
                owner,
            },
            position: Position(origin),
            velocity: Velocity(direction.normalize_or_zero() * speed),
            render_info: RenderInfo::default(),
            room_entity: RoomEntity,
        }
    }
}

/// Spawns an experience gem pickup on the ground.
#[derive(Bundle)]
pub struct ExperienceGemBundle {
    /// Experience gem component — value and magnet speed.
    pub gem: ExperienceGem,
    /// 3D world position.
    pub position: Position,
    /// 3D movement velocity (for physics-based movement toward player).
    pub velocity: Velocity,
    /// Visual mesh and material handles.
    pub render_info: RenderInfo,
    /// Room-scoping marker for cleanup on room exit.
    pub room_entity: RoomEntity,
}

impl ExperienceGemBundle {
    /// Creates a new experience gem bundle at the given position.
    pub fn new(value: u64, position: Vec3) -> Self {
        Self {
            gem: ExperienceGem {
                value,
                magnet_speed: 12.0,
            },
            position: Position(position),
            velocity: Velocity(Vec3::ZERO),
            render_info: RenderInfo::default(),
            room_entity: RoomEntity,
        }
    }
}
