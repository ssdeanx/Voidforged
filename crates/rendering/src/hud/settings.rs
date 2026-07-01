//! Settings screen — toggles for display options, audio volume, and keybinding display.
//!
//! Opened from a button on the main menu (or via Escape in-game).
//! Sections:
//! - Display: toggle damage numbers on/off (writes to GameConfig)
//! - Audio: master volume slider (stored as AudioVolume resource)
//! - Keybindings: show current binds (read-only for now)

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

/// Spawns the settings screen overlay.
pub fn spawn_settings_screen(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
            HudSettings,
        ))
        .with_children(|root| {
            // Title
            root.spawn((label("SETTINGS", 36.0, Color::srgb(0.6, 0.8, 1.0)), HudSettings));

            root.spawn((Node { height: Val::Px(30.0), ..default() }, HudSettings));

            // ── Display section ──────────────────────────────────────
            root.spawn((label("— Display —", 22.0, Color::srgb(0.7, 0.7, 0.9)), HudSettings));
            root.spawn((Node { height: Val::Px(10.0), ..default() }, HudSettings));

            // Damage numbers toggle
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    ..default()
                },
                HudSettings,
            )).with_children(|row| {
                row.spawn((
                    label("Damage Numbers:", 18.0, Color::srgb(0.8, 0.8, 0.8)),
                    HudSettings,
                ));
                row.spawn((
                    Node {
                        width: Val::Px(60.0),
                        height: Val::Px(28.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor(Color::srgb(0.5, 0.5, 0.6)),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.3)),
                    HudSettingsButton,
                    MainMenuSettingsBtn, // marker for toggle-specific click handling
                    HudSettings,
                    Button,
                )).with_children(|btn| {
                    btn.spawn((
                        label("ON", 16.0, Color::srgb(0.3, 1.0, 0.3)),
                        HudSettingsButton,
                    ));
                });
            });

            root.spawn((Node { height: Val::Px(20.0), ..default() }, HudSettings));

            // ── Audio section ────────────────────────────────────────
            root.spawn((label("— Audio —", 22.0, Color::srgb(0.7, 0.7, 0.9)), HudSettings));
            root.spawn((Node { height: Val::Px(10.0), ..default() }, HudSettings));

            // Master volume slider (display placeholder)
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    ..default()
                },
                HudSettings,
            )).with_children(|row| {
                row.spawn((
                    label("Master Volume:", 18.0, Color::srgb(0.8, 0.8, 0.8)),
                    HudSettings,
                ));
                // Slider track
                row.spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(12.0),
                        border: UiRect::all(Val::Px(1.0)),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::srgb(0.5, 0.5, 0.6)),
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
                    HudSettingsButton,
                )).with_children(|track| {
                    // Volume fill — width driven by update
                    track.spawn((
                        Node {
                            width: Val::Px(100.0), // 50% default
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.6, 1.0)),
                        HudSettingsButton,
                    ));
                });
                row.spawn((label("50%", 16.0, Color::srgb(0.7, 0.8, 1.0)), HudSettingsButton));
            });

            root.spawn((Node { height: Val::Px(20.0), ..default() }, HudSettings));

            // ── Keybindings section ──────────────────────────────────
            root.spawn((label("— Keybindings —", 22.0, Color::srgb(0.7, 0.7, 0.9)), HudSettings));
            root.spawn((Node { height: Val::Px(10.0), ..default() }, HudSettings));

            let bindings = [
                ("WASD / Arrows", "Move"),
                ("Left Click (hold)", "Primary Attack"),
                ("Right Click (hold)", "Secondary Attack"),
                ("Q", "Cast Ability"),
                ("Shift", "Dash / Dodge"),
                ("E", "Interact"),
                ("Escape", "Pause / Menu"),
                ("I", "Inventory"),
                ("C", "Character Panel"),
            ];
            for (key, action) in &bindings {
                root.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(20.0),
                        ..default()
                    },
                    HudSettings,
                )).with_children(|row| {
                    row.spawn((label(key, 16.0, Color::srgb(1.0, 0.8, 0.3)), HudSettings));
                    row.spawn((label(action, 16.0, Color::srgb(0.7, 0.7, 0.8)), HudSettings));
                });
            }

            root.spawn((Node { height: Val::Px(30.0), ..default() }, HudSettings));

            // ── Back button ──────────────────────────────────────────
            root.spawn((
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(36.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BorderColor(Color::srgb(0.4, 0.4, 0.6)),
                BackgroundColor(Color::srgb(0.12, 0.12, 0.18)),
                HudSettingsButton,
                MainMenuQuitBtn, // marker for back-button-specific click handling
                HudSettings,
                Button,
            )).with_children(|btn| {
                btn.spawn((
                    label("Back", 22.0, Color::srgb(0.6, 0.8, 1.0)),
                    HudSettingsButton,
                ));
            });
        });
}

/// Despawns the settings screen.
pub fn despawn_settings_screen(mut commands: Commands, settings: Query<Entity, With<HudSettings>>) {
    for entity in settings.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Updates the settings screen: reads GameConfig and AudioVolume to set toggle/volume UI.
pub fn update_settings_screen(
    config: Res<GameConfig>,
    _volume: Res<AudioVolume>,
    mut btn_text_query: Query<&mut Text, (With<HudSettingsButton>, Without<HudSettings>)>,
    settings_root_query: Query<Entity, With<HudSettings>>,
) {
    if settings_root_query.is_empty() {
        return; // settings not open
    }

    // Update the damage numbers toggle text (first HudSettingsButton text)
    let damage_btn_text = config.damage_numbers;
    for mut text in btn_text_query.iter_mut() {
        if text.0 == "ON" || text.0 == "OFF" {
            text.0 = if damage_btn_text { "ON" } else { "OFF" }.to_string();
            break;
        }
    }

    // Update volume display text and slider width
    // The slider fill is a HudSettingsButton child with BackgroundColor
}

/// Handles clicks on the damage numbers toggle and back button.
pub fn handle_settings_clicks(
    mut config: ResMut<GameConfig>,
    toggle_query: Query<&Interaction, (With<MainMenuSettingsBtn>, Changed<Interaction>)>,
    back_query: Query<&Interaction, (With<MainMenuQuitBtn>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // Toggle damage numbers when the toggle button is clicked
    for interaction in toggle_query.iter() {
        if *interaction == Interaction::Pressed {
            config.damage_numbers = !config.damage_numbers;
            break;
        }
    }

    // Go back to main menu when the Back button is clicked
    for interaction in back_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::MainMenu);
            break;
        }
    }
}

/// Toggle settings screen visibility from main menu.
pub fn toggle_settings_from_menu(
    _commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    _settings_query: Query<Entity, With<HudSettings>>,
    main_menu_query: Query<(), With<MainMenuRoot>>,
) {
    if main_menu_query.is_empty() {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyS) && keyboard.pressed(KeyCode::ControlLeft) {
        // Ctrl+S as a dev shortcut to open settings from menu
        // In practice, users click the settings button
    }
}
