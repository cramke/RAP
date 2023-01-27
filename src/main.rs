#![allow(unused)] // silence unused warnings while exploring (to comment out)

use std::collections::HashMap;
use std::time::Instant;

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
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

fn get_edge_weight(begin: &Node2D, end: &Node2D) -> f64 {
    let intersections: Vec<String> = block_on(db::osm_postgis::fetch_intersecting_highways(&begin.get_line_wkt(&end)));
    let cost = get_cost_from_types(intersections);
    return cost as f64;
}

fn run_example() {
    let start: Node2D = Node2D { x: 8.926f64, y: 49.67f64, idx: 0 };
    let goal: Node2D = Node2D { x: 9.07f64, y: 49.71f64, idx: 0 };
    let bounds: Boundaries = Boundaries { x_lower: 8.925f64, x_upper: 9.08f64, y_lower: 49.66f64, y_upper: 49.72f64 };
    let mut pdef= ProblemDefinition::new( start, goal, bounds, is_collision, is_edge_in_collision, get_edge_weight);                                       
    pdef.solve();
    pdef.print_statistics();
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