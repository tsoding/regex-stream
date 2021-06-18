type FsmIndex = usize;

const FSM_COLUMN_SIZE: usize = 130;
const FSM_LINEEND: FsmIndex = 129;

#[derive(Default, Clone, Copy)]
struct FsmAction {
    next: FsmIndex,
    offset: i32,
}

#[derive(Clone)]
struct FsmColumn {
    ts: [FsmAction; FSM_COLUMN_SIZE],
}

impl FsmColumn {
    fn new() -> Self {
        Self {
            ts: [Default::default(); FSM_COLUMN_SIZE]
        }
    }
}

struct Regex {
    cs: Vec<FsmColumn>
}

#[derive(Debug, Copy, Clone)]
enum Atom {
    Any,
    EndLine,
    Char(char),
}

#[derive(Debug)]
enum Token {
    Atom(Atom),
    Star(Atom),
}

fn lexer(src: &str) -> Vec<Token> {
    let bytes = src.as_bytes();

    let mut result = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let atom = match bytes[i] as char {
            '.'       => Atom::Any,
            '$'       => Atom::EndLine,
            '*' | '+' => panic!("Invalid target for quantifier"),
            x         => Atom::Char(x),
        };

        i += 1;

        match bytes.get(i).map(|x| *x as char) {
            Some('*') => {
                i += 1;
                result.push(Token::Star(atom));
            }
            Some('+') => {
                i += 1;
                result.push(Token::Atom(atom));
                result.push(Token::Star(atom));
            }
            _ => {
                result.push(Token::Atom(atom));
            }
        };
    }
    result
}

fn compile_atom(atom: &Atom, success: FsmIndex) -> FsmColumn {
    use Atom::*;

    let mut column = FsmColumn::new();

    match atom {
        Any => {
            for i in 32..127 {
                column.ts[i] = FsmAction {
                    next: success,
                    offset: 1,
                };
            }
        },
        EndLine => {
            column.ts[FSM_LINEEND] = FsmAction {
                next: success,
                offset: 1,
            };
        }
        Char(x) => {
            column.ts[*x as usize] = FsmAction {
                next: success,
                offset: 1,
            };
        }
    }

    column
}

impl Regex {
    fn compile(src: &str) -> Self {
        let tokens = lexer(src);
        let mut fsm = Self { cs: Vec::new() };
        fsm.cs.push(FsmColumn::new()); // default failed state

        for token in tokens.iter() {
            let current_state = fsm.cs.len();
            let next_state = fsm.cs.len() + 1;

            match token {
                Token::Atom(atom) => {
                    let column = compile_atom(atom, next_state);
                    fsm.cs.push(column);
                },

                Token::Star(atom) => {
                    let mut column = compile_atom(atom, current_state);
                    for action in column.ts.iter_mut() {
                        if action.next == 0 {
                            action.next = next_state;
                        } else {
                            assert!(action.next == current_state);
                        }
                    }
                    fsm.cs.push(column);
                }
            }
        }

        fsm
    }

    fn match_str(&self, input: &str) -> bool {
        let mut state = 1;
        let mut head = 0;
        let chars = input.chars().collect::<Vec<_>>();
        let n = chars.len();

        while 0 < state && state < self.cs.len() && head < n {
            let action = self.cs[state].ts[chars[head] as usize];
            state = action.next;
            head = (head as i32 + action.offset) as usize;
        }

        if state == 0 {
            return false;
        }

        if state < self.cs.len() {
            let action = self.cs[state].ts[FSM_LINEEND];
            state = action.next;
        }

        return state >= self.cs.len();
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for symbol in 0..FSM_COLUMN_SIZE {
            print!("{:03} => ", symbol);
            for column in self.cs.iter() {
                print!("({}, {}) ",
                       column.ts[symbol].next,
                       column.ts[symbol].offset);
            }
            println!();
        }
    }
}

fn test_regex(regex_src: &str, test_cases: &[(&str, bool)]) {
    let regex = Regex::compile(regex_src);

    println!("Testing {:?}", regex_src);
    for (input, expected_outcome) in test_cases {
        println!("  input: {:?}", input);
        println!("  match: {:?}", *expected_outcome);
        assert_eq!(regex.match_str(input), *expected_outcome);
        println!();
    }
}

fn main() {
    let tests = vec!{
        ("a+bc$", vec![
            ("Hello, World", false),
            ("bc", false),
            ("abc", true),
            ("aabc", true),
            ("aaabc", true),
            ("bbc", false),
            ("cbc", false),
            ("cbd", false),
            ("cbt", false),
            ("abcd", false),
        ], false),
        (".*bc", vec![
            ("bc", true),
            ("abc", true),
            ("aabc", true),
        ], true),
    };

    for (regex_src, test_cases, ignored) in tests.iter() {
        if !ignored {
            test_regex(regex_src, &test_cases);
        }
    }
}
