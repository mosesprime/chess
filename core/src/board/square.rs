use super::Bitboard;

const FILE_NAMES: &str = "abcdefgh";
const RANK_NAMES: &str = "12345678";

pub struct Square(u8);

impl Square {
    pub fn from_coord(rank: u8, file: u8) -> Self {
        debug_assert!(rank <= 7, "rank out of bounds");
        debug_assert!(file <= 7, "file out of bounds");
        Self((rank * 8) + file)
    }

    pub fn as_mask(&self) -> Bitboard {
        0 | (1 << self.0)        
    }

    fn rank(&self) -> u8 {
        self.0 / 8
    }

    fn file(&self) -> u8 {
        self.0 % 8
    }
}

#[test]
fn square_as_mask() {
    /* . . . . . . . .
     * . . . . . . . .
     * . . . . . . . .
     * . . . . . . . .
     * . . . . . . . .
     * . . x . . . . .
     * . . . . . . . . 
     * . . . . . . . . 
     */
    let s = Square::from_coord(2, 2);
    let b: Bitboard = 0b0000000000000000000000000000000000000000000001000000000000000000;
    assert_eq!(s.as_mask(), b, "banana")
}
