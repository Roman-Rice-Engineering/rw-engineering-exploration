#! /usr/bin/env bash

NETWORK_NAME="test"
DB_CONTAINER_NAME="mongo-test"
DEV_SERVER_CONTAINER_NAME="rw-engineering-dev-server"
DEV_SERVER_IMAGE_NAME="rw-engineering-dev"
SCRIPT_DIR="$(dirname $(realpath $0))"

# Set up cleanup
trap cleanup INT

if [ -z ${STORAGE_BUCKET_NAME} ]
then
	echo "Cannot find STORAGE_BUCKET_NAME - exiting"
	exit 1
else
	echo "Bucket env var exists..."
fi
if [ -z ${SERVICE_ACCOUNT_JSON} ]
then
	echo "Cannot find SERIVE_ACCOUNT_JSON - exiting"
	exit 1
else
	echo "Bucket env var exists..."
fi


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
DB_URI="mongodb://mongo-test:27017/test?directConnection=true"
if [ -z ${MONGO_URI} ]
then
	echo "Using predefined mongodb..."
else
	DB_URI=$MONGO_URI
fi

# Package dev server 
docker build --target dev $SCRIPT_DIR -t $DEV_SERVER_IMAGE_NAME
docker run \
	-v $SCRIPT_DIR/frontend/src:/rw-engineering/frontend/src \
	-v $SCRIPT_DIR/common/src:/rw-engineering/common/src \
	-v $SCRIPT_DIR/backend/src:/rw-engineering/backend/src \
	-e "DB_URI=$DB_URI" \
	-e "API_URL=/api/" \
	-e "IS_PRODUCTION=false" \
	-e "STORAGE_BUCKET_NAME=${STORAGE_BUCKET_NAME}" \
	-e "SERVICE_ACCOUNT=/cloud_storage_json/$(basename ${SERVICE_ACCOUNT_JSON})" \
	-v $(dirname $(realpath ${SERVICE_ACCOUNT_JSON})):/cloud_storage_json \
	--network $NETWORK_NAME -p 7000:80 -it --rm --name $DEV_SERVER_CONTAINER_NAME $DEV_SERVER_IMAGE_NAME

# Cleanup in case we reach the end of file
cleanup

