mod aes128_enc;
mod attack;

fn main() {
    // Original value of the key
    let key : [u8; 16]  = [16, 15, 14, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    // Test attack
    /*let key_cracked = */
    attack::attack(key);
}
