use std::time::Instant;

mod db;
use db::{osm_optimizer, osm_postgis};
use geo::Point;
use mpl::collision_checker::{CollisionChecker, NaiveCollisionChecker};
use mpl::optimizer::{Optimizer};
use mpl::boundaries::Boundaries;
use mpl::problem::{ProblemDefinition, Parameter};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Postgres, Pool};
use futures::executor::block_on;
use wkt::ToWkt;

struct GeoCollsionChecker{
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

    pub async fn fetch_is_contain(&self, param: &str) -> Vec<String> {
        let rows = osm_postgis::fetch_contains_wkt(&self.pool, param).await;
        return osm_postgis::process(&rows);
    }
}

impl CollisionChecker for GeoCollsionChecker {

    fn init(&self) -> bool {
        return true;
    }

    fn is_edge_colliding(&self, node: &geo::Point, end: &geo::Point) -> bool {
        return true;
    }

    fn is_node_colliding(&self, node: &geo::Point) -> bool {
        // block_on(self.fetch_is_contain(&node.wkt_string()));
        return false;
    }
}

fn run_example_postgis() {
    let start: Point = Point::new(8.936f64, 49.67f64);
    let goal: Point = Point::new(9.07f64, 49.71f64);
    let bounds: Boundaries = Boundaries::new(8.935f64, 9.08f64, 49.665f64, 49.72f64);
    let optimizer: Box<dyn Optimizer> = osm_optimizer::OSMPostgisOptimizer::new();
    let params: Parameter = Parameter::default();
    let collision_checker: Box<dyn CollisionChecker> = NaiveCollisionChecker::new_box();
    // let collision_checker: Box<dyn CollisionChecker> = GeoCollsionChecker::new();
    let mut pdef= ProblemDefinition::new( start, goal, bounds, optimizer, params, collision_checker);
    
    println!("#### mpf ####");
    let start = Instant::now();
    pdef.solve();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    let path: &str = "data/graph.dot";
    pdef.print_statistics(path);    
    let path: &str = "data/solution_path.txt";
    pdef.write_solution_path(path);
}

fn run_example_geo() {

}

fn main() {
    run_example_postgis();
}