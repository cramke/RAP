
use std::collections::HashMap;

use mpl::optimizer::{Optimizer};
use geo::{Point, LineString};
use wkt::ToWkt;

use sqlx::postgres::{PgPoolOptions};
use sqlx::{Pool, Postgres};
use futures::executor::block_on;

use crate::db::config::DBConfig;
use crate::db::{osm_postgis, costs};

#[derive(Debug, Clone)]
pub struct OSMPostgisOptimizer {
    pool: Pool<Postgres>,
    cost_map: HashMap<String, i32>
}
impl OSMPostgisOptimizer {
    pub fn new() -> Box<dyn Optimizer> {
        let pool: Pool<Postgres> = block_on(OSMPostgisOptimizer::make_db_connection(DBConfig::default()));
        let cost_map: HashMap<String, i32> = costs::read_highway_costs();
        return Box::new(OSMPostgisOptimizer{pool, cost_map});
    }
    
    pub async fn make_db_connection(db_config: DBConfig) -> Pool<Postgres> {
        const MAX_CONNECTIONS: u32 = 5;
        let db: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(&db_config.generate_url())
            .await
            .unwrap();
        return db;
    }

    pub async fn fetch_intersecting_highways(&self, param: &str) -> Vec<String> {
        let rows = osm_postgis::fetch_intersects_wkt(&self.pool, param).await;
        return osm_postgis::process(&rows);
    }

    async fn async_query(&self, begin: Point, end: Point) -> (Point, Point, f64) {
        let line: LineString = geo::LineString::from(vec![begin, end]);
        let intersections: Vec<String> = self.fetch_intersecting_highways(&line.wkt_string()).await;
        let cost = costs::get_cost_from_types(intersections, &self.cost_map);
        return (begin, end, cost as f64);
    }
}

impl Optimizer for OSMPostgisOptimizer {
    fn init(&mut self) -> bool {
        self.pool = block_on(OSMPostgisOptimizer::make_db_connection(DBConfig::default()));
        return true;
    }

    fn get_edge_weight(&self, begin: Point, end: Point) -> (Point, Point, f64) {
        return block_on(self.async_query(begin, end));
    }
}

