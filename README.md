## Russt

# About

This is a simple chess backend I wrote in Rust.
It features only a very basic UI, and is intended more as a demonstration of the negamax algorithm than an actual playable chess application.

To use the application, build the project as outlined below, and run the executable.
There is a basic shell, which allows the user to play against the computer.
To exit the shell, enter `quit` or press control-`C`.

At the start, the user is prompted to choose which color they would like to play as.
Enter `w` to play as white, and `b` to play as black.

Each move must be entered in Standard Algebraic Notation (SAN). More information about SAN can be found [here]().
After a valid move has been entered, the new board state will be printed to the terminal.
Then, the program will make a move with the other color and print the new board state.

To change the search depth of the negamax algorithm, change the magic number `5_u8` on line 22 to the desired value (for example, to change the search depth to `3`, write `3_u8` here instead).

### Dependencies

- Cargo 1.76 or later (I am running Cargo 1.76.0, but older versions will likely work as well)
- `chess` crate (see [here](https://docs.rs/chess/latest/chess/) for more info)

### Build Command

Navigate to the project directory use the standard `$ cargo build` command from the command line.
The executable will be created at `$ ./target/debug/russt`.
