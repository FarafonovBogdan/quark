mod db;

use db::Database;

fn main() {
    let db = Database::new("data/db", false).expect("Failed to open DB");

    db.set("key1", "value1").unwrap();
    db.set("key2", "value2").unwrap();

    println!("key1 = {:?}", db.get("key1"));
    println!("key2 = {:?}", db.get("key2"));

    db.delete("key1").unwrap();
    println!("After delete key1: {:?}", db.get("key1"));
}
