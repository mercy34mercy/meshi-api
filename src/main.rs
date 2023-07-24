use rocket::{get, launch, routes};
use rocket::serde::json::Json;
use serde_json::Value;
use serde_json::json;

mod api {
    pub mod postgre;
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
fn test() -> Json<Value> {
    let response = json!({
        "name": "masashi",
    });
    Json(response)
}

#[get("/goods")]
async fn goods() -> Result<Json<Vec<api::postgre::Good>>, Json<String>> {
    api::postgre::fetch_goods().await
}

#[get("/category")]
async fn category() -> Result<Json<Vec<api::postgre::Category>>, Json<String>> {
    api::postgre::fetch_category().await
}

#[get("/store")]
async fn store() -> Result<Json<Vec<api::postgre::Store>>, Json<String>> {
    api::postgre::fetch_store().await
}

#[launch]
fn rocket() -> _ {
    // let api_key = std::env::var("SUPABASE_PUBLIC_API_KEY").expect("SUPABASE_PUBLIC_API_KEY must be set");
    // print!("{}", api_key);
    let config = rocket::Config {
        port: 8080,
        ..rocket::Config::default()
    };
    rocket::custom(config)
    .mount("/", routes![index, goods, category, store,test])
}
