use bevy::prelude::*;

/// Headless server app setup (no rendering, no input).
pub fn build_server_app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, ir_core::CorePlugin, ir_gameplay::GameplayPlugin));
    app
}
