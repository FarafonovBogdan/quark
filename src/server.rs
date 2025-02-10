use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::db::Database;
use crate::sharding::ShardConfig;

#[derive(Deserialize)]
struct KeyQuery {
    key: String,
}

#[derive(Serialize, Deserialize)]
struct KeyValue {
    key: String,
    value: String,
}

pub fn routes(db: Arc<Database>, shard_config: Arc<ShardConfig>) -> Router {
    Router::new()
        .route("/get", get(get_value))
        .route("/set", post(set_value))
        .route("/del", post(delete_value))
        .route("/shard-info", get(shard_info))
        .with_state((db, shard_config))
}

/// Возвращает информацию о текущем шарде

async fn shard_info(
    State((_, shard_config)): State<(Arc<Database>, Arc<ShardConfig>)>,
) -> impl IntoResponse {
    println!("Shard Info: {:?}", shard_config); // Добавляем лог
    Json((*shard_config).clone()).into_response()
}

//// Получает значение (и перенаправляет, если ключ не на этом узле)
async fn get_value(
    State((db, shard_config)): State<(Arc<Database>, Arc<ShardConfig>)>,
    Query(params): Query<KeyQuery>,
) -> impl IntoResponse {
    if !shard_config.is_local_shard(&params.key) {
        let target = shard_config.get_shard_address(&params.key);
        let url = format!("http://{}/get?key={}", target, params.key);
        return reqwest::get(&url)
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .into_response(); // Добавляем `.into_response()`
    }

    match db.get(&params.key) {
        Some(value) => Json(value).into_response(), // Добавляем `.into_response()`
        None => Json("Key not found".to_string()).into_response(),
    }
}

async fn set_value(
    State((db, shard_config)): State<(Arc<Database>, Arc<ShardConfig>)>,
    Json(payload): Json<KeyValue>,
) -> impl IntoResponse {
    let target = shard_config.get_shard_address(&payload.key);

    if !shard_config.is_local_shard(&payload.key) {
        if target == shard_config.current_address() {
            return Json("Shard redirection loop detected!").into_response();
        }

        let url = format!("http://{}/set", target);
        println!("Redirecting to {}", url);
        return reqwest::Client::new()
            .post(&url)
            .json(&payload)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .into_response();
    }

    db.set(&payload.key, &payload.value).unwrap();
    println!(
        "Stored in local DB: key={}, value={}",
        payload.key, payload.value
    );
    Json("OK".to_string()).into_response()
}

/// Удаляет значение (и перенаправляет, если ключ не на этом узле)
async fn delete_value(
    State((db, shard_config)): State<(Arc<Database>, Arc<ShardConfig>)>,
    Query(params): Query<KeyQuery>,
) -> impl IntoResponse {
    if !shard_config.is_local_shard(&params.key) {
        let target = shard_config.get_shard_address(&params.key);
        let url = format!("http://{}/del?key={}", target, params.key);
        return reqwest::get(&url)
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .into_response();
    }

    db.delete(&params.key).unwrap();
    Json("Deleted".to_string()).into_response()
}
