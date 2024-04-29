#!/usr/bin/bash
set -e

# create network

docker network create mongodb-net || true

# up the servers

docker compose up -d

# waiting for configs1 to be ready
until docker exec -it configs1 mongosh --eval "db" 2>/dev/null; do
  echo "Waiting for configs1 to be ready..."
  sleep 1
done

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
   {_id: 2, host: \"shard1s3\"},
 ]
})"
docker exec -it shard2s1 mongosh --eval "rs.initiate({
 _id: \"shard2rs\",
 members: [
   {_id: 0, host: \"shard2s1\"},
   {_id: 1, host: \"shard2s2\"},
   {_id: 2, host: \"shard2s3\"},
 ]
})"
docker exec -it shard3s1 mongosh --eval "rs.initiate({
 _id: \"shard3rs\",
 members: [
   {_id: 0, host: \"shard3s1\"},
   {_id: 1, host: \"shard3s2\"},
   {_id: 2, host: \"shard3s3\"},
 ]
})"
docker exec -it shard1s1 mongosh --eval "rs.status()"

# Attendez que mongos1 soit prêt
until docker exec -it mongos1 mongosh --eval "db" > /dev/null; do
  echo "Waiting for mongos1 to be ready..."
  sleep 1
done

# setup the  mongos server
docker exec -it mongos1 mongosh --eval "sh.addShard(\"shard1rs/shard1s1:27017,shard1s2:27017,shard1s3:27017\")"
docker exec -it mongos1 mongosh --eval "sh.addShard(\"shard2rs/shard2s1:27017,shard2s2:27017,shard2s3:27017\")"
docker exec -it mongos1 mongosh --eval "sh.addShard(\"shard3rs/shard3s1:27017,shard3s2:27017,shard3s3:27017\")"

docker exec -it mongos1 mongosh --eval "db.Rustaurant.createUser(
  {
    user: \"rustaurant_operator\",
    pwd: \"A2kdjl32nfklsd94820DlfjdIDF83KFK33fnldk3EfZS\",
    roles: [ { role: \"readWrite\", db: \"Rustaurant\" } ]
  }
)" admin

# add MONGO_URI to .env file
echo "MONGO_URI=mongodb://rustaurant_operator:A2kdjl32nfklsd94820DlfjdIDF83KFK33fnldk3EfZS@127.0.0.1:30001" >.env
