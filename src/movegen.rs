use crate::bitboard::Bitboard;
use crate::board::Board;
use crate::types::{Color, Move, PieceType};

pub struct MoveGen;

impl MoveGen {
    pub fn generate(board: &Board) -> Vec<Move> {
        let mut moves = Vec::with_capacity(256);
        let us = board.side_to_move;
        let occ = board.occupied();
        let friends = board.by_color[us as usize];
        let enemies = board.by_color[us.opponent() as usize];

        // King moves
        let king_sq = (board.by_type[PieceType::King as usize] & friends).trailing_zeros() as u8;
        if king_sq < 64 {
            let mut king_moves = Bitboard(crate::tables::ATTACKS.king[king_sq as usize] & !friends);
            while let Some(to) = king_moves.next() {
                let is_capture = (enemies & (1u64 << to)) != 0;
                moves.push(Move::new(king_sq, to, if is_capture { Move::CAPTURE } else { Move::QUIET }));
            }
            MoveGen::generate_castling_moves(board, king_sq, &mut moves);
        }

        // Knight moves
        let mut knights = Bitboard(board.by_type[PieceType::Knight as usize] & friends);
        while let Some(from) = knights.next() {
            let mut knight_moves = Bitboard(crate::tables::ATTACKS.knight[from as usize] & !friends);
            while let Some(to) = knight_moves.next() {
                let is_capture = (enemies & (1u64 << to)) != 0;
                moves.push(Move::new(from, to, if is_capture { Move::CAPTURE } else { Move::QUIET }));
            }
        }

        // Pawn moves
        MoveGen::generate_pawn_moves(board, us, &mut moves);

        // Slider moves
        let rooks = board.by_type[PieceType::Rook as usize] & friends;
        let bishops = board.by_type[PieceType::Bishop as usize] & friends;
        let queens = board.by_type[PieceType::Queen as usize] & friends;

        let mut r_iter = Bitboard(rooks | queens);
        while let Some(from) = r_iter.next() {
            let mut rook_moves = Bitboard(board.get_rook_attacks(from, occ) & !friends);
            while let Some(to) = rook_moves.next() {
                let is_capture = (enemies & (1u64 << to)) != 0;
                moves.push(Move::new(from, to, if is_capture { Move::CAPTURE } else { Move::QUIET }));
            }
        }

        let mut b_iter = Bitboard(bishops | queens);
        while let Some(from) = b_iter.next() {
            let mut bishop_moves = Bitboard(board.get_bishop_attacks(from, occ) & !friends);
            while let Some(to) = bishop_moves.next() {
                let is_capture = (enemies & (1u64 << to)) != 0;
                moves.push(Move::new(from, to, if is_capture { Move::CAPTURE } else { Move::QUIET }));
            }
        }

        // Legal move filtering
        moves.retain(|m| {
            let next_board = board.make_move(*m);
            let us_idx = us as usize;
            let next_king_sq = (next_board.by_type[PieceType::King as usize]
                & next_board.by_color[us_idx])
                .trailing_zeros() as u8;
            if next_king_sq == 64 { return false; }
            !next_board.is_square_attacked(next_king_sq, next_board.side_to_move)
        });
        moves
    }

    fn generate_pawn_moves(board: &Board, us: Color, moves: &mut Vec<Move>) {
        let occ = board.occupied();
        let enemies = board.by_color[us.opponent() as usize];
        let pawns = board.by_type[PieceType::Pawn as usize] & board.by_color[us as usize];
        let mut iter = Bitboard(pawns);

        let (up, start_rank, promo_rank) = if us == Color::White {
            (8i16, 1, 7)
        } else {
            (-8i16, 6, 0)
        };

        while let Some(sq) = iter.next() {
            let rank = sq / 8;
            
            // Single push
            let to = (sq as i16 + up) as u8;
            if (occ & (1u64 << to)) == 0 {
                if rank == (promo_rank as i16 - (up / up.abs())) as u8 {
                    for f in &[Move::PROMOTION | 0, Move::PROMOTION | 1, Move::PROMOTION | 2, Move::PROMOTION | 3] {
                        moves.push(Move::new(sq, to, *f));
                    }
                } else {
                    moves.push(Move::new(sq, to, Move::QUIET));
                    // Double push
                    if rank == start_rank {
                        let to2 = (to as i16 + up) as u8;
                        if (occ & (1u64 << to2)) == 0 {
                            moves.push(Move::new(sq, to2, Move::DOUBLE_PAWN_PUSH));
                        }
                    }
                }
            }

            // Captures
            let mut attacks = Bitboard(crate::tables::ATTACKS.pawn[us as usize][sq as usize] & enemies);
            while let Some(cap_to) = attacks.next() {
                if cap_to / 8 == promo_rank {
                    for f in &[Move::PROMOTION | Move::CAPTURE | 0, Move::PROMOTION | Move::CAPTURE | 1, Move::PROMOTION | Move::CAPTURE | 2, Move::PROMOTION | Move::CAPTURE | 3] {
                        moves.push(Move::new(sq, cap_to, *f));
                    }
                } else {
                    moves.push(Move::new(sq, cap_to, Move::CAPTURE));
                }
            }

            // EP
            if let Some(ep_sq) = board.ep_square {
                if (crate::tables::ATTACKS.pawn[us as usize][sq as usize] & (1u64 << ep_sq)) != 0 {
                    moves.push(Move::new(sq, ep_sq, Move::EP_CAPTURE));
                }
            }
        }
    }

    fn generate_castling_moves(board: &Board, king_sq: u8, moves: &mut Vec<Move>) {
        let us = board.side_to_move;
        let them = us.opponent();
        let occ = board.occupied();
        let (k_mask, q_mask) = if us == Color::White { (1, 2) } else { (4, 8) };

        if (board.castling_rights & k_mask) != 0 {
            let (f1, g1) = if us == Color::White { (5, 6) } else { (61, 62) };
            if (occ & ((1u64 << f1) | (1u64 << g1))) == 0 {
                if !board.is_square_attacked(king_sq, them)
                    && !board.is_square_attacked(f1, them)
                    && !board.is_square_attacked(g1, them)
                {
                    moves.push(Move::new(king_sq, g1, Move::K_CASTLE));
                }
            }
        }
        if (board.castling_rights & q_mask) != 0 {
            let (d1, c1, b1) = if us == Color::White { (3, 2, 1) } else { (59, 58, 57) };
            if (occ & ((1u64 << d1) | (1u64 << c1) | (1u64 << b1))) == 0 {
                if !board.is_square_attacked(king_sq, them)
                    && !board.is_square_attacked(d1, them)
                    && !board.is_square_attacked(c1, them)
                {
                    moves.push(Move::new(king_sq, c1, Move::Q_CASTLE));
                }
            }
        }
    }
}
