pub const MG_PST: [[i32; 64]; 6] = [
    // Pawn
    [
        0, 0, 0, 0, 0, 0, 0, 0, 98, 134, 61, 95, 68, 126, 34, -11, -6, 7, 26, 31, 65, 56, 25, -20,
        -14, 13, 6, 21, 23, 12, 17, -23, -27, -2, -5, 12, 17, 6, 10, -25, -26, -4, -4, -10, 3, 3,
        33, -12, -35, -1, -20, -23, -15, 24, 38, -22, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    // Knight
    [
        -167, -89, -34, -49, 61, -97, -15, -107, -73, -41, 72, 36, 23, 62, 7, -17, -47, 60, 37, 65,
        84, 129, 73, 44, -9, 17, 19, 53, 37, 69, 18, 22, -13, 4, 16, 13, 28, 19, 21, -8, -23, -9,
        12, 10, 19, 17, 25, -16, -29, -53, -12, -3, -1, 18, -14, -19, -105, -21, -58, -33, -17,
        -28, -19, -23,
    ],
    // Bishop
    [
        -29, 4, -82, -37, -25, -42, 7, -8, -26, 16, -18, -13, 30, 59, 18, -47, -16, 37, 43, 40, 35,
        50, 37, -2, -4, 5, 19, 50, 37, 37, 7, -2, -6, 13, 13, 26, 34, 12, 10, 4, 0, 15, 15, 15, 14,
        27, 18, 10, 4, 15, 16, 0, 7, 21, 33, 1, -33, -3, -14, -21, -13, -12, -39, -21,
    ],
    // Rook
    [
        32, 42, 32, 51, 63, 9, 31, 43, 27, 32, 58, 62, 80, 67, 26, 44, -5, 19, 26, 36, 17, 45, 61,
        16, -24, -11, 7, 26, 24, 35, -8, -20, -36, -26, -12, -1, 9, -7, 6, -23, -45, -25, -16, -17,
        3, 0, -5, -33, -44, -16, -20, -9, -1, 11, -6, -71, -19, -13, 1, 17, 16, 7, -37, -26,
    ],
    // Queen
    [
        -28, 0, 29, 12, 59, 44, 43, 45, -24, -39, -5, 1, -16, 57, 28, 54, -13, -17, 7, 8, 29, 56,
        47, 57, -27, -27, -16, -16, -1, 17, -2, 1, -9, -26, -9, -10, -2, -4, 3, -3, -14, 2, -11,
        -2, -5, 2, 14, 5, -35, -8, 11, 2, 8, 15, -3, 1, -1, -18, -9, -19, -27, -21, -54, -50,
    ],
    // King
    [
        -65, 23, 16, -15, -56, -34, 2, 13, 29, -1, -20, -7, -8, -4, -38, -29, -9, 24, 2, -16, -20,
        6, 22, -22, -17, -20, -12, -27, -30, -25, -14, -36, -49, -1, -27, -39, -46, -44, -33, -51,
        -14, -14, -22, -46, -44, -30, -15, -27, 1, 7, -8, -64, -43, -16, 9, 8, -15, 36, 12, -54, 8,
        -28, 24, 14,
    ],
];

pub const EG_PST: [[i32; 64]; 6] = [
    // Pawn
    [
        0, 0, 0, 0, 0, 0, 0, 0, 178, 173, 158, 134, 147, 132, 165, 187, 94, 100, 85, 67, 56, 53,
        82, 84, 32, 24, 13, 5, -2, 4, 17, 17, 13, 9, -3, -7, -7, -8, 3, -1, 4, 7, -6, 1, 0, -5, -1,
        -8, 13, 8, 8, 10, 13, 0, 2, -7, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    // Knight
    [
        -58, -38, -13, -28, -31, -27, -63, -99, -25, -8, -25, -2, -9, -25, -24, -52, -24, -20, 10,
        9, -1, -9, -19, -41, -17, 3, 22, 22, 22, 11, 8, -18, -18, -6, 16, 25, 16, 17, 4, -18, -23,
        -3, -1, 15, 10, -3, -18, -22, -42, -20, -10, -5, -2, -20, -23, -44, -29, -51, -23, -15,
        -22, -18, -50, -64,
    ],
    // Bishop
    [
        -14, -21, -11, -8, -7, -9, -17, -24, -8, -4, 7, -12, -3, -13, -4, -14, 2, -8, 0, -1, -2, 6,
        0, 4, -3, 9, 12, 9, 14, 10, 3, 2, -6, 3, 13, 19, 7, 10, -3, -9, -12, -3, 5, 10, 7, 0, -2,
        -12, -30, -4, 5, -1, -2, 5, -5, -20, -25, -17, -16, -10, -8, -10, -17, -54,
    ],
    // Rook
    [
        13, 10, 18, 15, 12, 12, 8, 5, 11, 13, 13, 11, -3, 3, 8, 3, 7, 7, 7, 5, 4, -3, -5, -3, 4, 3,
        13, 1, 2, 1, -1, 2, 3, 5, 8, 4, -5, -6, -8, -11, -4, 0, -5, -1, -7, -12, -8, -16, -6, -6,
        0, 2, -9, -9, -11, -3, -9, 2, 3, -1, -5, -13, 4, -20,
    ],
    // Queen
    [
        -9, 22, 22, 27, 27, 19, 10, 20, -17, 20, 32, 41, 58, 25, 30, 0, -20, 6, 9, 49, 47, 35, 19,
        9, 3, 22, 24, 45, 57, 40, 57, 36, -18, 28, 19, 47, 31, 34, 39, 23, -16, -27, 15, 6, 9, 17,
        10, 5, -22, -23, -30, -16, -16, -23, -36, -32, -33, -28, -22, -43, -5, -32, -20, -41,
    ],
    // King
    [
        -74, -35, -18, -18, -11, 15, 4, -17, -12, 17, 14, 17, 17, 38, 23, 11, 10, 17, 23, 15, 20,
        45, 44, 13, -8, 22, 24, 27, 26, 33, 26, 3, -18, -4, 21, 24, 27, 23, 9, -11, -19, -3, 11,
        21, 23, 16, 7, -9, -27, -11, 4, 13, 14, 4, -5, -17, -53, -34, -21, -11, -28, -14, -24, -43,
    ],
];

pub const MG_VALUE: [i32; 6] = [82, 337, 365, 477, 1025, 0];
pub const EG_VALUE: [i32; 6] = [94, 281, 297, 512, 968, 0];

pub struct AttackTables {
    pub knight: [u64; 64],
    pub king: [u64; 64],
    pub pawn: [[u64; 64]; 2],
    pub bishop_masks: [u64; 64],
    pub rook_masks: [u64; 64],
    pub bishop_table: Vec<u64>,
    pub rook_table: Vec<u64>,
    pub bishop_magics: [Magic; 64],
    pub rook_magics: [Magic; 64],
}

pub struct Magic {
    pub magic: u64,
    pub mask: u64,
    pub offset: usize,
    pub shift: u8,
}

impl AttackTables {
    pub fn new() -> Self {
        let mut knight = [0u64; 64];
        let mut king = [0u64; 64];
        let mut pawn = [[0u64; 64]; 2];

        for sq in 0..64 {
            let f = (sq % 8) as i16;
            let r = (sq / 8) as i16;

            // Knight
            for &(df, dr) in &[
                (-2, -1),
                (-2, 1),
                (-1, -2),
                (-1, 2),
                (1, -2),
                (1, 2),
                (2, -1),
                (2, 1),
            ] {
                let nf = f + df;
                let nr = r + dr;
                if nf >= 0 && nf < 8 && nr >= 0 && nr < 8 {
                    knight[sq as usize] |= 1u64 << (nr * 8 + nf);
                }
            }

            // King
            for df in -1..=1 {
                for dr in -1..=1 {
                    if df == 0 && dr == 0 {
                        continue;
                    }
                    let nf = f + df;
                    let nr = r + dr;
                    if nf >= 0 && nf < 8 && nr >= 0 && nr < 8 {
                        king[sq as usize] |= 1u64 << (nr * 8 + nf);
                    }
                }
            }

            // Pawn attacks
            if r < 7 {
                // White
                if f > 0 {
                    pawn[0][sq as usize] |= 1u64 << ((r + 1) * 8 + f - 1);
                }
                if f < 7 {
                    pawn[0][sq as usize] |= 1u64 << ((r + 1) * 8 + f + 1);
                }
            }
            if r > 0 {
                // Black
                if f > 0 {
                    pawn[1][sq as usize] |= 1u64 << ((r - 1) * 8 + f - 1);
                }
                if f < 7 {
                    pawn[1][sq as usize] |= 1u64 << ((r - 1) * 8 + f + 1);
                }
            }
        }

        AttackTables {
            knight,
            king,
            pawn,
            bishop_masks: [0; 64],
            rook_masks: [0; 64],
            bishop_table: Vec::new(),
            rook_table: Vec::new(),
            bishop_magics: BISHOP_MAGICS,
            rook_magics: ROOK_MAGICS,
        }
    }
}

pub fn generate_slider_attacks(sq: usize, occ: u64, is_rook: bool) -> u64 {
    let mut attacks = 0u64;
    let r = (sq / 8) as i16;
    let f = (sq % 8) as i16;
    let directions: &[(i16, i16)] = if is_rook {
        &[(1, 0), (-1, 0), (0, 1), (0, -1)]
    } else {
        &[(1, 1), (1, -1), (-1, 1), (-1, -1)]
    };

    for &(dr, df) in directions {
        let mut nr = r + dr;
        let mut nf = f + df;
        while nr >= 0 && nr < 8 && nf >= 0 && nf < 8 {
            attacks |= 1u64 << (nr * 8 + nf);
            if (occ & (1u64 << (nr * 8 + nf))) != 0 {
                break;
            }
            nr += dr;
            nf += df;
        }
    }
    attacks
}

const BISHOP_MAGICS: [Magic; 64] = [
    Magic {
        magic: 0x0004001002008008,
        mask: 0x0040201008040200,
        offset: 0,
        shift: 58,
    },
    Magic {
        magic: 0x0001004402008010,
        mask: 0x0040201008040200,
        offset: 64,
        shift: 58,
    },
    Magic {
        magic: 0x0001008010402000,
        mask: 0x0000201008040200,
        offset: 128,
        shift: 59,
    },
    Magic {
        magic: 0x0001002008040200,
        mask: 0x0000001008040200,
        offset: 160,
        shift: 60,
    },
    Magic {
        magic: 0x0004000804020000,
        mask: 0x0000001008040200,
        offset: 176,
        shift: 60,
    },
    Magic {
        magic: 0x0001000402000000,
        mask: 0x0000201008040200,
        offset: 192,
        shift: 59,
    },
    Magic {
        magic: 0x0008002010402000,
        mask: 0x0040201008040200,
        offset: 224,
        shift: 58,
    },
    Magic {
        magic: 0x0004001002008008,
        mask: 0x0040201008040200,
        offset: 288,
        shift: 58,
    },
    Magic {
        magic: 0x0001004402008010,
        mask: 0x0040201008040200,
        offset: 352,
        shift: 58,
    },
    Magic {
        magic: 0x0004001002008008,
        mask: 0x0040201008040200,
        offset: 416,
        shift: 58,
    },
    Magic {
        magic: 0x0001008010402000,
        mask: 0x0000201008040200,
        offset: 480,
        shift: 59,
    },
    Magic {
        magic: 0x0001002008040200,
        mask: 0x0000001008040200,
        offset: 512,
        shift: 60,
    },
    Magic {
        magic: 0x0004000804020000,
        mask: 0x0000001008040200,
        offset: 528,
        shift: 60,
    },
    Magic {
        magic: 0x0001000402000000,
        mask: 0x0000201008040200,
        offset: 544,
        shift: 59,
    },
    Magic {
        magic: 0x0008002010402000,
        mask: 0x0040201008040200,
        offset: 576,
        shift: 58,
    },
    Magic {
        magic: 0x0004001002008008,
        mask: 0x0040201008040200,
        offset: 640,
        shift: 58,
    },
    Magic {
        magic: 0x0000008010402000,
        mask: 0x0000201008040200,
        offset: 704,
        shift: 59,
    },
    Magic {
        magic: 0x0000008010402000,
        mask: 0x0000201008040200,
        offset: 736,
        shift: 59,
    },
    Magic {
        magic: 0x0000002008040200,
        mask: 0x0000001008040200,
        offset: 768,
        shift: 60,
    },
    Magic {
        magic: 0x0000000804020000,
        mask: 0x0000001008040200,
        offset: 784,
        shift: 60,
    },
    Magic {
        magic: 0x0000000804020000,
        mask: 0x0000001008040200,
        offset: 800,
        shift: 60,
    },
    Magic {
        magic: 0x0000000402000000,
        mask: 0x0000201008040200,
        offset: 816,
        shift: 59,
    },
    Magic {
        magic: 0x0000002010402000,
        mask: 0x0000201008040200,
        offset: 848,
        shift: 59,
    },
    Magic {
        magic: 0x0000002010402000,
        mask: 0x0000201008040200,
        offset: 880,
        shift: 59,
    },
    Magic {
        magic: 0x0000002008040200,
        mask: 0x0000001008040200,
        offset: 912,
        shift: 60,
    },
    Magic {
        magic: 0x0000002008040200,
        mask: 0x0000001008040200,
        offset: 928,
        shift: 60,
    },
    Magic {
        magic: 0x0000002008040200,
        mask: 0x0000001008040200,
        offset: 944,
        shift: 60,
    },
    Magic {
        magic: 0x0000002008040200,
        mask: 0x0000001008040200,
        offset: 960,
        shift: 60,
    },
    Magic {
        magic: 0x0000002008040200,
        mask: 0x0000001008040200,
        offset: 976,
        shift: 60,
    },
    Magic {
        magic: 0x0000002008040200,
        mask: 0x0000001008040200,
        offset: 992,
        shift: 60,
    },
    Magic {
        magic: 0x0000002008040200,
        mask: 0x0000001008040200,
        offset: 1008,
        shift: 60,
    },
    Magic {
        magic: 0x0000002008040200,
        mask: 0x0000001008040200,
        offset: 1024,
        shift: 60,
    },
    Magic {
        magic: 0x0000000804020000,
        mask: 0x0000001008040200,
        offset: 1040,
        shift: 60,
    },
    Magic {
        magic: 0x0000000804020000,
        mask: 0x0000001008040200,
        offset: 1056,
        shift: 60,
    },
    Magic {
        magic: 0x0000000804020000,
        mask: 0x0000001008040200,
        offset: 1072,
        shift: 60,
    },
    Magic {
        magic: 0x0000000804020000,
        mask: 0x0000001008040200,
        offset: 1088,
        shift: 60,
    },
    Magic {
        magic: 0x0000000804020000,
        mask: 0x0000001008040200,
        offset: 1104,
        shift: 60,
    },
    Magic {
        magic: 0x0000000804020000,
        mask: 0x0000001008040200,
        offset: 1120,
        shift: 60,
    },
    Magic {
        magic: 0x0000000804020000,
        mask: 0x0000001008040200,
        offset: 1136,
        shift: 60,
    },
    Magic {
        magic: 0x0000000804020000,
        mask: 0x0000001008040200,
        offset: 1152,
        shift: 60,
    },
    Magic {
        magic: 0x0000000402000000,
        mask: 0x0000201008040200,
        offset: 1168,
        shift: 59,
    },
    Magic {
        magic: 0x0000000402000000,
        mask: 0x0000201008040200,
        offset: 1200,
        shift: 59,
    },
    Magic {
        magic: 0x0000000804020000,
        mask: 0x0000001008040200,
        offset: 1232,
        shift: 60,
    },
    Magic {
        magic: 0x0000000804020000,
        mask: 0x0000001008040200,
        offset: 1248,
        shift: 60,
    },
    Magic {
        magic: 0x0000000804020000,
        mask: 0x0000001008040200,
        offset: 1264,
        shift: 60,
    },
    Magic {
        magic: 0x0000000402000000,
        mask: 0x0000201008040200,
        offset: 1280,
        shift: 59,
    },
    Magic {
        magic: 0x0000000402000000,
        mask: 0x0000201008040200,
        offset: 1312,
        shift: 59,
    },
    Magic {
        magic: 0x0000000402000000,
        mask: 0x0000201008040200,
        offset: 1344,
        shift: 59,
    },
    Magic {
        magic: 0x0000201040200000,
        mask: 0x0040201008040200,
        offset: 1376,
        shift: 58,
    },
    Magic {
        magic: 0x0000201040200000,
        mask: 0x0040201008040200,
        offset: 1440,
        shift: 58,
    },
    Magic {
        magic: 0x0000201040200000,
        mask: 0x0040201008040200,
        offset: 1504,
        shift: 58,
    },
    Magic {
        magic: 0x0000040200000000,
        mask: 0x0000201008040200,
        offset: 1568,
        shift: 59,
    },
    Magic {
        magic: 0x0000040200000000,
        mask: 0x0000201008040200,
        offset: 1600,
        shift: 59,
    },
    Magic {
        magic: 0x0000201040200000,
        mask: 0x0040201008040200,
        offset: 1632,
        shift: 58,
    },
    Magic {
        magic: 0x0000201040200000,
        mask: 0x0040201008040200,
        offset: 1696,
        shift: 58,
    },
    Magic {
        magic: 0x0000201040200000,
        mask: 0x0040201008040200,
        offset: 1760,
        shift: 58,
    },
    Magic {
        magic: 0x0008002010402000,
        mask: 0x0040201008040200,
        offset: 1824,
        shift: 58,
    },
    Magic {
        magic: 0x0008002010402000,
        mask: 0x0040201008040200,
        offset: 1888,
        shift: 58,
    },
    Magic {
        magic: 0x0008002010402000,
        mask: 0x0040201008040200,
        offset: 1952,
        shift: 58,
    },
    Magic {
        magic: 0x0001000402000000,
        mask: 0x0000201008040200,
        offset: 2016,
        shift: 59,
    },
    Magic {
        magic: 0x0001000402000000,
        mask: 0x0000201008040200,
        offset: 2048,
        shift: 59,
    },
    Magic {
        magic: 0x0008002010402000,
        mask: 0x0040201008040200,
        offset: 2080,
        shift: 58,
    },
    Magic {
        magic: 0x0008002010402000,
        mask: 0x0040201008040200,
        offset: 2144,
        shift: 58,
    },
    Magic {
        magic: 0x0008002010402000,
        mask: 0x0040201008040200,
        offset: 2208,
        shift: 58,
    },
];

const ROOK_MAGICS: [Magic; 64] = [
    Magic {
        magic: 0x0080001000200040,
        mask: 0x000101010101017e,
        offset: 0,
        shift: 52,
    },
    Magic {
        magic: 0x0080001000200040,
        mask: 0x000202020202027c,
        offset: 4096,
        shift: 52,
    },
    Magic {
        magic: 0x0080001000200040,
        mask: 0x000404040404047a,
        offset: 8192,
        shift: 52,
    },
    Magic {
        magic: 0x0080001000200040,
        mask: 0x0008080808080876,
        offset: 12288,
        shift: 52,
    },
    Magic {
        magic: 0x0080001000200040,
        mask: 0x001010101010106e,
        offset: 16384,
        shift: 52,
    },
    Magic {
        magic: 0x0080001000200040,
        mask: 0x002020202020205e,
        offset: 20480,
        shift: 52,
    },
    Magic {
        magic: 0x0080001000200040,
        mask: 0x004040404040403e,
        offset: 24576,
        shift: 52,
    },
    Magic {
        magic: 0x0080001000200040,
        mask: 0x008080808080807e,
        offset: 28672,
        shift: 52,
    },
    Magic {
        magic: 0x0000800010002000,
        mask: 0x0001010101017e01,
        offset: 32768,
        shift: 52,
    },
    Magic {
        magic: 0x0000800010002000,
        mask: 0x0002020202027c02,
        offset: 36864,
        shift: 52,
    },
    Magic {
        magic: 0x0000800010002000,
        mask: 0x0004040404047a04,
        offset: 40960,
        shift: 52,
    },
    Magic {
        magic: 0x0000800010002000,
        mask: 0x00080808087608,
        offset: 45056,
        shift: 52,
    },
    Magic {
        magic: 0x0000800010002000,
        mask: 0x0010101010106e10,
        offset: 49152,
        shift: 52,
    },
    Magic {
        magic: 0x0000800010002000,
        mask: 0x0020202020205e20,
        offset: 53248,
        shift: 52,
    },
    Magic {
        magic: 0x0000800010002000,
        mask: 0x0040404040403e40,
        offset: 57344,
        shift: 52,
    },
    Magic {
        magic: 0x0000800010002000,
        mask: 0x0080808080807e80,
        offset: 61440,
        shift: 52,
    },
    Magic {
        magic: 0x0000008000100020,
        mask: 0x00010101017e0101,
        offset: 65536,
        shift: 52,
    },
    Magic {
        magic: 0x0000008000100020,
        mask: 0x00020202027c0202,
        offset: 69632,
        shift: 52,
    },
    Magic {
        magic: 0x0000008000100020,
        mask: 0x00040404047a0404,
        offset: 73728,
        shift: 52,
    },
    Magic {
        magic: 0x0000008000100020,
        mask: 0x0008080808760808,
        offset: 77824,
        shift: 52,
    },
    Magic {
        magic: 0x0000008000100020,
        mask: 0x00101010106e1010,
        offset: 81920,
        shift: 52,
    },
    Magic {
        magic: 0x0000008000100020,
        mask: 0x00202020205e2020,
        offset: 86016,
        shift: 52,
    },
    Magic {
        magic: 0x0000008000100020,
        mask: 0x00404040403e4040,
        offset: 90112,
        shift: 52,
    },
    Magic {
        magic: 0x0000008000100020,
        mask: 0x00808080807e8080,
        offset: 94208,
        shift: 52,
    },
    Magic {
        magic: 0x0000000080001000,
        mask: 0x000101017e010101,
        offset: 98304,
        shift: 52,
    },
    Magic {
        magic: 0x0000000080001000,
        mask: 0x000202027c020202,
        offset: 102400,
        shift: 52,
    },
    Magic {
        magic: 0x0000000080001000,
        mask: 0x000404047a040404,
        offset: 106496,
        shift: 52,
    },
    Magic {
        magic: 0x0000000080001000,
        mask: 0x0008080876080808,
        offset: 110592,
        shift: 52,
    },
    Magic {
        magic: 0x0000000080001000,
        mask: 0x001010106e101010,
        offset: 114688,
        shift: 52,
    },
    Magic {
        magic: 0x0000000080001000,
        mask: 0x002020205e202020,
        offset: 118784,
        shift: 52,
    },
    Magic {
        magic: 0x0000000080001000,
        mask: 0x004040403e404040,
        offset: 122880,
        shift: 52,
    },
    Magic {
        magic: 0x0000000080001000,
        mask: 0x008080807e808080,
        offset: 126976,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000800010,
        mask: 0x0001017e01010101,
        offset: 131072,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000800010,
        mask: 0x0002027c02020202,
        offset: 135168,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000800010,
        mask: 0x0004047a04040404,
        offset: 139264,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000800010,
        mask: 0x0008087608080808,
        offset: 143360,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000800010,
        mask: 0x0010106e10101010,
        offset: 147456,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000800010,
        mask: 0x0020205e20202020,
        offset: 151552,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000800010,
        mask: 0x0040403e40404040,
        offset: 155648,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000800010,
        mask: 0x0080807e80808080,
        offset: 159744,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000008000,
        mask: 0x00017e0101010101,
        offset: 163840,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000008000,
        mask: 0x00027c0202020202,
        offset: 167936,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000008000,
        mask: 0x00047a0404040404,
        offset: 172032,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000008000,
        mask: 0x0008760808080808,
        offset: 176128,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000008000,
        mask: 0x00106e1010101010,
        offset: 180224,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000008000,
        mask: 0x00205e2020202020,
        offset: 184320,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000008000,
        mask: 0x00403e4040404040,
        offset: 188416,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000008000,
        mask: 0x00807e8080808080,
        offset: 192512,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x7e01010101010101,
        offset: 196608,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x7c02020202020202,
        offset: 200704,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x7a04040404040404,
        offset: 204800,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x7608080808080808,
        offset: 208896,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x6e10101010101010,
        offset: 212992,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x5e20202020202020,
        offset: 217088,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x3e40404040404040,
        offset: 221184,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x7e80808080808080,
        offset: 225280,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x7e01010101010101,
        offset: 229376,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x7c02020202020202,
        offset: 233472,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x7a04040404040404,
        offset: 237568,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x7608080808080808,
        offset: 241664,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x6e10101010101010,
        offset: 245760,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x5e20202020202020,
        offset: 249856,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x3e40404040404040,
        offset: 253952,
        shift: 52,
    },
    Magic {
        magic: 0x0000000000000080,
        mask: 0x7e80808080808080,
        offset: 258048,
        shift: 52,
    },
];

lazy_static::lazy_static! {
    pub static ref ATTACKS: AttackTables = AttackTables::new();
}
