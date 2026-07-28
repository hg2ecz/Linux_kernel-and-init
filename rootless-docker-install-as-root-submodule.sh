#!/bin/bash

USER_NAME="$1"

apt update
apt install -y docker.io docker-compose docker-buildx rootlesskit uidmap slirp4netns dbus-user-session

systemctl disable --now docker.service docker.socket containerd.service

grep -q "^${USER_NAME}:" /etc/subuid || echo "${USER_NAME}:100000:65536" >> /etc/subuid
grep -q "^${USER_NAME}:" /etc/subgid || echo "${USER_NAME}:100000:65536" >> /etc/subgid
