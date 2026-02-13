use crate::types::{Color, Move, PieceType};
use crate::zobrist::ZOBRIST;

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub by_color: [u64; 2], // 0: White, 1: Black
    pub by_type: [u64; 6],  // P, N, B, R, Q, K
    pub side_to_move: Color,
    pub castling_rights: u8, // 1: WK, 2: WQ, 4: BK, 8: BQ
    pub ep_square: Option<u8>,
    #[allow(dead_code)]
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
    pub hash: u64,
    pub pins: u64, // Pins for the side to move
}

impl Board {
    pub fn new() -> Self {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn from_fen(fen: &str) -> Self {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        let mut board = Board {
            by_color: [0; 2],
            by_type: [0; 6],
            side_to_move: Color::White,
            castling_rights: 0,
            ep_square: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            hash: 0,
            pins: 0,
        };

        if parts.is_empty() {
            board.hash = board.calculate_hash();
            board.pins = board.calculate_pins();
            return board;
        }

        let rows: Vec<&str> = parts[0].split('/').collect();
        for (r, row) in rows.iter().enumerate() {
            let rank = 7 - r;
            let mut file = 0;
            for c in row.chars() {
                if let Some(digit) = c.to_digit(10) {
                    file += digit as usize;
                } else {
                    let piece_type = match c.to_ascii_lowercase() {
                        'p' => PieceType::Pawn,
                        'n' => PieceType::Knight,
                        'b' => PieceType::Bishop,
                        'r' => PieceType::Rook,
                        'q' => PieceType::Queen,
                        'k' => PieceType::King,
                        _ => PieceType::Pawn,
                    };
                    let color = if c.is_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };
                    if file < 8 {
                        let sq = rank * 8 + file;
                        board.by_color[color as usize] |= 1u64 << sq;
                        board.by_type[piece_type as usize] |= 1u64 << sq;
                    }
                    file += 1;
                }
            }
        }

        if parts.len() > 1 && parts[1] == "b" {
            board.side_to_move = Color::Black;
        }

        if parts.len() > 2 && parts[2] != "-" {
            for c in parts[2].chars() {
                match c {
                    'K' => board.castling_rights |= 1,
                    'Q' => board.castling_rights |= 2,
                    'k' => board.castling_rights |= 4,
                    'q' => board.castling_rights |= 8,
                    _ => {}
                }
            }
        }

        if parts.len() > 3 && parts[3] != "-" {
            let s = parts[3];
            if s.len() == 2 {
                let bytes = s.as_bytes();
                let file = bytes[0] - b'a';
                let rank = bytes[1] - b'1';
                if file < 8 && rank < 8 {
                    board.ep_square = Some(rank * 8 + file);
                }
            }
        }

        if parts.len() > 4 {
            board.halfmove_clock = parts[4].parse().unwrap_or(0);
        }

        if parts.len() > 5 {
            board.fullmove_number = parts[5].parse().unwrap_or(1);
        }

        board.hash = board.calculate_hash();
        board.pins = board.calculate_pins();
        board
    }

    pub fn calculate_hash(&self) -> u64 {
        let mut h = 0u64;
        for c in 0..2 {
            for pt in 0..6 {
                let mut bb = self.by_type[pt] & self.by_color[c];
                while bb != 0 {
                    let sq = bb.trailing_zeros() as usize;
                    h ^= ZOBRIST.pieces[c][pt][sq];
                    bb &= bb - 1;
                }
            }
        }
        if self.side_to_move == Color::Black {
            h ^= ZOBRIST.side;
        }
        h ^= ZOBRIST.castling[self.castling_rights as usize];
        if let Some(sq) = self.ep_square {
            h ^= ZOBRIST.ep[sq as usize];
        }
        h
    }

