use bevy::prelude::*;
use crate::resources::*;
use crate::events::*;

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
            .add_event::<RunEndEvent>();
    }
}
