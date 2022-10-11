/// Exercice 2: AES 3,5 rounds attack.
/// Inspired by https://www.davidwong.fr/blockbreakers/square_2_attack4rounds.html

use crate::aes128_enc::{aes128_enc, prev_aes128_round_key, SINV};
use array_tool::vec::{Union, Intersect};
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
    xored_all_rev_bytes == 0
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
    let mut cracked_key: Vec<Vec<u8>> = Vec::new();
    for _ in 0..16 {
        cracked_key.push(Vec::new())
    }
    while !cracked_key.iter().all(|v| v.len() == 1) {
        for index_key in 0..16 {
            let mut new_cracked_key: Vec<u8> = vec!();
            print!("\nKey[{}] = ", index_key);
            for n in 0..255 {
                let set_of_reversed_bytes = reverse_state(n, index_key, &generate_lamda_set(key));

                if check_key_guess(n, set_of_reversed_bytes) {
                    print!("{}  ", n);
                    new_cracked_key.push(n);
                }
                if cracked_key[index_key].is_empty() {
                    cracked_key[index_key].append(&mut new_cracked_key);
                } else {
                    cracked_key[index_key].intersect(new_cracked_key.clone());
                }
            }
        }
    }
    dbg!(cracked_key);
    // TODO: faire les prevs
    true
}