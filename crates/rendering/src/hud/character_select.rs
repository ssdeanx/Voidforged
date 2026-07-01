//! Character select screen — choose class, enter name, pick existing character.

use bevy::prelude::*;
use ir_core::*;
use crate::hud::components::*;

fn label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

fn btn_label(s: &str, size: f32) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(Color::srgb(0.9, 0.9, 0.95)),
    )
}

/// Spawns the full character select screen.
pub fn spawn_character_select(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.08, 1.0)),
            CharSelectRoot,
        ))
        .with_children(|root| {
            // ── Title ──────────────────────────────────────────────
            root.spawn((label("CHOOSE YOUR HERO", 38.0, Color::srgb(0.7, 0.5, 1.0)), CharSelectRoot));
            root.spawn((Node { height: Val::Px(16.0), ..default() }, CharSelectRoot));

            // ── Class cards row ────────────────────────────────────
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(12.0),
                    ..default()
                },
                CharSelectClassList,
                CharSelectRoot,
            )).with_children(|cards| {
                for class in CharacterClass::all() {
                    spawn_class_card(cards, class);
                }
            });

            root.spawn((Node { height: Val::Px(16.0), ..default() }, CharSelectRoot));

            // ── Stats preview panel ────────────────────────────────
            root.spawn((
                Node {
                    width: Val::Px(700.0),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(12.0)),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BorderColor(Color::srgb(0.3, 0.3, 0.4)),
                BackgroundColor(Color::srgba(0.1, 0.1, 0.13, 1.0)),
                CharSelectStatsPreview,
                CharSelectRoot,
            )).with_children(|preview| {
                preview.spawn((label("Select a class to see details", 16.0, Color::srgb(0.5, 0.5, 0.6)), CharSelectRoot));
            });

            root.spawn((Node { height: Val::Px(16.0), ..default() }, CharSelectRoot));

            // ── Name input area ────────────────────────────────────
            root.spawn((
                Node {
                    width: Val::Px(400.0),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                },
                CharSelectRoot,
            )).with_children(|name_row| {
                name_row.spawn((label("Name:", 20.0, Color::srgb(0.7, 0.7, 0.8)), CharSelectRoot));
                name_row.spawn((
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(34.0),
                        border: UiRect::all(Val::Px(1.0)),
                        padding: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    BorderColor(Color::srgb(0.4, 0.4, 0.5)),
                    BackgroundColor(Color::srgba(0.15, 0.15, 0.2, 1.0)),
                    CharSelectNameInput,
                    CharSelectRoot,
                )).with_children(|field| {
                    field.spawn((label("", 18.0, Color::srgb(0.9, 0.9, 1.0)), CharSelectNameInput, CharSelectRoot));
                });
            });

            root.spawn((Node { height: Val::Px(20.0), ..default() }, CharSelectRoot));

            // ── Confirm button ─────────────────────────────────────
            root.spawn((
                Node {
                    width: Val::Px(260.0),
                    height: Val::Px(48.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BorderColor(Color::srgb(0.4, 0.7, 1.0)),
                BackgroundColor(Color::srgba(0.1, 0.2, 0.35, 1.0)),
                CharSelectConfirmBtn,
                CharSelectRoot,
                Button,
            )).with_children(|btn| {
                btn.spawn((btn_label("ENTER THE WORLD", 22.0), CharSelectRoot));
            });

            root.spawn((Node { height: Val::Px(24.0), ..default() }, CharSelectRoot));

            // ── Existing characters section ────────────────────────
            root.spawn((label("─── Saved Characters ───", 18.0, Color::srgb(0.4, 0.4, 0.5)), CharSelectRoot));
            root.spawn((Node { height: Val::Px(10.0), ..default() }, CharSelectRoot));

            root.spawn((
                Node {
                    width: Val::Px(600.0),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    ..default()
                },
                CharSelectExistingList,
                CharSelectRoot,
            ));
        });
}

