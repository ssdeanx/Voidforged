//! Bevy ECS components for players, enemies, projectiles, character classes,
//! combat stats, spatial data, status effects, spawning markers, and more.
//!
//! These components are the building blocks of every entity in the game.
//! Related types (bundles, events, resources) live in their own modules.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Class Resource Component — per-class resource (Rage, Holy Power, Energy, Focus, Mana)
// ============================================================================

/// Per-class resource pool used to fuel abilities.
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

/// Cooldown timers for each ability slot — replaces ad-hoc `Local<f32>`.
#[derive(Component, Debug, Clone)]
pub struct AbilityCooldowns {
    pub primary: f32,
    pub secondary: f32,
    pub cast: f32,
    pub dash: f32,
}

impl Default for AbilityCooldowns {
    fn default() -> Self {
        Self {
            primary: 0.0,
            secondary: 0.0,
            cast: 0.0,
            dash: 0.0,
        }
    }
}

impl AbilityCooldowns {
    pub fn tick(&mut self, dt: f32) {
        self.primary = (self.primary - dt).max(0.0);
        self.secondary = (self.secondary - dt).max(0.0);
        self.cast = (self.cast - dt).max(0.0);
        self.dash = (self.dash - dt).max(0.0);
    }
}

/// Forced movement — general-purpose component for knockback, charge, disengage, etc.
/// Applied on top of normal movement so it doesn't override player input.
#[derive(Component, Debug, Clone)]
pub struct ForcedMovement {
    pub velocity: Vec3,
    pub damping: f32,
}

impl ForcedMovement {
    pub fn new(velocity: Vec3, damping: f32) -> Self {
        Self { velocity, damping }
    }
}

// ============================================================================
// Character Class
// ============================================================================

/// The five playable character classes. Each has unique stats, abilities, and playstyle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharacterClass {
    /// A battle-hardened melee fighter who uses Rage to fuel devastating attacks.
    /// Excels at close quarters with high armour and area damage.
    Warrior,
    /// A holy warrior who channels Holy Power into righteous melee strikes and healing light.
    /// Durable with strong self-sustain.
    Paladin,
    /// A shadowy operative who spends Energy on quick, precise strikes.
    /// Highest single-target damage with stealth and poison mechanics.
    Rogue,
    /// A wilderness expert who uses Focus for powerful ranged attacks.
    /// Pairs deadly accuracy with traps and mobility.
    Hunter,
    /// A master of arcane arts who wields Mana for devastating elemental magic.
    /// Unmatched area damage with teleportation and crowd control.
    Mage,
}

impl CharacterClass {
    /// Returns a vector containing every class variant for iteration.
    pub fn all() -> Vec<Self> {
        vec![Self::Warrior, Self::Paladin, Self::Rogue, Self::Hunter, Self::Mage]
    }

    /// Returns the human-readable display name for this class.
    pub fn display_name(&self) -> &str {
        match self {
            Self::Warrior => "Warrior",
            Self::Paladin => "Paladin",
            Self::Rogue => "Rogue",
            Self::Hunter => "Hunter",
            Self::Mage => "Mage",
        }
    }

    /// Returns the name of the class's primary resource (e.g. Rage, Mana).
    pub fn resource_name(&self) -> &str {
        match self {
            Self::Warrior => "Rage",
            Self::Paladin => "Holy Power",
            Self::Rogue => "Energy",
            Self::Hunter => "Focus",
            Self::Mage => "Mana",
        }
    }

    /// Returns a longer flavour description of the class for character creation.
    pub fn description(&self) -> &str {
        match self {
            Self::Warrior => "A battle-hardened melee fighter who uses Rage to fuel devastating attacks. Excels at close quarters with high armor and area damage.",
            Self::Paladin => "A holy warrior who channels Holy Power into righteous melee strikes and healing light. Durable with strong self-sustain.",
            Self::Rogue => "A shadowy operative who spends Energy on quick, precise strikes. Highest single-target damage with stealth and poison mechanics.",
            Self::Hunter => "A wilderness expert who uses Focus for powerful ranged attacks. Pairs deadly accuracy with traps and mobility.",
            Self::Mage => "A master of arcane arts who wields Mana for devastating elemental magic. Unmatched area damage with teleportation and crowd control.",
        }
    }

