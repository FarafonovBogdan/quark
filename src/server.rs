use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
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

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    status: String,
    data: Option<T>,
    error: Option<String>,
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        match self.error {
            Some(err) => (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<T> {
                    status: "error".to_string(),
                    data: None,
                    error: Some(err),
                }),
            )
                .into_response(),
            None => (
                StatusCode::OK,
                Json(ApiResponse {
                    status: "success".to_string(),
                    data: self.data,
                    error: None,
                }),
            )
                .into_response(),
        }
    }
}

async fn shard_info(
    State((_, shard_config)): State<(Arc<Database>, Arc<ShardConfig>)>,
) -> impl IntoResponse {
    println!("Shard Info: {:?}", shard_config);
    Json((*shard_config).clone()).into_response()
}

async fn get_value(
    State((db, shard_config)): State<(Arc<Database>, Arc<ShardConfig>)>,
    Query(params): Query<KeyQuery>,
) -> impl IntoResponse {
    if !shard_config.is_local_shard(&params.key) {
        let target = shard_config.get_shard_address(&params.key);
        if target == shard_config.current_address() {
            return Json("ERROR: Shard redirection loop detected!").into_response();
        }

        println!("DEBUG: Redirecting GET key={} to {}", params.key, target);
        return reqwest::Client::new()
            .get(format!("http://{}/get?key={}", target, params.key))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .into_response();
    }

    println!(
        "DEBUG: Fetching key={} locally from shard {}",
        params.key, shard_config.current_shard
    );
    match db.get(&params.key) {
        Some(value) => Json(value).into_response(),
        None => Json("Key not found".to_string()).into_response(),
    }
}

async fn set_value(
    State((db, shard_config)): State<(Arc<Database>, Arc<ShardConfig>)>,
    Json(payload): Json<KeyValue>,
) -> impl IntoResponse {
    if !shard_config.is_local_shard(&payload.key) {
        let target = shard_config.get_shard_address(&payload.key);
        if target == shard_config.current_address() {
            return Json("ERROR: Shard redirection loop detected!").into_response();
        }

        println!("DEBUG: Redirecting key={} to {}", payload.key, target);
        return reqwest::Client::new()
            .post(format!("http://{}/set", target))
            .json(&payload)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .into_response();
    }

    println!(
        "DEBUG: Storing key={} locally in shard {}",
        payload.key, shard_config.current_shard
    );
    db.set(&payload.key, &payload.value).unwrap();
    Json("OK".to_string()).into_response()
}

async fn delete_value(
    State((db, shard_config)): State<(Arc<Database>, Arc<ShardConfig>)>,
    Query(params): Query<KeyQuery>,
) -> impl IntoResponse {
    if !shard_config.is_local_shard(&params.key) {
        let target = shard_config.get_shard_address(&params.key);
        if target == shard_config.current_address() {
            return Json("ERROR: Shard redirection loop detected!").into_response();
        }

        println!("DEBUG: Redirecting DELETE key={} to {}", params.key, target);
        return reqwest::Client::new()
            .post(format!("http://{}/del", target)) // `POST`, а не `GET`
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
            .into_response();
    }

    println!(
        "DEBUG: Deleting key={} locally in shard {}",
        params.key, shard_config.current_shard
    );
    db.delete(&params.key).unwrap();
    Json("Deleted".to_string()).into_response()
}
