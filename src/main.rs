#![allow(unused)] // silence unused warnings while exploring (to comment out)

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use futures::executor::block_on;


mod db;
use db::osm_postgis;

fn main() -> Result<(), sqlx::Error> {
    db::osm_postgis::tester();
    block_on(db::osm_postgis::database());
    Ok(())
}