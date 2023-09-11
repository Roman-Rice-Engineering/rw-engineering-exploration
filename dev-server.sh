#! /usr/bin/env bash

NETWORK_NAME="test"
DB_CONTAINER_NAME="mongo-test"
SCRIPT_DIR="$(dirname $(realpath $0))"

# Set up cleanup
trap cleanup INT

function cleanup() {
	docker stop $DB_CONTAINER_NAME
	exit 0
}

# Ensure that test network exists
docker network ls | grep  " $NETWORK_NAME " > /dev/null \
	&& echo "Network '$NETWORK_NAME' found, continuing." \
	|| { echo "Network '$NETWORK_NAME' does not exist, creating network..."; docker network create test; }

# Start mongodb test database container -- accessible at 'localhost:27017'
docker run --name $DB_CONTAINER_NAME -d --rm -p 27017:27017 --network $NETWORK_NAME mongo:7.0.1
export DB_URI="mongodb://localhost:27017"

# Start backend server
BACKEND_PATH="$SCRIPT_DIR/target/debug/backend"
cd "$SCRIPT_DIR/backend" && cargo build && $BACKEND_PATH & 
export BACKEND_URL="http://localhost:8000"

#docker run -it --network test --rm mongo mongosh --host mongo-test test
cd "$SCRIPT_DIR/frontend" && trunk serve

# Cleanup in case we reach the end of file
cleanup

