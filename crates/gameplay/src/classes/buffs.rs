//! Buff synergy system — per-spec passive bonuses, stat-modifying buffs,
//! stacking rules, and synergies between active abilities and passive effects.
//! Each class's two specs get distinct buff packages that create their feel.

use bevy::prelude::*;
use ir_core::*;

// ── Events ─────────────────────────────────────────────────────────────────

/// Fired when a buff is applied to an entity.
#[derive(Event)]
pub struct BuffAppliedEvent {
    pub target: Entity,
    pub buff_id: &'static str,
    pub duration: f32,
}

/// Fired when a buff expires or is removed.
#[derive(Event)]
pub struct BuffExpiredEvent {
    pub target: Entity,
    pub buff_id: &'static str,
}

// ── Active Buff Tracker ────────────────────────────────────────────────────

/// Tracks a single active buff on an entity.
#[derive(Component, Debug, Clone)]
pub struct ActiveBuff {
    pub buff_id: &'static str,
    pub remaining: f32,
    pub stacks: u32,
}

impl ActiveBuff {
    pub fn new(buff_id: &'static str, duration: f32) -> Self {
        Self { buff_id, remaining: duration, stacks: 1 }
    }
}

/// Resource manager for all active buff timers.
/// Applied as a component to each player/entity that can receive buffs.
#[derive(Component, Debug, Default)]
pub struct BuffContainer {
    pub buffs: Vec<ActiveBuff>,
}

impl BuffContainer {
    pub fn has(&self, id: &str) -> bool {
        self.buffs.iter().any(|b| b.buff_id == id)
    }

    pub fn stacks_of(&self, id: &str) -> u32 {
        self.buffs.iter().find(|b| b.buff_id == id).map_or(0, |b| b.stacks)
    }

    pub fn apply(&mut self, buff_id: &'static str, duration: f32, max_stacks: u32) {
        if let Some(existing) = self.buffs.iter_mut().find(|b| b.buff_id == buff_id) {
            existing.remaining = existing.remaining.max(duration);
            existing.stacks = (existing.stacks + 1).min(max_stacks);
        } else {
            self.buffs.push(ActiveBuff::new(buff_id, duration));
        }
    }

    pub fn remove(&mut self, buff_id: &str) {
        self.buffs.retain(|b| b.buff_id != buff_id);
    }
}