    pub fn calculate_pins(&self) -> u64 {
        let mut pins = 0u64;
        let us = self.side_to_move;
        let them = us.opponent();
        let occ = self.occupied();

        let king_bit = self.by_type[PieceType::King as usize] & self.by_color[us as usize];
        if king_bit == 0 {
            return 0;
        }
        let king_sq = king_bit.trailing_zeros() as u8;

        let rooks = (self.by_type[PieceType::Rook as usize]
            | self.by_type[PieceType::Queen as usize])
            & self.by_color[them as usize];
        let mut bb = rooks;
        while bb != 0 {
            let sq = bb.trailing_zeros() as u8;
            let line = rook_line(king_sq, sq);
            if line != 0 {
                let pieces_on_line = line & occ;
                if pieces_on_line.count_ones() == 3 {
                    let us_pieces = pieces_on_line & self.by_color[us as usize];
                    if us_pieces.count_ones() == 2 {
                        pins |= us_pieces ^ (1u64 << king_sq);
                    }
                }
            }
            bb &= bb - 1;
        }

        let bishops = (self.by_type[PieceType::Bishop as usize]
            | self.by_type[PieceType::Queen as usize])
            & self.by_color[them as usize];
        bb = bishops;
        while bb != 0 {
            let sq = bb.trailing_zeros() as u8;
            let line = bishop_line(king_sq, sq);
            if line != 0 {
                let pieces_on_line = line & occ;
                if pieces_on_line.count_ones() == 3 {
                    let us_pieces = pieces_on_line & self.by_color[us as usize];
                    if us_pieces.count_ones() == 2 {
                        pins |= us_pieces ^ (1u64 << king_sq);
                    }
                }
            }
            bb &= bb - 1;
        }

        pins
    }

    pub fn get_knight_attacks(&self, sq: u8) -> u64 {
        crate::tables::ATTACKS.knight[sq as usize]
    }

    pub fn occupied(&self) -> u64 {
        self.by_color[0] | self.by_color[1]
    }

    pub fn get_piece_at(&self, sq: u8) -> Option<(PieceType, Color)> {
        let bit = 1u64 << sq;
        if (self.occupied() & bit) == 0 {
            return None;
        }
        let color = if (self.by_color[0] & bit) != 0 {
            Color::White
        } else {
            Color::Black
        };
        for pt in 0..6 {
            if (self.by_type[pt] & bit) != 0 {
                return Some((
                    match pt {
                        0 => PieceType::Pawn,
                        1 => PieceType::Knight,
                        2 => PieceType::Bishop,
                        3 => PieceType::Rook,
                        4 => PieceType::Queen,
                        5 => PieceType::King,
                        _ => unreachable!(),
                    },
                    color,
                ));
            }
        }
        None
    }

    pub fn make_move(&self, m: Move) -> Board {
        let mut next = *self;
        let from = m.from();
        let to = m.to();
        let flags = m.flags();
        let us = self.side_to_move;
        let them = us.opponent();

        let (piece, _) = self.get_piece_at(from).expect("No piece at from");

        next.hash ^= ZOBRIST.pieces[us as usize][piece as usize][from as usize];
        if let Some(sq) = self.ep_square {
            next.hash ^= ZOBRIST.ep[sq as usize];
        }
        next.hash ^= ZOBRIST.castling[self.castling_rights as usize];

        next.by_color[us as usize] &= !(1u64 << from);
        next.by_type[piece as usize] &= !(1u64 << from);

        if m.is_capture() {
            if flags == Move::EP_CAPTURE {
                let cap_sq = if us == Color::White { to - 8 } else { to + 8 };
                next.by_color[them as usize] &= !(1u64 << cap_sq);
                next.by_type[PieceType::Pawn as usize] &= !(1u64 << cap_sq);
                next.hash ^=
                    ZOBRIST.pieces[them as usize][PieceType::Pawn as usize][cap_sq as usize];
            } else {
                let (cap_piece, _) = self.get_piece_at(to).expect("Capture but no piece");
                next.by_color[them as usize] &= !(1u64 << to);
                next.by_type[cap_piece as usize] &= !(1u64 << to);
                next.hash ^= ZOBRIST.pieces[them as usize][cap_piece as usize][to as usize];
            }
            next.halfmove_clock = 0;
        } else if piece == PieceType::Pawn {
            next.halfmove_clock = 0;
        } else {
            next.halfmove_clock += 1;
        }

        let mut placed_piece = piece;
        if m.is_promotion() {
            placed_piece = match flags & 0b0011 {
                0 => PieceType::Knight,
                1 => PieceType::Bishop,
                2 => PieceType::Rook,
                3 => PieceType::Queen,
                _ => PieceType::Queen,
            };
        }

        next.by_color[us as usize] |= 1u64 << to;
        next.by_type[placed_piece as usize] |= 1u64 << to;
        next.hash ^= ZOBRIST.pieces[us as usize][placed_piece as usize][to as usize];

        if piece == PieceType::King {
            if flags == Move::K_CASTLE {
                let (r_from, r_to) = if us == Color::White { (7, 5) } else { (63, 61) };
                next.by_color[us as usize] &= !(1u64 << r_from);
                next.by_type[PieceType::Rook as usize] &= !(1u64 << r_from);
                next.by_color[us as usize] |= 1u64 << r_to;
                next.by_type[PieceType::Rook as usize] |= 1u64 << r_to;
                next.hash ^= ZOBRIST.pieces[us as usize][PieceType::Rook as usize][r_from as usize];
                next.hash ^= ZOBRIST.pieces[us as usize][PieceType::Rook as usize][r_to as usize];
            } else if flags == Move::Q_CASTLE {
                let (r_from, r_to) = if us == Color::White { (0, 3) } else { (56, 59) };
                next.by_color[us as usize] &= !(1u64 << r_from);
                next.by_type[PieceType::Rook as usize] &= !(1u64 << r_from);
                next.by_color[us as usize] |= 1u64 << r_to;
                next.by_type[PieceType::Rook as usize] |= 1u64 << r_to;
                next.hash ^= ZOBRIST.pieces[us as usize][PieceType::Rook as usize][r_from as usize];
                next.hash ^= ZOBRIST.pieces[us as usize][PieceType::Rook as usize][r_to as usize];
            }
        }

        let mut rights = next.castling_rights;
        if piece == PieceType::King {
            if us == Color::White {
                rights &= !3;
            } else {
                rights &= !12;
            }
        }
        if piece == PieceType::Rook {
            match from {
                0 => rights &= !2,
                7 => rights &= !1,
                56 => rights &= !8,
                63 => rights &= !4,
                _ => {}
            }
        }
        if m.is_capture() {
            match to {
                0 => rights &= !2,
                7 => rights &= !1,
                56 => rights &= !8,
                63 => rights &= !4,
                _ => {}
            }
        }
        next.castling_rights = rights;
        next.hash ^= ZOBRIST.castling[next.castling_rights as usize];

        if flags == Move::DOUBLE_PAWN_PUSH {
            let ep = if us == Color::White { to - 8 } else { to + 8 };
            next.ep_square = Some(ep);
            next.hash ^= ZOBRIST.ep[ep as usize];
        } else {
            next.ep_square = None;
        }

        next.side_to_move = them;
        next.hash ^= ZOBRIST.side;
        if us == Color::Black {
            next.fullmove_number += 1;
        }
        next.pins = next.calculate_pins();

        next
    }

