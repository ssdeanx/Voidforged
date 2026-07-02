//! Meta-progression upgrade tree UI — a full-screen grid of purchasable upgrades.
//!
//! Shows all upgrade cards from `all_upgrade_defs()`, grouped by category.
//! Players can click a card to purchase that upgrade (spending Dark Essence).
//! Uses icon textures from UiIconAssets when available.

use bevy::prelude::*;
use ir_core::*;
use ir_progression::upgrades::UpgradeCategory;
use crate::hud::components::*;
use crate::ui_icons::UiIconAssets;

/// Helper to create a label bundle with the given font size and color.
fn upgrade_label(s: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(s.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    )
}

// ── Purchase event ──────────────────────────────────────────────────────────

/// Fired when the player clicks a purchase button in the upgrade tree UI.
#[derive(Event, Debug)]
pub struct PurchaseUpgradeEvent {
    pub upgrade_id: String,
}

// ── Spawn ───────────────────────────────────────────────────────────────────

/// Spawns the full upgrade tree overlay.
pub fn spawn_upgrade_tree(mut commands: Commands, meta: Res<MetaProgression>, icons: Res<UiIconAssets>) {
    let defs = ir_progression::upgrades::all_upgrade_defs();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.08, 1.0)),
            UpgradeTreeRoot,
        ))
        .with_children(|root| {
            // ── Title bar ──────────────────────────────────────────
            root.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                UpgradeTreeRoot,
            ))
            .with_children(|title_bar| {
                title_bar.spawn((
                    upgrade_label(
                        &format!("Meta-Progression — Dark Essence: {}", meta.dark_essence),
                        32.0,
                        Color::srgb(0.7, 0.5, 1.0),
                    ),
                    UpgradeTreeRoot,
                ));
                // Close button hint
                title_bar.spawn((
                    upgrade_label("Press ESC to close", 18.0, Color::srgb(0.5, 0.5, 0.6)),
                    UpgradeTreeRoot,
                ));
            });

            // ── Category groups ────────────────────────────────────
            let categories = [
                (UpgradeCategory::Stats, "Stats", Color::srgb(0.4, 0.7, 1.0)),
                (UpgradeCategory::Weapons, "Weapon Unlocks", Color::srgb(1.0, 0.6, 0.3)),
                (UpgradeCategory::Utility, "Utility", Color::srgb(0.4, 1.0, 0.6)),
                (UpgradeCategory::Classes, "Classes", Color::srgb(1.0, 0.7, 0.9)),
            ];

            for (category, cat_name, cat_color) in &categories {
                let cat_defs: Vec<&ir_progression::upgrades::UpgradeDef> = defs
                    .iter()
                    .filter(|d| d.category == *category)
                    .collect();

                if cat_defs.is_empty() {
                    continue;
                }

                // Category header
                root.spawn((
                    upgrade_label(
                        &format!("── {} ──", cat_name),
                        22.0,
                        *cat_color,
                    ),
                    UpgradeTreeRoot,
                ));
                root.spawn((Node { height: Val::Px(10.0), ..default() }, UpgradeTreeRoot));

                // Card grid row
                root.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Auto,
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(12.0),
                        row_gap: Val::Px(12.0),
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    UpgradeTreeRoot,
                ))
                .with_children(|grid| {
                    for def in cat_defs {
                        spawn_upgrade_card(grid, def, &meta, &icons);
                    }
                });
            }
        });
}

