use bevy::animation::graph::{AnimationGraph, AnimationNodeIndex, AnimationNodeType};
use bevy::prelude::*;
use ir_core::{HitStun, Velocity};

/// Animation states for character/enemy models.
///
/// These map to GLTF animation clip names within the loaded model file.
/// Each variant's `as_str()` returns the key used in `AssetPipelineConfig.animations`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimState {
    Idle,
    Running,
    Attacking,
    Hit,
}

impl AnimState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Running => "run",
            Self::Attacking => "attack",
            Self::Hit => "hit",
        }
    }
}

/// Animation state machine component.
///
/// Attached to player and enemy entities. The `tick_animation_state_machine`
/// system reads `Velocity` and `HitStun` to decide transitions, and downstream
/// systems (or `tick_animation_clips`) apply the resulting state to the model.
#[derive(Component, Debug, Clone)]
pub struct AnimationStateMachine {
    pub current: AnimState,
    pub previous: AnimState,
    pub state_time: f32,
    pub attack_duration: f32,
    pub is_attacking: bool,
    pub paused: bool,
}

impl Default for AnimationStateMachine {
    fn default() -> Self {
        Self {
            current: AnimState::Idle,
            previous: AnimState::Idle,
            state_time: 0.0,
            attack_duration: 0.4,
            is_attacking: false,
            paused: false,
        }
    }
}

impl AnimationStateMachine {
    pub fn with_attack_duration(duration: f32) -> Self {
        Self {
            attack_duration: duration,
            ..Default::default()
        }
    }

    pub fn transition(&mut self, new_state: AnimState) {
        if self.current != new_state {
            self.previous = self.current;
            self.current = new_state;
            self.state_time = 0.0;
        }
    }

    pub fn just_transitioned(&self) -> bool {
        self.previous != self.current && self.state_time < 0.016
    }

    pub fn transition_blend(&self, crossfade_duration: f32) -> f32 {
        if self.previous == self.current {
            return 0.0;
        }
        (self.state_time / crossfade_duration).min(1.0)
    }
}

/// Drives the `AnimationStateMachine` from gameplay state.
///
/// Reads `Velocity` (for idle↔run transitions) and `HitStun` (for hit
/// reactions). When `is_attacking` is set externally (by an ability system),
/// transitions to `Attacking` state and back to idle/run after the
/// `attack_duration` window.
pub fn tick_animation_state_machine(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationStateMachine,
        Option<&Velocity>,
        Option<&HitStun>,
    )>,
) {
    let dt = time.delta_secs();
    for (mut asm, velocity, hitstun) in query.iter_mut() {
        if asm.paused {
            asm.state_time += dt;
            continue;
        }
        asm.state_time += dt;

        // Hitstun overrides everything
        if let Some(hs) = hitstun {
            if hs.remaining > 0.0 {
                if asm.current != AnimState::Hit {
                    asm.transition(AnimState::Hit);
                }
                continue;
            }
        }

        // Attacking state with timer
        if asm.is_attacking {
            if asm.current != AnimState::Attacking {
                asm.transition(AnimState::Attacking);
            }
            if asm.state_time >= asm.attack_duration {
                asm.is_attacking = false;
                if let Some(vel) = velocity {
                    if vel.0.length_squared() > 0.1 {
                        asm.transition(AnimState::Running);
                    } else {
                        asm.transition(AnimState::Idle);
                    }
                } else {
                    asm.transition(AnimState::Idle);
                }
            }
            continue;
        }

        // Idle ↔ Running based on velocity
        if let Some(vel) = velocity {
            if vel.0.length_squared() > 0.1 && asm.current == AnimState::Idle {
                asm.transition(AnimState::Running);
            } else if vel.0.length_squared() <= 0.1 && asm.current == AnimState::Running {
                asm.transition(AnimState::Idle);
            }
        }
    }
}

/// Applies animation clip changes to GLTF models based on `AnimationStateMachine`.
///
/// When the state machine transitions to a new `AnimState`, this system
/// searches the entity's `AnimationPlayer` graph for clip nodes and plays
/// them by order convention (0=idle, 1=run, 2=attack, 3=hit).
///
/// Blender exports should export animations in that order (idle, run, attack, hit).
/// For models with fewer clips, the last clip is held for higher states.
///
/// If no `AnimationPlayer` is present (e.g., placeholder quads or models
/// without animations), this system is a no-op — the placeholder remains static.
pub fn tick_animation_clips(
    parent_query: Query<(&AnimationStateMachine, &Children)>,
    mut child_query: Query<(&mut AnimationPlayer, &AnimationGraphHandle)>,
    animation_graphs: Res<Assets<AnimationGraph>>,
) {
    for (asm, children) in parent_query.iter() {
        if !asm.just_transitioned() {
            continue;
        }
        for &child in children.iter() {
            if let Ok((mut player, graph_handle)) = child_query.get_mut(child) {
                let Some(graph) = animation_graphs.get(graph_handle) else {
                    break;
                };
                // Collect clip node indices (exclude blend/add nodes)
                let clip_nodes: Vec<AnimationNodeIndex> = graph
                    .nodes()
                    .filter(|&idx| {
                        graph
                            .get(idx)
                            .is_some_and(|n| matches!(n.node_type, AnimationNodeType::Clip(_)))
                    })
                    .collect();
                if clip_nodes.is_empty() {
                    break;
                }
                // Convention: 0=idle, 1=run, 2=attack, 3=hit
                let idx = match asm.current {
                    AnimState::Idle => 0,
                    AnimState::Running => 1.min(clip_nodes.len() - 1),
                    AnimState::Attacking => 2.min(clip_nodes.len() - 1),
                    AnimState::Hit => 3.min(clip_nodes.len() - 1),
                };
                player.stop_all();
                player.play(clip_nodes[idx]).repeat();
                break;
            }
        }
    }
}
