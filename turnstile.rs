#![feature(variant_count)]
use std::{fmt::{self, Display}, ops::Index};
use std::io::{self, BufRead, Write};

#[derive(Copy, Clone)]
enum State {
    Locked = 0,
    Unlocked = 1,
}

impl Display for State {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			match &self {
				State::Locked => write!(f, "Locked"),
				State::Unlocked => write!(f, "Unlocked")
			}
	}
}
impl<T, const N: usize> Index<State> for [T; N] {
    type Output = T;

    fn index(&self, index: State) -> &Self::Output {
        &self[index as usize]
    }
}

enum Event {
	Push = 0,
	Coin = 1,
}
impl<T, const N: usize> Index<Event> for [T; N] {
    type Output = T;

    fn index(&self, index: Event) -> &Self::Output {
        &self[index as usize]
    }
}
const EVENTS_COUNT: usize = std::mem::variant_count::<Event>();
const STATES_COUNT: usize = std::mem::variant_count::<State>();
const FSM: [[State; EVENTS_COUNT]; STATES_COUNT] = [
  // PUSH    COIN
    [State::Locked, State::Unlocked],        // LOCKED
    [State::Locked, State::Unlocked],        // UNLOCKED
];



fn next_state(state: State, event: Event) -> State {
    FSM[state][event]
}

fn main() {
    let mut state = State::Locked;

    println!("State: {}", state.to_string());
    print!("> ");
    io::stdout().flush().unwrap();
    for line in io::stdin().lock().lines() {
        match line.unwrap().as_str() {
            "coin" => state = next_state(state, Event::Coin),
            "push" => state = next_state(state, Event::Push),
            "quit" => return,
            unknown => {
                eprintln!("ERROR: Unknown event {}", unknown);
            }
        }

        println!("State: {}", state.to_string());
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
