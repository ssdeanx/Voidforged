//! Warrior class — melee cleave, shield block, charge, combat roll.
//! Resource: Rage (generated on dealing/taking damage).
//!
//! Modular structure: abilities, passives, and auto-attack in sub-modules.

pub mod abilities;
pub mod auto_attack;
pub mod passives;

pub use abilities::*;
pub use auto_attack::*;
pub use passives::*;

/// Resource config: 100 Rage, regen 2/sec
pub fn resource_config() -> ir_core::ClassResource {
    ir_core::ClassResource::new(100.0, 2.0)
}
