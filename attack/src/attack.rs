/// Exercice 2: AES 3,5 rounds attack.
/// Inspired by https://www.davidwong.fr/blockbreakers/square_2_attack4rounds.html
use crate::aes128_enc::{aes128_enc, prev_aes128_round_key, SINV};
use rand::prelude::*;

/// Reverse the last 1/2 round with a guess at a position pos
/// Return the set of bytes reversed for all the 256 sets
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
fn check_key_guess(key_guess: u8, set_of_reversed_bytes: [u8; 256]) -> bool {
    let mut xored_all_rev_bytes = 0;
    for rev_byte in set_of_reversed_bytes {
        xored_all_rev_bytes ^= rev_byte;
    }
    //dbg!(xored_all_rev_bytes);
    if xored_all_rev_bytes == 0 {
        true
    } else {
        false
    }
}

fn generate_lamda_set(key : [u8; 16]) -> [[u8; 16]; 256] {
    let x = rand::random::<u8>();

    // 256 lambda_sets of size 16
    let mut lambda_set: [[u8; 16]; 256] = [[x; 16]; 256];

    for num_set in 0..256 {
        lambda_set[num_set][0] = num_set as u8;
    }

    // Encrypt the 256 sets
    for i in 0..256 {
        aes128_enc(&mut lambda_set[i], key, 4, false);
    }
    lambda_set
}

/// Square attack
pub fn attack(key : [u8; 16]) -> bool {
    let mut cracked_key = false;
    while !cracked_key {
        // Generate a random lambda-set and encrypt it 
        let lambda_set = generate_lamda_set(key);
        // Loop through all the key
        for key_index in 0..16 {
            // Try all values for the guess
            for guess in 0..255 {
                let set_of_reversed_bytes = reverse_state(guess, key_index, &lambda_set);
                if check_key_guess(guess, set_of_reversed_bytes) {

                }
                else {

                }
            }
        }
    }
    true
}