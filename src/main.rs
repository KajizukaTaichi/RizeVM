use serde::Deserialize;
use serde_json::from_str;
use std::collections::HashMap;
use std::env;
use std::fs;
use crossterm::{
    event::{self, KeyCode, KeyEvent},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use std::time::Duration;

fn wait_keypress() {
    let _ = enable_raw_mode();
    loop {
        if event::poll(Duration::from_secs(1)).unwrap_or(false) {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Char(_) | KeyCode::Enter | KeyCode::Esc | KeyCode::Tab => break,
                    _ => {}
                }
            }
        }
    }
    let _ = disable_raw_mode();
}

#[derive(Deserialize)]
struct Transition {
    write: char,
    direction: String,
    next_state: String,
}

#[derive(Deserialize)]
struct TuringMachineConfig {
    tape: String,
    start_state: String,
    transitions: HashMap<String, Transition>,
}

struct Tape {
    tape: Vec<char>,
    head: usize,
}

impl Tape {
    fn new(input: &str) -> Tape {
        Tape {
            tape: input.chars().collect(),
            head: 0,
        }
    }

    fn read(&self) -> char {
        self.tape.get(self.head).cloned().unwrap_or('_')
    }

    fn write(&mut self, c: char) {
        if self.head < self.tape.len() {
            self.tape[self.head] = c;
        } else {
            self.tape.push(c);
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        match direction {
            Direction::Left => {
                if self.head > 0 {
                    self.head -= 1;
                }
            }
            Direction::Right => {
                self.head += 1;
                if self.head >= self.tape.len() {
                    self.tape.push('_');
                }
            }
        }
    }
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_str(s: &str) -> Direction {
        match s {
            "Left" => Direction::Left,
            "Right" => Direction::Right,
            _ => panic!("Invalid direction: {}", s),
        }
    }
}

struct TuringMachine {
    tape: Tape,
    state: String,
    transitions: HashMap<(String, char), Transition>,
}

impl TuringMachine {
    fn new(tape: Tape, start_state: &str, transitions: HashMap<(String, char), Transition>) -> TuringMachine {
        TuringMachine {
            tape,
            state: start_state.to_string(),
            transitions,
        }
    }

    fn step(&mut self) -> bool {
        let current_char = self.tape.read();
        let key = (self.state.clone(), current_char);

        let result = if let Some(transition) = self.transitions.get(&key) {
            println!("Current State: {}, Read: {}", self.state, current_char);
            self.print_tape();

            self.tape.write(transition.write);
            self.tape.move_head(&Direction::from_str(&transition.direction));
            self.state = transition.next_state.clone();

            println!("Write: {}, Move: {}, Next State: {}", transition.write, transition.direction, self.state);
            self.print_tape();
            println!();

            true
        } else {
            false
        };

        wait_keypress();
        result
    }

    fn run(&mut self) {
        while self.step() {}
    }

    fn print_tape(&self) {
        for (i, &c) in self.tape.tape.iter().enumerate() {
            if i == self.tape.head {
                print!("[{}]", c);
            } else {
                print!(" {}", c);
            }
        }
        println!();
    }
}

fn main() {
    println!("Stem: Turing Educational Machine");
    println!("{}", "-".repeat(35));

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <transitions_json_file>", args[0]);
        return;
    }
    let transitions_file = &args[1];

    // Read json file
    let json = fs::read_to_string(transitions_file).expect("Unable to read file");
    let config: TuringMachineConfig = from_str(&json).expect("JSON was not well-formatted");

    // Construct transitions table
    let mut transitions = HashMap::new();
    for (key, transition) in config.transitions {
        let parts: Vec<&str> = key.split('_').collect();
        let state = parts[0].to_string();
        let symbol = parts[1].chars().next().unwrap();
        transitions.insert((state, symbol), transition);
    }

    // Initialize tape 
    let tape = Tape::new(&config.tape);

    // Initialize machine
    let mut machine = TuringMachine::new(tape, &config.start_state, transitions);

    // Run turing machine
    machine.run();
}
