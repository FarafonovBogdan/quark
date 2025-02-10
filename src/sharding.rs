use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize)] // Добавляем Serialize и Deserialize
pub struct ShardConfig {
    pub shards: Vec<String>,
    pub current_shard: usize,
}

impl ShardConfig {
    pub fn new(shards: Vec<String>, current_shard: usize) -> Self {
        Self {
            shards,
            current_shard,
        }
    }

    pub fn current_address(&self) -> &str {
        &self.shards[self.current_shard]
    }

    pub fn get_shard(&self, key: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.shards.len()
    }

    pub fn is_local_shard(&self, key: &str) -> bool {
        let shard_idx = self.get_shard(key);
        println!(
            "Key={} → Shard={}, Current={}, Local={}",
            key,
            shard_idx,
            self.current_shard,
            shard_idx == self.current_shard
        );
        shard_idx == self.current_shard
    }

    pub fn get_shard_address(&self, key: &str) -> &str {
        let shard_idx = self.get_shard(key);
        &self.shards[shard_idx]
    }
}
