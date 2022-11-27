use postgres::{Client, NoTls};
use postgis::{ewkb, LineString};
use std::env;

fn main() {
    println!("Hello, world!");
    foo();
}


fn foo() {   
    env::set_var("RUST_BACKTRACE", "full"); 
    let mut client = match Client::connect("postgresql://postgres:password@localhost:5432/osm", NoTls) {
        Ok(client) => client,
        Err(_) => panic!("No DB connection."),
    };

    let baz = String::from("0102000020E61000000300000012A5187B8AFF21408390E2D2E7D548407EC9213DA0FF21401822A7AFE7D54840AB6A381BE1FF2140ADE17DB0E7D54840");
    for _row in client.query("SELECT highway FROM highway_michelstadt WHERE ST_Crosses(highway_michelstadt.geometry, $1)", &[&baz]).unwrap() {
        println!("foo:", );
    }

    
}
