use crate::models::Contestant;
use crate::game::run_match_pair;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use mlua::Result;

/// Run a full tournament with all contestants using multithreading
pub fn run_tournament(
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