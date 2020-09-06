pub fn unpack_bcd(byte_array: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    for byte in byte_array {
        result.push(byte >> 4 & 0x0F);
        result.push(byte & 0x0F);
    }
    return result;
}
pub fn create_read_command(pos: i32, length: i32) -> Vec<u8> {
    let bpos = [pos >> 8 & 0xFF, pos & 0xFF];
    return vec![0x00, 0xB0, bpos[0] as u8, bpos[1] as u8, length as u8];
}

pub fn decode_bcd(byte_array: &[u8]) -> i32 {
    let mut result = String::new();
    for byte in byte_array {
        let a = byte & &0b00001111;
        result.push_str(&a.to_string())
    }
    return result.parse::<i32>().unwrap();
}
