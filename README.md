# Introduction
The following codebase is part of Rust learning and is based upon the Guessing game described in the [Rust Handbook - lesson 2](https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html).

# Rules
To play the guessing game, you need to provide a number guess in the terminal. You can guess as many times as you want, until you've found the secret number. The game will provide you with feedback on your guess and how it relates to the secret number:
1. When your guess is lower than the secret number, then you'll get a message stating your guess was too small. 
1. When your guess is high than the secret number, then you'll get a message stating your guess was too big. 
1. When your guess was exact the same as the secret number, then you'll get the message that you've won the game. 

# Get started
1. Make sure you have Rust and cargo installed to build the necessary assets. For installation instructions, see the [Rust handbook - instruction manual](https://rust-book.cs.brown.edu/ch01-01-installation.html)
1. run `cargo build` to build all the files, or use `cargo run` to build and run. 
1. If you want to generate production assets, run `cargo build --release`. 