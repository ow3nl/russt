pub mod pst;

use chess::{BitBoard, Board, BoardStatus, ChessMove, Color, MoveGen, Piece, EMPTY};

pub fn make_best_move(board: Board, depth: u8) -> Board {
    board.make_move_new(find_best_move(board, depth))
}

pub fn find_best_move(board: Board, depth: u8) -> ChessMove {
    let moves: MoveGen = MoveGen::new_legal(&board);
    // leaves depth 3: 600
    // leaves depth 4: 13160
    // leaves depth 5: 405385
    // leaves depth 6: 9771632
    // leaves depth 7: 309478263

    let mut max_score: i32 = std::i32::MIN;
    let mut best_move: Option<ChessMove> = None;
    for m in moves {
        let score: i32 = negamax(
            &board.make_move_new(m),
            depth - 1,
            std::i32::MIN
        );
        if score >= max_score {
            max_score = score;
            best_move = Some(m);
        }
    }

    best_move.expect("Should be impossble, check loop code.")
}

// FIXME: alpha beta not working rn
// TODO: what does alpha beta look like in negamax
// TODO: check capture moves first, alpha beta cutoff more likely
// TODO: killer heuristic
// TODO: aspiration search
// TODO: quiescence search
fn negamax(board: &Board, depth: u8, super_max: i32) -> i32 {
    if depth == 0 {
        evaluate_position(board)
    } else {
        match board.status() {
            BoardStatus::Checkmate => std::i32::MAX,
            BoardStatus::Stalemate => 0,
            BoardStatus::Ongoing => {
                let mut opp_moves: MoveGen = MoveGen::new_legal(board);

                let mut best_opp_score = std::i32::MIN;

                let opp_pieces = board.color_combined(
                    !board.side_to_move());
                opp_moves.set_iterator_mask(*opp_pieces);
                for m in &mut opp_moves {
                    let opp_score = negamax(
                        &board.make_move_new(m),
                        depth - 1,
                        best_opp_score,
                    );
                    // below line same as opp_score >= -super_max 
                    // super_max sometimes MIN, can't negate
                    if -opp_score <= super_max {
                        return -opp_score;
                    }
                    if opp_score >= best_opp_score {
                        best_opp_score = opp_score;
                    }
                }

                opp_moves.set_iterator_mask(!EMPTY);
                for m in &mut opp_moves {
                    let opp_score = negamax(
                        &board.make_move_new(m),
                        depth - 1,
                        best_opp_score,
                    );
                    // below line same as opp_score >= -super_max 
                    // super_max initialized as MIN, can't negate
                    if -opp_score <= super_max {
                        return -opp_score;
                    }
                    if opp_score >= best_opp_score {
                        best_opp_score = opp_score;
                    }
                }

                -best_opp_score
            },
        }
    }
}

fn negamax_unpruned(board: &Board, depth: u8) -> i32 {
    if depth == 0 {
        evaluate_position(board)
    } else {
        match board.status() {
            BoardStatus::Checkmate => std::i32::MAX,
            BoardStatus::Stalemate => 0,
            BoardStatus::Ongoing => {
                let opp_moves: MoveGen = MoveGen::new_legal(board);

                let mut best_opp_score = std::i32::MIN;
                for m in opp_moves {
                    let opp_score = negamax_unpruned(
                        &board.make_move_new(m),
                        depth - 1,
                    );
                    if opp_score >= best_opp_score {
                        best_opp_score = opp_score;
                    }
                }

                -best_opp_score
            },
        }
    }
}

// evaluates the position of the board for the color that just moved.
fn evaluate_position(board: &Board) -> i32 {
    match board.status() {
        BoardStatus::Checkmate => return std::i32::MAX,
        BoardStatus::Stalemate => return 0,
        BoardStatus::Ongoing => {},
    };

    let mut white_score: i32 = 0;

    let white_pieces: BitBoard = *board.color_combined(Color::White);
    white_score += piece_score(
        *board.pieces(Piece::Pawn) & white_pieces,
        pst::WP_PST
    );
    white_score += piece_score(
        *board.pieces(Piece::Knight) & white_pieces,
        pst::WN_PST
    );
    white_score += piece_score(
        *board.pieces(Piece::Bishop) & white_pieces,
        pst::WB_PST
    );
    white_score += piece_score(
        *board.pieces(Piece::Rook) & white_pieces,
        pst::WR_PST
    );
    white_score += piece_score(
        *board.pieces(Piece::Queen) & white_pieces,
        pst::WQ_PST
    );
    white_score += piece_score(
        *board.pieces(Piece::King) & white_pieces,
        pst::WK_PST
    );

    let mut black_score: i32 = 0;

    let black_pieces: BitBoard = *board.color_combined(Color::Black);
    black_score += piece_score(
        *board.pieces(Piece::Pawn) & black_pieces,
        pst::BP_PST
    );
    black_score += piece_score(
        *board.pieces(Piece::Knight) & black_pieces,
        pst::BN_PST
    );
    black_score += piece_score(
        *board.pieces(Piece::Bishop) & black_pieces,
        pst::BB_PST
    );
    black_score += piece_score(
        *board.pieces(Piece::Rook) & black_pieces,
        pst::BR_PST
    );
    black_score += piece_score(
        *board.pieces(Piece::Queen) & black_pieces,
        pst::BQ_PST
    );
    black_score += piece_score(
        *board.pieces(Piece::King) & black_pieces,
        pst::BK_PST
    );

    match board.side_to_move() {
        Color::White => black_score - white_score,  // black just moved
        Color::Black => white_score - black_score,  // white just moved
    }
}

fn piece_score(bitboard: BitBoard, weights: [i32; 64]) -> i32 {
    let mut bits: usize = bitboard.to_size(0);
    let mut score: i32 = 0;
    for i in 0..64 {
        if bits % 2 == 1 {score += weights[i];}
        bits >>= 1;
    }
    score
}