fn spawn_class_card(parent: &mut ChildBuilder, class: CharacterClass) {
    let data = class_stats_preview(class);
    let color = class_color(class);
    parent.spawn((
        Node {
            width: Val::Px(130.0),
            height: Val::Px(160.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(2.0)),
            row_gap: Val::Px(4.0),
            ..default()
        },
        BorderColor(color),
        BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 1.0)),
        CharSelectClassCard(class),
        CharSelectRoot,
        Button,
    )).with_children(|card| {
        card.spawn((label(class.display_name(), 18.0, color), CharSelectRoot));
        card.spawn((label(&format!("HP: {}", data.hp), 12.0, Color::srgb(0.6, 0.9, 0.6)), CharSelectRoot));
        card.spawn((label(&format!("Spd: {}", data.speed), 12.0, Color::srgb(0.5, 0.8, 1.0)), CharSelectRoot));
        card.spawn((label(&format!("Armor: {}", data.armor), 12.0, Color::srgb(0.8, 0.7, 0.5)), CharSelectRoot));
        card.spawn((label(data.weapon, 11.0, Color::srgb(0.7, 0.7, 0.7)), CharSelectRoot));
    });
}

struct ClassPreview {
    hp: u32,
    speed: f32,
    armor: u32,
    weapon: &'static str,
}

fn class_stats_preview(class: CharacterClass) -> ClassPreview {
    match class {
        CharacterClass::Warrior => ClassPreview { hp: 160, speed: 5.0, armor: 15, weapon: "Sword" },
        CharacterClass::Paladin => ClassPreview { hp: 140, speed: 4.8, armor: 12, weapon: "Sword" },
        CharacterClass::Rogue => ClassPreview { hp: 100, speed: 6.5, armor: 4, weapon: "Daggers" },
        CharacterClass::Hunter => ClassPreview { hp: 110, speed: 5.5, armor: 6, weapon: "Bow" },
        CharacterClass::Mage => ClassPreview { hp: 90, speed: 4.5, armor: 2, weapon: "Staff" },
    }
}

fn class_color(class: CharacterClass) -> Color {
    match class {
        CharacterClass::Warrior => Color::srgb(0.8, 0.3, 0.2),
        CharacterClass::Paladin => Color::srgb(0.9, 0.7, 0.3),
        CharacterClass::Rogue => Color::srgb(0.2, 0.7, 0.4),
        CharacterClass::Hunter => Color::srgb(0.2, 0.6, 0.3),
        CharacterClass::Mage => Color::srgb(0.3, 0.5, 1.0),
    }
}

// ── System: handle class card clicks ────────────────────────────────────────

/// Clicking a class card updates the selected class and shows details.
pub fn handle_class_selection(
    mut interaction_query: Query<(&Interaction, &CharSelectClassCard), Changed<Interaction>>,
    mut creation_state: ResMut<CharacterCreationState>,
    mut preview_text: Query<&mut Text, (With<CharSelectStatsPreview>, Without<CharSelectNameInput>)>,
) {
    for (interaction, card) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            creation_state.selected_class = Some(card.0);
            // Update preview text
            let class = card.0;
            let stats = class.base_stats();
            let desc = format!(
                "{}\n\n── Stats ──\nHP: {}  |  Speed: {}  |  Armor: {}\nDamage: +{}  |  Crit: {}%  |  Dodge: {}%\nResource: {}  |  Weapon: {:?}",
                class.description(),
                class.base_max_hp() as u32,
                stats.move_speed,
                stats.armor,
                stats.damage_bonus,
                (stats.crit_chance * 100.0) as u32,
                (stats.dodge_chance * 100.0) as u32,
                class.resource_name(),
                class.starting_weapon().kind,
            );
            for mut text in preview_text.iter_mut() {
                text.0 = desc.clone();
            }
        }
    }
}

// ── System: keyboard name input ────────────────────────────────────────────

