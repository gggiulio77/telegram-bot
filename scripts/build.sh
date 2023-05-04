#!/bin/bash

echo "building rust binary..."

cargo build --bin function --target x86_64-unknown-linux-musl --release

echo "building builder image with cache..."

docker build --target builder -t localhost:32000/telegram-bot:builder .

echo "building image with rust binary from builder cache..."

docker build --cache-from localhost:32000/telegram-bot:builder -t localhost:32000/telegram-bot:latest .

echo "pushing image to localhost:32000 registry"

sleep 2s

docker image push localhost:32000/telegram-bot:latest

image_digest=$(docker inspect --format='{{index .RepoDigests 0}}' localhost:32000/telegram-bot:latest)

echo "image digest: ${image_digest}"

sed -Ei.bak "{n;s~(image: ).*~\1$image_digest~}" /home/gg/telegram-bot/server.service.yml