//! Integration tests for the `ir-core` crate.
//! Tests cover items, components, events, and hitboxes.

use bevy::prelude::{Entity, Vec3};
use ir_core::*;

// ============================================================================
// Item Tests
// ============================================================================

#[test]
fn test_rarity_labels() {
    assert_eq!(ItemRarity::Common.label(), "Common");
    assert_eq!(ItemRarity::Uncommon.label(), "Uncommon");
    assert_eq!(ItemRarity::Rare.label(), "Rare");
    assert_eq!(ItemRarity::Epic.label(), "Epic");
    assert_eq!(ItemRarity::Legendary.label(), "Legendary");
}

#[test]
fn test_rarity_stat_multipliers() {
    assert!((ItemRarity::Common.stat_multiplier() - 1.0).abs() < f32::EPSILON);
    assert!((ItemRarity::Uncommon.stat_multiplier() - 1.3).abs() < f32::EPSILON);
    assert!((ItemRarity::Rare.stat_multiplier() - 1.6).abs() < f32::EPSILON);
    assert!((ItemRarity::Epic.stat_multiplier() - 2.0).abs() < f32::EPSILON);
    assert!((ItemRarity::Legendary.stat_multiplier() - 2.5).abs() < f32::EPSILON);
}

#[test]
fn test_rarity_stat_multipliers_increase() {
    assert!(ItemRarity::Common.stat_multiplier() < ItemRarity::Uncommon.stat_multiplier());
    assert!(ItemRarity::Uncommon.stat_multiplier() < ItemRarity::Rare.stat_multiplier());
    assert!(ItemRarity::Rare.stat_multiplier() < ItemRarity::Epic.stat_multiplier());
    assert!(ItemRarity::Epic.stat_multiplier() < ItemRarity::Legendary.stat_multiplier());
}

#[test]
fn test_rarity_hex_colors() {
    let mut hexes = std::collections::HashSet::new();
    hexes.insert(ItemRarity::Common.color_hex());
    hexes.insert(ItemRarity::Uncommon.color_hex());
    hexes.insert(ItemRarity::Rare.color_hex());
    hexes.insert(ItemRarity::Epic.color_hex());
    hexes.insert(ItemRarity::Legendary.color_hex());
    assert_eq!(hexes.len(), 5, "all rarities must have distinct hex colors");
}

#[test]
fn test_starter_items_exist() {
    let defs = starter_item_defs();
    assert!(!defs.is_empty());
    // Verify specific items exist
    assert!(defs.iter().any(|d| d.id == "iron_sword"));
    assert!(defs.iter().any(|d| d.id == "health_potion"));
    assert!(defs.iter().any(|d| d.id == "leather_chest"));
}

#[test]
fn test_starter_items_unique_ids() {
    let defs = starter_item_defs();
    let mut ids = std::collections::HashSet::new();
    for def in &defs {
        assert!(ids.insert(def.id), "duplicate id: {}", def.id);
    }
    assert_eq!(ids.len(), defs.len());
}

#[test]
fn test_item_instance_new() {
    let item = ItemInstance::new("iron_sword");
    assert_eq!(item.def_id, "iron_sword");
    assert_eq!(item.stack_count, 1);
    assert!((item.durability - 1.0).abs() < f32::EPSILON);
    assert!(item.is_usable());
}

#[test]
fn test_item_instance_stacked() {
    let item = ItemInstance::stacked("health_potion", 10);
    assert_eq!(item.stack_count, 10);
}

#[test]
fn test_item_instance_damage() {
    let mut item = ItemInstance::new("iron_sword");
    item.damage(0.3);
    assert!((item.durability - 0.7).abs() < f32::EPSILON);
    item.damage(1.0);
    assert!((item.durability - 0.0).abs() < f32::EPSILON);
    assert!(!item.is_usable());
}

// ============================================================================
// Inventory Tests
// ============================================================================

#[test]
fn test_inventory_new() {
    let inv = Inventory::new(10);
    assert_eq!(inv.slots.len(), 10);
    assert_eq!(inv.gold, 0);
    assert!(inv.has_space());
    assert_eq!(inv.used_slots(), 0);
}

