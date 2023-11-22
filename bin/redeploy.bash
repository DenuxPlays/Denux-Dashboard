#!/usr/bin/env bash

echo "Redeploying..."
set -e

echo "Shutting down containers..."
docker compose down

echo "Building new images..."
docker compose build

echo "Starting containers..."
docker compose up -d
