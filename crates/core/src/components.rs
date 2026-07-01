use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Character Class
// ============================================================================

/// The five playable classes. Each has unique stats, abilities, and playstyle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharacterClass {
    Warrior,
    Paladin,
    Rogue,
    Hunter,
    Mage,
}

impl CharacterClass {
    pub fn all() -> Vec<Self> {
        vec![Self::Warrior, Self::Paladin, Self::Rogue, Self::Hunter, Self::Mage]
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Warrior => "Warrior",
            Self::Paladin => "Paladin",
            Self::Rogue => "Rogue",
            Self::Hunter => "Hunter",
            Self::Mage => "Mage",
        }
    }

    pub fn resource_name(&self) -> &str {
        match self {
            Self::Warrior => "Rage",
            Self::Paladin => "Holy Power",
            Self::Rogue => "Energy",
            Self::Hunter => "Focus",
            Self::Mage => "Mana",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::Warrior => "A battle-hardened melee fighter who uses Rage to fuel devastating attacks. Excels at close quarters with high armor and area damage.",
            Self::Paladin => "A holy warrior who channels Holy Power into righteous melee strikes and healing light. Durable with strong self-sustain.",
            Self::Rogue => "A shadowy operative who spends Energy on quick, precise strikes. Highest single-target damage with stealth and poison mechanics.",
            Self::Hunter => "A wilderness expert who uses Focus for powerful ranged attacks. Pairs deadly accuracy with traps and mobility.",
            Self::Mage => "A master of arcane arts who wields Mana for devastating elemental magic. Unmatched area damage with teleportation and crowd control.",
        }
    }

    pub fn base_max_hp(&self) -> f32 {
        match self {
            Self::Warrior => 160.0,
            Self::Paladin => 140.0,
            Self::Rogue => 100.0,
            Self::Hunter => 110.0,
            Self::Mage => 90.0,
        }
    }

    pub fn base_stats(&self) -> CombatStats {
        match self {
            Self::Warrior => CombatStats {
                move_speed: 5.0,
                damage_bonus: 5.0,
                armor: 15.0,
                dodge_chance: 0.05,
                crit_chance: 0.10,
                crit_multiplier: 2.0,
                pickup_radius: 2.0,
                ..Default::default()
            },
            Self::Paladin => CombatStats {
                move_speed: 4.8,
                damage_bonus: 4.0,
                armor: 12.0,
                dodge_chance: 0.03,
                crit_chance: 0.08,
                crit_multiplier: 1.8,
                lifesteal: 0.05,
                pickup_radius: 2.0,
                ..Default::default()
            },
            Self::Rogue => CombatStats {
                move_speed: 6.5,
                damage_bonus: 3.0,
                armor: 4.0,
                dodge_chance: 0.20,
                crit_chance: 0.15,
                crit_multiplier: 2.5,
                pickup_radius: 3.0,
                ..Default::default()
            },
            Self::Hunter => CombatStats {
                move_speed: 5.5,
                damage_bonus: 6.0,
                armor: 6.0,
                dodge_chance: 0.10,
                crit_chance: 0.12,
                crit_multiplier: 2.2,
                pickup_radius: 2.5,
                ..Default::default()
            },
            Self::Mage => CombatStats {
                move_speed: 4.5,
                damage_bonus: 8.0,
                armor: 2.0,
                dodge_chance: 0.05,
                crit_chance: 0.10,
                crit_multiplier: 2.0,
                pickup_radius: 2.0,
                ..Default::default()
            },
        }
    }

    pub fn starting_weapon(&self) -> Weapon {
        match self {
            Self::Warrior => Weapon::new(WeaponKind::Sword, 14.0, 1.0, 3.5),
            Self::Paladin => Weapon::new(WeaponKind::Sword, 12.0, 0.9, 3.5),
            Self::Rogue => Weapon::new(WeaponKind::Dagger, 8.0, 2.0, 2.5),
            Self::Hunter => Weapon::new(WeaponKind::Bow, 15.0, 1.2, 25.0),
            Self::Mage => Weapon::new(WeaponKind::Staff, 18.0, 0.8, 20.0),
        }
    }

    /// Which ability fires on primary attack (LMB hold)
    pub fn primary_ability(&self) -> ClassAbilityId {
        match self {
            Self::Warrior => ClassAbilityId::MeleeCleave,
            Self::Paladin => ClassAbilityId::RighteousStrike,
            Self::Rogue => ClassAbilityId::Backstab,
            Self::Hunter => ClassAbilityId::AimedShot,
            Self::Mage => ClassAbilityId::Fireball,
        }
    }
    /// Which ability fires on secondary attack (RMB)
    pub fn secondary_ability(&self) -> ClassAbilityId {
        match self {
            Self::Warrior => ClassAbilityId::ShieldBlock,
            Self::Paladin => ClassAbilityId::HolyLight,
            Self::Rogue => ClassAbilityId::PoisonBlade,
            Self::Hunter => ClassAbilityId::MultiShot,
            Self::Mage => ClassAbilityId::Frostbolt,
        }
    }
    /// Which ability fires on cast (Q)
    pub fn cast_ability(&self) -> ClassAbilityId {
        match self {
            Self::Warrior => ClassAbilityId::Charge,
            Self::Paladin => ClassAbilityId::Consecration,
            Self::Rogue => ClassAbilityId::Vanish,
            Self::Hunter => ClassAbilityId::Trap,
            Self::Mage => ClassAbilityId::ArcaneBlast,
        }
    }
    /// Which ability triggers on dash
    pub fn dash_ability(&self) -> ClassAbilityId {
        match self {
            Self::Warrior => ClassAbilityId::CombatRoll,
            Self::Paladin => ClassAbilityId::DivineSteed,
            Self::Rogue => ClassAbilityId::Shadowstep,
            Self::Hunter => ClassAbilityId::Disengage,
            Self::Mage => ClassAbilityId::Blink,
        }
    }
}

