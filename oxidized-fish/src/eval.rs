use crate::board::Board;
use crate::types::{Color, PieceType};
use crate::tables::{MG_PST, EG_PST, MG_VALUE, EG_VALUE};

// Bonus/Penalty constants
const MG_ROOK_OPEN: i32 = 45;
const MG_ROOK_SEMI: i32 = 20;
const EG_ROOK_OPEN: i32 = 25;
const EG_ROOK_SEMI: i32 = 10;

const MG_PAWN_ISO: i32 = -10;
const EG_PAWN_ISO: i32 = -20;
const MG_PAWN_DOUBLED: i32 = -15;
const EG_PAWN_DOUBLED: i32 = -30;

const MG_BISHOP_PAIR: i32 = 30;
const EG_BISHOP_PAIR: i32 = 50;

const MG_ROOK_7TH: i32 = 25;
const EG_ROOK_7TH: i32 = 40;

const MG_KNIGHT_OUTPOST: i32 = 20;
const EG_KNIGHT_OUTPOST: i32 = 15;

const FILE_BB: [u64; 8] = [
    0x0101010101010101, 0x0202020202020202, 0x0404040404040404, 0x0808080808080808,
    0x1010101010101010, 0x2020202020202020, 0x4040404040404040, 0x8080808080808080
];

