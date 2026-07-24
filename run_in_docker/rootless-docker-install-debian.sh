#!/bin/bash

ROOT_SCRIPT="$(pwd)/rootless-docker-install-as-root-submodule.sh"

if command -v sudo >/dev/null 2>&1; then
    echo "User password needed."
    sudo "$ROOT_SCRIPT" "$USER"
else
    echo "Root password needed."
    su - -c "$ROOT_SCRIPT $USER"
fi

PATH=$PATH:/usr/share/docker.io/contrib/ dockerd-rootless-setuptool.sh install --force
