//! Class specialisations ("specs") — each class has two specs with distinct
//! utility/ultimate abilities, talent trees that modify core abilities, and
//! passive stat bonuses.  Icon IDs follow the convention `spec_{class}_{spec}`
//! and `talent_{class}_{spec}_{talent}` so asset naming maps 1:1.
//!
//! # Structure
//!
//! - [`TalentSpec`] — the enum identifying which spec a player chose.
//! - [`SpecDefinition`] — static data for a spec (display name, description,
//!   iconic assets, and a list of talent rows).
//! - [`TalentNode`] — one purchasable talent in a row with ranks and a typed
//!   modifier.
//! - [`TalentModifier`] — what a talent actually changes (stat bonus, ability
//!   override, resource efficiency, cooldown reduction, etc.).
//! - [`all_spec_defs()`] — returns all 10 spec definitions with full talent trees.
//!
//! # Usage
//!
//! ```ignore
//! let def = spec_definition(class, spec);
//! for row in &def.talent_rows { … }
//! ```

use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

/// The two specialisations available to each class.
///
/// Each spec has its own utility ability, ultimate ability, and talent tree.
/// `Default` is the first-listed spec for each class (chosen at character create).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TalentSpec {
    // ── Warrior ──
    /// Dual-wield damage dealer — bleed stacking, adrenaline, fast attacks.
    Berserker,
    /// Durable tank — taunt, shield wall, intercept.
    Protector,
    // ── Paladin ──
    /// Healer / support — holy light mastery, group heals, damage buffs.
    Holy,
    /// Melee DPS — holy damage burst, stuns, righteous fury.
    Retribution,
    // ── Rogue ──
    /// Poison / DoT specialist — deadly venom, ruptures, energy efficiency.
    Assassination,
    /// Mobility / AoE — swordplay, smoke bombs, blade flurry.
    Outlaw,
    // ── Hunter ──
    /// Sniper / precision — aimed shot mastery, vulnerability, rapid fire.
    Marksmanship,
    /// Traps / pets — explosive traps, companion summon, wilderness tactics.
    Survival,
    // ── Mage ──
    /// Control / AoE — blizzard, slows, water elemental.
    Frost,
    /// Burst / crit — ignite, combustion, meteor.
    Fire,
}

impl Default for TalentSpec {
    fn default() -> Self {
        Self::Berserker
    }
}

impl std::fmt::Display for TalentSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Berserker => write!(f, "Berserker"),
            Self::Protector => write!(f, "Protector"),
            Self::Holy => write!(f, "Holy"),
            Self::Retribution => write!(f, "Retribution"),
            Self::Assassination => write!(f, "Assassination"),
            Self::Outlaw => write!(f, "Outlaw"),
            Self::Marksmanship => write!(f, "Marksmanship"),
            Self::Survival => write!(f, "Survival"),
            Self::Frost => write!(f, "Frost"),
            Self::Fire => write!(f, "Fire"),
        }
    }
}

/// Holds the two specs available to a character class, with the one currently
/// chosen by the player.
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ChosenSpec {
    /// The active specialisation.
    pub spec: TalentSpec,
    /// Purchased talent ranks: key = talent `id` string, value = current rank (1-based).
    pub talent_ranks: Vec<TalentRank>,
}

impl Default for ChosenSpec {
    fn default() -> Self {
        Self {
            spec: TalentSpec::default(),
            talent_ranks: Vec::new(),
        }
    }
}

/// A single talent purchase record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentRank {
    pub id: String,
    pub rank: u32,
}

// ── Static Definitions ─────────────────────────────────────────────────

/// Static definition of a class spec — holds display info, talent rows,
/// and passive stat bonuses.
#[derive(Debug, Clone)]
pub struct SpecDefinition {
    pub class_id: &'static str,
    pub spec: TalentSpec,
    pub display_name: &'static str,
    pub description: &'static str,
    /// Icon asset path fragment (e.g. `spec_warrior_berserker`).
    pub icon_id: &'static str,
    /// Passive stat bonuses granted by having this spec active.
    pub passive_stats: &'static [SpecStatBonus],
    /// Talent tree: each `&[TalentNode]` is a row; rows unlock sequentially.
    pub talent_rows: &'static [&'static [TalentNode]],
}

