use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Bounds {
    pub x_lower: f64,
    pub x_upper: f64,
    pub y_lower: f64,
    pub y_upper: f64
}

#[derive(Deserialize, Debug)]
pub struct ProblemConfig{
    pub start_lon: f64,
    pub start_lat: f64,
    pub goal_lon: f64,
    pub goal_lat: f64,
    pub nodes_max: usize,
    pub k_nearest_neighbors: usize,
    pub boundaries: Bounds
}

impl ProblemConfig {
    pub fn from_file(path: &str) -> ProblemConfig {
        let config_raw = fs::read_to_string(path).unwrap();
        let settings: ProblemConfig = serde_json::from_str(&config_raw).expect("JSON did not work");
        settings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_file() {
        let path = "tests/data/problem_description_standard.json";
        let settings = ProblemConfig::from_file(path);
        assert_eq!(settings.k_nearest_neighbors, 5);
        assert_eq!(settings.nodes_max, 25);

    }
}
