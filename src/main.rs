use std::collections::HashMap;
use std::time::Instant;

use prm::optimizer::{Optimizer};
use sqlx::postgres::{PgPoolOptions};
use sqlx::{Row, Pool, Postgres};
use futures::executor::block_on;

mod db;
use db::osm_postgis;
use db::costs;

use prm::node::Node2D;
use prm::boundaries::Boundaries;
use prm::problem::ProblemDefinition;

use crate::db::osm_postgis::{OSMPostgis, query};

fn get_cost_from_types(intersections: Vec<String>) -> i32 {
    let test2: HashMap<String, i32> = costs::read_highway_costs();
    let mut cost_cumulative = 0;
    for intersection in intersections {
        let cost = test2.get(&intersection).unwrap();
        cost_cumulative = cost_cumulative + cost;
    }
    println!("Cumulative cost of line crossings: {}", cost_cumulative);
    return cost_cumulative;
}

fn is_collision(node: &Node2D) -> bool {
    return false;
}

fn is_edge_in_collision() -> bool {
    return false;
}

fn run_example() {
    let start: Node2D = Node2D { x: 8.926f64, y: 49.67f64, idx: 0 };
    let goal: Node2D = Node2D { x: 9.07f64, y: 49.71f64, idx: 0 };
    let bounds: Boundaries = Boundaries { x_lower: 8.925f64, x_upper: 9.08f64, y_lower: 49.66f64, y_upper: 49.72f64 };
    let optimizer: Box<dyn Optimizer> = OSMPostgisOptimizer::new();
    let mut pdef= ProblemDefinition::new( start, goal, bounds, is_collision, is_edge_in_collision, optimizer);                                       
    pdef.solve();
    pdef.print_statistics();
}

#[derive(Debug, Clone)]
struct OSMPostgisOptimizer {
    pool: Pool<Postgres>
}
impl OSMPostgisOptimizer {
    pub fn new() -> Box<dyn Optimizer> {
        let pool: Pool<Postgres> = block_on(OSMPostgisOptimizer::make_db_connection());
        return Box::new(OSMPostgisOptimizer{pool});
    }
    pub async fn make_db_connection() -> Pool<Postgres> {
        let mut db: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgresql://postgres:password@localhost:5432/osm")
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
        return true;
    }

    fn get_edge_weight(&self, begin: &Node2D, end: &Node2D) -> f64 {
        let intersections: Vec<String> = block_on(self.fetch_intersecting_highways(&begin.get_line_wkt(&end)));
        let cost = get_cost_from_types(intersections);
        println!("Used the wanted function");
        return cost as f64;
    }
}

fn main() {
    let node = Node2D::new(8.99910, 49.67114);
    let other = Node2D::new(8.99927, 49.67113);
    println!("{}", node.get_line_wkt(&other));
    let intersections: Vec<String> = block_on(db::osm_postgis::fetch_intersecting_highways(&node.get_line_wkt(&other)));
    get_cost_from_types(intersections);


    println!("#### PRM ####");
    let start = Instant::now();
    run_example();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    let conn: OSMPostgis = block_on(OSMPostgis::make_db_connection());
    let res = block_on(query(&conn.pool));
    const INDEX: &str = "highway";
    for row in res {
        println!("{} ", row.get::<String, _>(INDEX));
    }

}