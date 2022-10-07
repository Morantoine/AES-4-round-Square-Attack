use crate::aes128_enc::SINV;

pub fn check_guess(guess : u8, pos : usize, lambda_set : &mut [[u8; 16]; 256]) {
    fn original_byte(guess : u8, pos : usize, block : &mut [u8; 16]) -> u8 {
    // XOR with the guess, cancelling AddRoundKey
    block[pos] ^= guess;

    // Invert ShiftRows and SubBytes together
    let mut tmp : u8;
    /* Row 0 */
    block[0] = SINV[block[0] as usize];
    block[4] = SINV[block[4] as usize];
    block[8] = SINV[block[8] as usize];
    block[12] = SINV[block[12] as usize];
    /* Row 1 */
    tmp = block[1];
    block[1] = SINV[block[13] as usize];
    block[13] = SINV[block[9] as usize];
    block[9] = SINV[block[5] as usize];
    block[5] = SINV[tmp as usize];
    /* Row 2 */
    tmp = block[10];
    block[10] = SINV[block[2] as usize];
    block[2] = SINV[tmp as usize];
    tmp = block[14];
    block[14] = SINV[block[6] as usize];
    block[6] = SINV[tmp as usize];
    /* Row 3 */
    tmp = block[15];
    block[15] = SINV[block[3] as usize];
    block[3] = SINV[block[7] as usize];
    block[7] = SINV[block[11] as usize];
    block[11] = SINV[tmp as usize];

    // Return targeted byte
    return block[pos];
    }

    let mut byte_set : Vec<u8> = Vec::new();
    for lambda in lambda_set {
        byte_set.push(original_byte(guess, pos, lambda));
    }
    let result = byte_set.iter().fold(0, | tmp, next | tmp ^ next);
    if result.count_ones() % 2 == 0 {
        println!("Found {} as candidate !", guess);
    }
    else {
        println!("Sorry, {} gave a {} result.", guess, result);
    }

}