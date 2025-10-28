use clap::Parser;
use std::io;

#[derive(Parser)]
#[command(name = "hello Rust CLI")]
#[command(about = "Start your 100 days of Rust journey", long_about = None)]
struct Args {
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("=== Welcome to Hello Rust CLI ===");
    println!();

    let name = match args.name {
        Some(n) => n,
        None => get_input("What is your name? "),
    };

    let language = get_input("What is you current favourite programming language?");

    let goal = get_input("What do you want to build with this language");

    display_welcome(&name, &language, &goal);
}

fn get_input(prompt: &str) -> String {
    loop {
        println!("{}", prompt);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if !input.is_empty() {
            return input.to_string();
        }

        // Return the trimmed input
        println!("Input cannot be empty, please try again.");
    }
}

fn display_welcome(name: &str, language: &str, goal: &str) {
    println!();
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║          WELCOME TO YOUR RUST JOURNEY!           ║");
    println!("╚═══════════════════════════════════════════════════╝");
    println!();
    println!("🦀 Hello, {}!", name);
    println!(
        "📚 Moving from {} to Rust is an excellent choice!",
        language
    );
    println!("🎯 Your goal: {}", goal);
    println!();
    println!("Fun fact: Rust has been Stack Overflow's most loved");
    println!("programming language for 8+ years in a row!");
    println!();
    println!("Let's start building amazing things together! 🚀");
}