pub fn evaluate(board: &Board) -> i32 {
    let mut mg = [0; 2];
    let mut eg = [0; 2];
    let mut game_phase = 0;
    let mut mobility = [0; 2];
    let mut king_safety = [0; 2];
    let additional = [0; 2];
    
    let occ = board.occupied();
    const PHASE_VALUES: [i32; 6] = [0, 1, 1, 2, 4, 0];

    // Pre-calculate king zones for safety evaluation
    let king_sq = [
        (board.by_type[PieceType::King as usize] & board.by_color[0]).trailing_zeros() as u8,
        (board.by_type[PieceType::King as usize] & board.by_color[1]).trailing_zeros() as u8,
    ];
    let king_zone = [
        if king_sq[0] < 64 { crate::tables::ATTACKS.king[king_sq[0] as usize] } else { 0 },
        if king_sq[1] < 64 { crate::tables::ATTACKS.king[king_sq[1] as usize] } else { 0 },
    ];

    let pawns = [
        board.by_type[PieceType::Pawn as usize] & board.by_color[0],
        board.by_type[PieceType::Pawn as usize] & board.by_color[1],
    ];

    for c in 0..2 {
        let us = if c == 0 { Color::White } else { Color::Black };
        let them = us.opponent();
        let friends = board.by_color[c];
        let my_pawns = pawns[c];
        let enemy_pawns = pawns[1 - c];

        // Bishop pair bonus
        if (board.by_type[PieceType::Bishop as usize] & friends).count_ones() >= 2 {
            mg[c] += MG_BISHOP_PAIR;
            eg[c] += EG_BISHOP_PAIR;
        }

        for pt in 0..6 {
            let mut bb = board.by_type[pt] & friends;
            while bb != 0 {
                let sq = bb.trailing_zeros() as u8;
                let file = (sq % 8) as usize;
                let rank = sq / 8;
                let p_idx = if c == 0 { sq ^ 56 } else { sq };
                
                mg[c] += MG_VALUE[pt] + MG_PST[pt][p_idx as usize];
                eg[c] += EG_VALUE[pt] + EG_PST[pt][p_idx as usize];
                game_phase += PHASE_VALUES[pt];
                
                // Specific Piece Evaluation
                match pt {
                    0 => { // Pawn
                        // Doubled
                        if (my_pawns & FILE_BB[file] & !(1u64 << sq)) != 0 {
                            mg[c] += MG_PAWN_DOUBLED;
                            eg[c] += EG_PAWN_DOUBLED;
                        }
                        // Isolated
                        let left_file = if file > 0 { FILE_BB[file - 1] } else { 0 };
                        let right_file = if file < 7 { FILE_BB[file + 1] } else { 0 };
                        if (my_pawns & (left_file | right_file)) == 0 {
                            mg[c] += MG_PAWN_ISO;
                            eg[c] += EG_PAWN_ISO;
                        }
                        // Passed
                        if is_passed_pawn(sq, us, enemy_pawns) {
                            let r = if c == 0 { rank } else { 7 - rank };
                            eg[c] += (r as i32 * r as i32) * 5; // Quadratic bonus
                        }
                    },
                    1 => { // Knight
                        let attacks = crate::tables::ATTACKS.knight[sq as usize];
                        mobility[c] += (attacks & !friends & !enemy_pawns).count_ones() as i32 * 4;
                        let king_attacks = (attacks & king_zone[1 - c]).count_ones() as i32;
                        king_safety[c] += king_attacks * king_attacks * 10;

                        // Outpost
                        let r = if c == 0 { rank } else { 7 - rank };
                        if r >= 3 && r <= 5 {
                            let support = crate::tables::ATTACKS.pawn[them as usize][sq as usize] & my_pawns;
                            if support != 0 {
                                mg[c] += MG_KNIGHT_OUTPOST;
                                eg[c] += EG_KNIGHT_OUTPOST;
                            }
                        }
                    },
                    2 => { // Bishop
                        let attacks = board.get_bishop_attacks(sq, occ);
                        mobility[c] += (attacks & !friends & !enemy_pawns).count_ones() as i32 * 3;
                        let king_attacks = (attacks & king_zone[1 - c]).count_ones() as i32;
                        king_safety[c] += king_attacks * king_attacks * 10;
                    },
                    3 => { // Rook
                        let attacks = board.get_rook_attacks(sq, occ);
                        mobility[c] += (attacks & !friends).count_ones() as i32 * 2;
                        let king_attacks = (attacks & king_zone[1 - c]).count_ones() as i32;
                        king_safety[c] += king_attacks * king_attacks * 10;

                        // Open Files
                        let file_mask = FILE_BB[file];
                        if (my_pawns & file_mask) == 0 {
                            if (enemy_pawns & file_mask) == 0 {
                                mg[c] += MG_ROOK_OPEN;
                                eg[c] += EG_ROOK_OPEN;
                            } else {
                                mg[c] += MG_ROOK_SEMI;
                                eg[c] += EG_ROOK_SEMI;
                            }
                        }

                        // 7th Rank
                        let r = if c == 0 { rank } else { 7 - rank };
                        if r == 6 {
                            mg[c] += MG_ROOK_7TH;
                            eg[c] += EG_ROOK_7TH;
                        }
                    },
                    4 => { // Queen
                        let attacks = board.get_bishop_attacks(sq, occ) | board.get_rook_attacks(sq, occ);
                        mobility[c] += (attacks & !friends).count_ones() as i32 * 1;
                        let king_attacks = (attacks & king_zone[1 - c]).count_ones() as i32;
                        king_safety[c] += king_attacks * king_attacks * 10;
                    },
                    _ => {}
                }
                
                bb &= bb - 1;
            }
        }
    }

    let mg_score = (mg[0] + mobility[0] + king_safety[0] + additional[0]) - (mg[1] + mobility[1] + king_safety[1] + additional[1]);
    let eg_score = (eg[0] + mobility[0] + additional[0]) - (eg[1] + mobility[1] + additional[1]);

    let mg_phase = game_phase.min(24);
    let eg_phase = 24 - mg_phase;

    let score = (mg_score * mg_phase + eg_score * eg_phase) / 24;

    if board.side_to_move == Color::White {
        score
    } else {
        -score
    }
}

fn is_passed_pawn(sq: u8, color: Color, enemy_pawns: u64) -> bool {
    let file = (sq % 8) as i16;
    let rank = (sq / 8) as i16;
    let mut mask = 0u64;
    
    for f in (file - 1).max(0)..=(file + 1).min(7) {
        if color == Color::White {
            for r in (rank + 1)..8 {
                mask |= 1u64 << (r * 8 + f);
            }
        } else {
            for r in 0..rank {
                mask |= 1u64 << (r * 8 + f);
            }
        }
    }
    (mask & enemy_pawns) == 0
}
