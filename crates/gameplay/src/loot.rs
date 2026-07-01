//! Loot tables — weighted drop system that connects enemy kills to the new item system.
//! Replaces the ad-hoc roll logic in procedural::loot::spawn_loot.

use bevy::prelude::*;
use ir_core::ItemDatabase;
use ir_core::*;
use rand::Rng;

// ============================================================================
// Weighted Entry
// ============================================================================

/// A single entry in a loot table.
///
/// Each entry has an item ID (matching `ItemDef.id`), a relative weight
/// that controls probability, a configurable stack count range, and a
/// base drop-chance gate.
pub struct LootEntry {
    /// Matches `ItemDef.id` in the `ItemDatabase`.
    pub item_id: &'static str,
    /// Relative weight (higher = more likely to be selected).
    pub weight: f32,
    /// Per-drop minimum number of items.
    pub min_count: u16,
    /// Per-drop maximum number of items (exclusive upper bound).
    pub max_count: u16,
    /// Base probability (0.0–1.0) that this entry is rolled at all.
    pub drop_chance: f32,
}

impl LootEntry {
    /// Creates a new `LootEntry` with default stack range (1–2).
    pub const fn new(item_id: &'static str, weight: f32, drop_chance: f32) -> Self {
        Self {
            item_id,
            weight,
            min_count: 1,
            max_count: 2,
            drop_chance,
        }
    }

    /// Overrides the per-drop stack range to `min..max`.
    pub const fn stacked(mut self, min: u16, max: u16) -> Self {
        self.min_count = min;
        self.max_count = max;
        self
    }
}

// ============================================================================
// Loot Table
// ============================================================================

/// A collection of weighted loot entries. Rolled once when an enemy dies.
pub struct LootTable {
    /// The weighted entries making up this loot table.
    pub entries: Vec<LootEntry>,
}

impl LootTable {
    /// Creates a new `LootTable` from the given entries.
    pub const fn new(entries: Vec<LootEntry>) -> Self {
        Self { entries }
    }

    /// Rolls the table and returns a list of `(item_id, count)` pairs.
    ///
    /// Each entry goes through a drop-chance gate first, then a weighted
    /// roll against the total weight. `rarity_mult` and `tier_bonus`
    /// scale the drop rates and stack counts.
    pub fn roll(&self, rng: &mut impl Rng, rarity_mult: f32, tier_bonus: u32) -> Vec<(&'static str, u16)> {
        let mut results = Vec::new();
        let total_weight: f32 = self.entries.iter().map(|e| e.weight).sum();
        if total_weight <= 0.0 {
            return results;
        }

        for entry in &self.entries {
            // Drop chance gate
            if rng.gen::<f32>() >= entry.drop_chance * (1.0 + tier_bonus as f32 * 0.1) {
                continue;
            }
            // Weighted roll
            if rng.gen::<f32>() * total_weight < entry.weight {
                let count = rng.gen_range(entry.min_count..entry.max_count);
                let bonus = (tier_bonus as f32 * rarity_mult * 0.5) as u16;
                results.push((entry.item_id, count + bonus));
            }
        }
        results
    }
}

// ============================================================================
// Enemy Loot Tables (per variant)
// ============================================================================

