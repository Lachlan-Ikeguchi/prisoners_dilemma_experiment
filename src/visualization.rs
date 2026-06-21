use crate::models::{Contestant, SimulationResults};
use std::collections::HashMap;

/// Print tournament results in a formatted table
pub fn print_results(
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
pub fn display_visualization(results: &SimulationResults, contestants: &[Contestant]) {
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