//! Mage class — fireball, frostbolt, arcane blast, blink.
//! Resource: Mana (large pool, slow regen).

pub mod abilities;
pub mod auto_attack;
pub mod passives;
pub use abilities::*;
pub use auto_attack::*;

pub fn resource_config() -> ir_core::ClassResource {
    ir_core::ClassResource::new(200.0, 4.0)
}
