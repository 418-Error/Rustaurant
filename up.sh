#!/usr/bin/bash
set -e

# create network

docker network create mongodb-net || true

# up the servers

docker compose up -d

# setup the config servers

docker exec -it configs1 mongosh --eval "rs.initiate({
 _id: \"cfgrs\",
 configsvr: true,
 members: [
   {_id: 0, host: \"configs1\"},
   {_id: 1, host: \"configs2\"},
   {_id: 2, host: \"configs3\"}
 ]
})"

docker exec -it configs1 mongosh --eval "rs.status()"

# setup the shard servers

docker exec -it shard1s1 mongosh --eval "rs.initiate({
 _id: \"shard1rs\",
 members: [
   {_id: 0, host: \"shard1s1\"},
   {_id: 1, host: \"shard1s2\"},
   {_id: 2, host: \"shard1s3\"}
 ]
})"

docker exec -it shard1s1 mongosh --eval "rs.status()"

# setup the  mongos server

docker exec -it mongos1 mongosh --eval "db.createUser(
 {
    user: \"admin\",
    pwd: \"admin\",
    roles: [
      { role: \"clusterAdmin\", db: \"admin\" },
      { role: \"readWriteAnyDatabase\", db: \"admin\" },
      { role: \"userAdminAnyDatabase\", db: \"admin\" }
    ]
  }
)" admin

# add MONGO_URI to .env file
echo "MONGO_URI=mongodb://admin:admin@127.0.0.1:30001" >.env