#[test]
fn test_inventory_add_remove() {
    let mut inv = Inventory::new(5);
    assert!(inv.add_item(ItemInstance::new("iron_sword")));
    assert_eq!(inv.used_slots(), 1);
    assert!(inv.contains_def("iron_sword"));

    let removed = inv.remove_item(0);
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().def_id, "iron_sword");
    assert!(!inv.contains_def("iron_sword"));
}

#[test]
fn test_inventory_full() {
    let mut inv = Inventory::new(1);
    assert!(inv.add_item(ItemInstance::new("a")));
    assert!(!inv.add_item(ItemInstance::new("b")));
}

#[test]
fn test_inventory_gold() {
    let mut inv = Inventory::new(5);
    inv.add_gold(100);
    assert_eq!(inv.gold, 100);
    assert!(inv.remove_gold(50));
    assert_eq!(inv.gold, 50);
    assert!(!inv.remove_gold(100));
    assert_eq!(inv.gold, 50);
}

// ============================================================================
// Equipment Tests
// ============================================================================

#[test]
fn test_equipment_equip_unequip() {
    let mut eq = Equipment::default();
    assert!(eq.get(EquipSlot::MainHand).is_none());

    eq.equip(ItemInstance::new("iron_sword"), EquipSlot::MainHand);
    assert_eq!(eq.get(EquipSlot::MainHand).unwrap().def_id, "iron_sword");

    let old = eq.equip(ItemInstance::new("steel_sword"), EquipSlot::MainHand);
    assert_eq!(old.unwrap().def_id, "iron_sword");

    let removed = eq.unequip(EquipSlot::MainHand);
    assert_eq!(removed.unwrap().def_id, "steel_sword");
    assert!(eq.get(EquipSlot::MainHand).is_none());
}

#[test]
fn test_equipment_all_slots() {
    let mut eq = Equipment::default();
    let slots = [
        EquipSlot::MainHand, EquipSlot::OffHand, EquipSlot::Helmet,
        EquipSlot::Chest, EquipSlot::Boots, EquipSlot::Ring,
        EquipSlot::Amulet, EquipSlot::Trinket,
    ];
    for (i, slot) in slots.iter().enumerate() {
        eq.equip(ItemInstance::new(&format!("item_{}", i)), *slot);
    }
    for (i, slot) in slots.iter().enumerate() {
        assert_eq!(eq.get(*slot).unwrap().def_id, format!("item_{}", i));
    }
}

// ============================================================================
// Gear Score Tests
// ============================================================================

#[test]
fn test_gear_score_functions() {
    // Test rarity budget values
    assert!((rarity_budget(ItemRarity::Common) - 1.0).abs() < f32::EPSILON);
    assert!((rarity_budget(ItemRarity::Legendary) - 4.0).abs() < f32::EPSILON);

    // Test slot base ilvl
    assert_eq!(slot_base_ilvl(EquipSlot::MainHand), 5);
    assert_eq!(slot_base_ilvl(EquipSlot::Ring), 2);

    // Test compare items
    let current = vec![StatMod { stat: StatType::DamageBonus, value: 10.0 }];
    let upgraded = vec![StatMod { stat: StatType::DamageBonus, value: 15.0 }];
    let diffs = compare_items(&current, &upgraded);
    assert_eq!(diffs.len(), 1);
    assert!((diffs[0].new_value - 15.0).abs() < 0.01);
}

#[test]
fn test_compare_items_no_diff() {
    let stats = vec![StatMod { stat: StatType::DamageBonus, value: 10.0 }];
    let diffs = compare_items(&stats, &stats);
    assert!(diffs.is_empty());
}

#[test]
fn test_loot_table_ranges() {
    // Ensure all level ranges produce items
    assert!(!loot_table_for_level(1).is_empty());
    assert!(!loot_table_for_level(8).is_empty());
    assert!(!loot_table_for_level(15).is_empty());
    assert!(!loot_table_for_level(99).is_empty());
}

// ============================================================================
// StatType / StatMod Tests
// ============================================================================