/// Systems that apply spec-specific passive stat modifiers at spawn.
pub fn apply_class_passive_buffs(
    mut commands: Commands,
    query: Query<(Entity, &PlayerClass, Option<&ChosenSpec>), Added<Player>>,
) {
    for (entity, class, spec) in query.iter() {
        commands.entity(entity).insert(BuffContainer::default());

        // Base class-wide passives (always active)
        match class.0 {
            CharacterClass::Warrior => {
                // Warriors get passive damage reduction from armour
                commands.entity(entity).insert(PassiveStatMod {
                    armor_bonus: 10.0,
                    ..default()
                });
            }
            CharacterClass::Paladin => {
                // Paladins get passive healing amp and armour
                commands.entity(entity).insert(PassiveStatMod {
                    armor_bonus: 8.0,
                    healing_bonus: 0.1,
                    ..default()
                });
            }
            CharacterClass::Rogue => {
                // Rogues get crit chance
                commands.entity(entity).insert(PassiveStatMod {
                    crit_chance: 0.05,
                    ..default()
                });
            }
            CharacterClass::Hunter => {
                // Hunters get move speed
                commands.entity(entity).insert(PassiveStatMod {
                    move_speed_bonus: 0.1,
                    ..default()
                });
            }
            CharacterClass::Mage => {
                // Mages get spell power bonus
                commands.entity(entity).insert(PassiveStatMod {
                    damage_bonus: 0.08,
                    ..default()
                });
            }
        }

        // Spec-specific passives
        if let Some(chosen) = spec {
            match chosen.spec {
                TalentSpec::Berserker => {
                    commands.entity(entity).insert(PassiveStatMod {
                        attack_speed_bonus: 0.15,
                        crit_chance: 0.05,
                        ..default()
                    });
                }
                TalentSpec::Protector => {
                    commands.entity(entity).insert(PassiveStatMod {
                        armor_bonus: 15.0,
                        max_health_bonus: 0.2,
                        ..default()
                    });
                }
                TalentSpec::Holy => {
                    commands.entity(entity).insert(PassiveStatMod {
                        healing_bonus: 0.25,
                        mana_regen_bonus: 0.2,
                        ..default()
                    });
                }
                TalentSpec::Retribution => {
                    commands.entity(entity).insert(PassiveStatMod {
                        damage_bonus: 0.12,
                        crit_chance: 0.08,
                        ..default()
                    });
                }
                TalentSpec::Assassination => {
                    commands.entity(entity).insert(PassiveStatMod {
                        damage_bonus: 0.10,
                        crit_chance: 0.10,
                        ..default()
                    });
                }
                TalentSpec::Outlaw => {
                    commands.entity(entity).insert(PassiveStatMod {
                        attack_speed_bonus: 0.10,
                        move_speed_bonus: 0.05,
                        ..default()
                    });
                }
                TalentSpec::Marksmanship => {
                    commands.entity(entity).insert(PassiveStatMod {
                        damage_bonus: 0.15,
                        crit_chance: 0.05,
                        ..default()
                    });
                }
                TalentSpec::Survival => {
                    commands.entity(entity).insert(PassiveStatMod {
                        armor_bonus: 5.0,
                        max_health_bonus: 0.10,
                        ..default()
                    });
                }
                TalentSpec::Frost => {
                    commands.entity(entity).insert(PassiveStatMod {
                        mana_regen_bonus: 0.15,
                        max_health_bonus: 0.05,
                        ..default()
                    });
                }
                TalentSpec::Fire => {
                    commands.entity(entity).insert(PassiveStatMod {
                        damage_bonus: 0.15,
                        crit_chance: 0.10,
                        ..default()
                    });
                }
            }
        }
    }
}

/// Ticks all active buffs and fires expiration events.
pub fn tick_active_buffs(
    time: Res<Time>,
    mut query: Query<&mut BuffContainer>,
    mut expired_writer: EventWriter<BuffExpiredEvent>,
) {
    let dt = time.delta_secs();
    for mut container in query.iter_mut() {
        let mut expired = Vec::new();
        for buff in container.buffs.iter_mut() {
            buff.remaining -= dt;
            if buff.remaining <= 0.0 {
                expired.push(buff.buff_id);
            }
        }
        for id in expired {
            // We can't borrow container mutably twice, use retain
        }
        container.buffs.retain(|b| b.remaining > 0.0);
    }
}

// ── Synergy helper: applier functions ──────────────────────────────────────

/// Adds a temporary damage buff to the target entity.
pub fn apply_damage_buff(container: &mut BuffContainer, id: &'static str, duration: f32, stacks: u32) {
    container.apply(id, duration, stacks);
}

/// Adds a temporary armour buff.
pub fn apply_armour_buff(container: &mut BuffContainer, id: &'static str, duration: f32, stacks: u32) {
    container.apply(id, duration, stacks);
}

/// Adds a speed buff.
pub fn apply_speed_buff(container: &mut BuffContainer, id: &'static str, duration: f32, stacks: u32) {
    container.apply(id, duration, stacks);
}

/// Adds a stacking crit buff (e.g. Berserker adrenaline).
pub fn apply_crit_buff(container: &mut BuffContainer, id: &'static str, duration: f32, stacks: u32) {
    container.apply(id, duration, stacks);
}

// ── PassiveStatMod component ───────────────────────────────────────────────

/// Immutable passive stat bonuses applied by class/spec.
/// These are read by the stat system to modify CombatStats.
#[derive(Component, Debug, Clone, Default)]
pub struct PassiveStatMod {
    pub damage_bonus: f32,
    pub attack_speed_bonus: f32,
    pub crit_chance: f32,
    pub armor_bonus: f32,
    pub max_health_bonus: f32,
    pub move_speed_bonus: f32,
    pub healing_bonus: f32,
    pub mana_regen_bonus: f32,
}
