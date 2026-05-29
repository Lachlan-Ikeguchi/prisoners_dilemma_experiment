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

### Run a Tournament

```bash
cargo run -- -r 100 --repetitions 3
```

Options:
- `-c, --contestants-dir <DIR>` - Directory containing contestant Lua scripts (default: "contestents")
- `-r, --rounds <N>` - Number of rounds to play in each match (default: 100)
- `--repetitions <N>` - Number of times to repeat each pair of contestants (default: 1)
- `-v, --verbose` - Show verbose output with round-by-round details
- `-l, --list` - List available contestants and exit
- `-h, --help` - Print help
- `-V, --version` - Print version

### Examples

List all contestants:
```bash
cargo run -- --list
```

Run a quick tournament with 10 rounds:
```bash
cargo run -- -r 10
```

Run a comprehensive tournament with 200 rounds and 5 repetitions:
```bash
cargo run -- -r 200 --repetitions 5
```

Run with verbose output to see round-by-round decisions:
```bash
cargo run -- -r 10 -v
```

## Scoring

The framework uses standard Prisoner's Dilemma payoffs:
- Both cooperate: 3 points each
- One defects, one cooperates: 5 for defector, 0 for cooperator
- Both defect: 1 point each

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

## Dependencies

- Rust (2024 edition)
- `clap` - Command-line argument parsing
- `mlua` - Lua integration for Rust

## License

MIT License
