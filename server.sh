if [ -z ${SERVICE_ACCOUNT_JSON} ]
then
	echo "Cannot find SERIVE_ACCOUNT_JSON - exiting"
	exit 1
else
	echo "Bucket env var exists..."
fi
if [ -z ${DB_URI} ]
then
	echo "Cannot find DB_URI - exiting"
	exit 1
else
	echo "Bucket env var exists..."
fi

docker run \
	-e "SERVICE_ACCOUNT=/cloud_storage_json/$(basename ${SERVICE_ACCOUNT_JSON})" \
	-v $(dirname $(realpath ${SERVICE_ACCOUNT_JSON})):/cloud_storage_json \
	-e "DB_URI=$DB_URI" \
	-p 80:80 -it --rm --name rw-engineering rw-engineering