#[test]
fn test_stat_type_display_names() {
    assert_eq!(StatType::DamageBonus.display_name(), "Damage");
    assert_eq!(StatType::CritChance.display_name(), "Crit Chance");
    assert_eq!(StatType::DodgeChance.display_name(), "Dodge");
}

#[test]
fn test_stat_type_format() {
    // Flat format
    let dmg = StatType::DamageBonus.format_value(10.0);
    assert!(dmg.contains("10"), "Expected +10, got {}", dmg);

    // Percentage format
    let crit = StatType::CritChance.format_value(0.05);
    assert!(crit.contains("%"), "Expected % format, got {}", crit);
}

// ============================================================================
// Component Tests
// ============================================================================

#[test]
fn test_health_system() {
    let mut health = Health::new(100.0);
    assert!((health.fraction() - 1.0).abs() < f32::EPSILON);
    assert!(health.is_alive());

    health.take_damage(30.0, 0.0);
    assert!((health.current - 70.0).abs() < f32::EPSILON);

    health.heal(20.0);
    assert!((health.current - 90.0).abs() < f32::EPSILON);

    health.take_damage(200.0, 0.0);
    assert!(!health.is_alive());
}

#[test]
fn test_health_invulnerability() {
    let mut health = Health::new(100.0);
    health.take_damage(50.0, 10.0); // damage at time 10
    assert!((health.current - 50.0).abs() < f32::EPSILON);

    health.invulnerable_until = 15.0;
    assert!(!health.take_damage(50.0, 12.0)); // blocked by invulnerability
    assert!((health.current - 50.0).abs() < f32::EPSILON); // unchanged
}

#[test]
fn test_character_class_variants() {
    let all = CharacterClass::all();
    assert_eq!(all.len(), 5);

    for class in &all {
        assert!(!class.description().is_empty());
        assert!(!class.display_name().is_empty());
        assert!(!class.resource_name().is_empty());
        assert!(class.base_max_hp() > 0.0);
    }
}

#[test]
fn test_character_class_from_string() {
    assert_eq!("Warrior".parse::<CharacterClass>().unwrap(), CharacterClass::Warrior);
    assert_eq!("Mage".parse::<CharacterClass>().unwrap(), CharacterClass::Mage);
    assert_eq!("Paladin".parse::<CharacterClass>().unwrap(), CharacterClass::Paladin);
    assert_eq!("Rogue".parse::<CharacterClass>().unwrap(), CharacterClass::Rogue);
    assert_eq!("Hunter".parse::<CharacterClass>().unwrap(), CharacterClass::Hunter);
    assert!("Unknown".parse::<CharacterClass>().is_err());
}

#[test]
fn test_weapon_construction() {
    let weapon = Weapon::new(WeaponKind::Sword, 14.0, 1.0, 3.5);
    assert_eq!(weapon.kind, WeaponKind::Sword);
    assert!((weapon.damage - 14.0).abs() < f32::EPSILON);
    assert!((weapon.attack_speed - 1.0).abs() < f32::EPSILON);
    assert!((weapon.range - 3.5).abs() < f32::EPSILON);
    assert_eq!(weapon.cooldown_timer, 0.0);
    assert_eq!(weapon.evolution_stage, 0);
}

#[test]
fn test_player_default() {
    let player = Player::default();
    assert_eq!(player.level, 1);
    assert_eq!(player.experience, 0);
    assert_eq!(player.xp_to_next, 100);
}

#[test]
fn test_combat_stats_default() {
    let stats = CombatStats::default();
    assert!((stats.move_speed - 5.0).abs() < f32::EPSILON);
    assert!((stats.crit_chance - 0.05).abs() < f32::EPSILON);
    assert!((stats.crit_multiplier - 2.0).abs() < f32::EPSILON);
    assert!((stats.damage_taken_multiplier - 1.0).abs() < f32::EPSILON);
}

