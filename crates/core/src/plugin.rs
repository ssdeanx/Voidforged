use bevy::prelude::*;
use crate::components::*;
use crate::events::*;
use crate::resources::*;

/// Registers all core types, resources, events, and systems.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .init_state::<AppState>()
            .init_state::<RunState>()

            // Resources
            .init_resource::<PlayerInput>()
            .init_resource::<WaveState>()
            .init_resource::<RunProgression>()
            .init_resource::<MetaProgression>()
            .init_resource::<GameConfig>()

            // Events
            .add_event::<DamageEvent>()
            .add_event::<DeathEvent>()
            .add_event::<ExperienceGainEvent>()
            .add_event::<LevelUpEvent>()
            .add_event::<PickupEvent>()
            .add_event::<WaveStartEvent>()
            .add_event::<WaveClearedEvent>()
            .add_event::<RoomTransitionEvent>()
            .add_event::<RunStartEvent>()
            .add_event::<RunEndEvent>()

            // Game loop systems
            .add_systems(Update, (
                toggle_pause,
                handle_player_death,
                wave_announcer,
            ));
    }
}

/// Toggle pause with Escape key.
fn toggle_pause(
    input: Res<PlayerInput>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !input.pause {
        return;
    }
    match state.get() {
        AppState::Playing => next_state.set(AppState::Paused),
        AppState::Paused => next_state.set(AppState::Playing),
        _ => {}
    }
}

/// Transition to GameOver when the player dies.
fn handle_player_death(
    mut death_events: EventReader<DeathEvent>,
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut run_end_events: EventWriter<RunEndEvent>,
    wave_state: Res<WaveState>,
    progression: Res<RunProgression>,
) {
    // Check if player died via death event
    for event in death_events.read() {
        if let Ok(_health) = player_query.get(event.entity) {
            // Player is dead
            run_end_events.send(RunEndEvent {
                victory: false,
                wave_reached: wave_state.wave_number,
                kills: progression.kills,
                run_time: progression.run_time,
            });
            next_state.set(AppState::GameOver);
            return;
        }
    }

    // Also check if player HP is zero (belt-and-suspenders)
    if let Ok(health) = player_query.get_single() {
        if !health.is_alive() {
            run_end_events.send(RunEndEvent {
                victory: false,
                wave_reached: wave_state.wave_number,
                kills: progression.kills,
                run_time: progression.run_time,
            });
            next_state.set(AppState::GameOver);
        }
    }
}

/// Log wave events to console as placeholder announcements.
fn wave_announcer(
    mut wave_start_events: EventReader<WaveStartEvent>,
    mut wave_cleared_events: EventReader<WaveClearedEvent>,
) {
    for event in wave_start_events.read() {
        info!("=== WAVE {} START — {} enemies ===", event.wave_number, event.enemy_count);
    }
    for event in wave_cleared_events.read() {
        info!("=== WAVE {} CLEARED ===", event.wave_number);
    }
}