/// Captures alphanumeric keys for character name input.
pub fn handle_name_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut creation_state: ResMut<CharacterCreationState>,
    mut name_text: Query<&mut Text, (With<CharSelectNameInput>, Without<CharSelectStatsPreview>)>,
) {
    let mut changed = false;

    // Letter keys (A-Z)
    for key in [
        KeyCode::KeyA, KeyCode::KeyB, KeyCode::KeyC, KeyCode::KeyD, KeyCode::KeyE,
        KeyCode::KeyF, KeyCode::KeyG, KeyCode::KeyH, KeyCode::KeyI, KeyCode::KeyJ,
        KeyCode::KeyK, KeyCode::KeyL, KeyCode::KeyM, KeyCode::KeyN, KeyCode::KeyO,
        KeyCode::KeyP, KeyCode::KeyQ, KeyCode::KeyR, KeyCode::KeyS, KeyCode::KeyT,
        KeyCode::KeyU, KeyCode::KeyV, KeyCode::KeyW, KeyCode::KeyX, KeyCode::KeyY,
        KeyCode::KeyZ,
    ] {
        if keyboard.just_pressed(key) {
            let c = match key {
                KeyCode::KeyA => 'a', KeyCode::KeyB => 'b', KeyCode::KeyC => 'c',
                KeyCode::KeyD => 'd', KeyCode::KeyE => 'e', KeyCode::KeyF => 'f',
                KeyCode::KeyG => 'g', KeyCode::KeyH => 'h', KeyCode::KeyI => 'i',
                KeyCode::KeyJ => 'j', KeyCode::KeyK => 'k', KeyCode::KeyL => 'l',
                KeyCode::KeyM => 'm', KeyCode::KeyN => 'n', KeyCode::KeyO => 'o',
                KeyCode::KeyP => 'p', KeyCode::KeyQ => 'q', KeyCode::KeyR => 'r',
                KeyCode::KeyS => 's', KeyCode::KeyT => 't', KeyCode::KeyU => 'u',
                KeyCode::KeyV => 'v', KeyCode::KeyW => 'w', KeyCode::KeyX => 'x',
                KeyCode::KeyY => 'y', KeyCode::KeyZ => 'z',
                _ => unreachable!(),
            };
            let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
            let c = if shift { c.to_ascii_uppercase() } else { c };
            if creation_state.player_name.len() < 20 {
                creation_state.player_name.push(c);
                changed = true;
            }
        }
    }

    // Space
    if keyboard.just_pressed(KeyCode::Space) {
        if creation_state.player_name.len() < 20 {
            creation_state.player_name.push(' ');
            changed = true;
        }
    }

    // Backspace
    if keyboard.just_pressed(KeyCode::Backspace) {
        creation_state.player_name.pop();
        changed = true;
    }

    if changed {
        for mut text in name_text.iter_mut() {
            let name = &creation_state.player_name;
            text.0 = if name.is_empty() {
                "Enter name...".to_string()
            } else {
                name.clone()
            };
        }
    }
}

// ── System: confirm button / Enter key ─────────────────────────────────────

