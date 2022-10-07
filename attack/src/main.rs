mod aes128_enc;

fn main() {
    // the key in the documentation: 000102030405060708090a0b0c0d0e0f
    let key: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    // 256 lambda_sets of size 16
    let mut lambda_set: [[u8; 16]; 256] = [[0; 16]; 256];

    for num_set in 0..256 {
        lambda_set[num_set][0] = num_set as u8;
    }
    print!("{:#?}, {:#?}", lambda_set[0], lambda_set[1]);

    // Encrypt the 256 sets
    for i in 0..256 {
        aes128_enc::aes128_enc(&mut lambda_set[i], key, 3, true);
    }

    // Xor the 256 sets in the fisrt set
    for i in 0..256 {
        for j in 0..16 {
            lambda_set[0][j] ^= lambda_set[i][j];
        }
    }

    // print the firt set who have to be full of 0
    println!("{:#?}", lambda_set[0]);
}
