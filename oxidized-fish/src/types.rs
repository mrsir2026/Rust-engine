pub const INFINITY: i32 = 30000;
pub const MATE_VALUE: i32 = 29000;

pub static CONTEMPT: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
pub static THREADS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    pub fn opponent(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl PieceType {
    pub const ALL: [PieceType; 6] = [
        PieceType::Pawn,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
        PieceType::King,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    data: u16,
}

impl Move {
    pub const QUIET: u8 = 0;
    pub const DOUBLE_PAWN_PUSH: u8 = 1;
    pub const K_CASTLE: u8 = 2;
    pub const Q_CASTLE: u8 = 3;
    pub const CAPTURE: u8 = 4;
    pub const EP_CAPTURE: u8 = 5;
    pub const PROMOTION: u8 = 8;

    pub fn new(from: u8, to: u8, flags: u8) -> Self {
        let data = (from as u16) | ((to as u16) << 6) | ((flags as u16) << 12);
        Move { data }
    }

    pub fn from(&self) -> u8 {
        (self.data & 0x3F) as u8
    }

    pub fn to(&self) -> u8 {
        ((self.data >> 6) & 0x3F) as u8
    }

    pub fn flags(&self) -> u8 {
        ((self.data >> 12) & 0xF) as u8
    }

    pub fn is_capture(&self) -> bool {
        (self.flags() & 4) != 0
    }

    pub fn is_promotion(&self) -> bool {
        (self.flags() & 8) != 0
    }

    pub fn promoted_piece(&self) -> Option<PieceType> {
        if !self.is_promotion() {
            return None;
        }
        match self.flags() & 3 {
            0 => Some(PieceType::Knight),
            1 => Some(PieceType::Bishop),
            2 => Some(PieceType::Rook),
            3 => Some(PieceType::Queen),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let ranks = ['1', '2', '3', '4', '5', '6', '7', '8'];
        let f = self.from();
        let t = self.to();
        let mut s = format!(
            "{}{}{}{}",
            files[(f % 8) as usize],
            ranks[(f / 8) as usize],
            files[(t % 8) as usize],
            ranks[(t / 8) as usize]
        );

        if self.is_promotion() {
            match self.flags() & 3 {
                0 => s.push('n'),
                1 => s.push('b'),
                2 => s.push('r'),
                3 => s.push('q'),
                _ => {}
            }
        }
        s
    }
}
