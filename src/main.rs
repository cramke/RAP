#![allow(unused)] // silence unused warnings while exploring (to comment out)

use std::collections::HashMap;

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use futures::executor::block_on;


mod db;
use db::osm_postgis;
use db::costs;

fn main() -> Result<(), sqlx::Error> {
    db::osm_postgis::tester();
    block_on(db::osm_postgis::database());

    let test: HashMap<String, i32> = costs::get_hashmap_costs();
    let a: &i32 = test.get("path").unwrap();
    println!("{}", a);

    Ok(())

}