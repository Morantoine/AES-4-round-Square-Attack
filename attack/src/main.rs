mod aes128_enc;
mod attack;

fn main() {
    //// Xor the 256 sets in the first set
    //for i in 1..256 {
    //    for j in 0..16 {
    //        lambda_set[0][j] ^= lambda_set[i][j];
    //    }
    //}

    // Test attack
    /*let key_cracked = */
    attack::attack();
}
