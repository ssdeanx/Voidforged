//! Spec-specific ability rotation definitions.
//!
//! Each spec gets a priority-ordered rotation that defines which abilities
//! to use in what order, based on resource levels, buff uptime, and combat
//! state. The `AbilityRotation` component holds the current rotation state
//! for each entity (player or AI companion).

use bevy::prelude::*;
use ir_core::*;

// ── Rotation step types ────────────────────────────────────────────────────

/// Describes one step in an ability rotation.
pub enum RotationStep {
    /// Fire the primary attack.
    Primary,
    /// Fire the secondary attack.
    Secondary,
    /// Fire the cast (utility) ability.
    Cast,
    /// Fire the ultimate ability.
    Ultimate,
    /// Dash (mobility).
    Dash,
    /// Auto-attack (weapon swing).
    AutoAttack,
}

/// A priority-ordered rotation for a specific spec.
/// The AI or auto-rotation system iterates through steps in order
/// and executes the first one whose conditions are met.
pub struct SpecRotation {
    pub name: &'static str,
    pub steps: &'static [RotationStep],
}

// ── Component: Tracks current rotation state ───────────────────────────────

/// Component attached to players and AI companions.
/// Holds the rotation queue and internal timers.
#[derive(Component, Debug)]
pub struct RotationState {
    /// Whether auto-rotation is enabled (for AI or optional player assist).
    pub enabled: bool,
    /// Index into the current rotation's steps list.
    pub step_index: usize,
    /// Timer before advancing to the next step.
    pub step_timer: f32,
}

impl Default for RotationState {
    fn default() -> Self {
        Self {
            enabled: false,
            step_index: 0,
            step_timer: 0.0,
        }
    }
}

impl RotationState {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, ..default() }
    }
}

// ── Spec rotation definitions ──────────────────────────────────────────────

/// Returns the rotation for a given spec.
/// Each rotation is a priority list; the agent executes the first usable step.
pub fn rotation_for_spec(spec: TalentSpec) -> SpecRotation {
    use RotationStep::*;
    match spec {
        // ── Warrior ──
        TalentSpec::Berserker => SpecRotation {
            name: "Berserker Frenzy",
            steps: &[
                Cast,    // Charge to close gap
                Secondary, // Shield block / shout
                Primary, // Cleave spam (rage builder + AoE)
                Cast,    // Charge again
                Primary, // Cleave
                Ultimate, // Berserker rage burst
                Primary, // Cleave finisher
            ],
        },
        TalentSpec::Protector => SpecRotation {
            name: "Protector Bastion",
            steps: &[
                Cast,    // Taunt / intercept
                Secondary, // Shield block
                Primary, // Cleave (threat builder)
                Secondary, // Shield block
                Primary, // Cleave
                Cast,    // Taunt refresh
                Ultimate, // Shield wall
                Primary, // Cleave sustain
            ],
        },
        // ── Paladin ──
        TalentSpec::Holy => SpecRotation {
            name: "Holy Radiance",
            steps: &[
                Secondary, // Holy light (heal)
                Cast,    // Consecration (group heal / buff)
                Primary, // Righteous strike (holy power builder)
                Secondary, // Holy light
                Primary, // Righteous strike
                Ultimate, // Divine intervention
                Secondary, // Holy light sustain
            ],
        },
        TalentSpec::Retribution => SpecRotation {
            name: "Righteous Fury",
            steps: &[
                Cast,    // Consecration (AoE holy damage)
                Primary, // Righteous strike
                Secondary, // Holy light (self-heal)
                Cast,    // Consecration refresh
                Primary, // Righteous strike
                Ultimate, // Avenging wrath
                Primary, // Righteous strike burst
            ],
        },
        // ── Rogue ──
        TalentSpec::Assassination => SpecRotation {
            name: "Shadow Venom",
            steps: &[
                Cast,    // Vanish (stealth)
                Primary, // Backstab (opener bonus)
                Secondary, // Poison blade (DoT)
                Primary, // Backstab (builder)
                Secondary, // Poison blade refresh
                Primary, // Backstab spam
                Ultimate, // Death mark (execute)
                Cast,    // Vanish escape
            ],
        },
        TalentSpec::Outlaw => SpecRotation {
            name: "Blade Flurry",
            steps: &[
                Cast,    // Vanish (stealth)
                Primary, // Backstab (opener)
                Secondary, // Poison blade
                Cast,    // Vanish (reposition)
                Primary, // Backstab
                Ultimate, // Blade flurry (AoE burst)
                Primary, // Backstab cleanup
            ],
        },
        // ── Hunter ──
        TalentSpec::Marksmanship => SpecRotation {
            name: "Precision Aim",
            steps: &[
                Cast,    // Trap (snare / open)
                Secondary, // Multi shot (vuln builder)
                Primary, // Aimed shot (big damage)
                Secondary, // Multi shot refresh
                Primary, // Aimed shot
                Ultimate, // Rapid fire burst
                Primary, // Aimed shot finisher
            ],
        },
        TalentSpec::Survival => SpecRotation {
            name: "Wild Tactics",
            steps: &[
                Cast,    // Trap (CC)
                Secondary, // Multi shot
                Cast,    // Trap refresh
                Primary, // Aimed shot
                Secondary, // Multi shot
                Ultimate, // Wild call (pet / companion)
                Primary, // Aimed shot sustain
            ],
        },
        // ── Mage ──
        TalentSpec::Frost => SpecRotation {
            name: "Frozen Dominion",
            steps: &[
                Secondary, // Frostbolt (slow)
                Primary, // Fireball (damage)
                Secondary, // Frostbolt refresh
                Cast,    // Arcane blast (AoE burst)
                Primary, // Fireball
                Ultimate, // Blizzard (massive AoE)
                Primary, // Fireball cleanup
            ],
        },
        TalentSpec::Fire => SpecRotation {
            name: "Inferno Burst",
            steps: &[
                Cast,    // Arcane blast (AoE)
                Primary, // Fireball (ignite builder)
                Cast,    // Arcane blast refresh
                Primary, // Fireball
                Ultimate, // Meteor (massive burst)
                Primary, // Fireball spam
            ],
        },
    }
}

/// Returns a human-readable description of a step.
pub fn step_name(step: &RotationStep) -> &'static str {
    match step {
        RotationStep::Primary => "Primary",
        RotationStep::Secondary => "Secondary",
        RotationStep::Cast => "Cast",
        RotationStep::Ultimate => "Ultimate",
        RotationStep::Dash => "Dash",
        RotationStep::AutoAttack => "Auto Attack",
    }
}
