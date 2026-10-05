use std::io::stdin;
use std::collections::HashMap;
use rand::seq::IteratorRandom;

fn main() {
    let mut terms = HashMap::new();

    terms.insert("acetate".to_string(), "C2H3O2 -".to_string());
    terms.insert("ammonium".to_string(), "NH4 +".to_string());
    terms.insert("carbonate".to_string(), "CO3 2-".to_string());
    terms.insert("bicarbonate".to_string(), "HCO3 -".to_string());
    terms.insert("hydroxide".to_string(), "OH -".to_string());
    terms.insert("nitrate".to_string(), "NO3 -".to_string());
    terms.insert("chlorate".to_string(), "ClO3 -".to_string());
    terms.insert("sulfate".to_string(), "SO4 2-".to_string());
    terms.insert("phosphate".to_string(), "PO4 3-".to_string());

    let mut input = String::new();
    println!("1. Ion -> Formula");
    println!("2. Formula -> Ion");
    println!("3. Exit");
    stdin().read_line(&mut input).expect("Failed to read line :(");

    if input.trim() == "1" {
        game(1, terms);
    } else if input.trim() == "2" {
        game(2, terms);
    } else {
        println!("aw man :(")
    }
}

fn game(mode: u8, terms: HashMap<String, String>) {
    let mut running = true;

    while running {
        let mut input = String::new();
        let mut rng = rand::thread_rng();
        if let Some((key, value)) = terms.iter().choose(&mut rng) {
            if mode == 1 {
                println!("{}", key);
            } else if mode == 2 {
                println!("{}", value);
            }
            stdin().read_line(&mut input).expect("failed to read answer :(");

            if input.trim() == "exit".to_string() {
                running = false;
            } else if mode == 1 {
                if input.trim() == value.to_string() {
                    println!("You got it right!")
                } else {
                    println!("Wrong answer! The correct answer is {}", value);
                } 
            } else if mode == 2 {
                if input.trim() == key.to_string() {
                    println!("You got it right!")
                } else {
                    println!("Wrong answer! The correct answer is {}", key);
                } 
            }
        } else {
            println!("error :(");
        }
    }
}
