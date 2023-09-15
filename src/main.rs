#![feature(str_split_whitespace_remainder)]
#![feature(exact_size_is_empty)]
mod search;
use chess::*;
use std::io;
use std::str::{SplitAsciiWhitespace, FromStr};

fn main() {
    let mut board: Board = Board::default();
    let stdin = io::stdin();
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Invalid in");
        let mut words = input.split_ascii_whitespace();
        match words.next() {
            Some("uci") => println!("id name bitchess\nid author Alexander Ackermann\nuciok"),
            Some("ucinewgame") => board = Board::default(),
            Some("isready") => println!("readyok"),
            Some("position") => {
                board = match parse_position(&mut words) {
                    Ok(b) => b,
                    Err(e) => {
                        eprintln!("Position error: {}", e);
                        continue;
                    },
                };
            },
            Some("go") => parse_go(&mut words, &board),
            Some("d") => println!("Position: {}", board),
            Some("quit") => break,
            Some(c) => println!("Unknown command: '{}'.", c),
            None => continue,
        };
    }
}

fn parse_position(params: &mut SplitAsciiWhitespace) -> Result<Board, chess::Error> {
    let mut board: Board;
    let mut word = params.next();
    match word {
        Some("fen") => {
            let fen = params.remainder().unwrap_or_default();
            board = Board::from_str(&fen)?;
            word = params.nth(6);
        },
        Some("startpos") => {
            board = Board::default();
            word = params.next();
        },
        None => return Ok(Board::default()),
        _ => board = Board::default(),
    };
    match word {
        Some("moves") => {
            for mv in params {
                board = board.make_move_new(parse_mv(&board, mv).ok_or(Error::InvalidUciMove)?);
            }
        },
        None => return Ok(board),
        _ => return Err(Error::InvalidUciMove),
    }
    Ok(board)
}

fn parse_mv(board: &Board, s: &str) -> Option<ChessMove> {
    let len = s.len();
    if len != 4 && len != 5 {
        return None;
    }
    let src = Square::from_str(&s[..2]).ok()?;
    let dest = Square::from_str(&s[2..4]).ok()?;
    let promo = s.chars().nth(4).map(|c| match c {
        'q' => Some(Piece::Queen),
        'r' => Some(Piece::Rook),
        'b' => Some(Piece::Bishop),
        'n' => Some(Piece::Knight),
        _ => None
    }).flatten();
    let mv = ChessMove::new(src, dest, promo);
    board.legal(mv).then_some(mv)
}

fn parse_go(params: &mut SplitAsciiWhitespace, board: &Board) {
    let word = params.next();
    match word {
        Some("depth") => match params.next().unwrap_or_default().parse::<u8>() {
            Ok(0) => eprintln!("Infinite not implmented"),
            Ok(n) => println!("{}", search::think(&board, n)
                    .map(|(m, _)| format!("bestmove {}", m))
                    .unwrap_or("No move found!".to_string())),
            Err(e) => eprintln!("{}", e)
        },
        _ => eprintln!("Unrecognized command"),
    }
}
