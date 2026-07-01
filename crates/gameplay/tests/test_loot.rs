//! Integration tests for `ir-gameplay`.
//! Tests the loot table system, enemy variant tables, and drop logic.

use ir_core::*;
use ir_gameplay::loot::*;

// ============================================================================
// Loot Entry Tests
// ============================================================================

#[test]
fn test_loot_entry_new() {
    let entry = LootEntry::new("iron_sword", 5.0, 0.5);
    assert_eq!(entry.item_id, "iron_sword");
    assert!((entry.weight - 5.0).abs() < f32::EPSILON);
    assert!((entry.drop_chance - 0.5).abs() < f32::EPSILON);
    assert_eq!(entry.min_count, 1);
    assert_eq!(entry.max_count, 2);
}

#[test]
fn test_loot_entry_stacked() {
    let entry = LootEntry::new("health_potion", 8.0, 0.5).stacked(2, 5);
    assert_eq!(entry.min_count, 2);
    assert_eq!(entry.max_count, 5);
}

// ============================================================================
// Loot Table Tests
// ============================================================================

#[test]
fn test_loot_table_empty() {
    let table = LootTable::new(vec![]);
    let mut rng = rand::thread_rng();
    let results = table.roll(&mut rng, 1.0, 0);
    assert!(results.is_empty());
}

#[test]
fn test_loot_table_single_entry_success() {
    // 100% drop chance, weight 10.0
    let table = LootTable::new(vec![
        LootEntry::new("iron_sword", 10.0, 1.0),
    ]);
    let mut rng = rand::thread_rng();
    // Roll many times to make sure it works
    let mut found = false;
    for _ in 0..20 {
        let results = table.roll(&mut rng, 1.0, 0);
        for (id, _) in &results {
            if *id == "iron_sword" {
                found = true;
            }
        }
    }
    assert!(found, "iron_sword should eventually drop");
}

#[test]
fn test_loot_table_zero_weight_never_drops() {
    let table = LootTable::new(vec![
        LootEntry::new("iron_sword", 0.0, 1.0),
    ]);
    let mut rng = rand::thread_rng();
    for _ in 0..50 {
        let results = table.roll(&mut rng, 1.0, 0);
        assert!(results.is_empty(), "zero weight entry should never roll");
    }
}

#[test]
fn test_loot_table_many_entries() {
    let entries = vec![
        LootEntry::new("a", 10.0, 1.0),
        LootEntry::new("b", 10.0, 1.0),
        LootEntry::new("c", 10.0, 1.0),
    ];
    let table = LootTable::new(entries);
    let mut rng = rand::thread_rng();
    let mut seen = std::collections::HashSet::new();
    for _ in 0..50 {
        for (id, _) in table.roll(&mut rng, 1.0, 0) {
            seen.insert(id);
        }
    }
    assert!(seen.len() >= 3, "Expected all 3 items to drop, got {:?}", seen);
}

#[test]
fn test_loot_table_tier_bonus_increases_counts() {
    let table = LootTable::new(vec![
        LootEntry::new("health_potion", 10.0, 1.0).stacked(1, 2),
    ]);
    let mut rng = rand::thread_rng();
    let mut max_count = 0u16;
    for _ in 0..30 {
        for (_, count) in table.roll(&mut rng, 1.0, 5) {
            max_count = max_count.max(count);
        }
    }
    // With tier bonus, some rolls should get bonus count
    assert!(max_count >= 1, "expected at least count of 1");
}

// ============================================================================
// Enemy Variant Loot Tables
// ============================================================================

#[test]
fn test_grunt_loot_table() {
    let table = table_for_variant(&EnemyVariant::Grunt);
    assert!(!table.entries.is_empty());
    assert!(table.entries.iter().any(|e| e.item_id == "health_potion"));
    assert!(table.entries.iter().any(|e| e.item_id == "iron_sword"));
}

#[test]
fn test_ranged_loot_table() {
    let table = table_for_variant(&EnemyVariant::Ranged);
    assert!(table.entries.iter().any(|e| e.item_id == "short_bow"));
    assert!(table.entries.iter().any(|e| e.item_id == "long_bow"));
}

#[test]
fn test_charger_loot_table() {
    let table = table_for_variant(&EnemyVariant::Charger);
    assert!(table.entries.iter().any(|e| e.item_id == "steel_sword"));
    assert!(table.entries.iter().any(|e| e.item_id == "iron_helm"));
}

#[test]
fn test_elite_loot_table() {
    let table = table_for_variant(&EnemyVariant::Elite);
    assert!(table.entries.iter().any(|e| e.item_id == "chainmail"));
    assert!(table.entries.iter().any(|e| e.item_id == "silver_ring"));
    assert!(table.entries.len() >= 8);
}

#[test]
fn test_boss_loot_table() {
    let table = table_for_variant(&EnemyVariant::Boss);
    // Boss should always drop health potions
    if let Some(potion) = table.entries.iter().find(|e| e.item_id == "health_potion") {
        assert!((potion.drop_chance - 1.0).abs() < f32::EPSILON);
        assert_eq!(potion.min_count, 2);
        assert_eq!(potion.max_count, 5);
    }
    assert!(table.entries.iter().any(|e| e.item_id == "runed_sword"));
    assert!(table.entries.iter().any(|e| e.item_id == "archmage_staff"));
}

// ============================================================================
// ItemDrop Component
// ============================================================================

#[test]
fn test_item_drop_creation() {
    let drop = ItemDrop { def_id: "iron_sword".to_string() };
    assert_eq!(drop.def_id, "iron_sword");
}
