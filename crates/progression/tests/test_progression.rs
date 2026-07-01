//! Integration tests for `ir-progression`.
//! Tests upgrade purchasing, cost calculation, and stat application.

use ir_core::*;
use ir_progression::leveling::*;
use ir_progression::upgrades::*;

// ============================================================================
// Upgrade Cost Tests
// ============================================================================

#[test]
fn test_upgrade_cost_base() {
    let def = UpgradeDef {
        id: "test_upgrade", name: "Test", description: "A test upgrade.",
        category: UpgradeCategory::Stats, max_tier: 5, base_cost: 100,
        cost_multiplier: 2.0, icon_id: "icon_test",
        per_tier_stats: vec![StatBonus { stat: StatType::MaxHealth, value: 20.0 }],
    };
    assert_eq!(upgrade_cost(&def, 0), 100);
    assert_eq!(upgrade_cost(&def, 1), 200);
    assert_eq!(upgrade_cost(&def, 2), 400);
    assert_eq!(upgrade_cost(&def, 3), 800);
}

#[test]
fn test_upgrade_cost_maxed() {
    let def = UpgradeDef {
        id: "unlock_test", name: "Test Unlock", description: "Single-tie unlock.",
        category: UpgradeCategory::Weapons, max_tier: 1, base_cost: 300,
        cost_multiplier: 1.0, icon_id: "icon_test",
        per_tier_stats: vec![],
    };
    assert_eq!(upgrade_cost(&def, 0), 300);
    assert_eq!(upgrade_cost(&def, 1), u64::MAX);
}

#[test]
fn test_upgrade_cost_scaling() {
    let def = UpgradeDef {
        id: "expensive_up", name: "Expensive", description: "Expensive upgrade.",
        category: UpgradeCategory::Stats, max_tier: 3, base_cost: 150,
        cost_multiplier: 2.5, icon_id: "icon_test",
        per_tier_stats: vec![StatBonus { stat: StatType::DamageBonus, value: 3.0 }],
    };
    assert_eq!(upgrade_cost(&def, 0), 150);
    assert_eq!(upgrade_cost(&def, 1), 375);  // 150 * 2.5
    assert_eq!(upgrade_cost(&def, 2), 937);  // 150 * 2.5^2 = 937.5
}

#[test]
fn test_upgrade_cost_overflow() {
    let def = UpgradeDef {
        id: "overflow", name: "Overflow", description: "Already maxed test.",
        category: UpgradeCategory::Stats, max_tier: 0, base_cost: 100,
        cost_multiplier: 2.0, icon_id: "icon_test",
        per_tier_stats: vec![],
    };
    assert_eq!(upgrade_cost(&def, 0), u64::MAX);
}

// ============================================================================
// Upgrade Definition Tests
// ============================================================================

#[test]
fn test_upgrade_def_ids_unique() {
    let defs = all_upgrade_defs();
    let mut ids = std::collections::HashSet::new();
    for def in &defs {
        assert!(ids.insert(def.id), "duplicate upgrade id: {}", def.id);
    }
    assert_eq!(ids.len(), defs.len());
}

#[test]
fn test_all_upgrade_defs_not_empty() {
    let defs = all_upgrade_defs();
    assert!(!defs.is_empty());
    assert!(defs.len() >= 10);
}

#[test]
fn test_upgrade_category_variants() {
    let cat = UpgradeCategory::Stats;
    assert_eq!(format!("{:?}", cat), "Stats");
    assert_eq!(format!("{:?}", UpgradeCategory::Weapons), "Weapons");
    assert_eq!(format!("{:?}", UpgradeCategory::Classes), "Classes");
    assert_eq!(format!("{:?}", UpgradeCategory::Utility), "Utility");
}

// ============================================================================
// Purchase Tests
// ============================================================================

#[test]
fn test_purchase_upgrade_success() {
    let mut meta = MetaProgression::default();
    meta.dark_essence = 500;
    purchase_upgrade("max_hp_up", &mut meta).unwrap();
    assert_eq!(meta.dark_essence, 400); // cost was 100
    assert_eq!(meta.upgrades.len(), 1);
    assert_eq!(meta.upgrades[0].tier, 1);
}

