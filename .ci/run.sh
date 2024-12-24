#!/bin/sh

image_id=$(docker build -f .ci/Dockerfile -q .)

if [[ $? -ne 0 ]]; then echo "failed" && exit 1; fi

docker run --rm $image_id test

result=$?

echo "Cleaning up..."
docker rmi $image_id

exit $result
