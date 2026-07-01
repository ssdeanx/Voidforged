use crate::{classes, collection, combat, death, enemy, equipment, loot, pickup, player};
use bevy::prelude::*;
use ir_core::*;


/// The top-level [`Plugin`] that registers all gameplay systems.
///
/// Wires player input and movement, class ability dispatchers, enemy AI,
/// the combat pipeline (projectiles, hitboxes, damage, death), loot
/// spawning, pickup collection, equipment, stamina, and death/respawn
/// into the Bevy application.
pub struct GameplayPlugin;

/// Run condition: player can move in World, Dungeon, and Playing.
fn can_move(state: Res<State<AppState>>) -> bool {
    matches!(
        *state.get(),
        AppState::World | AppState::Dungeon | AppState::Playing
    )
}

/// Run condition: combat in all game states (World, Dungeon, Playing).
fn has_combat(state: Res<State<AppState>>) -> bool {
    matches!(
        *state.get(),
        AppState::World | AppState::Dungeon | AppState::Playing
    )
}

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            // Apply equipment when player spawns in any game state
            .add_systems(OnEnter(AppState::World), (player::apply_equipment,))
            .add_systems(OnEnter(AppState::Dungeon), (player::apply_equipment,))
            .add_systems(OnEnter(AppState::Playing), (player::apply_equipment,))
            // ── Player movement ─────────────────────────────────────
<<<<<<< HEAD
            .add_systems(Update, (
                player::read_player_input,
                player::player_movement,
                player::apply_player_velocity,
                player::player_world_collision,
            ).run_if(can_move))

            // ── Class ability dispatchers ───────────────────────────
            .add_systems(Update, (
                classes::primary_attack,
                classes::secondary_attack,
                classes::cast_ability,
                classes::dash_ability,
                classes::class_resource_regen,
                classes::tick_ability_cooldowns,
            ).run_if(has_combat))

=======
            .add_systems(
                Update,
                (
                    player::read_player_input,
                    player::player_movement,
                    player::apply_player_velocity,
                )
                    .run_if(can_move),
            )
            // ── Class ability dispatchers ───────────────────────────
            .add_systems(
                Update,
                (
                    classes::primary_attack,
                    classes::secondary_attack,
                    classes::cast_ability,
                    classes::dash_ability,
                    classes::class_resource_regen,
                )
                    .run_if(has_combat),
            )
>>>>>>> origin/master
            // ── Class-specific sub-systems ──────────────────────────
            .add_systems(
                Update,
                (classes::warrior::apply_charge_movement,).run_if(has_combat),
            )
            .add_systems(
                Update,
                (classes::paladin::tick_consecration,).run_if(has_combat),
            )
            .add_systems(Update, (classes::rogue::tick_poison,).run_if(has_combat))
            .add_systems(
                Update,
                (classes::rogue::apply_poison_damage,).run_if(has_combat),
            )
            .add_systems(
                Update,
                (classes::hunter::tick_trap_slow,).run_if(has_combat),
            )
            // ── Class resource generation ────────────────────────
            .add_systems(
                Update,
                (
                    classes::warrior::warrior_rage_on_damage,
                    classes::warrior::warrior_rage_on_take_damage,
                    classes::paladin::paladin_holy_power_on_hit,
                )
                    .run_if(has_combat),
            )
            // ── Class buff systems ─────────────────────────────────
            .add_systems(
                Update,
                (
                    classes::paladin::apply_holy_light,
                    classes::warrior::tick_shield_block,
                    classes::warrior::cleanup_shield_block,
                )
                    .run_if(has_combat),
            )
            // ── Enemy systems ───────────────────────────────────────
            .add_systems(
                Update,
                (
                    enemy::enemy_ai,
                    enemy::boss_phase_ai,
                    enemy::apply_enemy_velocity,
                    enemy::enemy_melee_attack,
                    enemy::enemy_ranged_attack,
                )
                    .run_if(has_combat),
            )
            // ── Combat pipeline ─────────────────────────────────────
<<<<<<< HEAD
            .add_systems(Update, (
                combat::move_projectiles,
                combat::projectile_hit,
                combat::projectile_hit_player,
                combat::apply_damage,
                combat::apply_knockback,
                combat::apply_stun_movement_block,
                combat::handle_death,
            ).chain().run_if(has_combat))

            // ── Status effect tick systems ──────────────────────────
            .add_systems(Update, (
                combat::tick_frozen,
                combat::tick_stun,
                combat::tick_hit_stun,
                combat::tick_hit_stop,
                combat::tick_hit_flash,
            ).run_if(has_combat))

=======
            .add_systems(
                Update,
                (
                    combat::move_projectiles,
                    combat::projectile_hit,
                    combat::projectile_hit_player,
                    combat::apply_damage,
                    combat::apply_knockback,
                    combat::apply_stun_movement_block,
                    loot::spawn_loot_from_table,
                    combat::handle_death,
                )
                    .chain()
                    .run_if(has_combat),
            )
            // ── Status effect tick systems ──────────────────────────
            .add_systems(
                Update,
                (
                    combat::tick_frozen,
                    combat::tick_stun,
                    combat::tick_hit_stun,
                    combat::tick_hit_stop,
                )
                    .run_if(has_combat),
            )
>>>>>>> origin/master
            // ── Hitbox processing ──────────────────────────────────
            .add_systems(
                Update,
                (combat::process_hitboxes, combat::process_enemy_hitboxes).run_if(has_combat),
            )
            // ── Pickup systems ──────────────────────────────────────
<<<<<<< HEAD
            .add_systems(Update, (
                pickup::gem_magnet,
                pickup::collect_health_pickups,
                pickup::collect_gold_pickups,
                pickup::item_pickup_interaction,
                collection::collect_gems,
            ).run_if(has_combat))

            // ── Stamina systems ───────────────────────────────────
            .add_systems(Update, (
                combat::stamina_regen,
            ).run_if(has_combat))

=======
            .add_systems(
                Update,
                (
                    pickup::gem_magnet,
                    pickup::collect_health_pickups,
                    pickup::collect_gold_pickups,
                    collection::collect_gems,
                )
                    .run_if(has_combat),
            )
            // ── Stamina systems ───────────────────────────────────
            .add_systems(
                Update,
                (combat::stamina_regen, combat::sprint_stamina_drain).run_if(has_combat),
            )
>>>>>>> origin/master
            // ── Equipment systems ──────────────────────────────────
            .add_systems(
                Update,
                (
                    equipment::handle_equip_event,
                    equipment::handle_unequip_event,
                )
                    .run_if(has_combat),
            )
            // ── Death & Respawn ────────────────────────────────────
            .add_systems(Update, death::handle_player_death_event);
    }
}
