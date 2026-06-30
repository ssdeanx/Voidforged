use bevy::prelude::*;

// ============================================================================
// Tag Components
// ============================================================================

/// Marks the player entity. Only one should exist at any time.
#[derive(Component, Debug, Clone)]
pub struct Player {
    /// Current character level (resets each run)
    pub level: u32,
    /// Total XP accumulated this run
    pub experience: u64,
    /// XP needed for next level
    pub xp_to_next: u64,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            level: 1,
            experience: 0,
            xp_to_next: 100,
        }
    }
}

/// Marks an enemy entity.
#[derive(Component, Debug, Clone)]
pub struct Enemy {
    /// Enemy type identifier for spawning and loot tables
    pub variant: EnemyVariant,
    /// Difficulty tier (scales with wave/zone depth)
    pub tier: u32,
    /// XP reward on death
    pub xp_reward: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EnemyVariant {
    Grunt,
    Ranged,
    Charger,
    Elite,
    Boss,
}

/// Marks a projectile entity.
#[derive(Component, Debug, Clone)]
pub struct Projectile {
    pub damage: f32,
    pub speed: f32,
    pub lifetime: f32,       // seconds remaining
    pub max_lifetime: f32,   // seconds before despawn
    pub piercing: bool,
    pub owner: ProjectileOwner,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectileOwner {
    Player,
    Enemy,
}

/// Marks an experience gem dropped by enemies.
#[derive(Component, Debug, Clone)]
pub struct ExperienceGem {
    pub value: u64,
    pub magnet_speed: f32,
}

/// Marks a collectible item on the ground.
#[derive(Component, Debug, Clone)]
pub struct Pickup {
    pub kind: PickupKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PickupKind {
    Health,
    Gold,
    TemporaryBoost,
}

/// Weapon component — attached to the player or enemies that use weapons.
#[derive(Component, Debug, Clone)]
pub struct Weapon {
    pub kind: WeaponKind,
    pub damage: f32,
    pub attack_speed: f32,  // attacks per second
    pub range: f32,
    pub cooldown_timer: f32,
    pub evolution_stage: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WeaponKind {
    Dagger,
    Sword,
    Bow,
    Staff,
    Aura,
    Whip,
    MagicMissile,
}

impl Weapon {
    pub fn new(kind: WeaponKind, damage: f32, attack_speed: f32, range: f32) -> Self {
        Self {
            kind,
            damage,
            attack_speed,
            range,
            cooldown_timer: 0.0,
            evolution_stage: 0,
        }
    }
}

/// Passive ability / buff attached to an entity.
#[derive(Component, Debug, Clone)]
pub struct Ability {
    pub kind: AbilityKind,
    pub tier: u32,
    pub duration: Option<f32>, // None = permanent while attached
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbilityKind {
    SpeedBoost,
    DamageAura,
    Shield,
    Thorns,
    MultiShot,
    PierceShot,
}

// ============================================================================
// Stat Components
// ============================================================================

/// Health pool.
#[derive(Component, Debug, Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub invulnerable_until: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
            invulnerable_until: 0.0,
        }
    }

    pub fn fraction(&self) -> f32 {
        self.current / self.max
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    pub fn take_damage(&mut self, amount: f32, time: f32) -> bool {
        if time < self.invulnerable_until {
            return false;
        }
        self.current = (self.current - amount).max(0.0);
        true
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }
}

/// Core combat stats.
#[derive(Component, Debug, Clone)]
pub struct CombatStats {
    pub damage_bonus: f32,
    pub attack_speed_bonus: f32,
    pub move_speed: f32,
    pub armor: f32,
    pub dodge_chance: f32,
    pub crit_chance: f32,
    pub crit_multiplier: f32,
    pub pickup_radius: f32,
}

impl Default for CombatStats {
    fn default() -> Self {
        Self {
            damage_bonus: 0.0,
            attack_speed_bonus: 0.0,
            move_speed: 5.0,
            armor: 0.0,
            dodge_chance: 0.0,
            crit_chance: 0.05,
            crit_multiplier: 2.0,
            pickup_radius: 2.0,
        }
    }
}

// ============================================================================
// Spatial Components
// ============================================================================

/// 3D position for the isometric world.
#[derive(Component, Debug, Clone)]
pub struct Position(pub Vec3);

impl Default for Position {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}

/// Velocity for movement.
#[derive(Component, Debug, Clone)]
pub struct Velocity(pub Vec3);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}

/// Rendering information — which model/mesh to use and visual variants.
#[derive(Component, Debug, Clone)]
pub struct RenderInfo {
    pub mesh_handle: Option<Handle<Mesh>>,
    pub material_handle: Option<Handle<StandardMaterial>>,
    pub scale: f32,
    pub tint: Color,
}

impl Default for RenderInfo {
    fn default() -> Self {
        Self {
            mesh_handle: None,
            material_handle: None,
            scale: 1.0,
            tint: Color::WHITE,
        }
    }
}

// ============================================================================
// Team Component
// ============================================================================

/// Which side an entity belongs to.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum Team {
    Player,
    Enemy,
    Neutral,
}

// ============================================================================
// Spawning Markers
// ============================================================================

/// Marker for entities that should be despawned when leaving the current room/zone.
#[derive(Component, Debug, Clone)]
pub struct RoomEntity;

/// Marker for the current room/zone entity (used by procedural generation).
#[derive(Component, Debug, Clone)]
pub struct Room;

/// Auto-despawn after a duration (for VFX, temp indicators).
#[derive(Component, Debug, Clone)]
pub struct Lifetime {
    pub remaining: f32,
}
