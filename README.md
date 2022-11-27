# Postgis
## Configure
Requires:
    Postgis
    QGIS: https://www.qgis.org/de/site/forusers/alldownloads.html#debian-ubuntu
    docker: 
docker volume create osm_data
docker run --name=osm_postgis -d -e POSTGRES_PASSWORD=password -e POSTGRES_DBNAME=osm -p 5432:5432 -v osm_data:/var/lib/postgresql postgis/postgis
docker container exec -it osm_postgis psql -U postgres -d osm
CREATE EXTENSION postgis;

## Upload Data
use Qgis overpass plugin to query data
Export to geojson
Use ./Tools/geojson.ipynb to upload geojson to postgis. 

## Connect
docker container exec -it osm_postgis bash
docker container exec -it osm_postgis psql -U postgres -d osm

