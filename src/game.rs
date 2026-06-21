use crate::models::{Action, Contestant, MatchResult, RoundResult};
use mlua::{Lua, Result};
use std::fs;

/// Run a match between two contestants
pub fn run_match(
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
pub fn get_decision(
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

/// Run a single match pair with repetitions
pub fn run_match_pair(
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