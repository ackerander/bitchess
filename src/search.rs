use chess::*;

fn eval(board: &Board) -> i16 {
    let w_bits = board.color_combined(Color::White);
    let score_piece = |p: chess::Piece, p_score: i16| -> i16 {
        let pieces = board.pieces(p);
        let w_pieces = pieces & w_bits;
        p_score * (w_pieces.popcnt() as i16 - (pieces ^ w_pieces).popcnt() as i16)
    };
    let score = score_piece(Piece::Pawn, 10) + score_piece(Piece::Knight, 32) +
                score_piece(Piece::Bishop, 33) + score_piece(Piece::Rook, 50) +
                score_piece(Piece::Queen, 90);
    if board.side_to_move() == Color::White { score } else { -score }
}

fn negamax(board: &Board, mut alpha: i16, beta: i16, depth: u8) -> i16 {
    let mvs = MoveGen::new_legal(&board);
    if mvs.is_empty() {
        return if *(board.checkers()) == EMPTY { 0 } else { -3000 - depth as i16 };
    }

    if depth == 0 {
        return quiescence(&board, alpha, beta);
    }
    // TODO inc nodes
    // nodes += 1;

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
    // TODO inc nodes
    // nodes += 1;

    let stand_pat = eval(&board);
    if stand_pat >= beta {
        return beta;
    }
    if alpha < stand_pat {
        alpha = stand_pat;
    }

    let mut mvs = MoveGen::new_legal(&board);
    // TODO en pass
    // Potential speedup search only caputres
    mvs.remove_mask(!board.color_combined(!board.side_to_move()));
    let mut new_board = Board::default();
    for m in mvs {
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
    // TODO inc nodes
    // nodes += 1;

    let mvs = MoveGen::new_legal(&board);
    let mut new_board = Board::default();
    let mut best = None;
    let mut alpha = -i16::MAX;
    let beta = i16::MAX;
    for mv in mvs {
        board.make_move(mv, &mut new_board);
        let score = -negamax(&new_board, -beta, -alpha, depth - 1);
        if score >= beta {
            return Some((mv, score));
        }
        if score > alpha {
            alpha = score;
            best = Some(mv);
        }
    }
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

        let board = Board::from_str("6R1/5P1k/1p4Rp/p7/3N2p1/6P1/P1P4P/2K5 w - - 0 43").unwrap();
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
