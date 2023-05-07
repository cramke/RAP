use std::time::Instant;
use geo::Point;
use mpl::collision_checker::{CollisionChecker};
use mpl::optimizer::{Optimizer};
use mpl::boundaries::Boundaries;
use mpl::problem::{ProblemDefinition, Parameter};
use rap::backend::server;
use rap::{osm_optimizer, osm_collision::GeoCollsionChecker};

fn run_example_postgis() {
    let start: Point = Point::new(8.936f64, 49.67f64);
    let goal: Point = Point::new(9.07f64, 49.71f64);
    let bounds: Boundaries = Boundaries::new(8.935f64, 9.08f64, 49.665f64, 49.72f64);
    let optimizer: Box<dyn Optimizer> = osm_optimizer::OSMPostgisOptimizer::new();
    let params: Parameter = Parameter{max_size:15, k_nearest_neighbors:5};
    let collision_checker: Box<dyn CollisionChecker> = GeoCollsionChecker::new();
    let mut pdef= ProblemDefinition::new( start, goal, bounds, optimizer, params, collision_checker);
    
    println!("#### mpl ####");
    let start = Instant::now();
    pdef.solve();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    let path: &str = "data/graph.dot";
    pdef.print_statistics(path);    
    let path: &str = "data/solution_path.txt";
    pdef.write_solution_path(path);
}

#[actix_web::main]
async fn main() {
    run_example_postgis();
    server::launch();
}