# Create the servers

sudo docker compose up -d

# setup the config servers

sudo docker exec -it configs1 mongosh --eval "rs.initiate({
 _id: \"cfgrs\",
 configsvr: true,
 members: [
   {_id: 0, host: \"configs1\"},
   {_id: 1, host: \"configs2\"},
   {_id: 2, host: \"configs3\"}
 ]
})"

sudo docker exec -it configs1 mongosh --eval "rs.status()"


# setup the shard servers

sudo docker exec -it shard1s1 mongosh --eval "rs.initiate({
 _id: \"shard1rs\",
 members: [
   {_id: 0, host: \"shard1s1\"},
   {_id: 1, host: \"shard1s2\"},
   {_id: 2, host: \"shard1s3\"}
 ]
})"


sudo docker exec -it shard1s1 mongosh --eval "rs.status()"


# setup the  mongos server

sudo docker exec -it mongos mongosh --eval "sh.addShard(\"shard1rs/shard1s1:27017,shard1s2:27017,shard1s3:27017\")"