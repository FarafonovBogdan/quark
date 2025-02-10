use axum::Router;
use clap::Parser;
use std::sync::Arc;
use tokio::net::TcpListener;

mod db;
mod server;
mod sharding;

use db::Database;
use server::routes;
use sharding::ShardConfig;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "0")]
    shard_index: usize,

    #[arg(long, default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let shard_config = Arc::new(ShardConfig::new(
        vec![
            "127.0.0.1:8080".to_string(),
            "127.0.0.1:8081".to_string(),
            "127.0.0.1:8082".to_string(),
        ],
        args.shard_index,
    ));

    // Уникальная папка для каждого шарда
    let db_path = format!("data/db_{}", args.port);
    let db = Arc::new(Database::new(&db_path, false).expect("Failed to open DB"));

    println!("Starting shard: {:?}", shard_config);
    let addr = format!("127.0.0.1:{}", args.port);
    let listener = TcpListener::bind(&addr).await.unwrap();

    let app = Router::new().merge(routes(db.clone(), shard_config.clone()));

    println!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
