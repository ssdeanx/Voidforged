//! Ability rank system — each ability has ranks that unlock at character levels.
//! Higher ranks increase damage, reduce cooldown, or add bonus effects.
//!
//! Data is structured as a lookup table per class per ability slot.
//! Call `current_rank(player_level)` to get the highest unlocked rank.

/// A single rank tier for an ability.
pub struct RankTier {
    /// Rank number (1-based, 1 = starting rank).
    pub rank: u32,
    /// Player level required to unlock this rank.
    pub level_required: u32,
    /// Damage multiplier at this rank (1.0 = 100% of base).
    pub damage_mult: f32,
    /// Cooldown multiplier (1.0 = 100% of base, lower = faster).
    pub cooldown_mult: f32,
}

impl RankTier {
    pub const fn new(rank: u32, level: u32, dmg: f32, cd: f32) -> Self {
        Self { rank, level_required: level, damage_mult: dmg, cooldown_mult: cd }
    }
}

/// Returns the highest unlocked rank index (0-based) for `player_level`.
pub fn current_rank_idx(tiers: &[RankTier], player_level: u32) -> usize {
    let mut idx = 0;
    for (i, tier) in tiers.iter().enumerate() {
        if player_level >= tier.level_required {
            idx = i;
        }
    }
    idx
}

/// Returns the highest unlocked `RankTier` for `player_level`.
pub fn current_rank(tiers: &[RankTier], player_level: u32) -> &RankTier {
    let idx = current_rank_idx(tiers, player_level);
    &tiers[idx]
}

/// Pre-defined rank tiers for all classes (1–5 ranks each).
pub mod rank_data {
    use super::RankTier;

    /// Generic melee ability ranks (warrior/paladin primary, rogue primary).
    pub const MELEE_PRIMARY: &[RankTier] = &[
        RankTier::new(1, 1, 1.0, 1.0),
        RankTier::new(2, 3, 1.15, 0.95),
        RankTier::new(3, 6, 1.30, 0.90),
        RankTier::new(4, 10, 1.50, 0.85),
        RankTier::new(5, 15, 1.75, 0.80),
    ];

    /// Generic ranged primary ranks (hunter/mage).
    pub const RANGED_PRIMARY: &[RankTier] = &[
        RankTier::new(1, 1, 1.0, 1.0),
        RankTier::new(2, 3, 1.12, 0.95),
        RankTier::new(3, 6, 1.25, 0.90),
        RankTier::new(4, 10, 1.45, 0.85),
        RankTier::new(5, 15, 1.65, 0.80),
    ];

    /// Utility ability ranks (6s base CD).
    pub const UTILITY: &[RankTier] = &[
        RankTier::new(1, 4, 1.0, 1.0),
        RankTier::new(2, 8, 1.2, 0.90),
        RankTier::new(3, 12, 1.4, 0.80),
    ];

    /// Ultimate ability ranks (30s base CD).
    pub const ULTIMATE: &[RankTier] = &[
        RankTier::new(1, 8, 1.0, 1.0),
        RankTier::new(2, 14, 1.3, 0.85),
        RankTier::new(3, 20, 1.6, 0.70),
    ];

    /// Secondary ability ranks.
    pub const SECONDARY: &[RankTier] = &[
        RankTier::new(1, 2, 1.0, 1.0),
        RankTier::new(2, 5, 1.15, 0.90),
        RankTier::new(3, 9, 1.30, 0.85),
        RankTier::new(4, 13, 1.50, 0.80),
    ];

    /// Cast ability ranks.
    pub const CAST: &[RankTier] = &[
        RankTier::new(1, 3, 1.0, 1.0),
        RankTier::new(2, 7, 1.2, 0.90),
        RankTier::new(3, 11, 1.4, 0.80),
        RankTier::new(4, 16, 1.6, 0.75),
    ];
}
