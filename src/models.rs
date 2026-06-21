use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use chrono;

/// Configuration for the Prisoner's Dilemma simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// Directory containing contestant Lua scripts
    #[serde(default = "default_contestants_dir")]
    pub contestants_dir: String,
    
    /// Number of rounds to play in each match
    #[serde(default = "default_rounds")]
    pub rounds: usize,
    
    /// Number of times to repeat each pair of contestants
    #[serde(default = "default_repetitions")]
    pub repetitions: usize,
    
    /// Number of threads to use (optional)
    #[serde(default)]
    pub threads: Option<usize>,
    
    /// Show verbose output
    #[serde(default)]
    pub verbose: bool,
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
pub struct SimulationResults {
    /// Configuration used for the simulation
    pub config: SimulationConfig,
    
    /// List of contestant names
    pub contestants: Vec<String>,
    
    /// Individual match results: contestant1 -> contestant2 -> (score1, score2)
    pub match_results: BTreeMap<String, BTreeMap<String, (i32, i32)>>,
    
    /// Total scores for each contestant
    pub total_scores: BTreeMap<String, i32>,
    
    /// Ranking of contestants by total score
    pub rankings: Vec<(String, i32)>,
    
    /// Timestamp of when the simulation was run
    #[serde(default = "default_timestamp")]
    pub timestamp: String,
}

fn default_timestamp() -> String {
    chrono::Local::now().to_rfc3339()
}

/// Represents a contestant in the Prisoner's Dilemma game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contestant {
    pub name: String,
    pub description: String,
    pub script_path: String,
}

impl Contestant {
    pub fn new(name: String, description: String, script_path: String) -> Self {
        Self {
            name,
            description,
            script_path,
        }
    }
}

/// Action a contestant can take
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Cooperate,
    Defect,
}

impl Action {
    pub fn to_string(&self) -> &'static str {
        match self {
            Action::Cooperate => "Cooperate",
            Action::Defect => "Defect",
        }
    }
}

/// Result of a single round
pub struct RoundResult {
    pub player1_score: i32,
    pub player2_score: i32,
}

impl RoundResult {
    pub fn new(player1_action: Action, player2_action: Action) -> Self {
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

/// Individual match result for parallel processing
#[derive(Debug, Clone)]
pub struct MatchResult {
    pub contestant1_name: String,
    pub contestant2_name: String,
    pub score1: i32,
    pub score2: i32,
}