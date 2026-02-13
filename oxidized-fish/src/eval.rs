use crate::board::Board;
use crate::tables::{EG_PST, EG_VALUE, MG_PST, MG_VALUE};
use crate::types::{Color, PieceType, CONTEMPT};

const MG_ROOK_OPEN: i32 = 55;
const MG_ROOK_SEMI: i32 = 28;
const EG_ROOK_OPEN: i32 = 25;
const EG_ROOK_SEMI: i32 = 14;

const MG_PAWN_ISO: i32 = -20;
const EG_PAWN_ISO: i32 = -32;
const MG_PAWN_DOUBLED: i32 = -28;
const EG_PAWN_DOUBLED: i32 = -50;
const MG_PAWN_BACKWARD: i32 = -18;
const EG_PAWN_BACKWARD: i32 = -24;

const MG_BISHOP_PAIR: i32 = 45;
const EG_BISHOP_PAIR: i32 = 75;

const MG_ROOK_7TH: i32 = 40;
const EG_ROOK_7TH: i32 = 60;

const MG_KNIGHT_OUTPOST: i32 = 32;
const EG_KNIGHT_OUTPOST: i32 = 22;

const MG_TEMPO: i32 = 18;
const MG_KING_SHIELD: i32 = 18;

const FILE_BB: [u64; 8] = [
    0x0101010101010101,
    0x0202020202020202,
    0x0404040404040404,
    0x0808080808080808,
    0x1010101010101010,
    0x2020202020202020,
    0x4040404040404040,
    0x8080808080808080,
];

const RANK_BB: [u64; 8] = [
    0xFF,
    0xFF00,
    0xFF0000,
    0xFF000000,
    0xFF00000000,
    0xFF0000000000,
    0xFF000000000000,
    0xFF00000000000000,
];

const CENTER_BB: u64 = 0x0000001818000000;
const EXTENDED_CENTER_BB: u64 = 0x00003C3C3C3C0000;

