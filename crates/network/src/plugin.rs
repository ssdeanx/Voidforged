use bevy::prelude::*;

/// Marker resource indicating that the `multiplayer` feature is active.
///
/// Systems that conditionally perform networking should check this
/// resource before running. When the feature is disabled this resource
/// is still present but `.0` is `false`.
#[derive(Debug, Resource)]
pub struct IsMultiplayer(pub bool);

/// Plugin that registers the network protocol layer.
///
/// When the `multiplayer` feature is enabled this plugin also registers
/// the client / server resources and events that drive the I/O systems.
///
/// # Client setup
///
/// ```ignore
/// app.add_plugins(ir_network::NetworkPlugin)
///    .insert_resource(ir_network::ClientConfig { .. });
/// ```
///
/// # Server setup
///
/// The `ir_server` crate wraps this plugin with server-specific resources:
/// ```ignore
/// app.add_plugins(ir_server::ServerPlugin);
/// ```
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        // Always register the multiplayer marker so systems can query it.
        app.insert_resource(IsMultiplayer(cfg!(feature = "multiplayer")));

        // ── Feature-gated client/server setup ──
        #[cfg(feature = "multiplayer")]
        {
            use crate::client::{ChatReceivedEvent, ConnectedEvent, DisconnectedEvent, SnapshotReceivedEvent};
            use crate::server::{ClientConnectedEvent, ClientDisconnectedEvent, ClientJoinedRoomEvent, ClientLeftRoomEvent};

            // Client resources and events
            app.init_resource::<crate::client::NetworkClient>()
                .init_resource::<crate::client::ClientConfig>()
                .add_event::<ConnectedEvent>()
                .add_event::<DisconnectedEvent>()
                .add_event::<SnapshotReceivedEvent>()
                .add_event::<ChatReceivedEvent>();

            // Server resources and events
            app.init_resource::<crate::server::NetworkServer>()
                .init_resource::<crate::server::ServerConfig>()
                .add_event::<ClientConnectedEvent>()
                .add_event::<ClientDisconnectedEvent>()
                .add_event::<ClientJoinedRoomEvent>()
                .add_event::<ClientLeftRoomEvent>();

            // ── I/O runner system set (stub — actual poll/receive to be
            //    implemented when the WebSocket layer is written) ──
            // app.add_systems(PreUpdate, (
            //     server_io_receive::system,
            //     client_io_receive::system,
            // ));

            info!("NetworkPlugin: multiplayer feature enabled");
        }

        #[cfg(not(feature = "multiplayer"))]
        {
            info!("NetworkPlugin: multiplayer feature disabled — networking stubbed");
        }
    }
}
