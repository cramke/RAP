use std::time::Instant;

mod db;
use db::osm_optimizer;
use prm::optimizer::{Optimizer};
use prm::node::Node2D;
use prm::boundaries::Boundaries;
use prm::problem::ProblemDefinition;

fn is_collision(_node: &Node2D) -> bool {
    // TODO: no collision checking as of now
    return false;
}

fn is_edge_in_collision() -> bool {
    return false;
}

fn run_example() {
    let start: Node2D = Node2D { x: 8.926f64, y: 49.67f64, idx: 0 };
    let goal: Node2D = Node2D { x: 9.07f64, y: 49.71f64, idx: 0 };
    let bounds: Boundaries = Boundaries { x_lower: 8.925f64, x_upper: 9.08f64, y_lower: 49.66f64, y_upper: 49.72f64 };
    let optimizer: Box<dyn Optimizer> = osm_optimizer::OSMPostgisOptimizer::new();
    let mut pdef= ProblemDefinition::new( start, goal, bounds, is_collision, is_edge_in_collision, optimizer);                                       
    pdef.solve();
    pdef.print_statistics();
}

fn main() {
    println!("#### PRM ####");
    let start = Instant::now();
    run_example();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}