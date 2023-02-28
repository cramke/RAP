use sqlx::postgres::PgRow;
use sqlx::{Row, Postgres, Pool};

pub async fn fetch_intersections_wkt(pool: &Pool<Postgres>, param: &str) -> Vec<PgRow> {
    let sql = "SELECT * FROM highway_michelstadt WHERE ST_Crosses(highway_michelstadt.geometry, ST_GeomFromText(($1), 4326))";
    sqlx::query(sql)
        .bind(param)
        .fetch_all(pool).await.unwrap()
}

pub async fn fetch_contains_wkt(pool: &Pool<Postgres>, param: &str) -> Vec<PgRow> {
    let sql: &str = "SELECT * FROM obstruction_michelstadt WHERE ST_Crosses(obstruction_michelstadt.geometry, ST_GeomFromText(($1), 4326))";
    sqlx::query(sql)
        .bind(param)
        .fetch_all(pool).await.unwrap()
}
    
pub fn process(rows: &Vec<PgRow>) -> Vec<String> {
    const INDEX: &str = "highway";
    let mut highways: Vec<String> = Vec::new();
    for row in rows {
        let highway: String = row.get::<String, _>(INDEX);
        highways.push(highway);
    }
    return highways;
}