    /// Returns the base maximum HP for this class at level 1.
    pub fn base_max_hp(&self) -> f32 {
        match self {
            Self::Warrior => 160.0,
            Self::Paladin => 140.0,
            Self::Rogue => 100.0,
            Self::Hunter => 110.0,
            Self::Mage => 90.0,
        }
    }

    /// Returns the base [`CombatStats`] for this class at level 1.
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

    pub fn is_unlocked(&self, unlocks: &[String]) -> bool {
        match self.starting_weapon().kind {
            WeaponKind::Sword => true, // Sword and Aura are always available
            WeaponKind::Aura | WeaponKind::MagicMissile | WeaponKind::Whip => true,
            kind => {
                let name = format!("{:?}", kind).to_lowercase();
                unlocks.iter().any(|u| u == &name)
            }
        }
    }

    /// Returns the starting [`Weapon`] for this class at level 1.
    pub fn starting_weapon(&self) -> Weapon {
        match self {
            Self::Warrior => Weapon::new(WeaponKind::Sword, 14.0, 1.0, 3.5),
            Self::Paladin => Weapon::new(WeaponKind::Sword, 12.0, 0.9, 3.5),
            Self::Rogue => Weapon::new(WeaponKind::Dagger, 8.0, 2.0, 2.5),
            Self::Hunter => Weapon::new(WeaponKind::Bow, 15.0, 1.2, 25.0),
            Self::Mage => Weapon::new(WeaponKind::Staff, 18.0, 0.8, 20.0),
        }
    }

    /// Returns resource costs for each ability slot: (primary, secondary, cast, dash).
    /// Most classes have 0-cost for some slots (e.g. Warrior has no resource cost on any slot,
    /// generating Rage by dealing/taking damage instead).
    pub fn resource_costs(&self) -> (f32, f32, f32, f32) {
        match self {
            Self::Warrior => (0.0, 0.0, 0.0, 0.0),
            Self::Paladin => (0.0, 3.0, 0.0, 0.0), // Holy Light costs 3 Holy Power
            Self::Rogue => (15.0, 20.0, 0.0, 0.0),
            Self::Hunter => (10.0, 15.0, 0.0, 0.0),
            Self::Mage => (20.0, 15.0, 30.0, 0.0),
        }
    }

    /// Returns the [`ClassAbilityId`] for this class's primary attack (LMB hold).
    pub fn primary_ability(&self) -> ClassAbilityId {
        match self {
            Self::Warrior => ClassAbilityId::MeleeCleave,
            Self::Paladin => ClassAbilityId::RighteousStrike,
            Self::Rogue => ClassAbilityId::Backstab,
            Self::Hunter => ClassAbilityId::AimedShot,
            Self::Mage => ClassAbilityId::Fireball,
        }
    }

    /// Returns the [`ClassAbilityId`] for this class's secondary attack (RMB).
    pub fn secondary_ability(&self) -> ClassAbilityId {
        match self {
            Self::Warrior => ClassAbilityId::ShieldBlock,
            Self::Paladin => ClassAbilityId::HolyLight,
            Self::Rogue => ClassAbilityId::PoisonBlade,
            Self::Hunter => ClassAbilityId::MultiShot,
            Self::Mage => ClassAbilityId::Frostbolt,
        }
    }

    /// Returns the [`ClassAbilityId`] for this class's cast ability (Q).
    pub fn cast_ability(&self) -> ClassAbilityId {
        match self {
            Self::Warrior => ClassAbilityId::Charge,
            Self::Paladin => ClassAbilityId::Consecration,
            Self::Rogue => ClassAbilityId::Vanish,
            Self::Hunter => ClassAbilityId::Trap,
            Self::Mage => ClassAbilityId::ArcaneBlast,
        }
    }

