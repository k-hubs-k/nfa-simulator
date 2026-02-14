use serde::Deserialize;
use std::{
  collections::{HashMap, HashSet},
  fs::File,
  io::{self, BufReader, Write},
};

#[derive(Deserialize, Debug)]
struct NFAConfig {
  start_state: String,
  accept_states: HashSet<String>,
  transitions: HashMap<String, HashMap<Option<char>, Vec<String>>>,
  description: Option<String>,
}

fn simulate_generic_nfa(config: &NFAConfig, input: &str) -> bool {
  // 1. Initialization (starting state)
  let mut current_states = HashSet::new();
  current_states.insert(config.start_state.clone());

  // 2. Get where can we go before reading the first char
  current_states = epsilon_closure(config, &current_states);

  for c in input.chars() {
    let mut next_states = HashSet::new();

    // 3. Classic transition: move based on 'c'
    for state in &current_states {
      if let Some(char_transitions) = config.transitions.get(state)
                && let Some(destinations) = char_transitions.get(&Some(c)) {
                for dest in destinations {
                    next_states.insert(dest.clone());
                }
            }
    }

    // 4. Epsilon closure
    current_states = epsilon_closure(config, &next_states);

    // If there's no remaining possible state, stop (optimization)
    if current_states.is_empty() {
      return false;
    }
  }

  // Check if any of the current states are in the list of accepting states
  current_states
    .iter()
    .any(|s| config.accept_states.contains(s))
}

fn epsilon_closure(config: &NFAConfig, states: &HashSet<String>) -> HashSet<String> {
  let mut closure = states.clone();
  let mut stack: Vec<String> = states.iter().cloned().collect();

  let epsilon_char = Some('*');
  while let Some(state) = stack.pop() {
    if let Some(transitions) = config.transitions.get(&state)
            && let Some(epsilon_dests) = transitions.get(&epsilon_char)
        {
            for dest in epsilon_dests {
                if !closure.contains(dest) {
                    closure.insert(dest.clone());
                    stack.push(dest.clone());
                }
            }
        }
  }

  closure
}

fn readline(prompt: &str) -> io::Result<String> {
  print!("{}", prompt);
  io::stdout().flush()?;

  let mut input = String::new();
  io::stdin().read_line(&mut input)?;

  let trimmed = input.trim();

  Ok(trimmed.to_string())
}

fn read_json_config(file_path: &str) -> Result<NFAConfig, Box<dyn std::error::Error>> {
  let file = File::open(file_path)?;
  let reader = BufReader::new(file);
  let config: NFAConfig = serde_json::from_reader(reader)?;
  Ok(config)
}

fn main() {
  let file_path: String;

  loop {
    match readline("File config path: ") {
      Ok(v) => {
        file_path = v;
        break;
      }
      Err(e) => {
        println!("Error: {}", e)
      }
    }
  }

  // 1. Load config on startup
  let config = match read_json_config(&file_path) {
    Ok(c) => c,
    Err(e) => {
      eprintln!("Error loading configs: {}", e);
      return;
    }
  };

  let separator = "=".repeat(40);

  println!();
  println!("{separator}");

  println!("**AUTOMATE LOADED**");
  println!(
    "Description : {}",
    config
      .description
      .as_deref()
      .unwrap_or("No description provided")
  );

  println!("{separator}");
  println!();

  println!("--- Automate ready (type 'exit' to quit) ---");
  loop {
    let input = readline("> ").expect("Reading error");
    if input == "exit" {
      break;
    }

    // 2. Use the loaded config
    if simulate_generic_nfa(&config, &input) {
      println!("✅ Good");
    } else {
      println!("❌ Bad");
    }
  }
}