#[test]
fn test_stamina_mechanics() {
    let mut stam = Stamina::default();
    assert!((stam.max - 100.0).abs() < f32::EPSILON);
    assert!(stam.has(50.0));
    assert!(stam.has(100.0));
    assert!(!stam.has(101.0));

    stam.spend(40.0);
    assert!((stam.current - 60.0).abs() < f32::EPSILON);
    assert!((stam.fraction() - 0.6).abs() < f32::EPSILON);

    // Clamp at zero
    stam.spend(200.0);
    assert!((stam.current - 0.0).abs() < f32::EPSILON);
}

#[test]
fn test_status_effects() {
    let frozen = Frozen::new(2.0);
    assert!((frozen.remaining - 2.0).abs() < f32::EPSILON);

    let stun = Stun::new(0.5);
    assert!((stun.remaining - 0.5).abs() < f32::EPSILON);

    let hitstun = HitStun::new(0.1);
    assert!((hitstun.remaining - 0.1).abs() < f32::EPSILON);

    let hitstop = HitStop::new(0.05);
    assert!((hitstop.remaining - 0.05).abs() < f32::EPSILON);
}

#[test]
fn test_knockback() {
    let kb = Knockback::new(Vec3::new(10.0, 0.0, 0.0), 5.0);
    assert!((kb.velocity.x - 10.0).abs() < f32::EPSILON);
    assert!((kb.damping - 5.0).abs() < f32::EPSILON);
}

#[test]
fn test_dash_cooldown_default() {
    let dc = DashCooldown::default();
    assert!(!dc.active);
    assert!(!dc.fired_dash_attack);
    assert!((dc.duration - 0.25).abs() < f32::EPSILON);
}

// ============================================================================
// Equipment Slot Tests
// ============================================================================

#[test]
fn test_equip_slot_display_names() {
    assert_eq!(EquipSlot::MainHand.display_name(), "Main Hand");
    assert_eq!(EquipSlot::OffHand.display_name(), "Off Hand");
    assert_eq!(EquipSlot::Helmet.display_name(), "Helmet");
    assert_eq!(EquipSlot::Chest.display_name(), "Chest");
    assert_eq!(EquipSlot::Boots.display_name(), "Boots");
    assert_eq!(EquipSlot::Ring.display_name(), "Ring");
    assert_eq!(EquipSlot::Amulet.display_name(), "Amulet");
    assert_eq!(EquipSlot::Trinket.display_name(), "Trinket");
}

// ============================================================================
// Hitbox Tests
// ============================================================================

#[test]
fn test_hitbox_shapes() {
    let cone = HitboxShape::Cone { range: 3.0, half_angle: 0.5 };
    let circle = HitboxShape::Circle { radius: 5.0 };
    let rect = HitboxShape::Rect { width: 2.0, length: 4.0 };
    let point = HitboxShape::Point { range: 1.5 };

    match cone {
        HitboxShape::Cone { range, half_angle } => {
            assert!((range - 3.0).abs() < f32::EPSILON);
            assert!((half_angle - 0.5).abs() < f32::EPSILON);
        }
        _ => panic!("expected Cone"),
    }
    match circle {
        HitboxShape::Circle { radius } => assert!((radius - 5.0).abs() < f32::EPSILON),
        _ => panic!("expected Circle"),
    }
    match rect {
        HitboxShape::Rect { width, length } => {
            assert!((width - 2.0).abs() < f32::EPSILON);
            assert!((length - 4.0).abs() < f32::EPSILON);
        }
        _ => panic!("expected Rect"),
    }
    match point {
        HitboxShape::Point { range } => assert!((range - 1.5).abs() < f32::EPSILON),
        _ => panic!("expected Point"),
    }
}

#[test]
fn test_damage_hitbox_new() {
    let source = Entity::from_raw(1);
    let hitbox = DamageHitbox::new(
        HitboxShape::Cone { range: 3.0, half_angle: 0.5 },
        10.0,
        source,
        DamageType::Physical,
        0.5,
        ProjectileOwner::Player,
        4.0,
    );
    assert!((hitbox.damage - 10.0).abs() < f32::EPSILON);
    assert_eq!(hitbox.damage_type, DamageType::Physical);
    assert!((hitbox.lifetime - 0.5).abs() < f32::EPSILON);
    assert_eq!(hitbox.owner, ProjectileOwner::Player);
    assert!(hitbox.hit_enemies.is_empty());
}

