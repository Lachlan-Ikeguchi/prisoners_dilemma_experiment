# Prisoner's Dilemma Experiment Framework

A Rust-based framework for running Prisoner's Dilemma experiments with contestants defined in Lua scripts.

## Overview

This framework allows you to:
- Define contestant strategies in Lua scripts
- Run tournaments between multiple contestants
- Configure the number of rounds and repetitions
- View detailed results and rankings

## Structure

- `src/main.rs` - Main Rust application with tournament logic
- `contestents/` - Directory containing Lua contestant scripts
- `Cargo.toml` - Rust project configuration

## Contestant Scripts

Each contestant is defined in a Lua file in the `contestents/` directory. Each script must:

1. Have a name and description in comments at the top:
   ```lua
   -- name: CONTESTANT_NAME
   -- description of behavior
   ```

2. Implement a `decide` function that takes two parameters:
   - `round` - The current round number (starting from 1)
   - `history` - A Lua table containing the opponent's previous actions (indexed from 1)

3. Return either `"cooperate"` or `"defect"` (or `"c"` or `"d"` for short)

### Example Contestant

```lua
-- name: TIT FOR TAT
-- will immediately retaliate for defection, otherwise cooperate

function decide(round, history)
    -- On first round, cooperate
    if round == 1 then
        return "cooperate"
    end
    
    -- Check opponent's last move
    if history[#history] == "defect" then
        return "defect"
    else
        return "cooperate"
    end
end
```

## Included Contestants

The framework comes with several pre-defined contestants:

- **ANGEL** - Always cooperates
- **DEVIL** - Always defects
- **RANDOM** - Randomly cooperates or defects (50/50)
- **TIT FOR TAT** - Cooperates first, then mirrors opponent's last move
- **GRUDGER** - Cooperates until opponent defects, then always defects
- **ALWAYS RETALIATE** - Cooperates first round, then always defects
- **SUSPICIOUS TIT FOR TAT** - Defects first, then mirrors opponent's last move
- **TWO TITS FOR TAT** - Defects twice after opponent defects once, then forgives

## Usage

### Build

```bash
cargo build --release
```

### Configuration Files

The program now uses TOML configuration files instead of command line arguments. Create a configuration file like `config.toml`:

```toml
# Prisoner's Dilemma Simulation Configuration
contestants_dir = "contestents"
rounds = 100
repetitions = 1
verbose = false

# Optional: specify number of threads
# threads = 4
```

An example configuration file is provided as `example_config.toml`.

### Run a Tournament

```bash
# Run with a configuration file
cargo run -- --config config.toml

# Run with default configuration (creates default_config.toml if it doesn't exist)
cargo run --

# Run with custom output file
cargo run -- --config config.toml --output my_results.toml
```

### Command Line Options

- `-c, --config <CONFIG>` - TOML configuration file for the simulation
- `-d, --data <DATA>` - TOML file containing simulation results to visualize
- `-v, --visualise` - Visualize the results (shows graphs, tables, and scoreboard in console)
- `-o, --output <OUTPUT>` - Output file for simulation results (default: simulation_results.toml)
- `-l, --list` - List available contestants and exit
- `-h, --help` - Print help
- `-V, --version` - Print version

### Examples

List all contestants:
```bash
cargo run -- --list
```

Run a tournament with custom configuration:
```bash
cargo run -- --config my_config.toml --output results.toml
```

Run a tournament and visualize results immediately:
```bash
cargo run -- --config config.toml --visualise
```

Visualize existing results:
```bash
cargo run -- --data simulation_results.toml --visualise
```

Run a quick tournament with 10 rounds:
```bash
# Create a config file with rounds = 10
cargo run -- --config quick_config.toml
```

## Scoring

The framework uses standard Prisoner's Dilemma payoffs:
- Both cooperate: 3 points each
- One defects, one cooperates: 5 for defector, 0 for cooperator
- Both defect: 1 point each

## Output Format

