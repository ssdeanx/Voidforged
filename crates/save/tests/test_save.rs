//! Integration tests for `ir-save`.
//! Tests save data construction, serialization round-trips, and defaults.

use ir_core::*;
use ir_save::*;

// ============================================================================
// PlayerSaveData Tests
// ============================================================================

#[test]
fn test_player_save_data_default() {
    let data = PlayerSaveData::default();
    assert_eq!(data.level, 1);
    assert_eq!(data.xp, 0);
    assert_eq!(data.gold, 0);
    assert!(data.completed_dungeons.is_empty());
}

#[test]
fn test_player_save_data_custom() {
    let data = PlayerSaveData {
        level: 10,
        xp: 5000,
        gold: 2500,
        completed_dungeons: vec!["d1".into(), "d2".into()],
    };
    assert_eq!(data.level, 10);
    assert_eq!(data.xp, 5000);
    assert_eq!(data.completed_dungeons.len(), 2);
}

// ============================================================================
// SaveData Tests
// ============================================================================

#[test]
fn test_save_data_default() {
    let data = SaveData::default();
    assert_eq!(data.version, 2);
    assert!(data.profiles.is_empty());
    assert_eq!(data.next_profile_id, 1);
}

#[test]
fn test_save_data_clone() {
    let data = SaveData::default();
    let cloned = data.clone();
    assert_eq!(cloned.version, data.version);
    assert_eq!(cloned.next_profile_id, data.next_profile_id);
}

// ============================================================================
// Serialization Tests
// ============================================================================

#[test]
fn test_player_save_data_serialization() {
    let original = PlayerSaveData {
        level: 5,
        xp: 1000,
        gold: 500,
        completed_dungeons: vec!["dungeon_01".into()],
    };

    let encoded = bincode::serialize(&original).expect("serialize");
    let decoded: PlayerSaveData = bincode::deserialize(&encoded).expect("deserialize");

    assert_eq!(decoded.level, original.level);
    assert_eq!(decoded.xp, original.xp);
    assert_eq!(decoded.gold, original.gold);
    assert_eq!(decoded.completed_dungeons, original.completed_dungeons);
}

#[test]
fn test_save_data_round_trip() {
    let profile = PlayerProfile {
        id: 1,
        name: "Hero".to_string(),
        class: CharacterClass::Warrior,
        level: 5,
        xp: 500,
        play_time: 3600.0,
        gold: 1000,
        completed_dungeons: vec!["dungeon_1".to_string()],
    };

    let profiles = PlayerProfiles {
        profiles: vec![profile],
        next_id: 2,
    };

    let meta = MetaProgression {
        dark_essence: 200,
        gold: 500,
        total_runs: 10,
        completed_runs: 3,
        highest_wave: 15,
        unlocks: vec!["dagger".to_string()],
        upgrades: vec![UpgradeTier {
            id: "max_hp_up".to_string(),
            tier: 2,
            cost: 200,
        }],
    };

    let original = SaveData {
        version: 2,
        profiles: profiles.profiles.clone(),
        next_profile_id: profiles.next_id,
        meta: meta.clone(),
    };

    let encoded = bincode::serialize(&original).expect("serialize save data");
    let decoded: SaveData = bincode::deserialize(&encoded).expect("deserialize save data");

    assert_eq!(decoded.version, original.version);
    assert_eq!(decoded.profiles.len(), 1);
    assert_eq!(decoded.profiles[0].name, "Hero");
    assert_eq!(decoded.meta.dark_essence, 200);
    assert_eq!(decoded.next_profile_id, 2);
}

#[test]
fn test_empty_save_data_serialization() {
    let original = SaveData::default();
    let encoded = bincode::serialize(&original).expect("serialize empty");
    let decoded: SaveData = bincode::deserialize(&encoded).expect("deserialize empty");
    assert_eq!(decoded.version, original.version);
    assert!(decoded.profiles.is_empty());
}

// ============================================================================
// Resource Tests
// ============================================================================

#[test]
fn test_save_state_default() {
    let state = SaveState::default();
    assert!(state.data.is_none());
}

#[test]
fn test_pending_save_default() {
    let pending = PendingSave::default();
    assert!(!pending.0);
}

// ============================================================================
// Profile Definition Tests
// ============================================================================

#[test]
fn test_player_profile_construction() {
    let profile = PlayerProfile {
        id: 1,
        name: "TestHero".to_string(),
        class: CharacterClass::Mage,
        level: 3,
        xp: 200,
        play_time: 900.0,
        gold: 200,
        completed_dungeons: vec![],
    };
    assert_eq!(profile.class, CharacterClass::Mage);
    assert_eq!(profile.level, 3);
}

#[test]
fn test_player_profiles_default() {
    let profiles = PlayerProfiles::default();
    assert!(profiles.profiles.is_empty());
    assert_eq!(profiles.next_id, 0);
}
