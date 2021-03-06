use super::lookup::*;
use super::pawn::*;
use super::slider::*;
use bb::{BB, EMPTY};
use piece::*;
use position::Position;
use side::Side;
use square::Square;

#[inline]
pub fn slider_non_diag_rays_to_squares(source: BB, attacker: BB, position: &Position) -> BB {
    let empty = position.bb_empty();

    pin_ray_non_diag(source, empty, attacker)
}

#[inline]
pub fn slider_diag_rays_to_squares(source: BB, attacker: BB, position: &Position) -> BB {
    let empty = position.bb_empty();

    pin_ray_diag(source, empty, attacker)
}

/// returns squares king may not move to
/// - removes king from occupied to handle attacking sliders correctly
pub fn king_danger_squares(king: BB, attacker: Side, position: &Position) -> BB {
    let occupied_without_king = position.bb_occupied() & !king;

    let mut attacked_squares = EMPTY;

    let (diag_attackers, non_diag_attackers) = position.bb_sliders(attacker);
    attacked_squares |= bishop_attacks(diag_attackers, occupied_without_king);
    attacked_squares |= rook_attacks(non_diag_attackers, occupied_without_king);

    let knights = position.bb_pc(KNIGHT.pc(attacker));
    attacked_squares |= knight_moves_from_bb(knights);

    let kings = position.bb_pc(KING.pc(attacker));
    debug_assert_eq!(kings.pop_count(), 1);
    attacked_squares |= king_moves_from_sq(kings.bitscan());

    let pawns = position.bb_pc(PAWN.pc(attacker));
    for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[attacker.to_usize()].iter() {
        let targets = pawns.rot_left(shift as u32) & file_mask;
        attacked_squares |= targets;
    }

    attacked_squares
}

#[allow(dead_code)]
pub fn attacked_squares_ignoring_ep(attacker: Side, position: &Position) -> BB {
    let occupied = position.bb_occupied();
    let mut attacked_squares = EMPTY;

    let (diag_attackers, non_diag_attackers) = position.bb_sliders(attacker);
    attacked_squares = attacked_squares | rook_attacks(non_diag_attackers, occupied);
    attacked_squares = attacked_squares | bishop_attacks(diag_attackers, occupied);

    let knights = position.bb_pc(KNIGHT.pc(attacker));
    attacked_squares |= knight_moves_from_bb(knights);

    let kings = position.bb_pc(KING.pc(attacker));
    debug_assert_eq!(kings.pop_count(), 1);
    attacked_squares |= king_moves_from_sq(kings.bitscan());

    let pawns = position.bb_pc(PAWN.pc(attacker));
    for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[attacker.to_usize()].iter() {
        let targets = pawns.rot_left(shift as u32) & file_mask;
        attacked_squares = attacked_squares | targets;
    }

    attacked_squares
}

pub fn checks_to_sq(sq: Square, attacker: Side, position: &Position) -> BB {
    let occupied = position.bb_occupied();

    let mut attackers = EMPTY;

    let knights = position.bb_pc(KNIGHT.pc(attacker));
    attackers |= knight_moves_from_sq(sq) & knights;

    let pawns = position.bb_pc(PAWN.pc(attacker));
    let sq_bb = BB::new(sq);
    for &(shift, file_mask) in PAWN_CAPTURE_FILE_MASKS[attacker.flip().to_usize()].iter() {
        attackers |= sq_bb.rot_left(shift as u32) & file_mask & pawns;
    }

    let (diag_attackers, non_diag_attackers) = position.bb_sliders(attacker);

    attackers |= rook_attacks_from_sq(sq, occupied) & non_diag_attackers;
    attackers |= bishop_attacks_from_sq(sq, occupied) & diag_attackers;

    attackers
}

#[cfg(test)]
mod test {
    use super::*;
    use bb::*;
    use side::*;
    use square::*;
    use unindent;

    #[test]
    fn test_checks_to_sq() {
        let position =
            &Position::from_fen("rnbqkbnr/pppppp1p/8/8/8/8/PPPPPPPP/RNB1KBNR w").unwrap();
        let attacks = checks_to_sq(C6, BLACK, position);

        let expected = unindent::unindent(
            "
              ABCDEFGH
            8|.#......|8
            7|.#.#....|7
            6|........|6
            5|........|5
            4|........|4
            3|........|3
            2|........|2
            1|........|1
              ABCDEFGH
            ",
        );
        assert_eq!(attacks.to_string(), expected);
    }

    #[test]
    fn test_slider_rays_to_square() {
        let position =
            &Position::from_fen("rnbqk1nr/pppppppp/8/6b2/8/8/PPPPPPPP/RNBQKBNR w").unwrap();
        let attacks = slider_diag_rays_to_squares(BB::new(D2), BB::new(G5), position);

        let expected = unindent::unindent(
            "
              ABCDEFGH
            8|........|8
            7|........|7
            6|........|6
            5|........|5
            4|.....#..|4
            3|....#...|3
            2|........|2
            1|........|1
              ABCDEFGH
            ",
        );

        assert_eq!(attacks.to_string(), expected);
    }
}
