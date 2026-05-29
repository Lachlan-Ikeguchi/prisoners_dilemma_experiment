use clap::Parser;
use mlua::{Lua, Result};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};
use std::fs;
use std::sync::Mutex;

/// Configuration for the Prisoner's Dilemma simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimulationConfig {
    /// Directory containing contestant Lua scripts
    #[serde(default = "default_contestants_dir")]
    contestants_dir: String,
    
    /// Number of rounds to play in each match
    #[serde(default = "default_rounds")]
    rounds: usize,
    
    /// Number of times to repeat each pair of contestants
    #[serde(default = "default_repetitions")]
    repetitions: usize,
    
    /// Number of threads to use (optional)
    #[serde(default)]
    threads: Option<usize>,
    
    /// Show verbose output
    #[serde(default)]
    verbose: bool,
}

fn default_contestants_dir() -> String {
    "contestents".to_string()
}

fn default_rounds() -> usize {
    100
}

fn default_repetitions() -> usize {
    1
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            contestants_dir: default_contestants_dir(),
            rounds: default_rounds(),
            repetitions: default_repetitions(),
            threads: None,
            verbose: false,
        }
    }
}

/// Data structure for storing simulation results
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SimulationResults {
    /// Configuration used for the simulation
    config: SimulationConfig,
    
    /// List of contestant names
    contestants: Vec<String>,
    
    /// Individual match results: contestant1 -> contestant2 -> (score1, score2)
    match_results: BTreeMap<String, BTreeMap<String, (i32, i32)>>,
    
    /// Total scores for each contestant
    total_scores: BTreeMap<String, i32>,
    
    /// Ranking of contestants by total score
    rankings: Vec<(String, i32)>,
    
    /// Timestamp of when the simulation was run
    #[serde(default = "default_timestamp")]
    timestamp: String,
}

fn default_timestamp() -> String {
    chrono::Local::now().to_rfc3339()
}

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

/// Represents a contestant in the Prisoner's Dilemma game
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Contestant {
    name: String,
    description: String,
    script_path: String,
}

impl Contestant {
    fn new(name: String, description: String, script_path: String) -> Self {
        Self {
            name,
            description,
            script_path,
        }
    }
}

/// Action a contestant can take
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Action {
    Cooperate,
    Defect,
}

impl Action {
    fn to_string(&self) -> &'static str {
        match self {
            Action::Cooperate => "Cooperate",
            Action::Defect => "Defect",
        }
    }
}

/// Result of a single round
struct RoundResult {
    player1_score: i32,
    player2_score: i32,
}

impl RoundResult {
    fn new(player1_action: Action, player2_action: Action) -> Self {
        // Standard Prisoner's Dilemma payoffs:
        // - Both cooperate: 3 points each
        // - One defects, one cooperates: 5 for defector, 0 for cooperator
        // - Both defect: 1 point each
        let (player1_score, player2_score) = match (player1_action, player2_action) {
            (Action::Cooperate, Action::Cooperate) => (3, 3),
            (Action::Cooperate, Action::Defect) => (0, 5),
            (Action::Defect, Action::Cooperate) => (5, 0),
            (Action::Defect, Action::Defect) => (1, 1),
        };

        Self {
            player1_score,
            player2_score,
        }
    }
}

/// Load all contestants from a directory
fn load_contestants(dir: &str) -> Result<Vec<Contestant>> {
    let paths = fs::read_dir(dir)?;
    let mut contestants = Vec::new();

    for path in paths {
        let path = path?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("lua") {
            let content = fs::read_to_string(&path)?;
            let name = extract_name(&content);
            let description = extract_description(&content);
            let script_path = path.to_string_lossy().into_owned();

            contestants.push(Contestant::new(name, description, script_path));
        }
    }

    Ok(contestants)
}

/// Extract the name from Lua script comments
fn extract_name(content: &str) -> String {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("-- name:") {
            return trimmed.trim_start_matches("-- name:").trim().to_string();
        }
    }
    "Unknown".to_string()
}

/// Extract the description from Lua script comments
fn extract_description(content: &str) -> String {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("-- will")
            || (trimmed.starts_with("-- ") && !trimmed.starts_with("-- name:"))
        {
            return trimmed.trim_start_matches("--").trim().to_string();
        }
    }
    "No description".to_string()
}

