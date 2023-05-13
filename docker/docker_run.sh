# Starts the old database.
# docker start osm_postgis

####################
## New Dockerfile ##
docker container run \
    -d \
    --name=pg \
    -p 5432:5432 \
    -e POSTGRES_PASSWORD=password \
    -e PGDATA=/pgdata \
    -v $(pwd)/docker/pgdata:/pgdata \
    postgres:latest