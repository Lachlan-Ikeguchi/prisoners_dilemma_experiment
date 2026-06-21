use crate::models::{SimulationConfig, SimulationResults};
use std::fs;

/// Save simulation results to a TOML file
pub fn save_results(results: &SimulationResults, path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let toml_string = toml::to_string(results)?;
    fs::write(path, toml_string)?;
    Ok(())
}

/// Load simulation results from a TOML file
pub fn load_simulation_results(path: &str) -> std::result::Result<SimulationResults, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let results: SimulationResults = toml::from_str(&content)?;
    Ok(results)
}

/// Load simulation configuration from a TOML file
pub fn load_simulation_config(path: &str) -> std::result::Result<SimulationConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: SimulationConfig = toml::from_str(&content)?;
    Ok(config)
}

/// Create a default configuration TOML file
pub fn create_default_config() -> String {
    let config = SimulationConfig::default();
    toml::to_string(&config).unwrap()
}