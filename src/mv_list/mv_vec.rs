use mv::Move;
use square::Square;
use bb::{BB, END_ROWS};
use castle::Castle;
use piece::*;
use std::fmt;
use std;
use mv_list::MoveList;

/// MoveCounter implements MoveList and collects moves in a vector. Use `iter` to access the moves once they have been added.
#[derive(Clone)]
pub struct MoveVec {
    moves: Vec<Move>,
}

impl fmt::Display for MoveVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for MoveVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl MoveList for MoveVec {
    fn add_moves(&mut self, from: Square, targets: BB, enemy: BB) {
        self.insert_moves(from, targets & (!enemy), Move::new_push);
        self.insert_moves(from, targets & enemy, Move::new_capture);
    }

    fn add_castle(&mut self, castle: Castle) {
        self.moves.push(Move::new_castle(castle));
    }

    fn add_pawn_ep_capture(&mut self, from: Square, to: Square) {
        self.moves.push(Move::new_ep_capture(from, to));
    }

    fn add_pawn_pushes(&mut self, shift: usize, targets: BB) {
        self.insert_promos_by_shift(shift, (targets & END_ROWS), Move::new_promotion);
        self.insert_moves_by_shift(shift, (targets & !END_ROWS), Move::new_push);
    }

    fn add_pawn_captures(&mut self, shift: usize, targets: BB) {
        self.insert_promos_by_shift(shift, (targets & END_ROWS), Move::new_capture_promotion);
        self.insert_moves_by_shift(shift, (targets & !END_ROWS), Move::new_capture);
    }
}

impl MoveVec {
    pub fn new() -> MoveVec {
        MoveVec { moves: Vec::new() }
    }

    pub fn to_string(&self) -> String {
        self.iter().map(|mv: &Move| mv.to_string()).collect::<Vec<String>>().join(",")
    }


    pub fn iter(&self) -> std::slice::Iter<Move> {
        self.moves.iter()
    }

    pub fn at(&self, idx: usize) -> Move {
        self.moves[idx]
    }

    pub fn truncate(&mut self, len: usize) {
        self.moves.truncate(len);
    }

    fn insert_moves<F: Fn(Square, Square) -> Move>(&mut self, from: Square, targets: BB, f: F) {
        for (to, _) in targets.iter() {
            self.moves.push(f(from, to));
        }
    }

    fn insert_moves_by_shift<F: Fn(Square, Square) -> Move>(&mut self,
                                                            shift: usize,
                                                            targets: BB,
                                                            f: F) {
        for (to, _) in targets.iter() {
            let from = to.rotate_right(shift);
            self.moves.push(f(from, to));
        }
    }

    pub fn len(&self) -> usize {
        self.moves.len()
    }

    fn insert_promos_by_shift<F: Fn(Square, Square, Kind) -> Move>(&mut self,
                                                                   shift: usize,
                                                                   targets: BB,
                                                                   f: F) {
        for (to, _) in targets.iter() {
            let from = to.rotate_right(shift);
            self.moves.push(f(from, to, QUEEN));
            self.moves.push(f(from, to, KNIGHT));
            self.moves.push(f(from, to, BISHOP));
            self.moves.push(f(from, to, ROOK));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use board::*;
    use gen::*;

    #[test]
    fn test_move_vec() {
        let board = &Board::from_fen(STARTING_POSITION_FEN).unwrap();
        let mut list = MoveVec::new();

        legal_moves(&board, &mut list);

        assert_eq!(list.len(), 20);
    }
}