//! Class-based abilities — modular per-class implementations.
//!
//! Each class module exports ability system functions that the central
//! dispatcher in `abilities.rs` routes to based on `PlayerClass`.

pub mod abilities;
pub mod auto_attack;
pub mod ability_ranks;
pub mod buffs;
pub mod rotation;
pub mod hunter;
pub mod mage;
pub mod paladin;
pub mod rogue;
pub mod warrior;

pub use abilities::{
    cast_ability, class_resource_regen, dash_ability, primary_attack,
    secondary_attack, tick_ability_cooldowns, utility_ability, ultimate_ability,
};
pub use auto_attack::AutoAttackCooldown;
pub use buffs::{
    apply_class_passive_buffs, apply_crit_buff, apply_damage_buff,
    apply_speed_buff, BuffAppliedEvent, BuffContainer, BuffExpiredEvent, PassiveStatMod, tick_active_buffs,
};
pub use rotation::{rotation_for_spec, step_name, RotationState, SpecRotation, RotationStep};