The simulation results are saved in TOML format and include:

- **Configuration**: The settings used for the simulation
- **Contestants**: List of all contestant names
- **Match Results**: Detailed results of each match between contestants
- **Total Scores**: Aggregate scores for each contestant
- **Rankings**: Contestants sorted by total score
- **Timestamp**: When the simulation was run

Example output file structure:
```toml
contestants = ["TIT FOR TAT", "ALWAYS COOPERATE", "ALWAYS DEFECT"]
rankings = [["ALWAYS DEFECT", 150], ["TIT FOR TAT", 90], ["ALWAYS COOPERATE", 60]]
timestamp = "2024-01-01T12:00:00.000000+00:00"

[config]
contestants_dir = "contestents"
rounds = 100
repetitions = 1
verbose = false

[match_results.TIT_FOR_TAT]
"ALWAYS COOPERATE" = [90, 60]
"ALWAYS DEFECT" = [0, 150]

[total_scores]
"ALWAYS COOPERATE" = 60
"ALWAYS DEFECT" = 150
"TIT FOR TAT" = 90
```

## Visualization

The `--visualise` flag provides a comprehensive console-based visualization including:

- **Scoreboard**: Rankings with medals (🥇🥈🥉) and contestant descriptions
- **Graphs**: ASCII bar charts showing score distribution
- **Results Table**: Detailed match results and total scores
- **Statistics**: Average scores, highest/lowest scores, and other metrics
- **Sample Matches**: Head-to-head results between top contestants

### Example Visualization Output

```
📊 Prisoner's Dilemma Simulation Visualization
==============================================

📋 Configuration:
  Contestants Directory: contestents
  Rounds per Match: 100
  Repetitions: 1
  Timestamp: 2024-01-01T12:00:00.000000+00:00

🏆 Rankings:
  🥇 1. ALWAYS DEFECT: 150 (will always defect, no matter what)
  🥈 2. TIT FOR TAT: 90 (cooperates first, then mirrors opponent's last move)
  🥉 3. ALWAYS COOPERATE: 60 (always cooperates)

📋 Total Scores:
  Contestant              Score
  --------------------------
  ALWAYS DEFECT               150
  TIT FOR TAT                  90
  ALWAYS COOPERATE             60

📈 Summary Statistics:
  Total Contestants: 3
  Average Score: 100.0
  Highest Score: ALWAYS DEFECT (150)
  Lowest Score: ALWAYS COOPERATE (60)

📊 Score Distribution (ASCII Chart):
  ALWAYS DEFECT       ████████████████████████████████████████ 150
  TIT FOR TAT        ██████████████████                     90
  ALWAYS COOPERATE  ████████                              60

🎯 Sample Match Results (Top 3 vs Top 3):
  ALWAYS DEFECT vs TIT FOR TAT: 150 - 0
  ALWAYS DEFECT vs ALWAYS COOPERATE: 150 - 0
  TIT FOR TAT vs ALWAYS DEFECT: 0 - 150
  TIT FOR TAT vs ALWAYS COOPERATE: 90 - 60
```

## Adding New Contestants

To add a new contestant:

1. Create a new `.lua` file in the `contestents/` directory
2. Add the name and description in comments at the top
3. Implement the `decide` function

Example:
```lua
-- name: MY_STRATEGY
-- description of my strategy

function decide(round, history)
    -- Your strategy logic here
    return "cooperate"
end
```

## GUI Visualization (Optional)

For a graphical user interface, you can add the following dependencies to `Cargo.toml`:

```toml
[dependencies]
egui = "0.24"
egui_extras = "0.24"
eframe = { version = "0.24", features = ["default_fonts"] }
```

Then modify the visualization code to use egui for rendering graphs, tables, and scoreboards in a native window. The current console visualization provides all the same information in text format.

## Dependencies

- Rust (2024 edition)
- `clap` - Command-line argument parsing
- `mlua` - Lua integration for Rust

## License

MIT License
