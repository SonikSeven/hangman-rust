# Hangman

Welcome to the Hangman Game project! This simple, interactive command-line application implements the classic game of Hangman in Rust. The game selects a random word from a predefined list, and the player attempts to guess it by suggesting letters, within a limited number of guesses.

## Features

- **Interactive Gameplay:** Play through the command line with textual feedback for each action.
- **Score Tracking:** Keeps track of wins and losses throughout the session.
- **Dynamic Hint System:** Reveals correct guesses and displays them as part of the word being guessed.
- **Error Handling:** Provides feedback for invalid inputs such as non-lowercase and non-single-letter inputs.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## Installation

This application is written in Rust, so you'll need Rust installed on your computer to build it. If you don't have Rust installed, you can download it from [rust-lang.org](https://www.rust-lang.org/tools/install).

To install this project, follow these steps:

1. Clone the repository to your local machine:
   
   ```
   git clone https://github.com/SonikSeven/hangman-rust.git
   ```
   
2. Navigate to the cloned repository:
   
   ```
   cd hangman-rust
   ```

3. Build the project with Cargo:
   
   ```
   cargo build --release
   ```

## How to Run

Run the executable in `./target/release/`.

## How to Play

Once the application starts, you will enter an interactive command prompt. Here are the commands you can use:

- **play:** Starts a new game of Hangman.
- **results:** Shows the number of games won and lost during the current session.
- **exit:** Exits the application.

During the game, you will be prompted to input a letter you guess might be in the word. You'll get feedback based on your guess. The game continues until you successfully guess the word or exhaust your limited number of incorrect guesses.

## Game Rules

- Only lowercase single English letters are accepted.
- Repeated guesses of the same letter won't count against your remaining attempts, but you will be reminded that you've already guessed that letter.

## License

This project is licensed under the [MIT License](LICENSE.txt).
