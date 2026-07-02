//! Core plugin that registers all resources, events, and startup systems.

use crate::components::*;
use crate::db::{auto_save, init_save_db, save_on_quit};
use crate::events::*;
use crate::resources::*;
use bevy::prelude::*;

/// Small timer to ensure the loading screen shows for at least one frame.
#[derive(Resource)]
pub struct LoadingTimer(pub f32);

impl Default for LoadingTimer {
    fn default() -> Self {
        Self(0.2)
    }
}

/// The core plugin that wires up all shared game infrastructure.
///
/// Registers:
/// - State machines: [`AppState`], [`RunState`]
/// - Resources: [`PlayerInput`], [`PlayerProfiles`], [`CharacterCreationState`],
///   [`ItemDatabase`], [`MetaProgression`], [`RunProgression`], [`WaveState`],
///   [`LoadingTimer`], [`PlayTimer`], [`DungeonState`], [`ScreenShake`],
///   [`DeathPenalty`], [`Graveyard`]
/// - Events: All combat, progression, wave, equipment, and death events
/// - Startup systems: [`init_item_database`], [`init_save_db`]
/// - Core gameplay systems: loading screen, pause toggle, player death handling,
///   play timer, wave announcer
/// - Save systems: periodic auto-save and quit-save
///
/// This plugin must be added first by the app builder before any gameplay plugins.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .init_state::<AppState>()
            .init_state::<RunState>()

            // Resources
            .init_resource::<PlayerInput>()
            .init_resource::<PlayerProfiles>()
            .init_resource::<CharacterCreationState>()
            .init_resource::<ItemDatabase>()
            .init_resource::<MetaProgression>()
            .init_resource::<RunProgression>()
            .init_resource::<WaveState>()
            .init_resource::<LoadingTimer>()
            .init_resource::<PlayTimer>()
            .init_resource::<DungeonState>()
            .init_resource::<ScreenShake>()
            .init_resource::<DeathPenalty>()
            .init_resource::<Graveyard>()

            // Events
            .add_event::<DamageEvent>()
            .add_event::<DeathEvent>()
            .add_event::<PlayerDeathEvent>()
            .add_event::<DungeonEndEvent>()
            .add_event::<EquipItemEvent>()
            .add_event::<UnequipItemEvent>()
            .add_event::<WaveStartEvent>()
            .add_event::<WaveClearedEvent>()
            .add_event::<RunEndEvent>()
            .add_event::<DamageNumberEvent>()
            .add_event::<SpawnImpactEvent>()
            .add_event::<SpawnDeathEffectEvent>()
            .add_event::<HitDirectionEvent>()

            // Startup — populate item database + open save DB
            .add_systems(Startup, (init_item_database, init_save_db))

            // Loading → MainMenu transition
            .add_systems(OnEnter(AppState::Loading), (start_loading_timer,))
            .add_systems(Update, (finish_loading.run_if(in_state(AppState::Loading)),))

            // Game loop systems
            .add_systems(Update, (
                toggle_pause,
                handle_player_death,
                update_play_timer,
                wave_announcer,
            ))

            // Auto-save every 30s during gameplay
            .add_systems(Update, auto_save.run_if(in_state(AppState::Playing)))

            // Save-on-quit
            .add_systems(Last, save_on_quit);
    }
}

// ── Loading screen ──────────────────────────────────────────────────────

fn start_loading_timer(mut commands: Commands) {
    commands.insert_resource(LoadingTimer(0.2));
}

fn finish_loading(
    time: Res<Time>,
    mut timer: ResMut<LoadingTimer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    timer.0 -= time.delta_secs();
    if timer.0 <= 0.0 {
        next_state.set(AppState::MainMenu);
    }
}

// ── Pause ───────────────────────────────────────────────────────────────

fn toggle_pause(
    input: Res<PlayerInput>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if input.pause {
        let next = match *state.get() {
            AppState::Paused => AppState::Playing,
            AppState::Playing | AppState::World | AppState::Dungeon => AppState::Paused,
            _ => return,
        };
        next_state.set(next);
    }
}

// ── Player death → GameOver ─────────────────────────────────────────────

fn handle_player_death(
    mut player_death_events: EventWriter<PlayerDeathEvent>,
    health_query: Query<(Entity, &Health, &Transform), With<Player>>,
    dungeon_state: Res<DungeonState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut shake: ResMut<ScreenShake>,
) {
    let (entity, health, transform) = match health_query.get_single() {
        Ok(h) => h,
        Err(_) => return,
    };
    // Check if health just hit zero this frame
    if health.is_alive() {
        return;
    }

    let in_dungeon = dungeon_state.current.is_some();

    // Fire player death event
    player_death_events.send(PlayerDeathEvent {
        player: entity,
        killer: None,
        position: transform.translation,
        in_dungeon,
    });

    // Screen shake + transition
    shake.trauma = 1.0;
    info!("Player died! Sending to graveyard/end screen");

    if in_dungeon {
        next_state.set(AppState::GameOver);
    } else {
        // Open world: respawn at graveyard (handled by gameplay system)
        next_state.set(AppState::Playing);
    }
}

// ── Play timer ──────────────────────────────────────────────────────────

fn update_play_timer(
    state: Res<State<AppState>>,
    time: Res<Time>,
    mut play_timer: ResMut<PlayTimer>,
) {
    if !matches!(*state.get(), AppState::Playing | AppState::World | AppState::Dungeon) {
        return;
    }
    play_timer.0 += time.delta_secs();
}

// ── Wave announcer (placeholder) ────────────────────────────────────────

fn wave_announcer(
    mut events: EventReader<WaveStartEvent>,
    _wave_cleared: EventWriter<WaveClearedEvent>,
) {
    for _event in events.read() {
        // Placeholder — actual wave logic in dungeon/gameplay
    }
}
