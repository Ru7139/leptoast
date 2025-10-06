#[cfg(feature = "web")]
mod app {
    pub mod app_itself;
}

#[cfg(feature = "server")]
mod backend {
    pub mod axum_app;
}

#[cfg(feature = "web")]
mod router {
    pub mod homepage;
    pub mod router_itself;
}

#[cfg(feature = "server")]
mod sdb {
    pub mod common;
    pub mod read;
    pub mod write;
}

// use dioxus::prelude::*;

fn main() {
    #[cfg(feature = "server")]
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        // sdb::common::sdb_ws_connect_as_ruut(65535).await.unwrap();
        launch_server()
    });

    #[cfg(feature = "web")]
    dioxus::launch(app::app_itself::App);
}

#[cfg(feature = "server")]
async fn launch_server() {
    // Connect to dioxus' logging infrastructure
    dioxus::logger::initialize_default();

    // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile)
    let socket_addr = dioxus::cli_config::fullstack_address_or_localhost();

    // Build a custom axum router
    let router = axum::Router::new()
        // .serve_dioxus_application(ServeConfig::new().unwrap(), App)
        .into_make_service();

    // And launch it!
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