    /// Returns the [`ClassAbilityId`] for this class's dash ability.
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

/// Identifiers for every class ability in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClassAbilityId {
    // Warrior
    /// Wide cleave AoE attack.
    MeleeCleave,
    /// Shield block that reduces incoming damage briefly.
    ShieldBlock,
    /// Rush toward target location, dealing impact damage.
    Charge,
    /// Quick roll that grants brief invincibility frames.
    CombatRoll,
    // Paladin
    /// Righteous melee strike with bonus holy damage.
    RighteousStrike,
    /// Heal self or nearby ally.
    HolyLight,
    /// Consecrate ground, dealing damage over time in an area.
    Consecration,
    /// Mount a spectral steed for brief speed boost.
    DivineSteed,
    // Rogue
    /// High-damage attack from behind the target.
    Backstab,
    /// Coat weapon with poison for damage-over-time.
    PoisonBlade,
    /// Become invisible briefly, next attack deals bonus damage.
    Vanish,
    /// Teleport behind the target.
    Shadowstep,
    // Hunter
    /// Precise ranged shot with bonus crit chance.
    AimedShot,
    /// Fire multiple arrows in a cone.
    MultiShot,
    /// Place a snare trap that slows enemies.
    Trap,
    /// Leap backward, creating distance from enemies.
    Disengage,
    // Mage
    /// Launch a fireball that explodes on impact.
    Fireball,
    /// Freeze a target, slowing and dealing frost damage.
    Frostbolt,
    /// Powerful arcane burst in a cone.
    ArcaneBlast,
    /// Short-range teleport in the movement direction.
    Blink,
}

impl ClassAbilityId {
    /// Human-readable ability name for the HUD action bar.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::MeleeCleave => "Cleave",
            Self::ShieldBlock => "Shield",
            Self::Charge => "Charge",
            Self::CombatRoll => "Roll",
            Self::RighteousStrike => "Strike",
            Self::HolyLight => "Heal",
            Self::Consecration => "Consecrate",
            Self::DivineSteed => "Steed",
            Self::Backstab => "Backstab",
            Self::PoisonBlade => "Poison",
            Self::Vanish => "Vanish",
            Self::Shadowstep => "Shadowstep",
            Self::AimedShot => "Aimed Shot",
            Self::MultiShot => "Multi Shot",
            Self::Trap => "Trap",
            Self::Disengage => "Disengage",
            Self::Fireball => "Fireball",
            Self::Frostbolt => "Frostbolt",
            Self::ArcaneBlast => "Arcane Blast",
            Self::Blink => "Blink",
        }
    }
}

/// Component attached to the player entity — holds their class identity.
///
/// Read by the ability system to determine which skills the player can use.
#[derive(Component, Debug, Clone)]
pub struct PlayerClass(pub CharacterClass);

/// Player's chosen name from character creation.
///
/// Displayed in UI panels and above the player character.
#[derive(Component, Debug, Clone)]
pub struct PlayerName(pub String);

// ============================================================================
// Tag Components
// ============================================================================

/// Marks the player entity. Only one should exist at any time.
///
/// Carries the player's level and XP accumulated during the current run.
#[derive(Component, Debug, Clone)]
pub struct Player {
    /// Current character level (resets each run, persists across runs in profile).
    pub level: u32,
    /// Total XP accumulated this run toward the next level.
    pub experience: u64,
    /// XP required to reach the next level.
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
///
/// Carries the enemy's variant type, difficulty tier, and XP reward on death.
#[derive(Component, Debug, Clone)]
pub struct Enemy {
    /// Enemy type identifier (used for spawning, meshes, loot tables).
    pub variant: EnemyVariant,
    /// Difficulty tier — scales with wave/zone depth, multiplies stats.
    pub tier: u32,
    /// XP awarded to the player when this enemy is killed.
    pub xp_reward: u64,
}

/// Variant types for enemies — determines stats, behaviour, and loot.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EnemyVariant {
    /// Basic melee enemy with moderate HP and damage.
    Grunt,
    /// Ranged attacker that fires projectiles from a distance.
    Ranged,
    /// Fast melee charger that rushes the player.
    Charger,
    /// Higher-tier enemy with increased stats and rewards.
    Elite,
    /// Boss enemy with greatly multiplied stats and special behaviours.
    Boss,
}

