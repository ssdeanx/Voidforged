use bevy::prelude::*;
use ir_core::{HitStun, Velocity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimState {
    /// Character is standing still with no velocity.
    Idle,
    /// Character is moving (velocity magnitude > 0.1).
    Running,
    /// Character is performing an attack (triggered externally by the ability system).
    Attacking,
    /// Character is reacting to damage (hitstun active).
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

#[derive(Component, Debug, Clone)]
pub struct AnimationStateMachine {
    /// The currently active animation state.
    pub current: AnimState,
    /// The previous animation state (used for transition detection).
    pub previous: AnimState,
    /// Time elapsed (in seconds) since the last state transition.
    pub state_time: f32,
    /// Duration (in seconds) the attack state persists before reverting.
    pub attack_duration: f32,
    /// Whether the character is currently attacking (externally triggered).
    pub is_attacking: bool,
    /// If true, the state machine updates timing but does not change states.
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
        Self { attack_duration: duration, ..Default::default() }
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
        if self.previous == self.current { return 0.0; }
        (self.state_time / crossfade_duration).min(1.0)
    }
}

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
        if asm.paused { asm.state_time += dt; continue; }
        asm.state_time += dt;
        if let Some(hs) = hitstun {
            if hs.remaining > 0.0 {
                if asm.current != AnimState::Hit { asm.transition(AnimState::Hit); }
                continue;
            }
        }
        if asm.is_attacking {
            if asm.current != AnimState::Attacking { asm.transition(AnimState::Attacking); }
            if asm.state_time >= asm.attack_duration {
                asm.is_attacking = false;
                if let Some(vel) = velocity {
                    if vel.0.length_squared() > 0.1 { asm.transition(AnimState::Running); }
                    else { asm.transition(AnimState::Idle); }
                } else { asm.transition(AnimState::Idle); }
            }
            continue;
        }
        if let Some(vel) = velocity {
            if vel.0.length_squared() > 0.1 && asm.current == AnimState::Idle {
                asm.transition(AnimState::Running);
            } else if vel.0.length_squared() <= 0.1 && asm.current == AnimState::Running {
                asm.transition(AnimState::Idle);
            }
        }
    }
}
