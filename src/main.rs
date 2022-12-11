#![allow(unused)] // silence unused warnings while exploring (to comment out)

use std::collections::HashMap;

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use futures::executor::block_on;


mod db;
use db::osm_postgis;
use db::costs;

fn main() -> Result<(), sqlx::Error> {
    block_on(db::osm_postgis::database());

    let test2: HashMap<String, i32> = costs::read_highway_costs();
    let b: &i32 = test2.get("track").unwrap();
    println!("{}", b);

    Ok(())

}