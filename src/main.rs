use rand::Rng;
use std::io;
use dialoguer::{
    Select,
    Confirm,
    theme::ColorfulTheme
};

fn main() -> std::io::Result<()> {

    println!("BEGIN KEYCODE PROGRAM ------------------------------");
    let mut the_code: i32 = generate_code();
    let mut open = false;
    let mut remaining_attempts = 3;
    'main_program: loop { 
        let items = vec![
            "Get Code",
            "Enter Code",
            "Check Lock Status",
            "Lock",
            "End Program"
        ];
    
        let chosen : usize = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact()?;


        match chosen {
            4 => {
                break 'main_program;
            },
            0 => {
                println!("CODE: {}", the_code);
            },
            1 => {
                'code_entry: loop { 
                    println!("enter code:");
                    let code_is_correct = handle_code_entry(the_code);
                    if code_is_correct {
                        if Confirm::new().with_prompt("Keycode accepted. Unlock?").interact()? {
                            open = true;
                            println!("keycode accepted. unlocked.");
                            break 'code_entry;
                        } else {
                            println!("not unlocking.");
                            break 'code_entry;
                        }
                    } else {
                        if remaining_attempts > 1 {
                            remaining_attempts = remaining_attempts - 1;
                            println!("invalid keycode. {} attempts remaining.", remaining_attempts);
                        } else {
                            println!("too many attempts. module inaccessible.");
                            break 'code_entry;
                        }
                    }
                }
            },
            2 => {
                println!("LOCKED: {}", !open);
            },
            3 => {
                if open {
                    open = false;
                    the_code = generate_code();
                    println!("successful lock.")
                } else {
                    println!("already_locked");
                }
            },
            _ => {
                println!("invalid_input");
            }
        }
    }
    println!("END KEYCODE PROGRAM ------------------------------");
    Ok(()) // to satisfy returning a Result
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
