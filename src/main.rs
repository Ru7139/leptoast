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

use dioxus::prelude::*;

fn main() {
    #[cfg(feature = "web")]
    dioxus::launch(app::app_itself::App);



    #[cfg(feature = "server")]
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        // sdb::common::sdb_ws_connect_as_ruut(65535).await.unwrap();
        //
        use dioxus::server::
    });
}