impl Default for CharacterClass {
    fn default() -> Self { Self::Warrior }
}

impl std::str::FromStr for CharacterClass {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Warrior" => Ok(Self::Warrior),
            "Paladin" => Ok(Self::Paladin),
            "Rogue" => Ok(Self::Rogue),
            "Hunter" => Ok(Self::Hunter),
            "Mage" => Ok(Self::Mage),
            _ => Err(format!("Unknown class: {s}")),
        }
    }
}

/// Identifiers for every class ability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassAbilityId {
    // Warrior
    MeleeCleave,
    ShieldBlock,
    Charge,
    CombatRoll,
    // Paladin
    RighteousStrike,
    HolyLight,
    Consecration,
    DivineSteed,
    // Rogue
    Backstab,
    PoisonBlade,
    Vanish,
    Shadowstep,
    // Hunter
    AimedShot,
    MultiShot,
    Trap,
    Disengage,
    // Mage
    Fireball,
    Frostbolt,
    ArcaneBlast,
    Blink,
}

/// Component attached to the player — holds their class identity.
#[derive(Component, Debug, Clone)]
pub struct PlayerClass(pub CharacterClass);

/// Player's chosen name (from character creation).
#[derive(Component, Debug, Clone)]
pub struct PlayerName(pub String);

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
    pub max_health_bonus: f32,
    pub move_speed_bonus: f32,
    pub dash_cooldown_reduction: f32,
    pub lifesteal: f32,
    pub armor_penetration: f32,
    pub damage_taken_multiplier: f32,
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
            max_health_bonus: 0.0,
            move_speed_bonus: 0.0,
            dash_cooldown_reduction: 0.0,
            lifesteal: 0.0,
            armor_penetration: 0.0,
            damage_taken_multiplier: 1.0,
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

/// Per-enemy cooldown for attacks.
#[derive(Component, Debug, Clone)]
pub struct AttackCooldown {
    pub timer: f32,
    pub windup: f32,
}

