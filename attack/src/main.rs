mod aes128_enc;
mod attack;
use rand::random;

fn test(nb_test: u32) {
    for _ in 0..nb_test {
        let mut key: [u8; 16] = [0; 16];
        for i in 0..16 {
            key[i] = random::<u8>();
        }
        assert!(attack::attack(key) == key);
    }
    println!("{} tests réussis :D", nb_test);
}

fn main() {
    // Original value of the key
    let key: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    // Test attack
    /*let key_cracked = */
    attack::attack(key);

    test(100);
}
