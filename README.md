# NFA Simulator

A flexible, JSON-configurable **Nondeterministic Finite Automaton (NFA) simulator** written in Rust. Define any NFA through simple JSON configuration files and interactively test strings for acceptance.

## Features

- ✨ **Generic NFA Simulation**: Supports any NFA defined via JSON configuration
- 🔄 **Epsilon Transitions**: Full support for ε-transitions (represented as `*`)
- 📝 **JSON-Based Configuration**: Easy-to-write automaton definitions
- 🎯 **Interactive REPL**: Test multiple strings without restarting
- ⚡ **Fast & Efficient**: Built with Rust for optimal performance
- 🧪 **Educational Tool**: Perfect for learning automata theory and formal languages

## Installation

### Prerequisites

- Rust 1.75+ (uses 2024 edition features)
- Cargo (comes with Rust)

### Setup

1. Clone the repository:
```bash
git clone https://github.com/k-hubs-k/nfa-simulator
cd nfa-simulator
```

2. Build the project:
```bash
cargo build --release
```

3. Run the simulator:
```bash
cargo run --release
```

## Usage

### Running the Simulator

1. Start the program:
```bash
cargo run
```

2. Enter the path to your JSON configuration file when prompted:
```
File config path: src/automate.json
```

3. Test strings interactively:
```
========================================
**AUTOMATE LOADED**
Description : Binary strings ending with '0'
========================================

--- Automate ready (type 'exit' to quit) ---
> 10
✅ Good
> 11
❌ Bad
> 1010
✅ Good
> exit
```

### Example: Binary Strings Ending with '0'

Create a file `binary_ending_0.json`:

```json
{
  "description": "Accepts binary strings ending with '0'",
  "start_state": "A",
  "accept_states": ["C"],
  "transitions": {
    "A": {
      "0": ["B"],
      "1": ["B"]
    },
    "B": {
      "0": ["B", "C"],
      "1": ["B"]
    }
  }
}
```

This NFA accepts strings like: `0`, `10`, `100`, `1010`
And rejects: `1`, `11`, `101`, `1011`

### Example: Epsilon Transitions

Create a file `epsilon_example.json`:

```json
{
  "description": "NFA with epsilon transitions",
  "start_state": "A",
  "accept_states": ["C"],
  "transitions": {
    "A": {
      "*": ["B", "C"]
    },
    "B": {
      "0": ["A"]
    }
  }
}
```

This NFA uses `*` to represent ε-transitions, allowing state changes without consuming input.

## JSON Configuration Format

### Structure

```json
{
  "description": "Optional description of the automaton",
  "start_state": "StateName",
  "accept_states": ["State1", "State2"],
  "transitions": {
    "SourceState": {
      "symbol": ["DestState1", "DestState2"]
    }
  }
}
```

### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String | No | Human-readable description of the automaton |
| `start_state` | String | Yes | The initial state of the NFA |
| `accept_states` | Array[String] | Yes | List of accepting/final states |
| `transitions` | Object | Yes | State transition mapping |

### Transition Format

- **Keys**: Source state names (strings)
- **Values**: Objects mapping symbols to destination state arrays
  - Use `"*"` for epsilon (ε) transitions
  - Use single characters for regular transitions (e.g., `"0"`, `"1"`, `"a"`)
  - Each symbol maps to an array of destination states (supports nondeterminism)

### Example Patterns

**Multiple transitions from one state:**
```json
"A": {
  "0": ["B"],
  "1": ["C", "D"]
}
```

**Epsilon transition:**
```json
"A": {
  "*": ["B"]
}
```

**Self-loop:**
```json
"A": {
  "0": ["A"]
}
```

## Dependencies

- **Rust**: Edition 2024 (requires Rust 1.75+)
- **serde**: 1.0 (with derive feature)
- **serde_json**: 1.0

## Project Structure

```
.
├── Cargo.toml          # Project configuration
├── README.md           # This file
└── src/
    ├── main.rs         # Main simulator code
    ├── automate.json   # Example NFA configuration
    ├── test.json       # Example with epsilon transitions
    └── teste_2.json    # Additional example
```

## License

This project is open source. Please add your preferred license.

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.

## Educational Resources

This simulator is perfect for:
- Learning automata theory
- Understanding NFA vs DFA concepts
- Experimenting with epsilon transitions
- Visualizing state transitions
- Testing formal language patterns

---

**Note**: This is an educational tool. For production pattern matching, consider using regex engines or DFA-based solutions.
