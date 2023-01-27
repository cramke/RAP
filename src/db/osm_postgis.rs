use std::time::Instant;

use sqlx::postgres::{PgPoolOptions, PgRow, PgPool, PgArguments};
use sqlx::{FromRow, Row, Postgres, Pool};
use futures::executor::block_on;

pub struct OSMPostgis {
    pub pool: Pool<Postgres>,
}

impl OSMPostgis {
    pub async fn make_db_connection() -> Self {
        let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:password@localhost:5432/osm")
        .await
        .unwrap();
    return OSMPostgis{pool: db};
    }

    pub async fn fetch_intersecting_highways(&self, param: &str) -> Vec<String> {
        let rows = fetch_intersections_wkt(&self.pool, param).await;
        return process(&rows);
    }
}

pub async fn make_db_connection() -> OSMPostgis {
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:password@localhost:5432/osm")
        .await
        .unwrap();
    return OSMPostgis{pool: db};
}

pub async fn query(pool: &Pool<Postgres>) -> Vec<PgRow> {
    let param: &str = "0102000020E61000000300000012A5187B8AFF21408390E2D2E7D548407EC9213DA0FF21401822A7AFE7D54840AB6A381BE1FF2140ADE17DB0E7D54840";
    let rows: Vec<PgRow> = sqlx::query("SELECT * FROM highway_michelstadt WHERE ST_Crosses(highway_michelstadt.geometry, ($1))")
        .bind(param)
        .fetch_all(pool).await.unwrap();
    return rows;
}

pub async fn fetch_intersections_wkt(pool: &Pool<Postgres>, param: &str) -> Vec<PgRow> {
    let rows: Vec<PgRow> = sqlx::query("SELECT * FROM highway_michelstadt WHERE ST_Crosses(highway_michelstadt.geometry, ST_GeomFromText(($1), 4326))")
        .bind(param)
        .fetch_all(pool).await.unwrap();
    return rows;
}

pub fn process(rows: &Vec<PgRow>) -> Vec<String> {
    const INDEX: &str = "highway";
    let cost: i32 = 0;
    let mut highways: Vec<String> = Vec::new();
    for row in rows {
        let mut highway: String = row.get::<String, _>(INDEX);
        highways.push(highway);
    }
    return highways;
}

pub async fn fetch_intersecting_highways(param: &str) -> Vec<String> {
    let start = Instant::now();    
    let db = make_db_connection().await;
    let duration = start.elapsed();
    println!("Time elapsed in making db connection is: {:?}", duration);

    let start2 = Instant::now();    
    let rows = fetch_intersections_wkt(&db.pool, param).await;
    let duration2 = start2.elapsed();
    println!("Time elapsed in fetching from db: {:?}", duration2);
    return process(&rows);
}