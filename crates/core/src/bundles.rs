use bevy::prelude::*;
use crate::components::*;

/// Spawns a full player entity with all required components.
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub combat_stats: CombatStats,
    pub position: Position,
    pub velocity: Velocity,
    pub team: Team,
    pub render_info: RenderInfo,
    pub room_entity: RoomEntity,
}

impl PlayerBundle {
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

/// Spawns an enemy entity.
#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub health: Health,
    pub combat_stats: CombatStats,
    pub position: Position,
    pub velocity: Velocity,
    pub team: Team,
    pub render_info: RenderInfo,
    pub room_entity: RoomEntity,
}

impl EnemyBundle {
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
        }
    }
}

/// Spawns a projectile entity.
#[derive(Bundle)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub position: Position,
    pub velocity: Velocity,
    pub render_info: RenderInfo,
    pub room_entity: RoomEntity,
}

impl ProjectileBundle {
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

/// Spawns an experience gem pickup.
#[derive(Bundle)]
pub struct ExperienceGemBundle {
    pub gem: ExperienceGem,
    pub position: Position,
    pub velocity: Velocity,
    pub render_info: RenderInfo,
    pub room_entity: RoomEntity,
}

impl ExperienceGemBundle {
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
