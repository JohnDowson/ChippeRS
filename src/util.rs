#[allow(dead_code)]
pub fn u16_to_nibbles(n: u16) -> [u8; 4] {
    [
        ((n & 0xF000) >> 12) as u8,
        ((n & 0x0F00) >> 8) as u8,
        ((n & 0x00F0) >> 4) as u8,
        (n & 0x000F) as u8,
    ]
}
pub fn u8_2_to_nibbles(n: [u8; 2]) -> [u8; 4] {
    [
        (n[0] & 0xF0) >> 4,
        (n[0] & 0x0F),
        (n[1] & 0xF0) >> 4,
        (n[1] & 0x0F),
    ]
}
pub fn nibbles_to_u8(n: u8, nn: u8) -> u8 {
    (n << 4) | nn
}
pub fn nibbles_to_u16(n: u8, nn: u8, nnn: u8) -> u16 {
    ((n as u16) << 8) | (((nn << 4) | nnn) as u16)
}
