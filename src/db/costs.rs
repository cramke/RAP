use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_highway_costs() -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();

    let file_path: &str = "/home/samtal/rrisk/data/highway_costs.toml";
    let a = File::open(file_path);
    let reader = BufReader::new(a.unwrap());

    for line in reader.lines() {
        let text = line.unwrap();
        let a: Vec<&str> = text.split("=").collect();
        let key: &str = a.get(0).unwrap().trim();
        let b = a.get(1).unwrap().trim();
        let value: i32 = b.parse().unwrap();
        map.insert(key.to_string(), value);
    }

    return map;

}

pub fn get_cost_from_types(intersections: Vec<String>) -> i32 {
    let test2: HashMap<String, i32> = read_highway_costs();
    let mut cost_cumulative = 0;
    for intersection in intersections {
        let cost = test2.get(&intersection).unwrap();
        cost_cumulative = cost_cumulative + cost;
    }
    println!("Cumulative cost of line crossings: {}", cost_cumulative);
    return cost_cumulative;
}