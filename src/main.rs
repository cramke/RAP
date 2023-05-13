use std::time::Instant;
use geo::Point;
use mpl::collision_checker::{CollisionChecker};
use mpl::optimizer::{Optimizer};
use mpl::boundaries::Boundaries;
use mpl::problem::{ProblemDefinition, Parameter};
use rap::backend::server;
use rap::{osm_optimizer, osm_collision, problem_definition};

fn run_example_postgis() {
    let path = "data/configs/problem_description_standard.json";
    let p_config = problem_definition::ProblemConfig::from_file(path);

    let start: Point = Point::new(p_config.start_lon, p_config.start_lat);
    let goal: Point = Point::new(p_config.goal_lon, p_config.goal_lat);
    let bounds: Boundaries = Boundaries::new(
        p_config.boundaries.x_lower,
        p_config.boundaries.x_upper,
        p_config.boundaries.y_lower,
        p_config.boundaries.y_upper,
    );
    let optimizer: Box<dyn Optimizer> = osm_optimizer::OSMPostgisOptimizer::new();
    let params: Parameter = Parameter{max_size:p_config.nodes_max, k_nearest_neighbors:p_config.k_nearest_neighbors};
    let collision_checker: Box<dyn CollisionChecker> = osm_collision::GeoCollisionChecker::new();
    let mut pdef= ProblemDefinition::new( start, goal, bounds, optimizer, params, collision_checker);
    
    println!("#### Planning ####");
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
#[allow(unused_must_use)]
async fn main() {
    run_example_postgis();
    server::launch(); // needs to be await'ed to execute. But not on commits because server would never stop.
    // let res = server::launch().await;
}