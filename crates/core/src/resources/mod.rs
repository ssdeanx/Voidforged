//! Resources — modular sub-modules for all game state and configuration types.
//! Re-exports every public type from sub-modules.

pub mod state;
pub mod profiles;
pub mod game;
pub mod input;
pub mod combat;
pub mod dungeon;
pub mod meta;
pub mod assets;
pub mod death;
pub mod items_db;

pub use state::*;
pub use profiles::*;
pub use game::*;
pub use input::*;
pub use combat::*;
pub use dungeon::*;
pub use meta::*;
pub use assets::*;
pub use death::*;
pub use items_db::*;
