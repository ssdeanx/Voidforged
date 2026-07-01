use bevy::prelude::*;
use crate::components::*;
use crate::events::*;
use crate::resources::*;

/// Small timer to ensure loading screen shows for at least one frame.
#[derive(Resource)]
pub struct LoadingTimer(pub f32);

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
            .init_resource::<CursorWorldPos>()
            .init_resource::<CameraTransform>()
            .init_resource::<PlayTimer>()
            .init_resource::<DungeonState>()
            .init_resource::<ScreenShake>()

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
            .add_event::<DamageNumberEvent>()
            .add_event::<SpawnImpactEvent>()

            // Loading timer
            .insert_resource(LoadingTimer(0.5))

            // Loading → MainMenu transition after assets are created
            .add_systems(OnEnter(AppState::Loading), (
                start_loading_timer,
            ))
            .add_systems(Update, (
                finish_loading.run_if(in_state(AppState::Loading)),
            ))

            // Game loop systems
            .add_systems(Update, (
                toggle_pause,
                handle_player_death,
                update_play_timer,
                wave_announcer,
            ));
    }
}

/// Ticks the play timer while in a game state (World, Dungeon, or Playing).
fn update_play_timer(
    time: Res<Time>,
    mut timer: ResMut<PlayTimer>,
    state: Res<State<AppState>>,
    progression: Res<RunProgression>,
) {
    match *state.get() {
        AppState::World | AppState::Dungeon | AppState::Playing => {
            timer.0 += time.delta_secs();
        }
        AppState::GameOver => {
            // Freeze timer — don't reset
        }
        _ => {
            timer.0 = progression.run_time;
        }
    }
}
/// Toggle pause with Escape key in World, Dungeon, or Playing.
fn toggle_pause(
    input: Res<PlayerInput>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !input.pause {
        return;
    }
    match *state.get() {
        AppState::World | AppState::Dungeon | AppState::Playing => {
            next_state.set(AppState::Paused);
        }
        AppState::Paused => {
            next_state.set(AppState::Playing);
        }
        _ => {}
    }
}

/// Transition to GameOver when the player dies.
fn handle_player_death(
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut run_end_events: EventWriter<RunEndEvent>,
    wave_state: Res<WaveState>,
    mut progression: ResMut<RunProgression>,
    state: Res<State<AppState>>,
    time: Res<Time>,
    play_timer: Res<PlayTimer>,
) {
    if *state.get() != AppState::Playing && *state.get() != AppState::Dungeon && *state.get() != AppState::World {
        return;
    }
    let health = match player_query.get_single() {
        Ok(h) => h,
        Err(_) => return,
    };
    if !health.is_alive() && time.elapsed_secs_f64() as f32 >= health.invulnerable_until {
        progression.run_time = play_timer.0;
        run_end_events.send(RunEndEvent {
            victory: false,
            wave_reached: wave_state.wave_number,
            kills: progression.kills,
            run_time: progression.run_time,
        });
        next_state.set(AppState::GameOver);
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
        info!("=== WAVE {} CLEARED ===\n", event.wave_number);
    }
}

/// Initializes the loading timer when entering Loading state.
fn start_loading_timer(mut timer: ResMut<LoadingTimer>) {
    timer.0 = 0.5;
}

/// Waits for the timer to elapse then transitions to MainMenu.
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
