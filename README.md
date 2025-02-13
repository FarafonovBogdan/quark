## Distributed Key-Value Store
# Features:
1. Sharding – requests are automatically routed to the correct node.
2. HTTP API – simple REST interface for interacting with the database.
3. Logging – transparent request tracking and forwarding.
4. Horizontal scaling – easily add new nodes.

  ## Installation & Setup
 ### 1. Clone the repository
```
git clone https://github.com/FarafonovBogdan/quark.git 
```
``` 
cd distributed-kv-store 
```
 ### 2. Install dependencies
``` cargo build ```
 ### 3. Start three nodes (shards)
``` 
  cargo run -- --shard-index 0 --port 8080 
  cargo run -- --shard-index 1 --port 8081 
  cargo run -- --shard-index 2 --port 8082
```
 ### API Usage
- Set a key
  ```
  curl -X POST -H "Content-Type: application/json" -d '{"key": "user123", "value": "Hello"}' http://127.0.0.1:8080/set
  ```
- Get a key
  ```
  curl "http://127.0.0.1:8080/get?key=user123"
  ```
- Delete a key
  ```
  curl "http://127.0.0.1:8080/del?key=user123"
  ```
  ### If the key is stored on a different shard, the request is automatically forwarded to the correct server.

# How It Works
- The /set request determines the correct shard for the given key and either stores it locally or forwards the request.
- The /get request first checks the local node – if the key isn’t found, it forwards the request to the correct shard.
- The /del request works the same way – first checking locally, then forwarding if necessary.
- Logs help track where requests are going and which node is responsible for each key.
