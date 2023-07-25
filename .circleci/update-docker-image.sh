#!/bin/bash

VERSION=0.0.0
REPO=banucalin/yamp
PUSH=1

IMG_ID=$(docker images $REPO -aq)

if [ ! -z $IMG_ID ]; then
    echo "Removing docker image: $IMG_ID"
    docker rmi $IMG_ID
fi

docker build -t $REPO:$VERSION .

if [ $PUSH -eq 1 ]; then
    docker login -u banucalin
    docker push $REPO:$VERSION
fi