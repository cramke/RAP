use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader};

pub fn read_highway_costs() -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();

    let file_path: &str = "/home/samtal/rrisk/data/highway_costs.toml";
    let a = File::open(file_path);
    let reader = BufReader::new(a.unwrap());

    for line in reader.lines() {
        let text = line.unwrap();
        let mut a: Vec<&str> = text.split("=").collect();
        let key: &str = a.get(0).unwrap().trim();
        let b = a.get(1).unwrap().trim();
        let value: i32 = b.parse().unwrap();
        map.insert(key.to_string(), value);
    }

    return map;

}

pub fn get_hashmap_costs() -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("path".to_string(), 5);
    return map;
}