    pub fn is_square_attacked(&self, sq: u8, attacker: Color) -> bool {
        let us = attacker;
        let occ = self.occupied();
        let pawn_attacks = crate::tables::ATTACKS.pawn[us.opponent() as usize][sq as usize];
        if (pawn_attacks & self.by_type[PieceType::Pawn as usize] & self.by_color[us as usize]) != 0
        {
            return true;
        }
        let knight_attacks = crate::tables::ATTACKS.knight[sq as usize];
        if (knight_attacks & self.by_type[PieceType::Knight as usize] & self.by_color[us as usize])
            != 0
        {
            return true;
        }
        let king_attacks = crate::tables::ATTACKS.king[sq as usize];
        if (king_attacks & self.by_type[PieceType::King as usize] & self.by_color[us as usize]) != 0
        {
            return true;
        }
        if (self.get_rook_attacks(sq, occ)
            & (self.by_type[PieceType::Rook as usize] | self.by_type[PieceType::Queen as usize])
            & self.by_color[us as usize])
            != 0
        {
            return true;
        }
        if (self.get_bishop_attacks(sq, occ)
            & (self.by_type[PieceType::Bishop as usize] | self.by_type[PieceType::Queen as usize])
            & self.by_color[us as usize])
            != 0
        {
            return true;
        }
        false
    }

    pub fn get_rook_attacks(&self, sq: u8, occ: u64) -> u64 {
        crate::tables::generate_slider_attacks(sq as usize, occ, true)
    }

    pub fn get_bishop_attacks(&self, sq: u8, occ: u64) -> u64 {
        crate::tables::generate_slider_attacks(sq as usize, occ, false)
    }

    pub fn get_attackers(&self, sq: u8, occ: u64) -> u64 {
        let mut attackers = 0u64;
        attackers |= crate::tables::ATTACKS.pawn[Color::White as usize][sq as usize]
            & self.by_type[PieceType::Pawn as usize]
            & self.by_color[Color::Black as usize];
        attackers |= crate::tables::ATTACKS.pawn[Color::Black as usize][sq as usize]
            & self.by_type[PieceType::Pawn as usize]
            & self.by_color[Color::White as usize];
        attackers |=
            crate::tables::ATTACKS.knight[sq as usize] & self.by_type[PieceType::Knight as usize];
        attackers |=
            crate::tables::ATTACKS.king[sq as usize] & self.by_type[PieceType::King as usize];
        attackers |= self.get_rook_attacks(sq, occ)
            & (self.by_type[PieceType::Rook as usize] | self.by_type[PieceType::Queen as usize]);
        attackers |= self.get_bishop_attacks(sq, occ)
            & (self.by_type[PieceType::Bishop as usize] | self.by_type[PieceType::Queen as usize]);
        attackers
    }

