use sqlx::postgres::{PgPoolOptions, PgRow, PgPool};
use sqlx::{FromRow, Row, Postgres, Pool};
use futures::executor::block_on;

pub fn tester() {
    println!("Yeat");
}

pub struct OSMPostgis {
    pool: Pool<Postgres>
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
    let param = "0102000020E61000000300000012A5187B8AFF21408390E2D2E7D548407EC9213DA0FF21401822A7AFE7D54840AB6A381BE1FF2140ADE17DB0E7D54840";
    let rows = sqlx::query("SELECT * FROM highway_michelstadt WHERE ST_Crosses(highway_michelstadt.geometry, ($1))")
    .bind(param)
    .fetch_all(pool).await.unwrap();
    return rows;
}

pub fn process(rows: &Vec<PgRow>) {
    for row in rows {
        let a = row.get::<String, _>("highway");
        print!("{}", a);
    }
}

pub async fn database() {
	println!("Run some async function.");
    let db = make_db_connection().await;
    let rows = query(&db.pool).await;
    process(&rows);

    println!("\nDone with async");
}