//! Combat systems — projectile movement, damage pipeline, hitbox processing,
//! knockback, status effects, and stamina. Each concern in its own sub-module.

pub mod damage;
pub mod hitboxes;
pub mod knockback;
pub mod projectiles;
pub mod stamina;
pub mod status_effects;

pub use damage::*;
pub use hitboxes::*;
pub use knockback::*;
pub use projectiles::*;
pub use stamina::*;
pub use status_effects::*;
