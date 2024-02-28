use chess::{Board, Color, Piece, Square};

pub fn show_board(board: Board) -> String {
    let mut board_string: String = String::new();
    
    board_string.push_str("\n|----BLACK----|\n");
    board_string.push(piece_char(board.piece_on(Square::A8), board.color_on(Square::A8)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::B8), board.color_on(Square::B8)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::C8), board.color_on(Square::C8)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::D8), board.color_on(Square::D8)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::E8), board.color_on(Square::E8)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::F8), board.color_on(Square::F8)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::G8), board.color_on(Square::G8)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::H8), board.color_on(Square::H8)));
    board_string.push('\n');
    board_string.push(piece_char(board.piece_on(Square::A7), board.color_on(Square::A7)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::B7), board.color_on(Square::B7)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::C7), board.color_on(Square::C7)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::D7), board.color_on(Square::D7)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::E7), board.color_on(Square::E7)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::F7), board.color_on(Square::F7)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::G7), board.color_on(Square::G7)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::H7), board.color_on(Square::H7)));
    board_string.push('\n');
    board_string.push(piece_char(board.piece_on(Square::A6), board.color_on(Square::A6)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::B6), board.color_on(Square::B6)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::C6), board.color_on(Square::C6)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::D6), board.color_on(Square::D6)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::E6), board.color_on(Square::E6)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::F6), board.color_on(Square::F6)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::G6), board.color_on(Square::G6)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::H6), board.color_on(Square::H6)));
    board_string.push('\n');
    board_string.push(piece_char(board.piece_on(Square::A5), board.color_on(Square::A5)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::B5), board.color_on(Square::B5)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::C5), board.color_on(Square::C5)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::D5), board.color_on(Square::D5)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::E5), board.color_on(Square::E5)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::F5), board.color_on(Square::F5)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::G5), board.color_on(Square::G5)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::H5), board.color_on(Square::H5)));
    board_string.push('\n');
    board_string.push(piece_char(board.piece_on(Square::A4), board.color_on(Square::A4)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::B4), board.color_on(Square::B4)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::C4), board.color_on(Square::C4)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::D4), board.color_on(Square::D4)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::E4), board.color_on(Square::E4)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::F4), board.color_on(Square::F4)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::G4), board.color_on(Square::G4)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::H4), board.color_on(Square::H4)));
    board_string.push('\n');
    board_string.push(piece_char(board.piece_on(Square::A3), board.color_on(Square::A3)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::B3), board.color_on(Square::B3)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::C3), board.color_on(Square::C3)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::D3), board.color_on(Square::D3)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::E3), board.color_on(Square::E3)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::F3), board.color_on(Square::F3)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::G3), board.color_on(Square::G3)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::H3), board.color_on(Square::H3)));
    board_string.push('\n');
    board_string.push(piece_char(board.piece_on(Square::A2), board.color_on(Square::A2)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::B2), board.color_on(Square::B2)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::C2), board.color_on(Square::C2)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::D2), board.color_on(Square::D2)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::E2), board.color_on(Square::E2)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::F2), board.color_on(Square::F2)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::G2), board.color_on(Square::G2)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::H2), board.color_on(Square::H2)));
    board_string.push('\n');
    board_string.push(piece_char(board.piece_on(Square::A1), board.color_on(Square::A1)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::B1), board.color_on(Square::B1)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::C1), board.color_on(Square::C1)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::D1), board.color_on(Square::D1)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::E1), board.color_on(Square::E1)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::F1), board.color_on(Square::F1)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::G1), board.color_on(Square::G1)));
    board_string.push(' ');
    board_string.push(piece_char(board.piece_on(Square::H1), board.color_on(Square::H1)));
    board_string.push('\n');
    board_string.push_str("|----WHITE----|\n");
    board_string.push_str("\n===============");

    board_string
}

fn piece_char(piece: Option<Piece>, color: Option<Color>) -> char {
    match piece {
        Some(p) => match p {
            Piece::Pawn => match color {
                Some(Color::White) => 'P',
                Some(Color::Black) => 'p',
                None => '.'
            },
            Piece::Knight => match color {
                Some(Color::White) => 'N',
                Some(Color::Black) => 'n',
                None => '.'
            },
            Piece::Bishop => match color {
                Some(Color::White) => 'B',
                Some(Color::Black) => 'b',
                None => '.'
            },
            Piece::Rook => match color {
                Some(Color::White) => 'R',
                Some(Color::Black) => 'r',
                None => '.'
            },
            Piece::Queen => match color {
                Some(Color::White) => 'Q',
                Some(Color::Black) => 'q',
                None => '.'
            },
            Piece::King => match color {
                Some(Color::White) => 'K',
                Some(Color::Black) => 'k',
                None => '.'
            },
        }
        None => '.',
    }
}
