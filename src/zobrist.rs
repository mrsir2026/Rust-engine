use lazy_static::lazy_static;

pub struct Zobrist {
    pub pieces: [[[u64; 64]; 6]; 2],
    pub side: u64,
    pub castling: [u64; 16],
    pub ep: [u64; 64],
}

lazy_static! {
    pub static ref ZOBRIST: Zobrist = {
        let mut pieces = [[[0u64; 64]; 6]; 2];
        let mut castling = [0u64; 16];
        let mut ep = [0u64; 64];

        let mut seed = 1070372;
        fn xorshift64(seed: &mut u64) -> u64 {
            *seed ^= *seed << 13;
            *seed ^= *seed >> 7;
            *seed ^= *seed << 17;
            *seed
        }

        for c in 0..2 {
            for pt in 0..6 {
                for sq in 0..64 {
                    pieces[c][pt][sq] = xorshift64(&mut seed);
                }
            }
        }

        let side = xorshift64(&mut seed);

        for i in 0..16 {
            castling[i] = xorshift64(&mut seed);
        }

        for i in 0..64 {
            ep[i] = xorshift64(&mut seed);
        }

        Zobrist {
            pieces,
            side,
            castling,
            ep,
        }
    };
}