#[test]
fn test_damage_hitbox_with_hit_reaction() {
    let hitbox = DamageHitbox::new(
        HitboxShape::Circle { radius: 2.0 },
        20.0,
        Entity::from_raw(1),
        DamageType::Magic,
        0.3,
        ProjectileOwner::Player,
        0.0,
    ).with_hit_reaction(0.2, 0.3, 0.1);
    assert!((hitbox.hit_stun_duration - 0.2).abs() < f32::EPSILON);
    assert!((hitbox.hit_flash_duration - 0.3).abs() < f32::EPSILON);
    assert!((hitbox.hit_stop_duration - 0.1).abs() < f32::EPSILON);
}

#[test]
fn test_hitbox_damage_variants() {
    assert_eq!(format!("{:?}", DamageType::Physical), "Physical");
    assert_eq!(format!("{:?}", DamageType::Magic), "Magic");
    assert_eq!(format!("{:?}", DamageType::True), "True");
}

// ============================================================================
// Event Tests
// ============================================================================

#[test]
fn test_event_construction() {
    let dmg = DamageEvent {
        target: Entity::from_raw(1),
        source: Entity::from_raw(2),
        amount: 15.0,
        is_critical: true,
        damage_type: DamageType::Physical,
    };
    assert!((dmg.amount - 15.0).abs() < f32::EPSILON);
    assert!(dmg.is_critical);

    let death = DeathEvent {
        entity: Entity::from_raw(1),
        killer: Some(Entity::from_raw(2)),
        enemy_variant: Some(EnemyVariant::Boss),
    };
    assert_eq!(death.enemy_variant, Some(EnemyVariant::Boss));

    let xp = ExperienceGainEvent { amount: 100, source: Entity::from_raw(1) };
    assert_eq!(xp.amount, 100);

    let lvl = LevelUpEvent { new_level: 5 };
    assert_eq!(lvl.new_level, 5);
}

#[test]
fn test_wave_events() {
    let start = WaveStartEvent { wave_number: 1, enemy_count: 10 };
    assert_eq!(start.wave_number, 1);
    assert_eq!(start.enemy_count, 10);

    let cleared = WaveClearedEvent { wave_number: 3 };
    assert_eq!(cleared.wave_number, 3);
}

#[test]
fn test_run_events() {
    let end = RunEndEvent { victory: true, wave_reached: 10, kills: 50, run_time: 600.0 };
    assert!(end.victory);
    assert_eq!(end.wave_reached, 10);
}

// ============================================================================
// Resource Tests
// ============================================================================

#[test]
fn test_item_database() {
    let mut db = ItemDatabase::default();
    assert!(db.get("iron_sword").is_none());

    let def = ItemDef {
        id: "test_item",
        name: "Test",
        description: "A test item",
        category: ItemCategory::Weapon,
        slot: Some(EquipSlot::MainHand),
        rarity: ItemRarity::Common,
        base_stats: vec![],
        max_stack: 1,
        icon_id: "",
        required_level: 1,
        vendor_price: 10,
    };
    db.register(def);
    assert!(db.get("test_item").is_some());
}

#[test]
fn test_meta_progression_default() {
    let meta = MetaProgression::default();
    assert_eq!(meta.dark_essence, 0);
    assert_eq!(meta.total_runs, 0);
    assert_eq!(meta.highest_wave, 0);
}

// ============================================================================
// ItemCategory Tests
// ============================================================================

#[test]
fn test_item_category_distinct() {
    let mut set = std::collections::HashSet::new();
    set.insert(ItemCategory::Weapon as u8);
    set.insert(ItemCategory::Armor as u8);
    set.insert(ItemCategory::Accessory as u8);
    set.insert(ItemCategory::Consumable as u8);
    set.insert(ItemCategory::Material as u8);
    set.insert(ItemCategory::Quest as u8);
    assert_eq!(set.len(), 6);
}
