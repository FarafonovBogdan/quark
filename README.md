## Distributed Key-Value Store
# Features:
### Sharding – requests are automatically routed to the correct node.
### HTTP API – simple REST interface for interacting with the database.
### Logging – transparent request tracking and forwarding.
### Horizontal scaling – easily add new nodes.

  ## Installation & Setup
 ### 1. Clone the repository
``` git clone https://github.com/FarafonovBogdan/quark.git ```
``` cd distributed-kv-store ```
 ### 2. Install dependencies
``` cargo build --release ```
 ### 3. Start three nodes (shards)
``` cargo run -- --port 8080 ```
``` cargo run -- --port 8081 ```
``` cargo run -- --port 8082 ```
 ### API Usage
- Set a key
  ``` curl -X POST -H "Content-Type: application/json" -d '{"key": "user123", "value": "Hello"}' http://127.0.0.1:8080/set ```
- Get a key
  ``` curl "http://127.0.0.1:8080/get?key=user123" ```
- Delete a key
  ``` curl "http://127.0.0.1:8080/del?key=user123" ```
  ### If the key is stored on a different shard, the request is automatically forwarded to the correct server.

# How It Works
- The /set request determines the correct shard for the given key and either stores it locally or forwards the request.
- The /get request first checks the local node – if the key isn’t found, it forwards the request to the correct shard.
- The /del request works the same way – first checking locally, then forwarding if necessary.
- Logs help track where requests are going and which node is responsible for each key.
