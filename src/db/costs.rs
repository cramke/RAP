use std::collections::HashMap;

pub fn get_hashmap_costs() -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("path".to_string(), 5);
    return map;
}