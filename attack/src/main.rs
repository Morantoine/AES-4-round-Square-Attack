mod aes128_enc;
mod attack;

fn main() {
    // the key in the documentation: 000102030405060708090a0b0c0d0e0f
    //let key: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let key: [u8; 16] = [0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c];
    // 43, 126, 21, 22, 40...
    // 256 lambda_sets of size 16
    //let mut lambda_set: [[u8; 16]; 256] = [[0; 16]; 256];
    let mut lambda_set: [[u8; 16]; 256] = [[0xCC; 16]; 256];

    for num_set in 0..256 {
        lambda_set[num_set][0] = num_set as u8;
    }

    // Encrypt the 256 sets
    for i in 0..256 {
        aes128_enc::aes128_enc(&mut lambda_set[i], key, 4, false);
    }

    //// Xor the 256 sets in the first set
    //for i in 1..256 {
    //    for j in 0..16 {
    //        lambda_set[0][j] ^= lambda_set[i][j];
    //    }
    //}

    // Test attack
    /*let key_cracked = */
    attack::attack(&lambda_set);
}
