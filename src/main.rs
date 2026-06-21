mod models;
mod io;
mod contestants;
mod game;
mod tournament;
mod visualization;

use clap::Parser;
use models::{SimulationConfig, SimulationResults};
use io::{load_simulation_config, load_simulation_results, save_results, create_default_config};
use contestants::load_contestants;
use tournament::run_tournament;
use visualization::{print_results, display_visualization};
use std::collections::BTreeMap;
use std::fs;


/// Command line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// TOML configuration file for the simulation
    #[arg(short, long)]
    config: Option<String>,
    
    /// TOML file containing simulation results to visualize
    #[arg(short, long)]
    data: Option<String>,
    
    /// Visualize the results (shows graphs, tables, and scoreboard in console)
    #[arg(short, long)]
    visualise: bool,
    
    /// Output file for simulation results (default: simulation_results.toml)
    #[arg(short, long, default_value = "simulation_results.toml")]
    output: String,
    
    /// List available contestants and exit
    #[arg(short, long)]
    list: bool,
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Check if we should list contestants
    if args.list {
        let config = if let Some(config_path) = &args.config {
            load_simulation_config(config_path)?
        } else {
            SimulationConfig::default()
        };
        
        let contestants = load_contestants(&config.contestants_dir)?;
        
        if contestants.is_empty() {
            eprintln!(
                "No contestants found in directory: {}",
                config.contestants_dir
            );
            std::process::exit(1);
        }
        
        println!("Available Contestants:");
        for contestant in &contestants {
            println!("  {}: {}", contestant.name, contestant.description);
        }
        return Ok(());
    }

    // Check if we should visualize existing data
    if args.visualise && args.data.is_some() {
        if let Some(data_path) = &args.data {
            // Load results from TOML file
            let results = load_simulation_results(data_path)?;
            let config = results.config.clone();
            let contestants = load_contestants(&config.contestants_dir)?;
            
            display_visualization(&results, &contestants);
            return Ok(());
        }
    }

    // Load configuration
    let config = if let Some(config_path) = &args.config {
        load_simulation_config(config_path)?
    } else {
        // Create a default config file if it doesn't exist
        let default_config_path = "default_config.toml";
        if !std::path::Path::new(default_config_path).exists() {
            fs::write(default_config_path, create_default_config())?;
            println!("Created default configuration file: {}", default_config_path);
        }
        load_simulation_config(default_config_path)?
    };

    // Load contestants
    let contestants = load_contestants(&config.contestants_dir)?;

    if contestants.is_empty() {
        eprintln!(
            "No contestants found in directory: {}",
            config.contestants_dir
        );
        std::process::exit(1);
    }

    // Configure thread pool if specified
    if let Some(num_threads) = config.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .ok(); // Ignore error if already initialized
    }

    println!("Running Prisoner's Dilemma Tournament");
    println!("Contestants: {}", contestants.len());
    println!("Rounds per match: {}", config.rounds);
    println!("Repetitions: {}", config.repetitions);
    if let Some(num_threads) = config.threads {
        println!("Threads: {}", num_threads);
    } else {
        println!("Threads: auto");
    }
    println!();

    // Run tournament
    let results = run_tournament(&contestants, config.rounds, config.repetitions, config.verbose)?;

    // Print results to console
    print_results(&contestants, &results);

    // Convert results to SimulationResults structure
    let mut match_results_btree = BTreeMap::new();
    for (contestant1, opponents) in &results {
        let mut opponent_results = BTreeMap::new();
        for (contestant2, scores) in opponents {
            opponent_results.insert(contestant2.clone(), *scores);
        }
        match_results_btree.insert(contestant1.clone(), opponent_results);
    }

    let mut total_scores = BTreeMap::new();
    for contestant in &contestants {
        let mut total = 0;
        if let Some(scores) = results.get(&contestant.name) {
            for (_, (score, _)) in scores {
                total += score;
            }
        }
        total_scores.insert(contestant.name.clone(), total);
    }

    let mut rankings: Vec<(String, i32)> = total_scores.iter()
        .map(|(name, score)| (name.clone(), *score))
        .collect();
    rankings.sort_by(|a, b| b.1.cmp(&a.1));

    let contestant_names: Vec<String> = contestants.iter()
        .map(|c| c.name.clone())
        .collect();

    let simulation_results = SimulationResults {
        config: config.clone(),
        contestants: contestant_names,
        match_results: match_results_btree,
        total_scores,
        rankings,
        timestamp: chrono::Local::now().to_rfc3339(),
    };

    // Save results to TOML file
    save_results(&simulation_results, &args.output)?;
    println!("\nSimulation results saved to: {}", args.output);

    // If visualize flag is set, show the visualization
    if args.visualise {
        display_visualization(&simulation_results, &contestants);
    }

    Ok(())
}