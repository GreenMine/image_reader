pub fn vec_as_number(vec: &Vec<u8>) -> usize {
    let mut answer: usize = vec[0] as usize;

    for &num in &vec[1..] {
        answer = (answer << 8) | (num as usize);
    }
    answer 
}

pub fn get_byte_as_tuple(byte: u8) -> (u8, u8) {
    (byte >> 4 & 0xF, byte & 0xF)
}