/// Spawns a single upgrade card in the grid.
fn spawn_upgrade_card(
    parent: &mut ChildBuilder,
    def: &ir_progression::upgrades::UpgradeDef,
    meta: &MetaProgression,
    icons: &UiIconAssets,
) {
    let current_tier = meta
        .upgrades
        .iter()
        .find(|u| u.id == def.id)
        .map(|u| u.tier)
        .unwrap_or(0);

    let is_maxed = current_tier >= def.max_tier;
    let cost = ir_progression::upgrades::upgrade_cost(def, current_tier);
    let can_afford = meta.dark_essence >= cost && !is_maxed;

    // Card colors based on state
    let (bg_color, border_color, text_color) = if is_maxed {
        (
            Color::srgba(0.1, 0.15, 0.1, 1.0),
            Color::srgb(0.3, 0.5, 0.3),
            Color::srgb(0.4, 0.7, 0.4),
        )
    } else if can_afford {
        (
            Color::srgba(0.12, 0.12, 0.2, 1.0),
            Color::srgb(0.4, 0.7, 1.0),
            Color::srgb(0.9, 0.9, 0.95),
        )
    } else {
        (
            Color::srgba(0.08, 0.08, 0.1, 1.0),
            Color::srgb(0.25, 0.25, 0.3),
            Color::srgb(0.4, 0.4, 0.45),
        )
    };

    let card_bundle = (
        Node {
            width: Val::Px(200.0),
            height: Val::Px(180.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            padding: UiRect::all(Val::Px(10.0)),
            border: UiRect::all(Val::Px(2.0)),
            row_gap: Val::Px(4.0),
            ..default()
        },
        BorderColor(border_color),
        BackgroundColor(bg_color),
        UpgradeTreeRoot,
    );

    // Only clickable (Button) if purchasable
    if can_afford {
        parent
            .spawn(card_bundle)
            .insert(Button)
            .with_children(|card| {
                card.spawn((
                    upgrade_label(def.name, 16.0, text_color),
                    UpgradeTreeRoot,
                ));
                card.spawn((
                    upgrade_label(def.description, 11.0, Color::srgb(0.6, 0.6, 0.7)),
                    UpgradeTreeRoot,
                ));
                card.spawn((Node { height: Val::Px(6.0), ..default() }, UpgradeTreeRoot));

                // Tier indicator
                let tier_text = if is_maxed {
                    "MAXED".to_string()
                } else {
                    format!("Tier {}/{}", current_tier, def.max_tier)
                };
                card.spawn((
                    upgrade_label(
                        &tier_text,
                        13.0,
                        if is_maxed {
                            Color::srgb(0.4, 1.0, 0.4)
                        } else {
                            Color::srgb(0.7, 0.7, 0.7)
                        },
                    ),
                    UpgradeTreeRoot,
                ));

                // Cost display
                if !is_maxed {
                    card.spawn((
                        upgrade_label(
                            &format!("Cost: {} DE", cost),
                            13.0,
                            if can_afford {
                                Color::srgb(0.7, 0.5, 1.0)
                            } else {
                                Color::srgb(0.5, 0.3, 0.7)
                            },
                        ),
                        UpgradeTreeRoot,
                    ));
                }

                // Stat preview
                if !def.per_tier_stats.is_empty() {
                    let bonus = &def.per_tier_stats[0];
                    card.spawn((
                        upgrade_label(
                            &format!("{}: {}", bonus.stat.display_name(), bonus.stat.format_value(bonus.value)),
                            11.0,
                            Color::srgb(0.5, 0.8, 0.5),
                        ),
                        UpgradeTreeRoot,
                    ));
                }

                // ── Upgrade icon texture ──────────────────────────
                let icon_handle = icons.get(def.icon_id)
                    .unwrap_or_else(|| icons.get("ui_settings")
                        .unwrap_or_default());
                card.spawn((
                    Node {
                        width: Val::Px(40.0),
                        height: Val::Px(28.0),
                        ..default()
                    },
                    ImageNode::new(icon_handle),
                    UpgradeTreeRoot,
                ));
            });
    } else {
        parent
            .spawn(card_bundle)
            .with_children(|card| {
                card.spawn((
                    upgrade_label(def.name, 16.0, text_color),
                    UpgradeTreeRoot,
                ));
                card.spawn((
                    upgrade_label(def.description, 11.0, Color::srgb(0.5, 0.5, 0.55)),
                    UpgradeTreeRoot,
                ));
                card.spawn((Node { height: Val::Px(6.0), ..default() }, UpgradeTreeRoot));

                let tier_text = if is_maxed {
                    "MAXED".to_string()
                } else {
                    format!("Tier {}/{}", current_tier, def.max_tier)
                };
                card.spawn((
                    upgrade_label(&tier_text, 13.0, Color::srgb(0.5, 0.5, 0.5)),
                    UpgradeTreeRoot,
                ));

                if !is_maxed {
                    card.spawn((
                        upgrade_label(
                            &format!("Cost: {} DE", cost),
                            13.0,
                            Color::srgb(0.4, 0.25, 0.55),
                        ),
                        UpgradeTreeRoot,
                    ));
                }

                if !def.per_tier_stats.is_empty() {
                    let bonus = &def.per_tier_stats[0];
                    card.spawn((
                        upgrade_label(
                            &format!("{}: {}", bonus.stat.display_name(), bonus.stat.format_value(bonus.value)),
                            11.0,
                            Color::srgb(0.35, 0.55, 0.35),
                        ),
                        UpgradeTreeRoot,
                    ));
                }

                // ── Icon texture ────────────────────────────────────
                let icon_handle = icons.get(def.icon_id)
                    .unwrap_or_else(|| icons.get("ui_settings")
                        .unwrap_or_default());
                card.spawn((
                    Node {
                        width: Val::Px(40.0),
                        height: Val::Px(28.0),
                        ..default()
                    },
                    ImageNode::new(icon_handle),
                    UpgradeTreeRoot,
                ));
            });
    }
}

// ── Interaction system ──────────────────────────────────────────────────────

/// Handles clicks on upgrade cards by sending a PurchaseUpgradeEvent.
/// Uses the card's UpgradeDef id, looked up via text content (name).
pub fn handle_upgrade_card_clicks(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &Parent), (With<Button>, Changed<Interaction>)>,
    children_query: Query<&Children>,
    text_query: Query<&Text>,
) {
    let defs = ir_progression::upgrades::all_upgrade_defs();
    for (interaction, parent) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }
        // Walk up to find the card entity, then find its name child
        if let Ok(children) = children_query.get(parent.get()) {
            for child in children.iter() {
                if let Ok(text) = text_query.get(*child) {
                    let name = &text.0;
                    // Find the upgrade def whose name matches
                    for def in defs.iter() {
                        if def.name == name.as_str() {
                            info!("Upgrade card clicked: {} (id={})", def.name, def.id);
                            commands.trigger(PurchaseUpgradeEvent {
                                upgrade_id: def.id.to_string(),
                            });
                            return;
                        }
                    }
                }
            }
        }
    }
}

