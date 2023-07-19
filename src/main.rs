use rocket::{get, launch, routes};
use rocket::serde::json::Json;
use dotenv::dotenv;

mod api {
    pub mod postgre;
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
    dotenv().ok();
    rocket::build().mount("/", routes![index, goods,category,store])
}
