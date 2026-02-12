#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bitboard(pub u64);

#[allow(dead_code)]
impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const UNIVERSE: Bitboard = Bitboard(!0);

    #[inline]
    pub fn new(val: u64) -> Self {
        Bitboard(val)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    #[inline]
    pub fn lsb(&self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    #[inline]
    pub fn pop_lsb(&mut self) -> u8 {
        let lsb = self.lsb();
        self.0 &= self.0 - 1;
        lsb
    }

    #[inline]
    pub fn set_bit(&mut self, square: u8) {
        self.0 |= 1u64 << square;
    }

    #[inline]
    pub fn clear_bit(&mut self, square: u8) {
        self.0 &= !(1u64 << square);
    }

    #[inline]
    pub fn test_bit(&self, square: u8) -> bool {
        (self.0 & (1u64 << square)) != 0
    }
}

impl Iterator for Bitboard {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            Some(self.pop_lsb())
        }
    }
}
