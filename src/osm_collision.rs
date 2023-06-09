use mpl::collision_checker::{CollisionChecker};
use wkt::ToWkt;
use futures::executor::block_on;
use futures::join;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{Postgres, Pool};
use geo::{Point, LineString};

use crate::db::config::DBConfig;

pub struct GeoCollisionChecker{
    pool: Pool<Postgres>,
}

impl GeoCollisionChecker {
    pub fn new() -> Box<dyn CollisionChecker> {
        let pool: Pool<Postgres> = block_on(GeoCollisionChecker::make_db_connection(DBConfig::default()));
        Box::new(
            GeoCollisionChecker{pool}, 
        )
    }
        
    pub async fn make_db_connection(db_config: DBConfig) -> Pool<Postgres> {
        const MAX_CONNECTIONS: u32 = 5;
        PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(&db_config.generate_url())
            .await
            .unwrap()
    }

    pub async fn async_is_colliding(&self, param: &str) -> (Vec<PgRow>, Vec<PgRow>) {
        let ground_collisions = self.fetch_ground_collision(param);
        let air_collisions = self.fetch_airspace_collision(param);
        join!(ground_collisions, air_collisions)
    }

    pub async fn fetch_ground_collision(&self, param: &str) -> Vec<PgRow> {
        let sql: &str = "SELECT name FROM obstruction_michelstadt WHERE ST_Intersects(obstruction_michelstadt.geometry, ST_GeomFromText(($1), 4326))";
        sqlx::query(sql).bind(param).fetch_all(&self.pool).await.unwrap()
    }

    pub async fn fetch_airspace_collision(&self, param: &str) -> Vec<PgRow> {
        let sql: &str = "SELECT name FROM obstruction_airspace_de WHERE ST_Intersects(obstruction_airspace_de.geometry, ST_GeomFromText(($1), 4326))";
        sqlx::query(sql).bind(param).fetch_all(&self.pool).await.unwrap()
    }
}

impl CollisionChecker for GeoCollisionChecker {

    fn init(&self) -> bool {
        true
    }

    fn is_edge_colliding(&self, start: &Point, end: &Point) -> bool {
        let line: LineString = LineString::from(vec![*start, *end]);
        let (ground_collisions, air_collisions) = block_on(self.async_is_colliding(&line.wkt_string()));
        !ground_collisions.is_empty() || !air_collisions.is_empty()
    }

    fn is_node_colliding(&self, node: &geo::Point) -> bool {
        let (ground_collisions, air_collisions) = block_on(self.async_is_colliding(&node.wkt_string()));
        !ground_collisions.is_empty() ||!air_collisions.is_empty()
    }
}