//! Hunter class — aimed shot, multi-shot, trap, disengage. Resource: Focus (medium regen, spent on abilities).

pub mod abilities;
pub mod auto_attack;
pub mod passives;
pub use abilities::*;
pub use auto_attack::*;

pub fn resource_config() -> ir_core::ClassResource {
    ir_core::ClassResource::new(100.0, 8.0)
}
