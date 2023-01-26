#![allow(unused)] // silence unused warnings while exploring (to comment out)

use std::collections::HashMap;

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use futures::executor::block_on;


mod db;
use db::osm_postgis;
use db::costs;

use prm::node::Node2D;

fn main() {
    block_on(db::osm_postgis::database());

    let test2: HashMap<String, i32> = costs::read_highway_costs();
    let b: &i32 = test2.get("track").unwrap();
    println!("{}", b); 

    let node = Node2D::new(8.99910, 49.67114);
    let other = Node2D::new(8.99927, 49.67113);
    println!("{}", node.get_line_wkt(&other));
    block_on(db::osm_postgis::database2(&node.get_line_wkt(&other)));
    // LINESTRING (8.99910 49.67114, 8.99927 49.67113)
}