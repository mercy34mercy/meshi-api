use postgrest::Postgrest;
use std::error::Error;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket::serde::json::Json;
use serde_json::json;
use rocket::http::Status;

#[derive(Deserialize,Serialize)]
pub struct Store {
    id: i32,
    name: String,
}

#[derive(Deserialize,Serialize)]
pub struct Category {
    id: i32,
    name: String,
}

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Good {
    id: i32,
    name: String,
    price: i32,
    image_url: String,
    store: Store,
    category: Category,
}

pub async fn fetch_store() -> Result<Json<Vec<Store>>, Json<String>> {
    match get_store().await {
        Ok(store) => Ok(Json(store)),
        Err(err) => {
            let response = json!({
                "status": Status::InternalServerError.code,
                "reason": err.to_string(),
            });
            Err(Json(response.to_string()))
        }
    }
} 

pub async fn fetch_category() -> Result<Json<Vec<Category>>, Json<String>> {
    match get_category().await {
        Ok(category) => Ok(Json(category)),
        Err(err) => {
            let response = json!({
                "status": Status::InternalServerError.code,
                "reason": err.to_string(),
            });
            Err(Json(response.to_string()))
        }
    }
}

pub async fn fetch_goods() -> Result<Json<Vec<Good>>, Json<String>> {
    match get_goods().await {
        Ok(goods) => Ok(Json(goods)),
        Err(err) => {
            let response = json!({
                "status": Status::InternalServerError.code,
                "reason": err.to_string(),
            });
            Err(Json(response.to_string()))
        }
    }
}

async fn get_store() -> Result<Vec<Store>, Box<dyn Error>> {
    let client = get_client()?;

    let resp = client
        .from("store")
        .select("*")
        .execute()
        .await
        .unwrap();
    let body = resp.text().await?;
    // First, parse the JSON body into a Value.
    let value: serde_json::Value = serde_json::from_str(&body)?;

    // Then, try to extract the array under the key "store".
    let store = value.as_array()
        .ok_or_else(|| format!("'data' key not found or is not an array"))?;
    
    // Now, try to deserialize the array into a Vec<Store>.
    let store: Vec<Store> = serde_json::from_value(serde_json::Value::Array(store.clone()))?;

    Ok(store)
}

async fn get_category() -> Result<Vec<Category>, Box<dyn Error>> {
    let client = get_client()?;

    let resp = client
        .from("category")
        .select("*")
        .execute()
        .await
        .unwrap();

    let body = resp.text().await?;

    // First, parse the JSON body into a Value.
    let value: serde_json::Value = serde_json::from_str(&body)?;

    // Then, try to extract the array under the key "category".
    let category = value.as_array()
        .ok_or_else(|| format!("'data' key not found or is not an array"))?;

    // Now, try to deserialize the array into a Vec<Category>.
    let category: Vec<Category> = serde_json::from_value(serde_json::Value::Array(category.clone()))?;

    Ok(category)
}

async fn get_goods() -> Result<Vec<Good>, Box<dyn Error>> {
    let client = get_client()?;
    let resp = client
        .from("goods")
        .select("*, store!store_id(*), category!category_id(*)")
        .execute()
        .await
        .unwrap();
    let body = resp.text().await?;
    // First, parse the JSON body into a Value.
    let value: serde_json::Value = serde_json::from_str(&body)?;

    print!("{}", body);

    // Then, try to extract the array under the key "goods".
    let goods = value.as_array()
        .ok_or_else(|| format!("'data' key not found or is not an array"))?;

    // Now, try to deserialize the array into a Vec<Good>.
    let goods: Vec<Good> = serde_json::from_value(serde_json::Value::Array(goods.clone()))?;
    Ok(goods)
}

pub fn get_client() -> Result<Postgrest, Box<dyn std::error::Error>> {
    dotenv::var("SUPABASE_PUBLIC_API_KEY").map(|key| {
        Postgrest::new("https://zzlzipzjibyvcltibmvu.supabase.co/rest/v1/")
            .insert_header("apikey", key)
    }).map_err(|e| e.into())
}
