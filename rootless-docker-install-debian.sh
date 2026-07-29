#!/bin/bash

ROOT_SCRIPT="$(pwd)/rootless-docker-install-as-root-submodule.sh"

if command -v sudo >/dev/null 2>&1; then
    echo "User password needed."
    sudo "$ROOT_SCRIPT" "$USER"
else
    echo "Root password needed."
    su - -c "$ROOT_SCRIPT $USER"
fi

# -- Install as user --

PATH=$PATH:/usr/share/docker.io/contrib/ dockerd-rootless-setuptool.sh install --force

LINE_TO_ADD='export DOCKER_HOST="unix://$XDG_RUNTIME_DIR/docker.sock"'
if ! grep -Fxq "$LINE_TO_ADD" "$HOME/.bashrc"; then
    echo -e "\n$LINE_TO_ADD" >> "$HOME/.bashrc"
fi
