//! Sound system skeleton — stub functions that log events.
//!
//! When audio assets are ready from Blender, replace the `info!` stubs
//! with actual `bevy_kira_audio` or `bevy_audio` calls.
//!
//! This module is private to the rendering crate.

use bevy::prelude::*;
use ir_core::*;

/// Placeholder audio plugin that logs sound events.
pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            stub_play_hit_sound,
            stub_play_death_sound,
            stub_play_impact_sound,
        ));
    }
}

/// Log on DamageEvent (play hit sound).
fn stub_play_hit_sound(mut events: EventReader<DamageEvent>) {
    for event in events.read() {
        info!(
            "[Audio] Play hit sound — target={:?} amount={:.1} crit={}",
            event.target, event.amount, event.is_critical
        );
    }
}

/// Log on DeathEvent (play death sound).
fn stub_play_death_sound(mut events: EventReader<DeathEvent>) {
    for event in events.read() {
        info!(
            "[Audio] Play death sound — entity={:?} variant={:?}",
            event.entity, event.enemy_variant
        );
    }
}

/// Log on SpawnImpactEvent (play impact sound).
fn stub_play_impact_sound(mut events: EventReader<SpawnImpactEvent>) {
    for event in events.read() {
        info!(
            "[Audio] Play impact sound — position={:.1?}",
            event.position
        );
    }
}
