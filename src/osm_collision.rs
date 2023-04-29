use mpl::collision_checker::{CollisionChecker};
use wkt::ToWkt;
use crate::db::{osm_postgis};
use futures::executor::block_on;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Postgres, Pool};
use geo::{Point, LineString};


pub struct GeoCollsionChecker{
    pool: Pool<Postgres>,
}

impl GeoCollsionChecker {
    pub fn new() -> Box<dyn CollisionChecker> {
        let pool: Pool<Postgres> = block_on(GeoCollsionChecker::make_db_connection());
        return Box::new(GeoCollsionChecker{pool});
    }
        
    pub async fn make_db_connection() -> Pool<Postgres> {
        const MAX_CONNECTIONS: u32 = 5;
        const URL: &str = "postgresql://postgres:password@localhost:5432/osm";
        let db: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(URL)
            .await
            .unwrap();
        return db;
    }

    pub async fn fetch_is_contain(&self, param: &str) -> Vec<PgRow> {
        osm_postgis::fetch_contains_wkt(&self.pool, param).await
    }
}

impl CollisionChecker for GeoCollsionChecker {

    fn init(&self) -> bool {
        return true;
    }

    fn is_edge_colliding(&self, start: &Point, end: &Point) -> bool {
        let line = LineString::from(vec![*start, *end]);
        let collisions = block_on(self.fetch_is_contain(&line.wkt_string()));
        collisions.len() > 0 
    }

    fn is_node_colliding(&self, node: &geo::Point) -> bool {
        let collisions = block_on(self.fetch_is_contain(&node.wkt_string()));
        collisions.len() > 0 
    }
}