/// Marks a projectile entity (player or enemy projectiles).
///
/// Contains damage, speed, lifetime, piercing status, and owner information.
#[derive(Component, Debug, Clone)]
pub struct Projectile {
    /// Damage dealt on impact.
    pub damage: f32,
    /// Movement speed in units per second.
    pub speed: f32,
    /// Seconds remaining before the projectile despawns.
    pub lifetime: f32,
    /// Maximum lifetime (set on creation, used for despawn timing).
    pub max_lifetime: f32,
    /// Whether the projectile passes through enemies (hitting multiple targets).
    pub piercing: bool,
    /// Whether this projectile was fired by the player or an enemy.
    pub owner: ProjectileOwner,
}

/// Distinguishes between player-owned and enemy-owned projectiles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectileOwner {
    /// Projectile was fired by the player or their abilities.
    Player,
    /// Projectile was fired by an enemy.
    Enemy,
}

/// Marks an experience gem dropped by enemies on death.
///
/// Flies toward the player when within magnet range.
#[derive(Component, Debug, Clone)]
pub struct ExperienceGem {
    /// Amount of XP granted when collected.
    pub value: u64,
    /// Speed at which the gem accelerates toward the player when in magnet range.
    pub magnet_speed: f32,
}

/// Marks a collectible item on the ground (health, gold, temporary buffs).
#[derive(Component, Debug, Clone)]
pub struct Pickup {
    /// What kind of pickup this is.
    pub kind: PickupKind,
}

/// Categorises the type of ground pickup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PickupKind {
    /// Restores HP on collection.
    Health,
    /// Adds gold to the player's inventory.
    Gold,
    /// Applies a temporary stat boost on collection.
    TemporaryBoost,
}

/// Weapon component — attached to the player or enemies that use weapons.
///
/// Defines the base damage, attack speed, range, and evolution stage.
#[derive(Component, Debug, Clone)]
pub struct Weapon {
    /// Weapon type (sword, bow, staff, etc.).
    pub kind: WeaponKind,
    /// Base damage dealt per hit.
    pub damage: f32,
    /// Attacks per second (higher = faster).
    pub attack_speed: f32,
    /// Maximum attack range in world units.
    pub range: f32,
    /// Current cooldown timer — attack is ready when <= 0.
    pub cooldown_timer: f32,
    /// Evolution stage (some weapons evolve after reaching certain milestones).
    pub evolution_stage: u32,
}

/// Types of weapons available in the game.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WeaponKind {
    /// Fast, low-damage melee weapon.
    Dagger,
    /// Balanced melee weapon.
    Sword,
    /// Ranged physical weapon.
    Bow,
    /// Ranged magical weapon.
    Staff,
    /// Passive damage aura (auto-hits nearby enemies).
    Aura,
    /// Medium-range melee weapon with sweep attacks.
    Whip,
    /// Homing magic projectile weapon.
    MagicMissile,
}

impl Weapon {
    /// Creates a new weapon with the given properties and cooldown ready.
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

/// Passive ability or buff attached to an entity.
///
/// Modifies stats or behaviour while attached. Can have a limited duration
/// (`None` = permanent until removed).
#[derive(Component, Debug, Clone)]
pub struct Ability {
    /// Which ability/buff this is.
    pub kind: AbilityKind,
    /// Power tier (higher = stronger effect).
    pub tier: u32,
    /// Remaining duration in seconds. `None` = permanent while attached.
    pub duration: Option<f32>,
}

/// Available passive ability and buff types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbilityKind {
    /// Increases movement speed.
    SpeedBoost,
    /// Deals damage to nearby enemies each frame.
    DamageAura,
    /// Absorbs a portion of incoming damage.
    Shield,
    /// Reflects a percentage of incoming damage back to the attacker.
    Thorns,
    /// Fires additional projectiles with each attack.
    MultiShot,
    /// Projectiles pass through extra targets.
    PierceShot,
}

// ============================================================================
// Stat Components
// ============================================================================

/// Health pool with damage, healing, and invulnerability window support.
///
/// The `invulnerable_until` field is a timestamp (seconds since session start)
/// during which the entity cannot take damage (used for dodge i-frames,
/// spawn protection, etc.).
#[derive(Component, Debug, Clone)]
pub struct Health {
    /// Current health value. Entity dies when this reaches 0.
    pub current: f32,
    /// Maximum health cap — healing cannot exceed this value.
    pub max: f32,
    /// Game time (seconds) before which the entity is invulnerable.
    pub invulnerable_until: f32,
}

