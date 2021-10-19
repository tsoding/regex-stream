use std::io::{self, BufRead, Write};

// `#[repr(usize)]` allows us to convert `State` varients into a `usize` based
// on it's index in the `State` enum using `<State> as usize`
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
enum State {
    Locked,   // 0
    Unlocked, // 1
    #[doc(hidden)]
    COUNT,    // 2 (quick and easy way to do this)
}

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
enum Event {
    Push,  // 0
    Coin,  // 1
    #[doc(hidden)]
    COUNT, // 2
}

const FSM: [[State; Event::COUNT as usize]; State::COUNT as usize] = [
//   Event::Push    Event::Coin
    [State::Locked, State::Unlocked], // State::Locked
    [State::Locked, State::Unlocked], // State::Unlocked
];

fn next_state(state: State, event: Event) -> State {
    FSM[state as usize][event as usize]
}

fn main() {
    let mut state = State::Locked;

    println!("State: {:?}", state);
    print!("> ");
    io::stdout().flush().unwrap();

    for line in io::stdin().lock().lines() {
        match line.unwrap().as_str() {
            "coin" => state = next_state(state, Event::Coin),
            "push" => state = next_state(state, Event::Push),
            "quit" => break,
            unknown => eprintln!("ERROR: Unknown event {}", unknown),
        }

        println!("State: {:?}", state);
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
