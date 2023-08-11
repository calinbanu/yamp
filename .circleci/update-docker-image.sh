#!/bin/bash

VERSION=1.1.0
REPO=banucalin/yamp
PUSH=1

docker build --build-arg VERSION=$VERSION -t $REPO:$VERSION .

if [ $PUSH -eq 1 ]; then
    # TODO(calin) Check if allready loged in
    # Login to docker hub
    docker login -u banucalin

    # Add latest tag
    docker tag $REPO:$VERSION $REPO:latest

    # Push image to docker hub
    docker push -a $REPO
fi