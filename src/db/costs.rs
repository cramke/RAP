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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_get_cost_empty() {
        assert!(true); 
        let intersections: Vec<String> = Vec::new();
        let cost_map: HashMap<String, i32> = HashMap::new();
        
        let result = get_cost_from_types(intersections, &cost_map);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_get_cost_positive_number() {
        assert!(true); 
        let road: String = "road".to_string();
        let intersections: Vec<String> = vec![road.clone()];
        let cost: i32 = 25;

        let mut cost_map: HashMap<String, i32> = HashMap::new();
        cost_map.insert(road, cost);
        
        let result = get_cost_from_types(intersections, &cost_map);
        assert_eq!(result, cost);
    }

    #[test]
    fn test_get_cost_negative_number() {
        assert!(true); 
        let road: String = "road".to_string();
        let intersections: Vec<String> = vec![road.clone()];
        let cost: i32 = -3;

        let mut cost_map: HashMap<String, i32> = HashMap::new();
        cost_map.insert(road, cost);
        
        let result = get_cost_from_types(intersections, &cost_map);
        assert_eq!(result, cost);
    }
}
