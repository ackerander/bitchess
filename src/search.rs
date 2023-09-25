use chess::*;
use std::time::Instant;

fn eval(board: &Board) -> i16 {
    let side_bits = board.color_combined(board.side_to_move());
    let mut score = 0;
    for e in ALL_PIECES.iter().zip([10, 32, 33, 50, 90].iter()) {
        let pieces = board.pieces(*e.0);
        let my_pieces = pieces & side_bits;
        score += e.1 * (my_pieces.popcnt() as i16 - (pieces ^ my_pieces).popcnt() as i16);
    }
    score
}

fn negamax(board: &Board, mut alpha: i16, beta: i16, depth: u8) -> i16 {
    let mvs = match OrdMoves::new_ordered(&board) {
        Some(m) => m,
        None => return if *(board.checkers()) == EMPTY { 0 } else { -3000 - depth as i16 },
    };

    if depth == 0 {
        return quiescence(&board, alpha, beta);
    }

    let mut new_board = Board::default();
    for m in mvs {
        board.make_move(m, &mut new_board);
        let score = -negamax(&new_board, -beta, -alpha, depth - 1);
        if score >= beta {
            return score;
        }
        if score > alpha {
            alpha = score;
        }
    }
    alpha
}

fn quiescence(board: &Board, mut alpha: i16, beta: i16) -> i16 {
    let stand_pat = eval(&board);
    if stand_pat >= beta {
        return beta;
    }
    if alpha < stand_pat {
        alpha = stand_pat;
    }

    let mut mvs = match OrdMoves::new_ordered(&board) {
        Some(m) => m,
        None => return alpha,
    };
    // TODO en pass
    let mut new_board = Board::default();
    while let Some(m) = mvs.next_capture() {
        board.make_move(m, &mut new_board);
        let score = -quiescence(&new_board, -beta, -alpha);
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    alpha
}

pub fn think(board: &Board, depth: u8) -> Option<(ChessMove, i16)> {
    if depth == 0 {
        return None;
    }

    let now = Instant::now();
    let mvs = OrdMoves::new_ordered(&board)?;
    let mut new_board = Board::default();
    let mut best = None;
    let mut alpha = -i16::MAX;
    let beta = i16::MAX;
    for m in mvs {
        board.make_move(m, &mut new_board);
        let score = -negamax(&new_board, -beta, -alpha, depth - 1);
        if score >= beta {
            return Some((m, score));
        }
        if score > alpha {
            alpha = score;
            best = Some(m);
        }
    }
    println!("Elapsed time: {:.2?}", now.elapsed());
    best.map(|m| (m, alpha))
}

/*********    Testing    *********/
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn mate_in_1() {
        let board = Board::from_str("r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5Q2/PPPP1PPP/RNB1K1NR w KQkq - 0 1").unwrap();
        assert_eq!(think(&board, 1).map(|(m, _)| m), Some(ChessMove::new(Square::F3, Square::F7, None)));

        let board = Board::from_str("r3k2r/pbppqpb1/1pn3p1/7p/1N2pPn1/1PP4N/PB1P2PP/2QRKR2 b kq f3 0 1").unwrap();
        assert_eq!(think(&board, 1).map(|(m, _)| m), Some(ChessMove::new(Square::E4, Square::F3, None)));

        let board = Board::from_str("8/1R3P2/4r1kp/6p1/2p3P1/1b3p2/1B3P1P/6K1 w - - 0 1").unwrap();
        assert_eq!(think(&board, 1).map(|(m, _)| m), Some(ChessMove::new(Square::F7, Square::F8, Some(Piece::Knight))));
    }
    #[test]
    fn mate_in_2() {
        let mut board = Board::from_str("r2qb1rk/ppb2p1p/2n1pPp1/B3N3/2B1P2Q/2P2R2/1P4PP/7K w - - 0 1").unwrap();
        let mv = ChessMove::new(Square::H4, Square::H7, None);
        assert_eq!(think(&board, 3).map(|(m, _)| m), Some(mv));
        board = board.make_move_new(mv).make_move_new(ChessMove::new(Square::H8, Square::H7, None));
        assert_eq!(think(&board, 1).map(|(m, _)| m), Some(ChessMove::new(Square::F3, Square::H3, None)));
    }
    #[test]
    fn fork() {
        let mut board = Board::from_str("r1q1k2r/pbp3pp/4p3/p3pp2/2PP4/b1N1P1P1/P1Q1BPP1/1K1R3R w kq - 0 17").unwrap();
        let mv = ChessMove::new(Square::C2, Square::A4, None);
        assert_eq!(think(&board, 3).map(|(m, _)| m), Some(mv));
        board = board.make_move_new(mv).make_move_new(ChessMove::new(Square::E8, Square::F7, None));
        assert_eq!(think(&board, 1).map(|(m, _)| m), Some(ChessMove::new(Square::A4, Square::A3, None)));
    }
}
