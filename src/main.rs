#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked,
}

fn main() {
    let code = String::from("code123");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        println!("*******************************************************************");
        println!("Hello welcome to the vault ! Would you please enter the password ?");
        println!("*******************************************************************");
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => {
                        continue;
                    }
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("Wrong input");
                entry.clear();
                state = State::Locked;
                continue;
            }

            State::Unlocked => {
                println!("The safe is unlocked");
                return;
            }
        }
    }
}
