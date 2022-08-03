use rand::Rng;
use std::io;

fn main() {
    // two executable functions: get_code and enter_code. 
    let the_code: i32 = generate_code();
    println!("type get_code or enter_code");
    println!("type s to stop.");
    let mut input = String::new();

    loop { 
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input.");
        
        let trimmed_input = input.trim();
        if trimmed_input == "s" {
            println!("terminating_program...");
            break;
        }
    }

    

}

// in theory, confers ownership.
fn generate_code() -> i32 {
    // 6-digit pin
    let a_code: i32 = rand::thread_rng().gen_range(100000..=999999);
    a_code
}