impl Health {
    /// Creates a new health pool at full HP with no invulnerability.
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
            invulnerable_until: 0.0,
        }
    }

    /// Returns the fraction of remaining health (0.0–1.0).
    pub fn fraction(&self) -> f32 {
        self.current / self.max
    }

    /// Returns true if the entity is still alive (current > 0).
    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    /// Attempts to apply damage, respecting invulnerability window.
    ///
    /// Returns `true` if damage was applied, `false` if the entity is
    /// currently invulnerable (`time < self.invulnerable_until`).
    pub fn take_damage(&mut self, amount: f32, time: f32) -> bool {
        if time < self.invulnerable_until {
            return false;
        }
        self.current = (self.current - amount).max(0.0);
        true
    }

    /// Heals the entity by the given amount, clamped to max HP.
    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }
}

/// Core combat stats that determine the entity's combat effectiveness.
///
/// These values are modified by equipment, abilities, and meta-progression
/// upgrades. Default values represent an unmodified level-1 character.
#[derive(Component, Debug, Clone)]
pub struct CombatStats {
    /// Flat bonus to all outgoing damage.
    pub damage_bonus: f32,
    /// Percentage bonus to attack speed (0.0–1.0+).
    pub attack_speed_bonus: f32,
    /// Base movement speed in units per second.
    pub move_speed: f32,
    /// Flat damage reduction applied to incoming physical damage.
    pub armor: f32,
    /// Chance to completely dodge an incoming attack (0.0–1.0).
    pub dodge_chance: f32,
    /// Chance for attacks to critically strike (0.0–1.0).
    pub crit_chance: f32,
    /// Critical hit damage multiplier (e.g. 2.0 = double damage).
    pub crit_multiplier: f32,
    /// Radius in world units for auto-pickup of items and XP gems.
    pub pickup_radius: f32,
    /// Bonus maximum health added on top of base HP.
    pub max_health_bonus: f32,
    /// Movement speed modifier (percentage-based).
    pub move_speed_bonus: f32,
    /// Fractional reduction of dash cooldown timer.
    pub dash_cooldown_reduction: f32,
    /// Percentage of outgoing damage returned as healing (0.0–1.0).
    pub lifesteal: f32,
    /// Flat armour penetration (ignores this much target armour).
    pub armor_penetration: f32,
    /// Total damage taken multiplier (1.0 = normal, <1.0 = reduced).
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

/// 3D position for entities in the isometric world.
///
/// Used as the primary spatial component — most entities have one.
#[derive(Component, Debug, Clone)]
pub struct Position(pub Vec3);

impl Default for Position {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}

/// 3D velocity for entity movement. Applied each frame by movement systems.
#[derive(Component, Debug, Clone)]
pub struct Velocity(pub Vec3);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}

/// Rendering information — which model/mesh to use and visual overrides.
#[derive(Component, Debug, Clone)]
pub struct RenderInfo {
    /// Mesh handle for the entity's 3D model (if custom).
    pub mesh_handle: Option<Handle<Mesh>>,
    /// Material handle for the entity's surface appearance.
    pub material_handle: Option<Handle<StandardMaterial>>,
    /// Uniform scale multiplier for the entity's transform.
    pub scale: f32,
    /// Colour tint applied on top of the material.
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

/// Which side an entity belongs to for friendly-fire and targeting.
#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum Team {
    /// Controlled by the player (player character, player projectiles, pets).
    Player,
    /// Hostile to the player (enemies, enemy projectiles, traps).
    Enemy,
    /// Neutral — not attacked by default (NPCs, environmental objects).
    Neutral,
}

// ============================================================================
// Spawning Markers
// ============================================================================

/// Marker for entities that should be despawned when leaving the current room or zone.
///
/// Applied to all gameplay entities spawned within a room (enemies, projectiles,
/// pickups). When the room is exited, all entities with this marker are cleaned up.
#[derive(Component, Debug, Clone)]
pub struct RoomEntity;