/// Run a match between two contestants
fn run_match(
    contestant1: &Contestant,
    contestant2: &Contestant,
    rounds: usize,
    verbose: bool,
) -> Result<(i32, i32)> {
    // Load both contestant scripts
    let script1 = fs::read_to_string(&contestant1.script_path)?;
    let script2 = fs::read_to_string(&contestant2.script_path)?;

    // Create separate Lua contexts for each contestant
    let lua1 = Lua::new();
    let lua2 = Lua::new();

    // Load scripts into their respective contexts
    lua1.load(&script1).exec()?;
    lua2.load(&script2).exec()?;

    let mut total1 = 0;
    let mut total2 = 0;

    // History of opponent's actions for each player
    let mut history1: Vec<Action> = Vec::new();
    let mut history2: Vec<Action> = Vec::new();

    for round in 0..rounds {
        // Get decisions from both contestants
        let decision1 = get_decision(&lua1, &contestant1.name, round, &history2)?;
        let decision2 = get_decision(&lua2, &contestant2.name, round, &history1)?;

        // Record actions in history
        history1.push(decision2.clone());
        history2.push(decision1.clone());

        // Calculate scores for this round
        let result = RoundResult::new(decision1.clone(), decision2.clone());
        total1 += result.player1_score;
        total2 += result.player2_score;

        if verbose {
            println!(
                "Round {}: {} vs {} -> {}: {}, {}: {}",
                round + 1,
                contestant1.name,
                contestant2.name,
                contestant1.name,
                decision1.to_string(),
                contestant2.name,
                decision2.to_string()
            );
            println!(
                "  Scores: {}: {}, {}: {}",
                contestant1.name, total1, contestant2.name, total2
            );
        }
    }

    Ok((total1, total2))
}

/// Get a decision from a contestant's Lua script
fn get_decision(
    lua: &Lua,
    contestant_name: &str,
    round: usize,
    opponent_history: &[Action],
) -> Result<Action> {
    // Convert history to a format Lua can understand
    let history_table = lua.create_table()?;
    for (i, action) in opponent_history.iter().enumerate() {
        let action_str = match action {
            Action::Cooperate => "cooperate",
            Action::Defect => "defect",
        };
        history_table.set(i + 1, action_str)?;
    }

    // Call the decide function in the Lua script
    let decide_fn: mlua::Function = lua.globals().get("decide")?;
    let result: String = decide_fn.call((round + 1, history_table))?;

    match result.to_lowercase().as_str() {
        "cooperate" | "c" => Ok(Action::Cooperate),
        "defect" | "d" => Ok(Action::Defect),
        _ => {
            eprintln!("Invalid decision from {}: {}", contestant_name, result);
            Ok(Action::Defect) // Default to defect on invalid input
        }
    }
}

/// Individual match result for parallel processing
#[derive(Debug, Clone)]
struct MatchResult {
    contestant1_name: String,
    contestant2_name: String,
    score1: i32,
    score2: i32,
}

/// Run a single match pair with repetitions
fn run_match_pair(
    contestant1: &Contestant,
    contestant2: &Contestant,
    rounds: usize,
    repetitions: usize,
    verbose: bool,
) -> Result<MatchResult> {
    let mut total1 = 0;
    let mut total2 = 0;

    for _ in 0..repetitions {
        let (score1, score2) = run_match(contestant1, contestant2, rounds, verbose)?;
        total1 += score1;
        total2 += score2;
    }

    Ok(MatchResult {
        contestant1_name: contestant1.name.clone(),
        contestant2_name: contestant2.name.clone(),
        score1: total1,
        score2: total2,
    })
}

/// Run a full tournament with all contestants using multithreading
fn run_tournament(
    contestants: &[Contestant],
    rounds: usize,
    repetitions: usize,
    verbose: bool,
) -> Result<HashMap<String, HashMap<String, (i32, i32)>>> {
    // Generate all unique match pairs (i, j) where i <= j
    let match_pairs: Vec<(usize, usize)> = (0..contestants.len())
        .flat_map(|i| (i..contestants.len()).map(move |j| (i, j)))
        .collect();

    // Use Mutex to safely collect results from multiple threads
    let results_mutex = Mutex::new(HashMap::<String, HashMap<String, (i32, i32)>>::new());

    // Process match pairs in parallel
    match_pairs.par_iter().for_each(|&(i, j)| {
        let contestant1 = &contestants[i];
        let contestant2 = &contestants[j];

        let match_result = run_match_pair(contestant1, contestant2, rounds, repetitions, verbose).unwrap();

        // Lock the mutex and insert results
        let mut results = results_mutex.lock().unwrap();

        // Store results for contestant1 -> contestant2
        results
            .entry(match_result.contestant1_name.clone())
            .or_default()
            .insert(
                match_result.contestant2_name.clone(),
                (match_result.score1, match_result.score2),
            );

        // If different contestants, store the reverse direction
        if i != j {
            results
                .entry(match_result.contestant2_name.clone())
                .or_default()
                .insert(
                    match_result.contestant1_name.clone(),
                    (match_result.score2, match_result.score1),
                );
        }
    });

    // Extract the results from the Mutex
    let results = results_mutex.into_inner().unwrap();
    Ok(results)
}

/// Save simulation results to a TOML file
fn save_results(results: &SimulationResults, path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let toml_string = toml::to_string(results)?;
    fs::write(path, toml_string)?;
    Ok(())
}

/// Load simulation results from a TOML file
fn load_simulation_results(path: &str) -> std::result::Result<SimulationResults, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let results: SimulationResults = toml::from_str(&content)?;
    Ok(results)
}

/// Load simulation configuration from a TOML file
fn load_simulation_config(path: &str) -> std::result::Result<SimulationConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: SimulationConfig = toml::from_str(&content)?;
    Ok(config)
}

/// Create a default configuration TOML file
fn create_default_config() -> String {
    let config = SimulationConfig::default();
    toml::to_string(&config).unwrap()
}

