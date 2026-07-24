#!/bin/bash

ROOT_SCRIPT="$(dirname "$0")/rootless-docker-install-as-root-submodule.sh"

if command -v sudo >/dev/null 2>&1; then
    echo "User password needed."
    sudo "bash -c $ROOT_SCRIPT" "$USER"
else
    echo "Root password needed."
    su - -c "bash -c $ROOT_SCRIPT $USER"
fi

/usr/share/docker.io/contrib/dockerd-rootless-setuptool.sh install --force
