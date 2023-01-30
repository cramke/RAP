
use prm::optimizer::{Optimizer};

use sqlx::postgres::{PgPoolOptions};
use sqlx::{Pool, Postgres};
use futures::executor::block_on;

use prm::node::Node2D;
use crate::db::{osm_postgis, costs};

#[derive(Debug, Clone)]
pub struct OSMPostgisOptimizer {
    pool: Pool<Postgres>
}
impl OSMPostgisOptimizer {
    pub fn new() -> Box<dyn Optimizer> {
        let pool: Pool<Postgres> = block_on(OSMPostgisOptimizer::make_db_connection());
        return Box::new(OSMPostgisOptimizer{pool});
    }
    
    pub async fn make_db_connection() -> Pool<Postgres> {
        const MAX_CONNECTIONS: u32 = 1;
        const URL: &str = "postgresql://postgres:password@localhost:5432/osm";
        let db: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(MAX_CONNECTIONS)
            .connect(URL)
            .await
            .unwrap();
        return db;
    }

    pub async fn fetch_intersecting_highways(&self, param: &str) -> Vec<String> {
        let rows = osm_postgis::fetch_intersections_wkt(&self.pool, param).await;
        return osm_postgis::process(&rows);
    }
}

impl Optimizer for OSMPostgisOptimizer {
    fn init(&mut self) -> bool {
        self.pool = block_on(OSMPostgisOptimizer::make_db_connection());
        return true;
    }

    fn get_edge_weight(&self, begin: &Node2D, end: &Node2D) -> f64 {
        let intersections: Vec<String> = block_on(self.fetch_intersecting_highways(&begin.get_line_wkt(&end)));
        let cost = costs::get_cost_from_types(intersections);
        return cost as f64;
    }
}

