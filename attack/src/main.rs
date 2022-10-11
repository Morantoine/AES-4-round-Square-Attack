mod aes128_enc;
mod attack;

fn main() {
    //// Xor the 256 sets in the first set
    //for i in 1..256 {
    //    for j in 0..16 {
    //        lambda_set[0][j] ^= lambda_set[i][j];
    //    }
    //}

    // Original value of the key
    let key : [u8; 16]  = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    // Test attack
    /*let key_cracked = */
    attack::attack(key);
}
