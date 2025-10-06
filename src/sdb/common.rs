use serde::{de::DeserializeOwned, Deserialize};
use surrealdb::{engine::any::Any, opt::auth::Root, Surreal};

pub static DB: std::sync::LazyLock<Surreal<Any>> = std::sync::LazyLock::new(Surreal::init);

#[rustfmt::skip]
pub async fn sdb_ws_connect_as_ruut(port: u16)
    -> Result<(), Box<dyn std::error::Error>> {
        DB.connect(&format!("ws://localhost:{}", port)).await?;
        DB.signin(Root { username: "ruut_dioxusap", password: "paas_dioxusap" }).await?;
        DB.use_ns("dioxusap").use_db("owner").await?;
        DB.query("INSERT INTO signin_logs {access: time::now()}").await?;
        Ok(println!("SurrealDB is connected at localhost:{} ---> access time recorded", port))
}

#[rustfmt::skip]
pub async fn query_get_data<T>(query_msg: &str, num_take: usize)
    -> Result<Vec<T>, Box<dyn std::error::Error>>
        where T:DeserializeOwned {
            Ok(DB.query(query_msg).await?.take::<Vec<T>>(num_take)?)
}