/// Typed stat bonus for a spec's passive or a talent rank.
#[derive(Debug, Clone, Copy)]
pub struct SpecStatBonus {
    pub stat: SpecStatType,
    pub value_per_rank: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecStatType {
    DamageBonus,
    MaxHealth,
    Armor,
    MoveSpeed,
    CritChance,
    CritMultiplier,
    Lifesteal,
    DodgeChance,
    AttackSpeed,
    ResourceRegen,
    ResourceMax,
    CooldownReduction,
    PickupRadius,
}

/// One talent node — a purchasable upgrade with up to `max_rank` ranks.
#[derive(Debug, Clone)]
pub struct TalentNode {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub icon_id: &'static str,
    pub max_rank: u32,
    pub cost_per_rank: u64,
    pub modifier: TalentModifier,
}

/// What a talent point actually changes when purchased.
#[derive(Debug, Clone)]
pub enum TalentModifier {
    /// Direct stat bonus added to CombatStats.
    StatBonus(SpecStatType, f32),
    /// Percentage cooldown reduction for a specific ability slot (0.0‑1.0).
    CooldownReduction(&'static str, f32),
    /// Resource cost multiplier (0.0‑1.0 = cheaper).
    ResourceEfficiency(&'static str, f32),
    /// Overrides a base ability's damage by a percentage multiplier.
    AbilityDamageBonus(&'static str, f32),
    /// Adds a secondary effect to an ability (bleed, slow, stun, etc.).
    AbilityAugment(&'static str, &'static str),
    /// Unlocks a new passive effect by name.
    UnlockPassive(&'static str),
}

// ── All 10 Spec Definitions ─────────────────────────────────────────────

/// Returns the [`SpecDefinition`] for a given class + spec combination.
pub fn spec_definition(class: super::components::CharacterClass, spec: TalentSpec) -> &'static SpecDefinition {
    all_spec_defs()
        .iter()
        .find(|d| d.spec == spec)
        .unwrap_or_else(|| {
            // Fallback: first spec for the class
            all_spec_defs().iter().find(|d| d.class_id == match class {
                super::components::CharacterClass::Warrior => "Warrior",
                super::components::CharacterClass::Paladin => "Paladin",
                super::components::CharacterClass::Rogue => "Rogue",
                super::components::CharacterClass::Hunter => "Hunter",
                super::components::CharacterClass::Mage => "Mage",
            }).unwrap()
        })
}

/// Returns all 10 spec definitions with full talent trees.
pub fn all_spec_defs() -> &'static [SpecDefinition] {
    &SPEC_DEFS
}

const SPEC_DEFS: &[SpecDefinition] = &[
    // ═══════════════════════════════════════════════════════════════════════
    // WARRIOR
    // ═══════════════════════════════════════════════════════════════════════
    SpecDefinition {
        class_id: "Warrior",
        spec: TalentSpec::Berserker,
        display_name: "Berserker",
        description: "Dual-wield fury — stack bleeds, adrenaline-fueled rampage.",
        icon_id: "spec_warrior_berserker",
        passive_stats: &[
            SpecStatBonus { stat: SpecStatType::AttackSpeed, value_per_rank: 0.05 },
            SpecStatBonus { stat: SpecStatType::Lifesteal, value_per_rank: 0.01 },
        ],
        talent_rows: &[
            // Row 1 — Bleed mechanics
            &[
                TalentNode {
                    id: "berserker_deep_wounds",
                    name: "Deep Wounds",
                    description: "Cleave applies a stacking bleed that deals 3% weapon damage per second for 3s.",
                    icon_id: "talent_warrior_berserker_deep_wounds",
                    max_rank: 3,
                    cost_per_rank: 50,
                    modifier: TalentModifier::AbilityAugment("MeleeCleave", "Bleed"),
                },
                TalentNode {
                    id: "berserker_bloodthirst",
                    name: "Bloodthirst",
                    description: "Heal for 2% of damage dealt per rank.",
                    icon_id: "talent_warrior_berserker_bloodthirst",
                    max_rank: 5,
                    cost_per_rank: 30,
                    modifier: TalentModifier::StatBonus(SpecStatType::Lifesteal, 0.02),
                },
                TalentNode {
                    id: "berserker_unyielding",
                    name: "Unyielding",
                    description: "Increase max Rage by 25 per rank.",
                    icon_id: "talent_warrior_berserker_unyielding",
                    max_rank: 2,
                    cost_per_rank: 60,
                    modifier: TalentModifier::StatBonus(SpecStatType::ResourceMax, 25.0),
                },
            ],
            // Row 2 — Adrenaline & speed
            &[
                TalentNode {
                    id: "berserker_adrenaline",
                    name: "Adrenaline Rush",
                    description: "Gain 5 Rage per rank on dodge or taking damage.",
                    icon_id: "talent_warrior_berserker_adrenaline",
                    max_rank: 3,
                    cost_per_rank: 75,
                    modifier: TalentModifier::UnlockPassive("AdrenalineRush"),
                },
                TalentNode {
                    id: "berserker_savage_strikes",
                    name: "Savage Strikes",
                    description: "Cleave deals 8% more damage per rank.",
                    icon_id: "talent_warrior_berserker_savage_strikes",
                    max_rank: 3,
                    cost_per_rank: 60,
                    modifier: TalentModifier::AbilityDamageBonus("MeleeCleave", 0.08),
                },
            ],
            // Row 3 — War Cry enhancements
            &[
                TalentNode {
                    id: "berserker_deafening_roar",
                    name: "Deafening Roar",
                    description: "War Cry also stuns enemies for 1s per rank.",
                    icon_id: "talent_warrior_berserker_deafening_roar",
                    max_rank: 2,
                    cost_per_rank: 100,
                    modifier: TalentModifier::AbilityAugment("WarCry", "Stun"),
                },
                TalentNode {
                    id: "berserker_battle_fury",
                    name: "Battle Fury",
                    description: "War Cry grants 10% attack speed for 5s per rank.",
                    icon_id: "talent_warrior_berserker_battle_fury",
                    max_rank: 3,
                    cost_per_rank: 80,
                    modifier: TalentModifier::AbilityAugment("WarCry", "AttackSpeedBuff"),
                },
            ],
        ],
    },
    SpecDefinition {
        class_id: "Warrior",
        spec: TalentSpec::Protector,
        display_name: "Protector",
        description: "Immovable fortress — taunt, shield wall, and vengeance.",
        icon_id: "spec_warrior_protector",
        passive_stats: &[
            SpecStatBonus { stat: SpecStatType::Armor, value_per_rank: 5.0 },
            SpecStatBonus { stat: SpecStatType::MaxHealth, value_per_rank: 15.0 },
        ],
        talent_rows: &[
            // Row 1 — Block & armor
            &[
                TalentNode {
                    id: "protector_bulwark",
                    name: "Bulwark",
                    description: "Increase armor by 6 per rank.",
                    icon_id: "talent_warrior_protector_bulwark",
                    max_rank: 5,
                    cost_per_rank: 30,
                    modifier: TalentModifier::StatBonus(SpecStatType::Armor, 6.0),
                },
                TalentNode {
                    id: "protector_vengeance",
                    name: "Vengeance",
                    description: "Gain 3 Rage per rank when blocking damage.",
                    icon_id: "talent_warrior_protector_vengeance",
                    max_rank: 3,
                    cost_per_rank: 50,
                    modifier: TalentModifier::UnlockPassive("Vengeance"),
                },
                TalentNode {
                    id: "protector_toughness",
                    name: "Toughness",
                    description: "Increase max HP by 20 per rank.",
                    icon_id: "talent_warrior_protector_toughness",
                    max_rank: 5,
                    cost_per_rank: 25,
                    modifier: TalentModifier::StatBonus(SpecStatType::MaxHealth, 20.0),
                },
            ],
            // Row 2 — Taunt enhancements
            &[
                TalentNode {
                    id: "protector_challenging_shout",
                    name: "Challenging Shout",
                    description: "Taunt also reduces target damage by 10% per rank for 3s.",
                    icon_id: "talent_warrior_protector_challenging_shout",
                    max_rank: 3,
                    cost_per_rank: 80,
                    modifier: TalentModifier::AbilityAugment("Taunt", "DamageDebuff"),
                },
                TalentNode {
                    id: "protector_intervene",
                    name: "Intervene",
                    description: "Take 10% reduced damage per rank from taunted enemies.",
                    icon_id: "talent_warrior_protector_intervene",
                    max_rank: 3,
                    cost_per_rank: 75,
                    modifier: TalentModifier::StatBonus(SpecStatType::Armor, 8.0),
                },
            ],
            // Row 3 — Shield Wall
            &[
                TalentNode {
                    id: "protector_iron_wall",
                    name: "Iron Wall",
                    description: "Shield Wall lasts 1s longer per rank.",
                    icon_id: "talent_warrior_protector_iron_wall",
                    max_rank: 2,
                    cost_per_rank: 100,
                    modifier: TalentModifier::AbilityAugment("ShieldWall", "ExtendedDuration"),
                },
                TalentNode {
                    id: "protector_fortress",
                    name: "Fortress",
                    description: "Shield Wall heals 5% max HP per rank over its duration.",
                    icon_id: "talent_warrior_protector_fortress",
                    max_rank: 2,
                    cost_per_rank: 120,
                    modifier: TalentModifier::AbilityAugment("ShieldWall", "HealOverTime"),
                },
            ],
        ],
    },
    // ═══════════════════════════════════════════════════════════════════════
    // PALADIN
    // ═══════════════════════════════════════════════════════════════════════
    SpecDefinition {
        class_id: "Paladin",
        spec: TalentSpec::Holy,
        display_name: "Holy",
        description: "Radiant healer — consecrated light, blessings, and divine intervention.",
        icon_id: "spec_paladin_holy",
        passive_stats: &[
            SpecStatBonus { stat: SpecStatType::MaxHealth, value_per_rank: 10.0 },
            SpecStatBonus { stat: SpecStatType::Lifesteal, value_per_rank: 0.02 },
        ],
        talent_rows: &[
            &[
                TalentNode {
                    id: "holy_sanctified",
                    name: "Sanctified",
                    description: "Holy Power regen rate +0.2 per rank.",
                    icon_id: "talent_paladin_holy_sanctified",
                    max_rank: 3,
                    cost_per_rank: 50,
                    modifier: TalentModifier::StatBonus(SpecStatType::ResourceRegen, 0.2),
                },
                TalentNode {
                    id: "holy_beacon",
                    name: "Beacon of Light",
                    description: "Holy Light also heals a second target for 30% per rank.",
                    icon_id: "talent_paladin_holy_beacon",
                    max_rank: 2,
                    cost_per_rank: 80,
                    modifier: TalentModifier::AbilityAugment("HolyLight", "BeaconHeal"),
                },
            ],
            &[
                TalentNode {
                    id: "holy_aura_mastery",
                    name: "Aura Mastery",
                    description: "Nearby allies deal 4% more damage per rank.",
                    icon_id: "talent_paladin_holy_aura_mastery",
                    max_rank: 3,
                    cost_per_rank: 75,
                    modifier: TalentModifier::UnlockPassive("AuraMastery"),
                },
                TalentNode {
                    id: "holy_light_charge",
                    name: "Light's Charge",
                    description: "Righteous Strike generates 1 extra Holy Power per rank.",
                    icon_id: "talent_paladin_holy_light_charge",
                    max_rank: 2,
                    cost_per_rank: 60,
                    modifier: TalentModifier::AbilityAugment("RighteousStrike", "BonusHolyPower"),
                },
            ],
            &[
                TalentNode {
                    id: "holy_radiant_blessing",
                    name: "Radiant Blessing",
                    description: "Blessing of Might also grants 5% max HP per rank.",
                    icon_id: "talent_paladin_holy_radiant_blessing",
                    max_rank: 2,
                    cost_per_rank: 100,
                    modifier: TalentModifier::AbilityAugment("BlessingOfMight", "BonusMaxHP"),
                },
                TalentNode {
                    id: "holy_divine_mercy",
                    name: "Divine Mercy",
                    description: "Divine Intervention cooldown reduced by 15% per rank.",
                    icon_id: "talent_paladin_holy_divine_mercy",
                    max_rank: 3,
                    cost_per_rank: 90,
                    modifier: TalentModifier::CooldownReduction("DivineIntervention", 0.15),
                },
            ],
        ],
    },
    SpecDefinition {
        class_id: "Paladin",
        spec: TalentSpec::Retribution,
        display_name: "Retribution",
        description: "Wrathful crusader — holy damage, hammer stuns, avenging fury.",
        icon_id: "spec_paladin_retribution",
        passive_stats: &[
            SpecStatBonus { stat: SpecStatType::DamageBonus, value_per_rank: 2.0 },
            SpecStatBonus { stat: SpecStatType::CritChance, value_per_rank: 0.01 },
        ],
        talent_rows: &[
            &[
                TalentNode {
                    id: "retribution_crusader_strike",
                    name: "Crusader Strike",
                    description: "Righteous Strike generates 1 extra Holy Power per rank.",
                    icon_id: "talent_paladin_retribution_crusader_strike",
                    max_rank: 2,
                    cost_per_rank: 50,
                    modifier: TalentModifier::AbilityAugment("RighteousStrike", "BonusHolyPower"),
                },
                TalentNode {
                    id: "retribution_zeal",
                    name: "Zeal",
                    description: "Attack speed +4% per rank.",
                    icon_id: "talent_paladin_retribution_zeal",
                    max_rank: 3,
                    cost_per_rank: 40,
                    modifier: TalentModifier::StatBonus(SpecStatType::AttackSpeed, 0.04),
                },
            ],
            &[
                TalentNode {
                    id: "retribution_righteous_fury",
                    name: "Righteous Fury",
                    description: "Crit chance +3% per rank on Holy damage.",
                    icon_id: "talent_paladin_retribution_righteous_fury",
                    max_rank: 3,
                    cost_per_rank: 65,
                    modifier: TalentModifier::StatBonus(SpecStatType::CritChance, 0.03),
                },
                TalentNode {
                    id: "retribution_seal_of_might",
                    name: "Seal of Might",
                    description: "Consecration deals 15% more damage per rank.",
                    icon_id: "talent_paladin_retribution_seal_of_might",
                    max_rank: 3,
                    cost_per_rank: 60,
                    modifier: TalentModifier::AbilityDamageBonus("Consecration", 0.15),
                },
            ],
            &[
                TalentNode {
                    id: "retribution_avenging_light",
                    name: "Avenging Light",
                    description: "Avenging Wrath deals 20% more damage per rank.",
                    icon_id: "talent_paladin_retribution_avenging_light",
                    max_rank: 2,
                    cost_per_rank: 100,
                    modifier: TalentModifier::AbilityDamageBonus("AvengingWrath", 0.20),
                },
                TalentNode {
                    id: "retribution_holy_wrath",
                    name: "Holy Wrath",
                    description: "Hammer of Justice stun duration +0.5s per rank.",
                    icon_id: "talent_paladin_retribution_holy_wrath",
                    max_rank: 2,
                    cost_per_rank: 80,
                    modifier: TalentModifier::AbilityAugment("HammerOfJustice", "ExtendedStun"),
                },
            ],
        ],
    },
    // ═══════════════════════════════════════════════════════════════════════
    // ROGUE
    // ═══════════════════════════════════════════════════════════════════════
    SpecDefinition {
        class_id: "Rogue",
        spec: TalentSpec::Assassination,
        display_name: "Assassination",
        description: "Shadowy poisoner — deadly toxins, deep cuts, and venomous strikes.",
        icon_id: "spec_rogue_assassination",
        passive_stats: &[
            SpecStatBonus { stat: SpecStatType::CritMultiplier, value_per_rank: 0.1 },
            SpecStatBonus { stat: SpecStatType::DamageBonus, value_per_rank: 1.0 },
        ],
        talent_rows: &[
            &[
                TalentNode {
                    id: "assassination_hemorrhage",
                    name: "Hemorrhage",
                    description: "Backstab causes target to bleed for 20% of damage over 3s per rank.",
                    icon_id: "talent_rogue_assassination_hemorrhage",
                    max_rank: 3,
                    cost_per_rank: 50,
                    modifier: TalentModifier::AbilityAugment("Backstab", "Bleed"),
                },
                TalentNode {
                    id: "assassination_venomous_wounds",
                    name: "Venomous Wounds",
                    description: "Attacks on poisoned targets restore 3 Energy per rank.",
                    icon_id: "talent_rogue_assassination_venomous_wounds",
                    max_rank: 3,
                    cost_per_rank: 55,
                    modifier: TalentModifier::UnlockPassive("VenomousWounds"),
                },
            ],
            &[
                TalentNode {
                    id: "assassination_master_poisoner",
                    name: "Master Poisoner",
                    description: "Poison Blade stack limit +2 per rank and duration +1s.",
                    icon_id: "talent_rogue_assassination_master_poisoner",
                    max_rank: 2,
                    cost_per_rank: 75,
                    modifier: TalentModifier::AbilityAugment("PoisonBlade", "ExtendedPoison"),
                },
                TalentNode {
                    id: "assassination_lethal_dose",
                    name: "Lethal Dose",
                    description: "Deadly Poison deals 15% more damage per rank.",
                    icon_id: "talent_rogue_assassination_lethal_dose",
                    max_rank: 3,
                    cost_per_rank: 60,
                    modifier: TalentModifier::AbilityDamageBonus("DeadlyPoison", 0.15),
                },
            ],
            &[
                TalentNode {
                    id: "assassination_rupture",
                    name: "Rupture",
                    description: "Rupture damage +25% per rank.",
                    icon_id: "talent_rogue_assassination_rupture",
                    max_rank: 2,
                    cost_per_rank: 100,
                    modifier: TalentModifier::AbilityDamageBonus("Rupture", 0.25),
                },
                TalentNode {
                    id: "assassination_kingsbane",
                    name: "Kingsbane",
                    description: "Rupture also reduces target armor by 10% per rank.",
                    icon_id: "talent_rogue_assassination_kingsbane",
                    max_rank: 2,
                    cost_per_rank: 120,
                    modifier: TalentModifier::AbilityAugment("Rupture", "ArmorShred"),
                },
            ],
        ],
    },
    SpecDefinition {
        class_id: "Rogue",
        spec: TalentSpec::Outlaw,
        display_name: "Outlaw",
        description: "Swashbuckling brawler — smoke, steel, and dirty tricks.",
        icon_id: "spec_rogue_outlaw",
        passive_stats: &[
            SpecStatBonus { stat: SpecStatType::DodgeChance, value_per_rank: 0.02 },
            SpecStatBonus { stat: SpecStatType::MoveSpeed, value_per_rank: 0.3 },
        ],
        talent_rows: &[
            &[
                TalentNode {
                    id: "outlaw_fleet_footed",
                    name: "Fleet-Footed",
                    description: "Dodge chance +3% per rank.",
                    icon_id: "talent_rogue_outlaw_fleet_footed",
                    max_rank: 3,
                    cost_per_rank: 40,
                    modifier: TalentModifier::StatBonus(SpecStatType::DodgeChance, 0.03),
                },
                TalentNode {
                    id: "outlaw_dirty_tricks",
                    name: "Dirty Tricks",
                    description: "Backstab also interrupts enemy attacks per rank (higher rank = longer lockout).",
                    icon_id: "talent_rogue_outlaw_dirty_tricks",
                    max_rank: 2,
                    cost_per_rank: 60,
                    modifier: TalentModifier::AbilityAugment("Backstab", "Interrupt"),
                },
            ],
            &[
                TalentNode {
                    id: "outlaw_adrenaline_rush",
                    name: "Adrenaline Rush",
                    description: "Energy regen +5/sec per rank.",
                    icon_id: "talent_rogue_outlaw_adrenaline_rush",
                    max_rank: 3,
                    cost_per_rank: 65,
                    modifier: TalentModifier::StatBonus(SpecStatType::ResourceRegen, 5.0),
                },
                TalentNode {
                    id: "outlaw_swordplay",
                    name: "Swordplay",
                    description: "Backstab damage +10% per rank.",
                    icon_id: "talent_rogue_outlaw_swordplay",
                    max_rank: 3,
                    cost_per_rank: 50,
                    modifier: TalentModifier::AbilityDamageBonus("Backstab", 0.10),
                },
            ],
            &[
                TalentNode {
                    id: "outlaw_blinding_powder",
                    name: "Blinding Powder",
                    description: "Smoke Bomb also blinds enemies for 1s per rank.",
                    icon_id: "talent_rogue_outlaw_blinding_powder",
                    max_rank: 2,
                    cost_per_rank: 100,
                    modifier: TalentModifier::AbilityAugment("SmokeBomb", "Blind"),
                },
                TalentNode {
                    id: "outlaw_blade_dancer",
                    name: "Blade Dancer",
                    description: "Blade Flurry deals 20% more damage per rank.",
                    icon_id: "talent_rogue_outlaw_blade_dancer",
                    max_rank: 2,
                    cost_per_rank: 110,
                    modifier: TalentModifier::AbilityDamageBonus("BladeFlurry", 0.20),
                },
            ],
        ],
    },
    // ═══════════════════════════════════════════════════════════════════════
    // HUNTER
    // ═══════════════════════════════════════════════════════════════════════
    SpecDefinition {
        class_id: "Hunter",
        spec: TalentSpec::Marksmanship,
        display_name: "Marksmanship",
        description: "Deadly sniper — precision shots, hunter's mark, and rapid volleys.",
        icon_id: "spec_hunter_marksmanship",
        passive_stats: &[
            SpecStatBonus { stat: SpecStatType::CritMultiplier, value_per_rank: 0.15 },
            SpecStatBonus { stat: SpecStatType::DamageBonus, value_per_rank: 2.0 },
        ],
        talent_rows: &[
            &[
                TalentNode {
                    id: "marksmanship_precision",
                    name: "Precision",
                    description: "Crit multiplier +0.2 per rank.",
                    icon_id: "talent_hunter_marksmanship_precision",
                    max_rank: 3,
                    cost_per_rank: 50,
                    modifier: TalentModifier::StatBonus(SpecStatType::CritMultiplier, 0.2),
                },
                TalentNode {
                    id: "marksmanship_long_shot",
                    name: "Long Shot",
                    description: "Aimed Shot range +15% per rank.",
                    icon_id: "talent_hunter_marksmanship_long_shot",
                    max_rank: 2,
                    cost_per_rank: 40,
                    modifier: TalentModifier::AbilityAugment("AimedShot", "ExtendedRange"),
                },
            ],
            &[
                TalentNode {
                    id: "marksmanship_steady_aim",
                    name: "Steady Aim",
                    description: "Standing still for 1s grants 10% damage bonus per rank on next Aimed Shot.",
                    icon_id: "talent_hunter_marksmanship_steady_aim",
                    max_rank: 3,
                    cost_per_rank: 65,
                    modifier: TalentModifier::UnlockPassive("SteadyAim"),
                },
                TalentNode {
                    id: "marksmanship_piercing_shots",
                    name: "Piercing Shots",
                    description: "Aimed Shot ignores 10% armor per rank.",
                    icon_id: "talent_hunter_marksmanship_piercing_shots",
                    max_rank: 3,
                    cost_per_rank: 60,
                    modifier: TalentModifier::AbilityAugment("AimedShot", "ArmorPiercing"),
                },
            ],
            &[
                TalentNode {
                    id: "marksmanship_deadly_mark",
                    name: "Deadly Mark",
                    description: "Hunter's Mark increases damage taken by 8% per rank.",
                    icon_id: "talent_hunter_marksmanship_deadly_mark",
                    max_rank: 3,
                    cost_per_rank: 90,
                    modifier: TalentModifier::AbilityAugment("HuntersMark", "IncreasedVulnerability"),
                },
                TalentNode {
                    id: "marksmanship_barrage",
                    name: "Barrage",
                    description: "Rapid Fire deals 15% more damage per rank.",
                    icon_id: "talent_hunter_marksmanship_barrage",
                    max_rank: 2,
                    cost_per_rank: 100,
                    modifier: TalentModifier::AbilityDamageBonus("RapidFire", 0.15),
                },
            ],
        ],
    },
    SpecDefinition {
        class_id: "Hunter",
        spec: TalentSpec::Survival,
        display_name: "Survival",
        description: "Wilderness expert — traps, pets, and adaptive tactics.",
        icon_id: "spec_hunter_survival",
        passive_stats: &[
            SpecStatBonus { stat: SpecStatType::MaxHealth, value_per_rank: 10.0 },
            SpecStatBonus { stat: SpecStatType::PickupRadius, value_per_rank: 0.5 },
        ],
        talent_rows: &[
            &[
                TalentNode {
                    id: "survival_trap_mastery",
                    name: "Trap Mastery",
                    description: "Trap slow amount +10% per rank and duration +1s.",
                    icon_id: "talent_hunter_survival_trap_mastery",
                    max_rank: 3,
                    cost_per_rank: 50,
                    modifier: TalentModifier::AbilityAugment("Trap", "ImprovedSlow"),
                },
                TalentNode {
                    id: "survival_wilderness_tactics",
                    name: "Wilderness Tactics",
                    description: "Focus cost of all abilities reduced by 5% per rank.",
                    icon_id: "talent_hunter_survival_wilderness_tactics",
                    max_rank: 3,
                    cost_per_rank: 40,
                    modifier: TalentModifier::ResourceEfficiency("all", 0.05),
                },
            ],
            &[
                TalentNode {
                    id: "survival_thick_hide",
                    name: "Thick Hide",
                    description: "Armor +5 per rank.",
                    icon_id: "talent_hunter_survival_thick_hide",
                    max_rank: 3,
                    cost_per_rank: 45,
                    modifier: TalentModifier::StatBonus(SpecStatType::Armor, 5.0),
                },
                TalentNode {
                    id: "survival_loyal_companion",
                    name: "Loyal Companion",
                    description: "Call Pet cooldown reduced by 15% per rank.",
                    icon_id: "talent_hunter_survival_loyal_companion",
                    max_rank: 2,
                    cost_per_rank: 70,
                    modifier: TalentModifier::CooldownReduction("CallPet", 0.15),
                },
            ],
            &[
                TalentNode {
                    id: "survival_enormous_trap",
                    name: "Enormous Trap",
                    description: "Explosive Trap radius +20% per rank.",
                    icon_id: "talent_hunter_survival_enormous_trap",
                    max_rank: 2,
                    cost_per_rank: 100,
                    modifier: TalentModifier::AbilityAugment("ExplosiveTrap", "IncreasedRadius"),
                },
                TalentNode {
                    id: "survival_concussive",
                    name: "Concussive",
                    description: "Explosive Trap also knocks back enemies per rank.",
                    icon_id: "talent_hunter_survival_concussive",
                    max_rank: 2,
                    cost_per_rank: 90,
                    modifier: TalentModifier::AbilityAugment("ExplosiveTrap", "Knockback"),
                },
            ],
        ],
    },
    // ═══════════════════════════════════════════════════════════════════════
    // MAGE
    // ═══════════════════════════════════════════════════════════════════════
    SpecDefinition {
        class_id: "Mage",
        spec: TalentSpec::Frost,
        display_name: "Frost",
        description: "Chilling controller — blizzards, frozen debuffs, and elemental allies.",
        icon_id: "spec_mage_frost",
        passive_stats: &[
            SpecStatBonus { stat: SpecStatType::MaxHealth, value_per_rank: 5.0 },
            SpecStatBonus { stat: SpecStatType::CooldownReduction, value_per_rank: 0.03 },
        ],
        talent_rows: &[
            &[
                TalentNode {
                    id: "frost_chill_effects",
                    name: "Chill Effects",
                    description: "Frostbolt slow amount +10% per rank and duration +1s.",
                    icon_id: "talent_mage_frost_chill_effects",
                    max_rank: 3,
                    cost_per_rank: 50,
                    modifier: TalentModifier::AbilityAugment("Frostbolt", "ImprovedSlow"),
                },
                TalentNode {
                    id: "frost_ice_barrier",
                    name: "Ice Barrier",
                    description: "Gain a shield equal to 5% max HP per rank after casting Frostbolt.",
                    icon_id: "talent_mage_frost_ice_barrier",
                    max_rank: 3,
                    cost_per_rank: 55,
                    modifier: TalentModifier::UnlockPassive("IceBarrier"),
                },
            ],
            &[
                TalentNode {
                    id: "frost_mana_efficiency",
                    name: "Mana Efficiency",
                    description: "All Mana costs reduced by 6% per rank.",
                    icon_id: "talent_mage_frost_mana_efficiency",
                    max_rank: 3,
                    cost_per_rank: 40,
                    modifier: TalentModifier::ResourceEfficiency("all", 0.06),
                },
                TalentNode {
                    id: "frost_frozen_core",
                    name: "Frozen Core",
                    description: "Frostbolt damage +12% per rank.",
                    icon_id: "talent_mage_frost_frozen_core",
                    max_rank: 3,
                    cost_per_rank: 60,
                    modifier: TalentModifier::AbilityDamageBonus("Frostbolt", 0.12),
                },
            ],
            &[
                TalentNode {
                    id: "frost_arctic_winds",
                    name: "Arctic Winds",
                    description: "Blizzard radius +20% per rank and slow increased.",
                    icon_id: "talent_mage_frost_arctic_winds",
                    max_rank: 2,
                    cost_per_rank: 90,
                    modifier: TalentModifier::AbilityAugment("Blizzard", "ExpandedArea"),
                },
                TalentNode {
                    id: "frost_elemental_power",
                    name: "Elemental Power",
                    description: "Water Elemental deals 20% more damage per rank.",
                    icon_id: "talent_mage_frost_elemental_power",
                    max_rank: 2,
                    cost_per_rank: 100,
                    modifier: TalentModifier::AbilityDamageBonus("WaterElemental", 0.20),
                },
            ],
        ],
    },
    SpecDefinition {
        class_id: "Mage",
        spec: TalentSpec::Fire,
        display_name: "Fire",
        description: "Pyromaniac inferno — ignite, critical mass, and devastating meteors.",
        icon_id: "spec_mage_fire",
        passive_stats: &[
            SpecStatBonus { stat: SpecStatType::CritChance, value_per_rank: 0.02 },
            SpecStatBonus { stat: SpecStatType::DamageBonus, value_per_rank: 3.0 },
        ],
        talent_rows: &[
            &[
                TalentNode {
                    id: "fire_ignite",
                    name: "Ignite",
                    description: "Fireball applies a burn dealing 8% of damage per rank over 3s.",
                    icon_id: "talent_mage_fire_ignite",
                    max_rank: 3,
                    cost_per_rank: 50,
                    modifier: TalentModifier::AbilityAugment("Fireball", "Burn"),
                },
                TalentNode {
                    id: "fire_critical_mass",
                    name: "Critical Mass",
                    description: "Crit chance +4% per rank on Fire spells.",
                    icon_id: "talent_mage_fire_critical_mass",
                    max_rank: 3,
                    cost_per_rank: 55,
                    modifier: TalentModifier::StatBonus(SpecStatType::CritChance, 0.04),
                },
            ],
            &[
                TalentNode {
                    id: "fire_pyromaniac",
                    name: "Pyromaniac",
                    description: "Killing a burning enemy restores 10 Mana per rank.",
                    icon_id: "talent_mage_fire_pyromaniac",
                    max_rank: 3,
                    cost_per_rank: 60,
                    modifier: TalentModifier::UnlockPassive("Pyromaniac"),
                },
                TalentNode {
                    id: "fire_combustion_mastery",
                    name: "Combustion Mastery",
                    description: "Combustion spell power bonus +15% per rank.",
                    icon_id: "talent_mage_fire_combustion_mastery",
                    max_rank: 2,
                    cost_per_rank: 80,
                    modifier: TalentModifier::AbilityDamageBonus("Combustion", 0.15),
                },
            ],
            &[
                TalentNode {
                    id: "fire_meteor_storm",
                    name: "Meteor Storm",
                    description: "Meteor radius +20% per rank.",
                    icon_id: "talent_mage_fire_meteor_storm",
                    max_rank: 2,
                    cost_per_rank: 110,
                    modifier: TalentModifier::AbilityAugment("Meteor", "IncreasedRadius"),
                },
                TalentNode {
                    id: "fire_burnout",
                    name: "Burnout",
                    description: "Fireball mana cost reduced by 15% per rank.",
                    icon_id: "talent_mage_fire_burnout",
                    max_rank: 2,
                    cost_per_rank: 70,
                    modifier: TalentModifier::ResourceEfficiency("Fireball", 0.15),
                },
            ],
        ],
    },
];