    pub fn see_value(&self, m: Move) -> i32 {
        let values = [100, 320, 330, 500, 900, 20000];
        let from = m.from();
        let to = m.to();
        let (mut piece, _) = self.get_piece_at(from).unwrap();
        let victim = self
            .get_piece_at(to)
            .map(|(p, _)| p)
            .unwrap_or(PieceType::Pawn);
        let mut score = values[victim as usize];
        if m.is_promotion() {
            let promo = m.promoted_piece().unwrap();
            score += values[promo as usize] - values[PieceType::Pawn as usize];
            piece = promo;
        }
        let mut occ = self.occupied();
        let mut attackers = self.get_attackers(to, occ);
        occ &= !(1u64 << from);
        attackers &= occ;
        let mut us = self.side_to_move.opponent();
        let mut res = vec![score];
        loop {
            let my_attackers = attackers & self.by_color[us as usize];
            if my_attackers == 0 {
                break;
            }
            let mut best_pt = PieceType::King;
            let mut attacker_sq = 64;
            for pt in PieceType::ALL {
                let subset = my_attackers & self.by_type[pt as usize];
                if subset != 0 {
                    best_pt = pt;
                    attacker_sq = subset.trailing_zeros() as u8;
                    break;
                }
            }
            score = values[piece as usize] - score;
            res.push(score);
            piece = best_pt;
            occ &= !(1u64 << attacker_sq);
            if best_pt == PieceType::Pawn
                || best_pt == PieceType::Bishop
                || best_pt == PieceType::Queen
            {
                attackers |= self.get_bishop_attacks(to, occ)
                    & (self.by_type[PieceType::Bishop as usize]
                        | self.by_type[PieceType::Queen as usize]);
            }
            if best_pt == PieceType::Rook || best_pt == PieceType::Queen {
                attackers |= self.get_rook_attacks(to, occ)
                    & (self.by_type[PieceType::Rook as usize]
                        | self.by_type[PieceType::Queen as usize]);
            }
            attackers &= occ;
            us = us.opponent();
        }
        let mut val = 0;
        for s in res.into_iter().rev() {
            val = (s - val).max(0);
        }
        val
    }

    pub fn is_in_check(&self) -> bool {
        let king_bit =
            self.by_type[PieceType::King as usize] & self.by_color[self.side_to_move as usize];
        if king_bit == 0 {
            return true;
        }
        self.is_square_attacked(
            king_bit.trailing_zeros() as u8,
            self.side_to_move.opponent(),
        )
    }

    pub fn is_legal(&self, m: Move) -> bool {
        let next_board = self.make_move(m);
        let us = self.side_to_move;
        let king_bit =
            next_board.by_type[PieceType::King as usize] & next_board.by_color[us as usize];
        if king_bit == 0 {
            return false;
        }
        !next_board.is_square_attacked(king_bit.trailing_zeros() as u8, us.opponent())
    }
}

fn rook_line(sq1: u8, sq2: u8) -> u64 {
    let f1 = sq1 % 8;
    let r1 = sq1 / 8;
    let f2 = sq2 % 8;
    let r2 = sq2 / 8;
    if f1 == f2 {
        let mut mask = 0u64;
        for r in r1.min(r2)..=r1.max(r2) {
            mask |= 1u64 << (r * 8 + f1);
        }
        mask
    } else if r1 == r2 {
        let mut mask = 0u64;
        for f in f1.min(f2)..=f1.max(f2) {
            mask |= 1u64 << (r1 * 8 + f);
        }
        mask
    } else {
        0
    }
}

fn bishop_line(sq1: u8, sq2: u8) -> u64 {
    let f1 = (sq1 % 8) as i16;
    let r1 = (sq1 / 8) as i16;
    let f2 = (sq2 % 8) as i16;
    let r2 = (sq2 / 8) as i16;
    let df = (f2 - f1).abs();
    let dr = (r2 - r1).abs();
    if df == dr && df > 0 {
        let mut mask = 0u64;
        let step_f = (f2 - f1).signum();
        let step_r = (r2 - r1).signum();
        for i in 0..=df {
            mask |= 1u64 << ((r1 + i * step_r) * 8 + (f1 + i * step_f));
        }
        mask
    } else {
        0
    }
}
