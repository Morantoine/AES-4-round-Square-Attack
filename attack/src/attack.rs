/// Exercice 2: AES 3,5 rounds attack.
/// Inspired by https://www.davidwong.fr/blockbreakers/square_2_attack4rounds.html
use crate::aes128_enc::SINV;

/// Reverse the last 1/2 round with a guess at a position pos
/// Return the byte reversed for all the 256 sets
fn reverse_state(guess: u8, pos: usize, lambda_set: &[[u8; 16]; 256]) -> [u8; 256] {
    let mut set_of_reversed_bytes: [u8; 256] = [0; 256];
    for (index, lambda) in lambda_set.iter().enumerate() {
        let before_add_round_key = lambda[pos] ^ guess;
        let before_sub_byte = SINV[before_add_round_key as usize]; // ShiftRows and SubBytes
        set_of_reversed_bytes[index] = before_sub_byte;
    }
    set_of_reversed_bytes
}

/// If all the reversed bytes at the guessed position of the lambda set xored give 0, then
/// this is probably the key
fn check_key_guess(key_gess: u8, set_of_reversed_bytes: [u8; 256]) -> Option<u8> {
    let mut xored_all_rev_bytes = 0;
    for rev_byte in set_of_reversed_bytes {
        xored_all_rev_bytes ^= rev_byte;
    }
    //dbg!(xored_all_rev_bytes);
    if xored_all_rev_bytes == 0 {
        Some(key_gess)
    } else {
        None
    }
}

/// Square attack
pub fn attack(lambda_set: &[[u8; 16]; 256]) -> [u8; 16] {
    let mut key: [u8; 16] = [0; 16];
    for index_key in 0..1 {
        print!("Key[{}] = ", index_key);
        for n in 0..1 {
            let set_of_reversed_bytes = reverse_state(n, index_key, lambda_set);
            //dbg!(set_of_reversed_bytes);

            match check_key_guess(n, set_of_reversed_bytes) {
                Some(n) => print!("{}  ", n),
                None => print!(""),
            }

            if set_of_reversed_bytes.len() == 1 {
                // only possibility
                key[index_key] = set_of_reversed_bytes[0];
            } else {
                // need to test the false positives
                // TODO
            }
        }
        println!();
    }
    key
}
