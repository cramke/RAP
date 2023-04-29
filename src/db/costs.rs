use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_highway_costs() -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();

    let file_path: &str = "/home/samtal/rap/data/highway_costs.toml";
    let a = File::open(file_path);
    let reader = BufReader::new(a.unwrap());

    for line in reader.lines() {
        let text = line.unwrap();
        let parsed_line: Vec<&str> = text.split("=").collect();
        let key: &str = parsed_line.first().unwrap().trim();
        let value: i32 = parsed_line.get(1).unwrap().trim().parse().unwrap();
        map.insert(key.to_string(), value);
    }
    return map;
}

pub fn get_cost_from_types(intersections: Vec<String>, cost_map: &HashMap<String, i32>) -> i32 {
    let costs: i32 = intersections.iter()
                                    .map( |single_intersect| cost_map.get(single_intersect)
                                    .unwrap())
                                    .sum();
    return costs;
}