/// System that processes purchase events and applies them to meta-progression.
pub fn process_purchase_events(
    mut purchase_events: EventReader<PurchaseUpgradeEvent>,
    mut meta: ResMut<MetaProgression>,
) {
    for event in purchase_events.read() {
        match ir_progression::upgrades::purchase_upgrade(&event.upgrade_id, &mut meta) {
            Ok(()) => {
                info!("Upgrade purchased: {}", event.upgrade_id);
            }
            Err(e) => {
                warn!("Purchase failed for {}: {}", event.upgrade_id, e);
            }
        }
    }
}

/// Closes upgrade tree when ESC is pressed.
pub fn close_upgrade_tree(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    tree_query: Query<Entity, With<UpgradeTreeRoot>>,
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        // Despawn tree nodes
        for entity in tree_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        // Go back to MainMenu
        if *state.get() == AppState::MainMenu || *state.get() == AppState::GameOver {
            // If we were in MainMenu, stay there (tree was overlay)
            // If in GameOver, go back
        } else {
            next_state.set(AppState::MainMenu);
        }
    }
}

/// Despawn all upgrade tree entities.
pub fn despawn_upgrade_tree(mut commands: Commands, tree: Query<Entity, With<UpgradeTreeRoot>>) {
    for entity in tree.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Toggle upgrade tree from MainMenu when pressing U.
pub fn toggle_upgrade_tree_from_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    commands: Commands,
    meta: Res<MetaProgression>,
    icons: Res<UiIconAssets>,
    tree_query: Query<Entity, With<UpgradeTreeRoot>>,
) {
    if keyboard.just_pressed(KeyCode::KeyU) {
        if tree_query.is_empty() {
            spawn_upgrade_tree(commands, meta, icons);
        } else {
            despawn_upgrade_tree(commands, tree_query);
        }
    }
}
