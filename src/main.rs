use rand::Rng;
use std::env;

fn main() {
    // Read arguments from the command line
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <password_length> <characters>", args[0]);
        std::process::exit(1);
    }

    let password_length = args[1].parse::<usize>().unwrap_or_else(|_| {
        eprintln!("Error: Invalid password length.");
        std::process::exit(1);
    });

    let characters = &args[2];

    let password = generate_random_password(password_length, characters);
    println!("{}", password);
}

fn generate_random_password(length: usize, characters: &str) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..characters.len());
            characters.chars().nth(idx).unwrap()
        })
        .collect()
}
