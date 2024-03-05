#!/usr/bin/bash

set -e

docker compose down

docker volume rm rustaurant_configs1
docker volume rm rustaurant_configs2
docker volume rm rustaurant_configs3
docker volume rm rustaurant_shard1s1
docker volume rm rustaurant_shard1s2
docker volume rm rustaurant_shard1s3
