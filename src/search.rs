use chess::*;
use std::cmp;

fn eval(board: &Board) -> i16 {
    let w_bits = board.color_combined(Color::White);
    let score_piece = |p: chess::Piece, p_score: i16| -> i16 {
        let pieces = board.pieces(p);
        let w_pieces = pieces & w_bits;
        p_score * (w_pieces.popcnt() - (pieces ^ w_pieces).popcnt()) as i16
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
        // return eval(&board);
    }
    // TODO inc nodes
    // nodes += 1;

    let mut new_board = Board::default();
    let mut score = i16::MIN;
    for m in mvs {
        board.make_move(m, &mut new_board);
        score = cmp::max(score, -negamax(&new_board, -beta, -alpha, depth - 1));
        alpha = cmp::max(score, alpha);
        if alpha >= beta {
            break;
        }
    }
    score
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
    mvs.remove_mask(!board.color_combined(!board.side_to_move()));
    let mut new_board = Board::default();
    for m in mvs {
        board.make_move(m, &mut new_board);
        let score = -quiescence(&board, -beta, -alpha);
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    alpha
}

pub fn think(board: &Board, mut alpha: i16, beta: i16, depth: u8) -> Option<(ChessMove, i16)> {
    if depth == 0 {
        return None;
    }
    // TODO inc nodes
    // nodes += 1;

    let mvs = MoveGen::new_legal(&board);
    let mut new_board = Board::default();
    let mut best = (None, i16::MIN);
    for m in mvs {
        board.make_move(m, &mut new_board);
        let eval = -negamax(&new_board, -beta, -alpha, depth - 1);
        if best.1 < eval {
            best = (Some(m), eval);
        }
        alpha = cmp::max(best.1, alpha);
        if alpha >= beta {
            break;
        }
    }
    best.0.map(|m| (m, best.1))
}