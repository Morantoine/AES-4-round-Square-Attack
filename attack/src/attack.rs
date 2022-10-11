/// Exercice 2: AES 3,5 rounds attack.
/// Inspired by https://www.davidwong.fr/blockbreakers/square_2_attack4rounds.html
use crate::aes128_enc::{aes128_enc, prev_aes128_round_key, SINV};

/// Reverse the last 1/2 round with a guess at a position pos
/// Return the byte reversed for all the 256 sets
fn reverse_state(guess: u8, pos: usize, lambda_set: &[[u8; 16]; 256]) -> [u8; 256] {
    let mut set_of_reversed_bytes: [u8; 256] = [0; 256];
    for (index, lambda) in lambda_set.iter().enumerate() {
        let before_add_round_key = lambda[pos] ^ guess;
        // no need to shift rows
        let before_sub_byte = SINV[before_add_round_key as usize]; // SubBytes
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

fn generate_lamda_set() -> [[u8; 16]; 256] {
    let x = rand::random::<u8>();
    // the key in the documentation: 000102030405060708090a0b0c0d0e0f
    let key: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    // 256 lambda_sets of size 16
    let mut lambda_set: [[u8; 16]; 256] = [[0; 16]; 256];
    //let mut lambda_set: [[u8; 16]; 256] = [[0xCC; 16]; 256];

    for num_set in 0..256 {
        lambda_set[num_set][0] = num_set as u8;
    }

    // Encrypt the 256 sets
    for i in 0..256 {
        aes128_enc(&mut lambda_set[i], key, 4, false);
    }

    lambda_set
}

/*
fn guess_position(lambda_set: &[[u8; 16]; 256], pos: u8) -> u8 {
for lambda in lambda_set {
    let correct_guesses = Vec::new();
    for guess in 1..0x100 {
        reversed_bytes = reverse_state(guess, position_in_state, encrypted_delta_set)
        if is_guess_correct(reversed_bytes):
            correct_guesses.append(guess)
    }
    if len(correct_guesses) == 1:
    break
}
return correct_guesses[0]
}

 */

/// Square attack
pub fn attack() -> [u8; 16] {
    let mut key: [u8; 16] = [0; 16];
    for index_key in 0..16 {
        print!("Key[{}] = ", index_key);
        for n in 0..255 {
            let set_of_reversed_bytes = reverse_state(n, index_key, lambda_set);
            //dbg!(set_of_reversed_bytes);
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
