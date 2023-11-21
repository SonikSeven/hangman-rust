use rand::Rng;
use std::{collections::HashSet, io};

struct Stats {
    wins: u8,
    fails: u8,
}

fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    println!("H A N G M A N");
    let mut stats = Stats { wins: 0, fails: 0 };
    loop {
        println!("\nType \"play\" to play the game, \"results\" to show the scoreboard, and \"exit\" to quit: ");
        match input().as_str() {
            "play" => play(&mut stats),
            "results" => println!(
                "You won: {} times.\nYou lost: {} times.",
                stats.wins, stats.fails
            ),
            "exit" => return,
            _ => println!("Unknown command"),
        }
    }
}

fn print_hint(word: &str, user_letters: &HashSet<char>) {
    let hint: String = word
        .chars()
        .map(|c| if user_letters.contains(&c) { c } else { '-' })
        .collect();
    println!("\n{hint}");
}

fn play(stats: &mut Stats) {
    let words = ["python", "java", "kotlin", "javascript", "rust"];
    let word = words[rand::thread_rng().gen_range(0..words.len())];
    let word_letters: HashSet<char> = word.chars().collect();
    let mut user_letters: HashSet<char> = HashSet::new();
    let mut attempts = 8;

    while attempts > 0 {
        print_hint(word, &user_letters);
        println!("Attempts left: {attempts}\nInput a letter: ");
        let user_input = input();

        if user_input.len() != 1 {
            println!("You should input a single letter");
            continue;
        }

        let new_letter = user_input.chars().next().unwrap();

        if !new_letter.is_lowercase() {
            println!("Please enter a lowercase English letter");
            continue;
        } else if user_letters.contains(&new_letter) {
            println!("You've already guessed this letter");
            continue;
        } else if !word.contains(new_letter) {
            println!("That letter doesn't appear in the word");
            attempts -= 1;
        }

        user_letters.insert(new_letter);

        if HashSet::difference(&word_letters, &user_letters).count() == 0 {
            println!("\n{word}\nYou guessed the word!\nYou survived!");
            stats.wins += 1;
            return;
        }
    }
    println!("\nYou lost!");
    stats.fails += 1;
}