/// Returns the loot table appropriate for the given enemy variant.
pub fn table_for_variant(variant: &EnemyVariant) -> LootTable {
    match variant {
        EnemyVariant::Grunt => LootTable::new(vec![
            LootEntry::new("health_potion", 8.0, 0.30),
            LootEntry::new("iron_sword", 2.0, 0.05),
            LootEntry::new("iron_dagger", 3.0, 0.08),
            LootEntry::new("short_bow", 2.0, 0.04),
            LootEntry::new("leather_helm", 2.0, 0.06),
            LootEntry::new("leather_chest", 2.0, 0.05),
            LootEntry::new("leather_boots", 2.0, 0.06),
            LootEntry::new("copper_ring", 1.0, 0.03),
        ]),
        EnemyVariant::Ranged => LootTable::new(vec![
            LootEntry::new("health_potion", 6.0, 0.25),
            LootEntry::new("short_bow", 4.0, 0.10),
            LootEntry::new("long_bow", 2.0, 0.05),
            LootEntry::new("leather_chest", 2.0, 0.06),
        ]),
        EnemyVariant::Charger => LootTable::new(vec![
            LootEntry::new("health_potion", 6.0, 0.30),
            LootEntry::new("iron_sword", 4.0, 0.10),
            LootEntry::new("steel_sword", 2.0, 0.05),
            LootEntry::new("iron_helm", 3.0, 0.08),
            LootEntry::new("copper_ring", 2.0, 0.05),
        ]),
        EnemyVariant::Elite => LootTable::new(vec![
            LootEntry::new("health_potion", 8.0, 0.50),
            LootEntry::new("steel_sword", 4.0, 0.15),
            LootEntry::new("steel_dagger", 4.0, 0.12),
            LootEntry::new("long_bow", 3.0, 0.10),
            LootEntry::new("chainmail", 3.0, 0.10),
            LootEntry::new("iron_helm", 3.0, 0.10),
            LootEntry::new("iron_boots", 3.0, 0.10),
            LootEntry::new("silver_ring", 2.0, 0.08),
            LootEntry::new("apprentice_staff", 2.0, 0.06),
        ]),
        EnemyVariant::Boss => LootTable::new(vec![
            LootEntry::new("health_potion", 10.0, 1.0).stacked(2, 5),
            LootEntry::new("silver_ring", 6.0, 0.40),
            LootEntry::new("runed_sword", 5.0, 0.30),
            LootEntry::new("archmage_staff", 4.0, 0.25),
            LootEntry::new("plate_chest", 4.0, 0.25),
            LootEntry::new("iron_boots", 4.0, 0.30),
            LootEntry::new("iron_helm", 4.0, 0.30),
        ]),
    }
}

// ============================================================================
// System: spawn item drops from loot table on enemy death
// ============================================================================

/// Listens for `DeathEvent` and spawns item pickups based on the enemy's loot table.
///
/// Reads the enemy's variant and tier, rolls the appropriate loot table,
/// and spawns `ItemDrop` entities with randomized offsets around the death
/// position. Despawns the dead enemy entity after loot generation.
pub fn spawn_loot_from_table(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    enemy_query: Query<(&Enemy, &Transform)>,
    item_db: Res<ItemDatabase>,
) {
    for event in death_events.read() {
        let Ok((enemy, transform)) = enemy_query.get(event.entity) else {
            continue;
        };
        let table = table_for_variant(&enemy.variant);
        let mut rng = rand::thread_rng();
        let tier_bonus = enemy.tier / 2; // every 2 tiers = +1 tier bonus

        let drops = table.roll(&mut rng, enemy.tier as f32 * 1.5, tier_bonus);

        for (item_id, count) in drops {
            // Only spawn if we have the item def registered
            if item_db.get(item_id).is_none() {
                continue;
            }

            let pos = transform.translation + Vec3::Y * 0.5;
            for _ in 0..count {
                let offset = Vec3::new(rng.gen::<f32>() - 0.5, 0.0, rng.gen::<f32>() - 0.5);
                commands.spawn((
                    ItemDrop {
                        def_id: item_id.to_string(),
                    },
                    Transform::from_translation(pos + offset * 0.8),
                    RoomEntity,
                ));
            }
        }

        // Despawn the dead enemy (after loot is collected)
        commands.entity(event.entity).despawn();
    }
}

/// Marker component for a dropped item on the ground.
///
/// Stores the item definition ID so the pickup system can look up
/// the item's properties when the player collects it.
#[derive(Component, Debug, Clone)]
pub struct ItemDrop {
    /// The `ItemDef.id` for the database lookup.
    pub def_id: String,
}
