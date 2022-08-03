use rand::Rng;
use std::io;

fn main() {
    // two executable functions: get_code and enter_code. 
    println!("type get_code or enter_code");
    println!("type s to stop.");

    let the_code: i32 = generate_code();
    let mut open = false;

    loop { 
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input.");
        
        let trimmed_input = input.trim();
        if trimmed_input == "s" {
            println!("terminating_program...");
            break;
        } else if trimmed_input == "get_code" {
            println!("CODE: {}", the_code);
        } else if trimmed_input == "enter_code" {
            println!("enter code:");
            let code_is_correct = handle_code_entry(the_code);
            if code_is_correct {
                open = true;
                println!("keycode_accepted. unlocking...");
            } else {
                println!("invalid keycode. try again.");
            }
        } else if trimmed_input == "check_is_unlocked" {
            println!("UNLOCKED: {}", open);
        } else {
            println!("invalid_input");
        }
    }
}

// in theory, confers ownership.
fn generate_code() -> i32 {
    // 6-digit pin
    let a_code: i32 = rand::thread_rng().gen_range(100000..=999999);
    a_code
}

fn handle_code_entry(the_code: i32) -> bool {
    let mut code_guess = String::new();
    io::stdin() 
        .read_line(&mut code_guess)
        .expect("failed to read input.");
    
    let trimmed_code_guess: i32 = code_guess.trim().parse().expect("must be a number.");
    if the_code == trimmed_code_guess {
        true 
    } else {
        false
    }
}
