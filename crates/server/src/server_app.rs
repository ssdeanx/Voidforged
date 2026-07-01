use bevy::prelude::*;

/// Build a headless dedicated-server [`App`].
///
/// The server runs with `MinimalPlugins` (no rendering, no audio, no input)
/// plus the core simulation crates and the full network stack.
///
/// # Usage
///
/// ```ignore
/// let mut app = ir_server::server_app::build_server_app();
/// app.insert_resource(ir_network::ServerConfig {
///     bind_addr: "0.0.0.0:9876".to_string(),
///     ..default()
/// });
/// app.run();
/// ```
pub fn build_server_app() -> App {
    let mut app = App::new();

    app.add_plugins((
        // Minimal Bevy (no renderer, no audio) — headless-friendly
        MinimalPlugins,
        // Core simulation layers
        ir_core::CorePlugin,
        ir_gameplay::GameplayPlugin,
        // Network stack (client+server resources, events, I/O stubs)
        ir_network::NetworkPlugin,
        // Server-specific room/lobby/tick management
        crate::ServerPlugin,
    ));

    // ── Server-side resource defaults ──────────────────────────
    // These can be overridden by the binary before calling `.run()`.
    app.insert_resource(ir_network::ServerConfig {
        bind_addr: "0.0.0.0:9876".to_string(),
        ..default()
    });

    info!("build_server_app: dedicated server configured");

    app
}
