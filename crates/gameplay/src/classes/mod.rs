//! Class-based abilities — modular per-class implementations.
//!
//! Each class module exports ability system functions that the central
//! dispatcher in `abilities.rs` routes to based on `PlayerClass`.

pub mod abilities;
pub mod hunter;
pub mod mage;
pub mod paladin;
pub mod rogue;
pub mod warrior;

pub use abilities::{
<<<<<<< HEAD
    cast_ability, class_resource_regen, dash_ability, primary_attack,
    secondary_attack, tick_ability_cooldowns,
=======
    cast_ability, class_resource_regen, dash_ability, primary_attack, secondary_attack,
    ClassResource,
>>>>>>> origin/master
};