pub fn evaluate(board: &Board) -> i32 {
    let mut mg = [0; 2];
    let mut eg = [0; 2];
    let mut game_phase = 0;
    let mut mobility = [0; 2];
    let mut king_safety_bonus = [0; 2];
    let mut king_danger = [0; 2];
    let mut piece_threats = [0; 2];
    let mut center_control = [0; 2];

    let occ = board.occupied();
    const PHASE_VALUES: [i32; 6] = [0, 1, 1, 2, 4, 0];
    const PIECE_VALUES: [i32; 6] = [100, 325, 335, 500, 915, 20000];

    let king_sq = [
        (board.by_type[PieceType::King as usize] & board.by_color[0]).trailing_zeros() as u8,
        (board.by_type[PieceType::King as usize] & board.by_color[1]).trailing_zeros() as u8,
    ];

    let king_ring = [
        if king_sq[0] < 64 {
            get_king_ring(king_sq[0])
        } else {
            0
        },
        if king_sq[1] < 64 {
            get_king_ring(king_sq[1])
        } else {
            0
        },
    ];

    let pawns = [
        board.by_type[PieceType::Pawn as usize] & board.by_color[0],
        board.by_type[PieceType::Pawn as usize] & board.by_color[1],
    ];

    for c in 0..2 {
        let us = if c == 0 { Color::White } else { Color::Black };
        let them = us.opponent();
        let friends = board.by_color[c];
        let enemies = board.by_color[them as usize];
        let my_pawns = pawns[c];
        let enemy_pawns = pawns[them as usize];

        if (board.by_type[PieceType::Bishop as usize] & friends).count_ones() >= 2 {
            mg[c] += MG_BISHOP_PAIR;
            eg[c] += EG_BISHOP_PAIR;
        }

        let mut minor_count = 0;

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

                // Dev Penalty
                if (pt == 1 || pt == 2) && (if c == 0 { rank == 0 } else { rank == 7 }) {
                    mg[c] -= 35;
                }

                // Center
                let bit = 1u64 << sq;
                if (bit & CENTER_BB) != 0 {
                    center_control[c] += 40;
                } else if (bit & EXTENDED_CENTER_BB) != 0 {
                    center_control[c] += 15;
                }

                let piece_attacks = match pt {
                    0 => crate::tables::ATTACKS.pawn[c][sq as usize],
                    1 => crate::tables::ATTACKS.knight[sq as usize],
                    2 => board.get_bishop_attacks(sq, occ),
                    3 => board.get_rook_attacks(sq, occ),
                    4 => board.get_bishop_attacks(sq, occ) | board.get_rook_attacks(sq, occ),
                    5 => crate::tables::ATTACKS.king[sq as usize],
                    _ => 0,
                };

                center_control[c] += (piece_attacks & CENTER_BB).count_ones() as i32 * 12;

                // Threat detection
                let attackers = board.get_attackers(sq, occ) & enemies;
                if attackers != 0 {
                    let defenders = board.get_attackers(sq, occ) & friends;
                    if (attackers & enemy_pawns) != 0 || defenders == 0 {
                        piece_threats[c] -= PIECE_VALUES[pt] / 2;
                    } else {
                        for enemy_pt in 0..pt {
                            if (attackers & board.by_type[enemy_pt]) != 0 {
                                piece_threats[c] -= PIECE_VALUES[pt] / 4;
                                break;
                            }
                        }
                    }
                }

                match pt {
                    0 => {
                        if (my_pawns & FILE_BB[file] & !(1u64 << sq)) != 0 {
                            mg[c] += MG_PAWN_DOUBLED;
                            eg[c] += EG_PAWN_DOUBLED;
                        }
                        let left_file = if file > 0 { FILE_BB[file - 1] } else { 0 };
                        let right_file = if file < 7 { FILE_BB[file + 1] } else { 0 };
                        if (my_pawns & (left_file | right_file)) == 0 {
                            mg[c] += MG_PAWN_ISO;
                            eg[c] += EG_PAWN_ISO;
                        }
                        if is_passed_pawn(sq, us, enemy_pawns) {
                            let r = if c == 0 { rank } else { 7 - rank };
                            mg[c] += (r as i32 * r as i32) * 10;
                            eg[c] += (r as i32 * r as i32) * 10;
                        }
                        if is_backward_pawn(sq, us, my_pawns, enemy_pawns) {
                            mg[c] += MG_PAWN_BACKWARD;
                            eg[c] += EG_PAWN_BACKWARD;
                        }
                        if is_connected_pawn(sq, us, my_pawns) {
                            let r = if c == 0 { rank } else { 7 - rank };
                            mg[c] += (r as i32) * 6;
                        }
                    }
                    1 => {
                        minor_count += 1;
                        mobility[c] +=
                            (piece_attacks & !friends & !enemy_pawns).count_ones() as i32 * 8;
                        king_danger[c] +=
                            (piece_attacks & king_ring[them as usize]).count_ones() as i32 * 30; // Increased from 25
                        if (if c == 0 {
                            rank >= 3 && rank <= 5
                        } else {
                            rank >= 2 && rank <= 4
                        }) {
                            if (crate::tables::ATTACKS.pawn[them as usize][sq as usize] & my_pawns)
                                != 0
                            {
                                mg[c] += MG_KNIGHT_OUTPOST;
                                eg[c] += EG_KNIGHT_OUTPOST;
                            }
                        }
                    }
                    2 => {
                        minor_count += 1;
                        mobility[c] +=
                            (piece_attacks & !friends & !enemy_pawns).count_ones() as i32 * 7;
                        king_danger[c] +=
                            (piece_attacks & king_ring[them as usize]).count_ones() as i32 * 25;
                        // Increased from 18
                    }
                    3 => {
                        mobility[c] += (piece_attacks & !friends).count_ones() as i32 * 6;
                        king_danger[c] +=
                            (piece_attacks & king_ring[them as usize]).count_ones() as i32 * 25; // Increased from 18
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
                        let r = if c == 0 { rank } else { 7 - rank };
                        if r == 6 {
                            mg[c] += MG_ROOK_7TH;
                            eg[c] += EG_ROOK_7TH;
                        }
                        if (FILE_BB[king_sq[them as usize] as usize % 8] & file_mask) != 0 {
                            king_danger[c] += 20; // Increased from 15
                        }
                    }
                    4 => {
                        mobility[c] += (piece_attacks & !friends).count_ones() as i32 * 4;
                        king_danger[c] +=
                            (piece_attacks & king_ring[them as usize]).count_ones() as i32 * 45;
                        // Increased from 35
                    }
                    5 => {
                        let r = if c == 0 { rank } else { 7 - rank };
                        if r <= 1 {
                            let shield_rank = if c == 0 { RANK_BB[1] } else { RANK_BB[6] };
                            let shield = shield_rank
                                & (FILE_BB[file]
                                    | (if file > 0 { FILE_BB[file - 1] } else { 0 })
                                    | (if file < 7 { FILE_BB[file + 1] } else { 0 }))
                                & my_pawns;
                            king_safety_bonus[c] += (shield.count_ones() as i32) * MG_KING_SHIELD;
                            if (FILE_BB[file] & my_pawns) == 0 {
                                king_danger[them as usize] += 25;
                            }
                        }
                    }
                    _ => {}
                }
                bb &= bb - 1;
            }
        }

        if minor_count > 1 && game_phase < 15 {
            mg[c] += 20;
        }
    }

    let total_mg = [
        mg[0] + mobility[0] + king_safety_bonus[0] + piece_threats[0] + center_control[0]
            - (king_danger[1] * king_danger[1] / 50),
        mg[1] + mobility[1] + king_safety_bonus[1] + piece_threats[1] + center_control[1]
            - (king_danger[0] * king_danger[0] / 50),
    ];

    let total_eg = [
        eg[0] + mobility[0] + piece_threats[0] / 2 + center_control[0] / 3,
        eg[1] + mobility[1] + piece_threats[1] / 2 + center_control[1] / 3,
    ];

    let mg_phase = game_phase.min(24);
    let eg_phase = 24 - mg_phase;

    let mut score =
        ((total_mg[0] - total_mg[1]) * mg_phase + (total_eg[0] - total_eg[1]) * eg_phase) / 24;

    if board.side_to_move == Color::White {
        score += MG_TEMPO;
    } else {
        score -= MG_TEMPO;
    }

    let contempt = CONTEMPT.load(std::sync::atomic::Ordering::Relaxed);
    if board.side_to_move == Color::White {
        score += contempt;
    } else {
        score -= contempt;
    }

    if board.side_to_move == Color::White {
        score
    } else {
        -score
    }
}

