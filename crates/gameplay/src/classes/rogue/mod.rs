//! Rogue class — backstab, poison blade, vanish, shadowstep.
//! Resource: Energy (fast regen, capped pool).

pub mod abilities;
pub mod auto_attack;
pub mod passives;
pub use abilities::*;
pub use auto_attack::*;

pub fn resource_config() -> ir_core::ClassResource {
    ir_core::ClassResource::new(100.0, 20.0)
}
