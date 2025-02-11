#!/bin/bash


PORTS=(8080 8081 8082)
SHARD_IDS=(0 1 2)


echo "Starting servers with sharding..."
for i in "${!PORTS[@]}"; do
    PORT=${PORTS[$i]}
    SHARD_ID=${SHARD_IDS[$i]}

    cargo run -- --shard-index "$SHARD_ID" --port "$PORT" &
    PIDS+=($!)
    echo "Server (shard $SHARD_ID) started on port $PORT (PID: ${PIDS[-1]})"
done


sleep 5


echo "Testing API with sharding..."
KEY="user123"
VALUE="Hello, Sharding!"


FIRST_PORT=${PORTS[0]}
echo "Adding key ($KEY) to port $FIRST_PORT"
curl -X POST "http://localhost:$FIRST_PORT/set" -H "Content-Type: application/json" -d "{\"key\": \"$KEY\", \"value\": \"$VALUE\"}"


for PORT in "${PORTS[@]}"; do
    echo -e "\n Fetching key ($KEY) from port $PORT"
    curl -X GET "http://localhost:$PORT/get?key=$KEY"

    echo -e "\n-----------------------------"
done


echo "Stopping servers..."
for PID in "${PIDS[@]}"; do
    kill "$PID"
    echo "Waiting for process $PID to exit..."
    wait "$PID"
done

echo "All servers have been stopped."