/// Print tournament results in a formatted table
fn print_results(
    contestants: &[Contestant],
    results: &HashMap<String, HashMap<String, (i32, i32)>>,
) {
    // Use abbreviations for display if names are too long
    let display_names: Vec<String> = contestants
        .iter()
        .map(|c| {
            if c.name.len() > 12 {
                c.name.chars().take(10).collect::<String>() + ".."
            } else {
                c.name.clone()
            }
        })
        .collect();

    let col_width = 14; // Fixed column width for better readability

    // Print header
    print!("{:<col_width$}", "Contestant");
    for name in &display_names {
        print!("{:>col_width$}", name);
    }
    println!();

    // Print separator
    print!("{}", "-".repeat(col_width));
    for _ in &display_names {
        print!("{}", "-".repeat(col_width));
    }
    println!();

    // Print rows
    for (i, contestant) in contestants.iter().enumerate() {
        let display_name = &display_names[i];
        print!("{:<col_width$}", display_name);
        for opponent in contestants {
            if let Some(scores) = results
                .get(&contestant.name)
                .and_then(|m| m.get(&opponent.name))
            {
                print!("{:>col_width$}", scores.0);
            } else {
                print!("{:>col_width$}", "N/A");
            }
        }
        println!();
    }

    println!();

    // Print total scores
    println!("Total Scores (sorted by rank):");
    let mut total_scores: Vec<(String, i32)> = Vec::new();

    for contestant in contestants {
        let mut total = 0;
        if let Some(scores) = results.get(&contestant.name) {
            for (_, (score, _)) in scores {
                total += score;
            }
        }
        total_scores.push((contestant.name.clone(), total));
    }

    // Sort by score descending
    total_scores.sort_by(|a, b| b.1.cmp(&a.1));

    for (i, (name, score)) in total_scores.iter().enumerate() {
        println!("  {}. {}: {}", i + 1, name, score);
    }
}

/// Display visualization of simulation results in console
fn display_visualization(results: &SimulationResults, contestants: &[Contestant]) {
    println!("📊 Prisoner's Dilemma Simulation Visualization");
    println!("==============================================");
    println!();
    
    // Display configuration
    println!("📋 Configuration:");
    println!("  Contestants Directory: {}", results.config.contestants_dir);
    println!("  Rounds per Match: {}", results.config.rounds);
    println!("  Repetitions: {}", results.config.repetitions);
    println!("  Timestamp: {}", results.timestamp);
    println!();
    
    // Display rankings
    println!("🏆 Rankings:");
    for (i, (name, score)) in results.rankings.iter().enumerate() {
        let medal = match i {
            0 => "🥇 ",
            1 => "🥈 ",
            2 => "🥉 ",
            _ => "   ",
        };
        
        let contestant_desc = contestants.iter()
            .find(|c| c.name == *name)
            .map(|c| c.description.as_str())
            .unwrap_or("Unknown");
        
        println!("  {}{}. {}: {} ({})", medal, i + 1, name, score, contestant_desc);
    }
    println!();
    
    // Display total scores table
    println!("📋 Total Scores:");
    println!("  {:<25} {:>10}", "Contestant", "Score");
    println!("  {}", "-".repeat(37));
    for (name, score) in &results.total_scores {
        println!("  {:<25} {:>10}", name, score);
    }
    println!();
    
    // Summary statistics
    println!("📈 Summary Statistics:");
    let total_score: i32 = results.rankings.iter().map(|(_, score)| score).sum();
    let avg_score = total_score as f64 / results.rankings.len() as f64;
    println!("  Total Contestants: {}", results.contestants.len());
    println!("  Average Score: {:.1}", avg_score);
    
    if let (Some((top_name, top_score)), Some((bottom_name, bottom_score))) = 
        (results.rankings.first(), results.rankings.last()) {
        println!("  Highest Score: {} ({})", top_name, top_score);
        println!("  Lowest Score: {} ({})", bottom_name, bottom_score);
    }
    println!();
    
    // ASCII bar chart
    println!("📊 Score Distribution (ASCII Chart):");
    let max_score = results.rankings.first().map(|(_, score)| *score).unwrap_or(1);
    let chart_width = 50;
    
    for (name, score) in &results.rankings.iter().take(10).cloned().collect::<Vec<_>>() {
        let bar_length = ((*score as f64 / max_score as f64) * chart_width as f64).round() as usize;
        let bar = "█".repeat(bar_length);
        println!("  {:<20} {} {}", name, bar, score);
    }
    println!();
    
    // Sample match results
    println!("🎯 Sample Match Results (Top 5 vs Top 5):");
    let top_contestants: Vec<&String> = results.rankings.iter().take(5).map(|(name, _)| name).collect();
    
    if top_contestants.len() >= 2 {
        for i in 0..top_contestants.len() {
            for j in 0..top_contestants.len() {
                if i != j {
                    let name1 = top_contestants[i];
                    let name2 = top_contestants[j];
                    if let Some(scores) = results.match_results.get(name1).and_then(|m| m.get(name2)) {
                        println!("  {} vs {}: {} - {}", name1, name2, scores.0, scores.1);
                    }
                }
            }
        }
    }
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