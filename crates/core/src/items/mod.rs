//! Item system — modular sub-modules for all item-related types.
//! Re-exports every public type from sub-modules for convenient imports.

pub mod rarity;
pub mod slots;
pub mod modifiers;
pub mod definitions;
pub mod instance;
pub mod inventory;
pub mod equipment;
pub mod gear_score;

pub use rarity::*;
pub use slots::*;
pub use modifiers::*;
pub use definitions::*;
pub use instance::*;
pub use inventory::*;
pub use equipment::*;