/// Marker for the current room or zone entity (used by procedural generation).
#[derive(Component, Debug, Clone)]
pub struct Room;

/// Auto-despawn timer — entity is despawned after the duration elapses.
///
/// Used for temporary VFX, telegraph indicators, and one-frame markers.
#[derive(Component, Debug, Clone)]
pub struct Lifetime {
    /// Remaining seconds before despawn.
    pub remaining: f32,
}

/// Per-enemy cooldown tracking for attacks.
///
/// `timer` counts down from the attack interval; `windup` tracks the
/// telegraph phase before the attack actually fires.
#[derive(Component, Debug, Clone)]
pub struct AttackCooldown {
    /// Cooldown timer (seconds) — attack ready when <= 0.
    pub timer: f32,
    /// Windup timer (seconds) — tracks telegraph phase before attack fires.
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
///
/// Regenerates over time up to `max`. Spending stamina reduces `current`.
#[derive(Component, Debug, Clone)]
pub struct Stamina {
    /// Current stamina value.
    pub current: f32,
    /// Maximum stamina cap.
    pub max: f32,
    /// Stamina regenerated per second.
    pub regen_rate: f32,
    /// Prevents regen for this many seconds after spending stamina (wow-style lockout).
    pub stamina_lockout_timer: f32,
}

impl Default for Stamina {
    fn default() -> Self {
        Self { current: 100.0, max: 100.0, regen_rate: 15.0, stamina_lockout_timer: 0.0 }
    }
}

impl Stamina {
    /// Returns `true` if the entity has at least `amount` stamina.
    pub fn has(&self, amount: f32) -> bool { self.current >= amount }
    /// Reduces stamina by `amount`, clamped at zero, and sets lockout timer.
    pub fn spend(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
        self.stamina_lockout_timer = 1.0;
    }
    /// Reduces stamina by `amount` silently (no lockout timer).
    pub fn spend_silent(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }
    /// Returns the fraction of remaining stamina (0.0–1.0).
    pub fn fraction(&self) -> f32 { self.current / self.max }
}

/// Timer for spawning afterimage trail segments during dash.
/// Starts a 50ms countdown after each trail spawn.
#[derive(Component, Debug, Clone)]
pub struct DashTrailTimer(pub f32);

impl Default for DashTrailTimer {
    fn default() -> Self {
        Self(0.0)
    }
}

/// Dash cooldown component for the player's dodge roll.
#[derive(Component, Debug, Clone)]
pub struct DashCooldown {
    /// Cooldown timer (seconds) — dash is ready when <= 0.
    pub timer: f32,
    /// Whether the dash is currently active (entity is mid-roll).
    pub active: bool,
    /// Duration of the dash roll animation / i-frames in seconds.
    pub duration: f32,
    /// Whether the dash-triggered attack has been fired this dash.
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

/// Separate velocity component for knockback, with natural damping.
///
/// Applied additively on top of normal movement velocity so knockback
/// doesn't override or cancel player input.
#[derive(Component, Debug, Clone)]
pub struct Knockback {
    /// Current knockback velocity vector.
    pub velocity: Vec3,
    /// Deceleration factor (0.0 = no damping, higher = faster stop).
    pub damping: f32,
}

impl Knockback {
    /// Creates a new knockback with the given initial velocity and damping.
    pub fn new(velocity: Vec3, damping: f32) -> Self {
        Self { velocity, damping }
    }
}

// ============================================================================
// Status Effects
// ============================================================================

/// Frozen status — reduces movement speed by 60% for the duration.
///
/// Applied by Frostbolt (Mage secondary ability). Prevents dashing while active.
#[derive(Component, Debug, Clone)]
pub struct Frozen {
    /// Remaining duration in seconds.
    pub remaining: f32,
}

impl Frozen {
    /// Creates a new Frozen effect lasting `duration` seconds.
    pub fn new(duration: f32) -> Self {
        Self { remaining: duration }
    }
}

/// Stun status — prevents movement and actions for the duration.
///
/// Applied by heavy hits (charger, boss, critical hits) and certain abilities.
#[derive(Component, Debug, Clone)]
pub struct Stun {
    /// Remaining duration in seconds.
    pub remaining: f32,
}

impl Stun {
    /// Creates a new Stun effect lasting `duration` seconds.
    pub fn new(duration: f32) -> Self {
        Self { remaining: duration }
    }
}

/// HitStun — brief movement interrupt on damage taken (stagger).
///
/// Freezes the entity's movement velocity for a short duration
/// to provide hit-feedback. Does not prevent actions.
#[derive(Component, Debug, Clone)]
pub struct HitStun {
    /// Remaining duration in seconds.
    pub remaining: f32,
}

impl HitStun {
    /// Creates a new HitStun effect lasting `duration` seconds.
    pub fn new(duration: f32) -> Self {
        Self { remaining: duration }
    }
}

/// HitStop — brief local time freeze on the entity for game feel.
///
/// Pauses the entity's update for a few frames on hit impact,
/// creating a sense of weight and impact.
#[derive(Component, Debug, Clone)]
pub struct HitStop {
    /// Remaining duration in seconds.
    pub remaining: f32,
}

impl HitStop {
    /// Creates a new HitStop effect lasting `duration` seconds.
    pub fn new(duration: f32) -> Self {
        Self { remaining: duration }
    }
}

// ============================================================================
// Telegraph Visual
// ============================================================================

/// Visual telegraph indicator spawned during enemy windup.
///
/// Displays a warning zone on the ground before a telegraphed attack fires.
/// Despawns after `remaining` seconds.
#[derive(Component, Debug, Clone)]
pub struct TelegraphIndicator {
    /// Remaining seconds before the telegraph despawns.
    pub remaining: f32,
    /// Entity that this telegraph belongs to (for chaining to the attack).
    pub target_entity: Entity,
}

impl TelegraphIndicator {
    /// Creates a new telegraph indicator lasting `duration` seconds.
    pub fn new(duration: f32, target: Entity) -> Self {
        Self { remaining: duration, target_entity: target }
    }
}

/// Marker component for magical projectiles (Mage fireballs, frostbolts).
///
/// Used for visual distinction and collision filtering.
#[derive(Component, Debug, Clone)]
pub struct MagicProjectile;

/// Marker component for enemy projectiles, used for visual distinction.
#[derive(Component, Debug, Clone)]
pub struct EnemyProjectileMarker;

/// Marker for dash trail particle entities spawned during the dodge roll.
#[derive(Component, Debug, Clone)]
pub struct DashTrail;

/// HitFlash — brief white emissive overlay when damaged.
/// Applied by damage sources, ticked by tick_hit_flash, visually rendered
/// by the rendering plugin's material swap.
#[derive(Component, Debug, Clone)]
pub struct HitFlash {
    pub remaining: f32,
}

impl HitFlash {
    pub fn new(duration: f32) -> Self {
        Self { remaining: duration }
    }
}

/// Death animation state — scales entity down and despawns.
///
/// When attached, the death animation system scales the entity to zero
/// over `timer` seconds, then despawns it.
#[derive(Component, Debug, Clone)]
pub struct DeathAnimation {
    /// Remaining seconds before despawn.
    pub timer: f32,
    /// The entity's scale when DeathAnimation was inserted.
    pub initial_scale: f32,
}

impl DeathAnimation {
    /// Creates a new death animation with the given duration.
    pub fn new(duration: f32, initial_scale: f32) -> Self {
        Self { timer: duration, initial_scale }
    }
}

/// A marker component for projectile trail segments.
/// Spawned behind projectiles, auto-cleaned by Lifetime.
#[derive(Component, Debug, Clone)]
pub struct TrailSegment;

/// Respawn timer component for dead players awaiting respawn.
///
/// Attached after death in open-world mode. When `remaining` reaches 0,
/// the player is respawned at the nearest graveyard.
#[derive(Component, Debug, Clone)]
pub struct RespawnTimer {
    /// Seconds remaining before respawn.
    pub remaining: f32,
}

impl RespawnTimer {
    /// Creates a new respawn timer with the given wait duration.
    pub fn new(duration: f32) -> Self {
        Self { remaining: duration }
    }
}
