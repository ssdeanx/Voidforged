//! Menu screens: MainMenu, GameOver, Pause overlay.

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

// ── Main Menu ───────────────────────────────────────────────────────────────

pub fn spawn_main_menu_screen(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            MainMenuRoot,
        ))
        .with_children(|root| {
            root.spawn((label("ISOMETRIC ROGUELITE", 48.0, Color::srgb(0.7, 0.5, 1.0)), MainMenuRoot));
            root.spawn((Node { height: Val::Px(20.0), ..default() }, MainMenuRoot));
            root.spawn((label("Press ENTER to start", 24.0, Color::srgb(0.4, 0.7, 1.0)), MainMenuRoot));
            root.spawn((Node { height: Val::Px(40.0), ..default() }, MainMenuRoot));

            let controls = [
                ("WASD / Arrows", "Move"),
                ("Left Click (hold)", "Primary Attack"),
                ("Right Click (hold)", "Spread Attack"),
                ("Q", "Cast (piercing)"),
                ("Shift", "Dash / Dodge"),
                ("Escape", "Pause"),
            ];
            for (key, action) in &controls {
                root.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(20.0),
                        ..default()
                    },
                    MainMenuRoot,
                )).with_children(|row| {
                    row.spawn((label(key, 18.0, Color::srgb(1.0, 0.8, 0.3)), MainMenuRoot));
                    row.spawn((label(action, 18.0, Color::srgb(0.7, 0.7, 0.8)), MainMenuRoot));
                });
            }

            root.spawn((Node { height: Val::Px(30.0), ..default() }, MainMenuRoot));
            root.spawn((
                label("v0.4.0 — CachyOS + Rust + Bevy 0.15", 14.0, Color::srgb(0.4, 0.4, 0.5)),
                MainMenuRoot,
            ));
        });
}

pub fn despawn_main_menu(mut commands: Commands, menu: Query<Entity, With<MainMenuRoot>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// ── Game Over ───────────────────────────────────────────────────────────────

pub fn spawn_game_over_screen(mut commands: Commands, progression: Res<RunProgression>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            GameOverRoot,
        ))
        .with_children(|root| {
            root.spawn((label("GAME OVER", 48.0, Color::srgb(0.9, 0.2, 0.2)), GameOverRoot));
            root.spawn((Node { height: Val::Px(30.0), ..default() }, GameOverRoot));

            let lines = vec![
                format!("Kills:            {}", progression.kills),
                format!("Damage Dealt:     {:.0}", progression.damage_dealt),
                format!("Damage Taken:     {:.0}", progression.damage_taken),
                format!("Gold Collected:   {}", progression.gold_collected),
                format!("XP Earned:        {}", progression.xp_earned),
                format!("Run Time:         {:.1}s", progression.run_time),
            ];
            for line in &lines {
                root.spawn((label(line, 20.0, Color::srgb(0.8, 0.7, 0.7)), GameOverRoot));
            }

            root.spawn((Node { height: Val::Px(40.0), ..default() }, GameOverRoot));
            root.spawn((
                label("Press ENTER or SPACE to try again", 22.0, Color::srgb(0.6, 0.8, 1.0)),
                GameOverRoot,
            ));
        });
}

pub fn despawn_game_over(mut commands: Commands, screen: Query<Entity, With<GameOverRoot>>) {
    for entity in screen.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// ── Pause Overlay ───────────────────────────────────────────────────────────

pub fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        PauseOverlay,
    )).with_children(|root| {
        root.spawn((label("PAUSED", 48.0, Color::srgb(0.6, 0.8, 1.0)), PauseOverlay));
        root.spawn((Node { height: Val::Px(20.0), ..default() }, PauseOverlay));
        root.spawn((label("Press ESC to resume", 22.0, Color::srgb(0.6, 0.6, 0.7)), PauseOverlay));
    });
}

pub fn despawn_pause_overlay(mut commands: Commands, overlay: Query<Entity, With<PauseOverlay>>) {
    for entity in overlay.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
