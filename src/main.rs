use std::io::Write;
use std::io;
use chess::{Board, BoardStatus, ChessMove, Color, Error};

pub mod engine;
pub mod show;

fn main() {
    // create a board with the initial position
    let mut board: Board = Board::default();

    loop {
        print!("Do you want to play as white [w] or black [b]: ");
        io::stdout().flush().expect("Printing error");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect(
            "Failed to read input");
        match input.as_str() {
            "w\n" => break,
            "b\n" => {
                println!("{}", show::show_board(board));
                board = engine::make_best_move(board, 5_u8);
                break
            },
            _ => {println!("{}", input);}
        }
    }

    println!("{}", show::show_board(board));

    while board.status() == BoardStatus::Ongoing {
        let mut player_move: Result<ChessMove, Error> = Err(
            Error::InvalidSanMove);
        while player_move.is_err() {
            let mut input: String = String::new();
            print!("\n--> ");
            io::stdout().flush().expect("Printing error");
            io::stdin().read_line(&mut input).expect(
                "Failed to read input");
            player_move = ChessMove::from_san(
                &board, input.as_str());
            match player_move {
                Ok(_) => break,
                Err(_) => match input.as_str() {
                    "quit\n" => return,
                    _ => println!(
                        "Invalid input (please enter a valid move in SAN)"),
                },
            };
        }

        board = board.make_move_new(player_move.unwrap());
        println!("{}", show::show_board(board));
        board = engine::make_best_move(board, 5_u8);
        println!("{}", show::show_board(board));
    }

    match board.status() {
        BoardStatus::Stalemate => println!("Stalemate"),
        BoardStatus::Checkmate => match board.side_to_move() {
            Color::White => println!("Black wins"),
            Color::Black => println!("White wins"),
        },
        BoardStatus::Ongoing => println!(
            "Game still ongoing (shoudld be impossible)"),
    };
}
