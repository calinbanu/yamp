#!/bin/bash

echo "Dev enviroment setup"

export DEV_ENV_DOCKER_OPTIONS="--group-add=audio --group-add=video"

if [ -f "/dev/snd" ]; then
    export DEV_ENV_DOCKER_OPTIONS="$DEV_ENV_DOCKER_OPTIONS /dev/snd"
fi

if [ -f "/dev/dri" ]; then
    export DEV_ENV_DOCKER_OPTIONS="$DEV_ENV_DOCKER_OPTIONS /dev/dri"
fi

if [ -f "/dev/video0" ]; then
    export DEV_ENV_DOCKER_OPTIONS="$DEV_ENV_DOCKER_OPTIONS /dev/video0"
fi

echo "DEV_ENV_DOCKER_OPTIONS: $DEV_ENV_DOCKER_OPTIONS"