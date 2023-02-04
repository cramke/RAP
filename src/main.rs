use std::time::Instant;

mod db;
use db::osm_optimizer;
use geo::Point;
use mpf::collision_checker::{CollisionChecker, NaiveCollisionChecker};
use mpf::optimizer::{Optimizer};
use mpf::node::Node2D;
use mpf::boundaries::Boundaries;
use mpf::problem::{ProblemDefinition, Parameter};

fn is_collision(_node: &Node2D) -> bool {
    // TODO: no collision checking as of now
    return false;
}

fn is_edge_in_collision() -> bool {
    return false;
}

struct GeoCollsionChecker{}

impl CollisionChecker for GeoCollsionChecker {

    fn init(&self) -> bool {
        return true;
    }

    fn is_edge_colliding(&self, node: &geo::Point, end: &geo::Point) -> bool {
        return true;
    }

    fn is_node_colliding(&self, node: &geo::Point) -> bool {
        return true;
    }
}

fn run_example_postgis() {
    let start: Point = Point::new(8.936f64, 49.67f64);
    let goal: Point = Point::new(9.07f64, 49.71f64);
    let bounds: Boundaries = Boundaries::new(8.935f64, 9.08f64, 49.665f64, 49.72f64);
    let optimizer: Box<dyn Optimizer> = osm_optimizer::OSMPostgisOptimizer::new();
    let params: Parameter = Parameter::new(100usize);
    let collision_checker: Box<dyn CollisionChecker> = NaiveCollisionChecker::new();
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