impl Default for AttackCooldown {
    fn default() -> Self {
        Self { timer: 0.0, windup: 0.0 }
    }
}

// ============================================================================
// Stamina
// ============================================================================

/// Stamina resource for sprinting and dodging.
#[derive(Component, Debug, Clone)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32,
}

impl Default for Stamina {
    fn default() -> Self {
        Self { current: 100.0, max: 100.0, regen_rate: 15.0 }
    }
}

impl Stamina {
    pub fn has(&self, amount: f32) -> bool { self.current >= amount }
    pub fn spend(&mut self, amount: f32) { self.current = (self.current - amount).max(0.0); }
    pub fn fraction(&self) -> f32 { self.current / self.max }
}

/// Dash cooldown for player dodge roll.
#[derive(Component, Debug, Clone)]
pub struct DashCooldown {
    pub timer: f32,
    pub active: bool,
    pub duration: f32,
    pub fired_dash_attack: bool,
}

impl Default for DashCooldown {
    fn default() -> Self {
        Self {
            timer: 0.0,
            active: false,
            duration: 0.25,
            fired_dash_attack: false,
        }
    }
}

// ============================================================================
// Knockback System
// ============================================================================

/// Separate velocity for knockback, with natural damping/decay.
/// Applied on top of normal movement so knockback doesn't override player input.
#[derive(Component, Debug, Clone)]
pub struct Knockback {
    pub velocity: Vec3,
    pub damping: f32,
}

impl Knockback {
    pub fn new(velocity: Vec3, damping: f32) -> Self {
        Self { velocity, damping }
    }
}

// ============================================================================
// Status Effects
// ============================================================================

/// Frozen — reduces move speed by 60% for the duration.
/// Applied by Frostbolt (Mage secondary).
#[derive(Component, Debug, Clone)]
pub struct Frozen {
    pub remaining: f32,
}

impl Frozen {
    pub fn new(duration: f32) -> Self {
        Self { remaining: duration }
    }
}

/// Stun — prevents movement and actions for the duration.
/// Applied by heavy hits (charger, boss, critical hits).
#[derive(Component, Debug, Clone)]
pub struct Stun {
    pub remaining: f32,
}

impl Stun {
    pub fn new(duration: f32) -> Self {
        Self { remaining: duration }
    }
}

/// HitStun — brief movement interrupt on damage taken.
/// Freezes entity's movement velocity briefly (stagger).
#[derive(Component, Debug, Clone)]
pub struct HitStun {
    pub remaining: f32,
}

impl HitStun {
    pub fn new(duration: f32) -> Self {
        Self { remaining: duration }
    }
}

/// HitStop — brief pause on hit for game feel (local to the hit entity).
#[derive(Component, Debug, Clone)]
pub struct HitStop {
    pub remaining: f32,
}

impl HitStop {
    pub fn new(duration: f32) -> Self {
        Self { remaining: duration }
    }
}

// ============================================================================
// Telegraph Visual
// ============================================================================

/// Visual telegraph indicator spawned during enemy windup.
#[derive(Component, Debug, Clone)]
pub struct TelegraphIndicator {
    pub remaining: f32,
    pub target_entity: Entity,
}

impl TelegraphIndicator {
    pub fn new(duration: f32, target: Entity) -> Self {
        Self { remaining: duration, target_entity: target }
    }
}

/// Marker component for magical projectiles (Mage fireballs, frostbolts).
#[derive(Component, Debug, Clone)]
pub struct MagicProjectile;

/// Marker component for enemy projectiles, used for visual distinction.
#[derive(Component, Debug, Clone)]
pub struct EnemyProjectileMarker;

/// Marker for dash trail particle entity.
#[derive(Component, Debug, Clone)]
pub struct DashTrail;

/// Respawn timer component for dead players awaiting respawn.
#[derive(Component, Debug, Clone)]
pub struct RespawnTimer {
    pub remaining: f32,
}

impl RespawnTimer {
    pub fn new(duration: f32) -> Self {
        Self { remaining: duration }
    }
}
