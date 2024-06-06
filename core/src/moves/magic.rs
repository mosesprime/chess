use crate::board::Bitboard;

pub fn magic_index(mask: Bitboard, occupied: Bitboard, magic: u64, shift: u8, offset: u64) -> usize {
    let blockerboard = occupied & mask;
    ((blockerboard.wrapping_mul(magic) >> shift) + offset) as usize
}
