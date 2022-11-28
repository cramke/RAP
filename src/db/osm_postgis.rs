use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};

pub fn tester() {
    println!("Yeat");
}

pub struct OSMPostgis {
    pool: i32
}

pub async fn database() {
	println!("Run some async function.");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:password@localhost:5432/osm")
        .await
        .unwrap();

    let param = "0102000020E61000000300000012A5187B8AFF21408390E2D2E7D548407EC9213DA0FF21401822A7AFE7D54840AB6A381BE1FF2140ADE17DB0E7D54840";
    let rows = sqlx::query("SELECT * FROM highway_michelstadt WHERE ST_Crosses(highway_michelstadt.geometry, ($1))")
        .bind(param)
        .fetch_all(&pool).await.unwrap();
    for row in rows {
        let a = row.get::<String, _>("highway");
        print!("{}", a);
}
    println!("\nDone with asysn");
}