/// Creates a new character profile and transitions to World.
pub fn confirm_character(
    _commands: Commands,
    interaction_query: Query<&Interaction, (With<CharSelectConfirmBtn>, Changed<Interaction>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    creation_state: Res<CharacterCreationState>,
    mut profiles: ResMut<PlayerProfiles>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let confirmed = interaction_query.iter().any(|i| *i == Interaction::Pressed)
        || keyboard.just_pressed(KeyCode::Enter);

    if !confirmed {
        return;
    }

    let class = match creation_state.selected_class {
        Some(c) => c,
        None => return,
    };

    let name = if creation_state.player_name.is_empty() {
        class.display_name().to_string()
    } else {
        creation_state.player_name.clone()
    };

    let id = profiles.next_id;
    profiles.next_id += 1;
    profiles.profiles.push(PlayerProfile {
        id,
        name: name.clone(),
        class,
        level: 1,
        xp: 0,
        gold: 0,
        completed_dungeons: vec![],
        play_time: 0.0,
    });

    info!("Character created: {} the {} (id={})", name, class.display_name(), id);
    next_state.set(AppState::World);
}

// ── System: populate existing character list ───────────────────────────────

/// Renders the list of saved characters each time the screen loads.
pub fn populate_existing_characters(
    mut commands: Commands,
    profiles: Res<PlayerProfiles>,
    existing_list: Query<Entity, With<CharSelectExistingList>>,
) {
    let list_entity = match existing_list.get_single() {
        Ok(e) => e,
        Err(_) => return,
    };

    // Clear previous entries
    commands.entity(list_entity).despawn_descendants();

    let mut entity_commands = commands.entity(list_entity);
    for profile in &profiles.profiles {
        let pid = profile.id;
        let class_color = match profile.class {
            CharacterClass::Warrior => Color::srgb(0.8, 0.3, 0.2),
            CharacterClass::Paladin => Color::srgb(0.9, 0.7, 0.3),
            CharacterClass::Rogue => Color::srgb(0.2, 0.7, 0.4),
            CharacterClass::Hunter => Color::srgb(0.2, 0.6, 0.3),
            CharacterClass::Mage => Color::srgb(0.3, 0.5, 1.0),
        };
        entity_commands.with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(36.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(12.0),
                    padding: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(0.0), Val::Px(0.0)),
                    border: UiRect::bottom(Val::Px(1.0)),
                    ..default()
                },
                BorderColor(Color::srgb(0.2, 0.2, 0.25)),
                CharSelectExistingSlot(pid),
            )).with_children(|row| {
                row.spawn((label(&profile.name, 18.0, Color::srgb(0.9, 0.9, 0.9)), CharSelectRoot));
                row.spawn((label(&format!("{} Lv.{}", profile.class.display_name(), profile.level), 14.0, class_color), CharSelectRoot));
                row.spawn((Node { flex_grow: 1.0, ..default() }, CharSelectRoot));

                // Play button
                row.spawn((
                    Node {
                        width: Val::Px(60.0),
                        height: Val::Px(26.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor(Color::srgb(0.3, 0.6, 0.3)),
                    BackgroundColor(Color::srgba(0.1, 0.25, 0.1, 1.0)),
                    CharSelectExistingSlot(pid),
                    Button,
                )).with_children(|btn| {
                    btn.spawn((label("Play", 14.0, Color::srgb(0.4, 1.0, 0.4)), CharSelectRoot));
                });

                // Delete button
                row.spawn((
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(26.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor(Color::srgb(0.5, 0.2, 0.2)),
                    BackgroundColor(Color::srgba(0.25, 0.08, 0.08, 1.0)),
                    CharSelectDeleteBtn(pid),
                    Button,
                )).with_children(|btn| {
                    btn.spawn((label("✕", 14.0, Color::srgb(1.0, 0.3, 0.3)), CharSelectRoot));
                });
            });
        });
    }
}

// ── System: play existing character ────────────────────────────────────────

pub fn play_existing_character(
    interaction_query: Query<(&Interaction, &CharSelectExistingSlot), Changed<Interaction>>,
    profiles: Res<PlayerProfiles>,
    mut creation_state: ResMut<CharacterCreationState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, slot) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Some(profile) = profiles.profiles.iter().find(|p| p.id == slot.0) {
            creation_state.selected_class = Some(profile.class);
            creation_state.player_name = profile.name.clone();
            creation_state.editing_existing = Some(profile.id);
            info!("Loading character: {} the {}", profile.name, profile.class.display_name());
            next_state.set(AppState::World);
            return;
        }
    }
}

// ── System: delete character ───────────────────────────────────────────────

/// Clicking ✕ deletes a character profile.
pub fn delete_character(
    interaction_query: Query<(&Interaction, &CharSelectDeleteBtn), Changed<Interaction>>,
    mut profiles: ResMut<PlayerProfiles>,
) {
    for (interaction, btn) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        let id = btn.0;
        profiles.profiles.retain(|p| p.id != id);
        info!("Deleted character id={}", id);
    }
}

// ── Despawn ────────────────────────────────────────────────────────────────

/// Despawns the entire character select screen recursively.
pub fn despawn_character_select(
    mut commands: Commands,
    screen: Query<Entity, With<CharSelectRoot>>,
) {
    for entity in screen.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