fn get_king_ring(sq: u8) -> u64 {
    let mut ring = 0u64;
    let file = (sq % 8) as i16;
    let rank = (sq / 8) as i16;
    for r in (rank - 1)..=(rank + 1) {
        for f in (file - 1)..=(file + 1) {
            if r >= 0 && r < 8 && f >= 0 && f < 8 {
                ring |= 1u64 << (r * 8 + f);
            }
        }
    }
    let forward = if sq / 8 < 4 { 1 } else { -1 };
    let nr = rank + forward * 2;
    if nr >= 0 && nr < 8 {
        for f in (file - 1)..=(file + 1) {
            if f >= 0 && f < 8 {
                ring |= 1u64 << (nr * 8 + f);
            }
        }
    }
    ring
}

pub fn is_passed_pawn(sq: u8, color: Color, enemy_pawns: u64) -> bool {
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

fn is_backward_pawn(sq: u8, color: Color, my_pawns: u64, enemy_pawns: u64) -> bool {
    let file = (sq % 8) as i16;
    let rank = (sq / 8) as i16;
    let stop_sq = if color == Color::White {
        if rank >= 7 {
            return false;
        }
        sq + 8
    } else {
        if rank <= 0 {
            return false;
        }
        sq - 8
    };
    let mut neighbors = 0u64;
    for f in (file - 1).max(0)..=(file + 1).min(7) {
        if f == file {
            continue;
        }
        if color == Color::White {
            for r in 0..=rank {
                neighbors |= 1u64 << (r * 8 + f);
            }
        } else {
            for r in rank..8 {
                neighbors |= 1u64 << (r * 8 + f);
            }
        }
    }
    if (my_pawns & neighbors) == 0 {
        let enemy_attacks =
            crate::tables::ATTACKS.pawn[color.opponent() as usize][stop_sq as usize];
        if (enemy_pawns & enemy_attacks) != 0 {
            return true;
        }
    }
    false
}

fn is_connected_pawn(sq: u8, color: Color, my_pawns: u64) -> bool {
    let file = (sq % 8) as i16;
    let rank = (sq / 8) as i16;
    let mut neighbors = 0u64;
    for f in (file - 1).max(0)..=(file + 1).min(7) {
        if f == file {
            continue;
        }
        for r in (rank - 1).max(0)..=(rank + 1).min(7) {
            neighbors |= 1u64 << (r * 8 + f);
        }
    }
    (my_pawns & neighbors) != 0
}