#[test]
fn test_purchase_upgrade_multiple_tiers() {
    let mut meta = MetaProgression::default();
    meta.dark_essence = 10000;

    purchase_upgrade("max_hp_up", &mut meta).unwrap();
    assert_eq!(meta.upgrades[0].tier, 1);

    purchase_upgrade("max_hp_up", &mut meta).unwrap();
    assert_eq!(meta.upgrades[0].tier, 2);
    assert_eq!(meta.upgrades[0].cost, 200);
}

#[test]
fn test_purchase_upgrade_not_found() {
    let mut meta = MetaProgression::default();
    meta.dark_essence = 1000;
    let result = purchase_upgrade("nonexistent_upgrade", &mut meta);
    assert!(matches!(result, Err(PurchaseError::NotFound)));
}

#[test]
fn test_purchase_upgrade_max_tier() {
    let mut meta = MetaProgression::default();
    meta.dark_essence = 10000;
    purchase_upgrade("unlock_dagger", &mut meta).unwrap(); // max_tier=1
    let result = purchase_upgrade("unlock_dagger", &mut meta);
    assert!(matches!(result, Err(PurchaseError::MaxTier)));
}

#[test]
fn test_purchase_upgrade_insufficient_essence() {
    let mut meta = MetaProgression::default();
    meta.dark_essence = 10; // Not enough
    let result = purchase_upgrade("max_hp_up", &mut meta);
    match result {
        Err(PurchaseError::InsufficientEssence(_)) => {} // expected
        _ => panic!("Expected InsufficientEssence error"),
    }
}

#[test]
fn test_purchase_error_display() {
    assert_eq!(
        format!("{}", PurchaseError::NotFound),
        "Upgrade not found"
    );
    assert_eq!(
        format!("{}", PurchaseError::MaxTier),
        "Upgrade already at max tier"
    );
}

// ============================================================================
// Accumulated Stats Tests
// ============================================================================

#[test]
fn test_accumulated_upgrade_stats_empty() {
    let meta = MetaProgression::default();
    assert!(accumulated_upgrade_stats(&meta).is_empty());
}

#[test]
fn test_accumulated_upgrade_stats_with_purchases() {
    let mut meta = MetaProgression::default();
    meta.dark_essence = 10000;

    // Purchase max_hp_up twice (tier 2)
    purchase_upgrade("max_hp_up", &mut meta).unwrap();
    purchase_upgrade("max_hp_up", &mut meta).unwrap();

    let stats = accumulated_upgrade_stats(&meta);
    assert_eq!(stats.len(), 1);
    assert_eq!(stats[0].stat, StatType::MaxHealth);
    assert!((stats[0].value - 40.0).abs() < f32::EPSILON); // 2 tiers * 20 HP each
}

#[test]
fn test_accumulated_upgrade_stats_multiple_types() {
    let mut meta = MetaProgression::default();
    meta.dark_essence = 10000;

    purchase_upgrade("max_hp_up", &mut meta).unwrap();
    purchase_upgrade("dmg_up", &mut meta).unwrap();

    let stats = accumulated_upgrade_stats(&meta);
    assert_eq!(stats.len(), 2);
}

// ============================================================================
// Apply Stats Tests
// ============================================================================

#[test]
fn test_apply_upgrade_stats_no_bonus() {
    let meta = MetaProgression::default();
    let mut stats = CombatStats::default();
    apply_upgrade_stats(&mut stats, &meta);
    assert!((stats.max_health_bonus - 0.0).abs() < f32::EPSILON);
}

#[test]
fn test_apply_upgrade_stats_with_bonus() {
    let mut meta = MetaProgression::default();
    meta.dark_essence = 10000;

    purchase_upgrade("max_hp_up", &mut meta).unwrap();
    let mut stats = CombatStats::default();
    apply_upgrade_stats(&mut stats, &meta);
    assert!((stats.max_health_bonus - 20.0).abs() < f32::EPSILON);
}

// ============================================================================
// XP / Leveling Tests
// ============================================================================

#[test]
fn test_xp_to_next_formula() {
    let xp_l1 = (100.0 * 1.3_f64.powi(1)) as u64;
    let xp_l10 = (100.0 * 1.3_f64.powi(10)) as u64;
    assert!(xp_l10 > xp_l1);
    assert!(xp_l10 > 1000);
}
