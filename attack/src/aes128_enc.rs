//use rand::thread_rng;
//use rand::seq::SliceRandom;

const RC: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];
const AES_BLOCK_SIZE: usize = 16;
const AES_128_KEY_SIZE: usize = 16;

/* 
const SBOX: [u8; 256] = [
    0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
    0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
    0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
    0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
    0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
    0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
    0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
    0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
    0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
    0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
    0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
    0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
    0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
    0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
    0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
    0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16,
];

pub const SINV: [u8; 256] = [
    0x52, 0x09, 0x6A, 0xD5, 0x30, 0x36, 0xA5, 0x38, 0xBF, 0x40, 0xA3, 0x9E, 0x81, 0xF3, 0xD7, 0xFB,
    0x7C, 0xE3, 0x39, 0x82, 0x9B, 0x2F, 0xFF, 0x87, 0x34, 0x8E, 0x43, 0x44, 0xC4, 0xDE, 0xE9, 0xCB,
    0x54, 0x7B, 0x94, 0x32, 0xA6, 0xC2, 0x23, 0x3D, 0xEE, 0x4C, 0x95, 0x0B, 0x42, 0xFA, 0xC3, 0x4E,
    0x08, 0x2E, 0xA1, 0x66, 0x28, 0xD9, 0x24, 0xB2, 0x76, 0x5B, 0xA2, 0x49, 0x6D, 0x8B, 0xD1, 0x25,
    0x72, 0xF8, 0xF6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xD4, 0xA4, 0x5C, 0xCC, 0x5D, 0x65, 0xB6, 0x92,
    0x6C, 0x70, 0x48, 0x50, 0xFD, 0xED, 0xB9, 0xDA, 0x5E, 0x15, 0x46, 0x57, 0xA7, 0x8D, 0x9D, 0x84,
    0x90, 0xD8, 0xAB, 0x00, 0x8C, 0xBC, 0xD3, 0x0A, 0xF7, 0xE4, 0x58, 0x05, 0xB8, 0xB3, 0x45, 0x06,
    0xD0, 0x2C, 0x1E, 0x8F, 0xCA, 0x3F, 0x0F, 0x02, 0xC1, 0xAF, 0xBD, 0x03, 0x01, 0x13, 0x8A, 0x6B,
    0x3A, 0x91, 0x11, 0x41, 0x4F, 0x67, 0xDC, 0xEA, 0x97, 0xF2, 0xCF, 0xCE, 0xF0, 0xB4, 0xE6, 0x73,
    0x96, 0xAC, 0x74, 0x22, 0xE7, 0xAD, 0x35, 0x85, 0xE2, 0xF9, 0x37, 0xE8, 0x1C, 0x75, 0xDF, 0x6E,
    0x47, 0xF1, 0x1A, 0x71, 0x1D, 0x29, 0xC5, 0x89, 0x6F, 0xB7, 0x62, 0x0E, 0xAA, 0x18, 0xBE, 0x1B,
    0xFC, 0x56, 0x3E, 0x4B, 0xC6, 0xD2, 0x79, 0x20, 0x9A, 0xDB, 0xC0, 0xFE, 0x78, 0xCD, 0x5A, 0xF4,
    0x1F, 0xDD, 0xA8, 0x33, 0x88, 0x07, 0xC7, 0x31, 0xB1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xEC, 0x5F,
    0x60, 0x51, 0x7F, 0xA9, 0x19, 0xB5, 0x4A, 0x0D, 0x2D, 0xE5, 0x7A, 0x9F, 0x93, 0xC9, 0x9C, 0xEF,
    0xA0, 0xE0, 0x3B, 0x4D, 0xAE, 0x2A, 0xF5, 0xB0, 0xC8, 0xEB, 0xBB, 0x3C, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2B, 0x04, 0x7E, 0xBA, 0x77, 0xD6, 0x26, 0xE1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0C, 0x7D,
]; */

const SBOX : [u8; 256] = [
 0x4, 0x9b, 0x4f, 0xce, 0xcb, 0x20, 0xc, 0xa6, 0xd7, 0xfb, 0x92, 0xb6, 0x6c, 0xbb, 0x2e, 0xe2,
 0x7b, 0x2f, 0x29, 0x86, 0x11, 0x89, 0xef, 0xb9, 0x51, 0x13, 0x7d, 0x32, 0xc8, 0x28, 0x31, 0x88,
 0xf6, 0x41, 0x4c, 0x3a, 0x68, 0x40, 0xb3, 0x73, 0xab, 0x2b, 0xf0, 0x49, 0xba, 0x4d, 0x42, 0x1b,
 0x9f, 0xe, 0xc2, 0xc6, 0x4b, 0xb2, 0x24, 0xd2, 0x37, 0xf, 0x8e, 0x83, 0xb, 0x63, 0x8f, 0x62,
 0xe5, 0x5d, 0xdf, 0xcd, 0xbc, 0x8c, 0xfd, 0x60, 0x3b, 0xb4, 0x9a, 0x6b, 0xa, 0xb5, 0x78, 0xe9,
 0x5b, 0x77, 0xbe, 0xde, 0x66, 0x12, 0x71, 0x8b, 0x81, 0x6e, 0xac, 0xd0, 0xca, 0x1c, 0x82, 0x72,
 0x19, 0x21, 0xf3, 0x17, 0x18, 0x9c, 0xd8, 0x5, 0xa3, 0xf1, 0xb0, 0xe3, 0x53, 0x8a, 0x55, 0x25,
 0x3f, 0xc5, 0x39, 0x87, 0xaa, 0x96, 0x3e, 0x33, 0xe7, 0x0, 0x93, 0xe0, 0x45, 0xf4, 0xff, 0xb1,
 0x61, 0x35, 0x1d, 0x4e, 0x56, 0x95, 0x1f, 0xec, 0x5c, 0xc9, 0xed, 0xf2, 0xa5, 0xda, 0xbf, 0xa9,
 0x44, 0x46, 0xd6, 0x23, 0x2d, 0x14, 0x9d, 0x5a, 0xe8, 0xa4, 0x7e, 0x6f, 0x2c, 0xa1, 0x27, 0xf8,
 0x6, 0xcf, 0x10, 0xd4, 0xfa, 0x65, 0x84, 0xfc, 0x57, 0x9, 0x3d, 0xe6, 0x47, 0xe4, 0xad, 0xd9,
 0xfe, 0xa8, 0xd5, 0x97, 0x90, 0x30, 0xee, 0x3c, 0xdd, 0xea, 0x7a, 0x69, 0xbd, 0x79, 0xa2, 0x15,
 0x43, 0xd3, 0xc0, 0x7, 0xaf, 0x1, 0x3, 0x64, 0xa0, 0xb7, 0xd, 0xb8, 0x16, 0xe1, 0xeb, 0x1e,
 0x7f, 0x8, 0xc7, 0x5f, 0x38, 0x75, 0xf7, 0x91, 0x50, 0x74, 0x52, 0xd1, 0x58, 0x26, 0x34, 0xc1,
 0xc3, 0xf9, 0x36, 0x85, 0x76, 0x70, 0x6a, 0x5e, 0x8d, 0x94, 0x67, 0x54, 0x7c, 0x80, 0x1a, 0x99,
 0xae, 0xc4, 0x2a, 0xdc, 0x59, 0x4a, 0x48, 0xf5, 0x98, 0xcc, 0xa7, 0x22, 0xdb, 0x2, 0x6d, 0x9e
];

pub const SINV: [u8; 256] = [
    0x79, 0xc5, 0xfd, 0xc6, 0x0, 0x67, 0xa0, 0xc3, 0xd1, 0xa9, 0x4c, 0x3c, 0x6, 0xca, 0x31, 0x39,
    0xa2, 0x14, 0x55,0x19, 0x95, 0xbf, 0xcc, 0x63, 0x64, 0x60, 0xee, 0x2f, 0x5d, 0x82, 0xcf, 0x86,
    0x5, 0x61, 0xfb, 0x93, 0x36, 0x6f, 0xdd, 0x9e, 0x1d, 0x12, 0xf2, 0x29, 0x9c, 0x94, 0xe, 0x11,
    0xb5, 0x1e, 0x1b, 0x77, 0xde, 0x81, 0xe2, 0x38, 0xd4, 0x72, 0x23, 0x48, 0xb7, 0xaa, 0x76,0x70,
    0x25, 0x21, 0x2e, 0xc0, 0x90, 0x7c, 0x91, 0xac, 0xf6, 0x2b, 0xf5, 0x34, 0x22, 0x2d, 0x83, 0x2,
    0xd8, 0x18, 0xda, 0x6c, 0xeb, 0x6e, 0x84, 0xa8, 0xdc, 0xf4, 0x97, 0x50, 0x88, 0x41, 0xe7, 0xd3,
    0x47, 0x80, 0x3f, 0x3d, 0xc7, 0xa5, 0x54, 0xea, 0x24, 0xbb, 0xe6, 0x4b, 0xc, 0xfe, 0x59, 0x9b,
    0xe5, 0x56, 0x5f, 0x27, 0xd9, 0xd5, 0xe4, 0x51, 0x4e, 0xbd, 0xba, 0x10, 0xec, 0x1a, 0x9a, 0xd0,
    0xed, 0x58, 0x5e, 0x3b, 0xa6, 0xe3, 0x13, 0x73, 0x1f, 0x15, 0x6d, 0x57, 0x45, 0xe8, 0x3a, 0x3e,
    0xb4, 0xd7, 0xa, 0x7a, 0xe9, 0x85, 0x75, 0xb3, 0xf8, 0xef, 0x4a, 0x1, 0x65, 0x96, 0xff, 0x30,
    0xc8, 0x9d, 0xbe, 0x68, 0x99, 0x8c, 0x7, 0xfa, 0xb1, 0x8f, 0x74, 0x28, 0x5a, 0xae, 0xf0, 0xc4,
    0x6a, 0x7f, 0x35, 0x26, 0x49, 0x4d, 0xb, 0xc9, 0xcb, 0x17, 0x2c, 0xd, 0x44, 0xbc, 0x52, 0x8e,
    0xc2, 0xdf, 0x32, 0xe0, 0xf1, 0x71, 0x33, 0xd2, 0x1c, 0x89, 0x5c, 0x4, 0xf9, 0x43, 0x3, 0xa1,
    0x5b, 0xdb, 0x37, 0xc1, 0xa3, 0xb2, 0x92, 0x8, 0x66, 0xaf, 0x8d, 0xfc, 0xf3, 0xb8, 0x53, 0x42,
    0x7b, 0xcd, 0xf, 0x6b, 0xad, 0x40, 0xab, 0x78, 0x98, 0x4f, 0xb9, 0xce, 0x87, 0x8a, 0xb6, 0x16,
    0x2a, 0x69, 0x8b, 0x62, 0x7d, 0xf7, 0x20, 0xd6, 0x9f, 0xe1, 0xa4, 0x9, 0xa7, 0x46, 0xb0, 0x7e
];

fn xtime(p: u8) -> u8 {
    let mut m: u8 = p >> 7;
    m ^= 1;
    m = m.overflowing_sub(1).0;
    m &= 0x1B; // x^4 + x^3 + x + 1
    return (p << 1) ^ m;
}

fn aes_round(block: &mut [u8; AES_BLOCK_SIZE], round_key: &mut [u8], last_round: u8) {
    let mut tmp: u8;

    //let custom = true;
    //if custom {
    //    let mut S_CUST : Vec<u8> =  Vec::from_iter(0..= 255);
    //    S_CUST.shuffle(&mut thread_rng());
    //    SBOX.clone_from_slice(& S_CUST);
    //}

    /*
    SubBytes + ShiftRow
    */
    /* Row 0 */
    block[0] = SBOX[block[0] as usize];
    block[4] = SBOX[block[4] as usize];
    block[8] = SBOX[block[8] as usize];
    block[12] = SBOX[block[12] as usize];
    /* Row 1 */
    tmp = block[1];
    block[1] = SBOX[block[5] as usize];
    block[5] = SBOX[block[9] as usize];
    block[9] = SBOX[block[13] as usize];
    block[13] = SBOX[tmp as usize];
    /* Row 2 */
    tmp = block[2];
    block[2] = SBOX[block[10 as usize] as usize];
    block[10] = SBOX[tmp as usize];
    tmp = block[6];
    block[6] = SBOX[block[14 as usize] as usize];
    block[14] = SBOX[tmp as usize];
    /* Row 3 */
    tmp = block[15];
    block[15] = SBOX[block[11] as usize];
    block[11] = SBOX[block[7] as usize];
    block[7] = SBOX[block[3] as usize];
    block[3] = SBOX[tmp as usize];

    /*
     * MixColumns
     */
    for i in (last_round..16).step_by(4) {
        /* lastround = 16 if it is the last round, 0 otherwise */
        let i = i as usize;
        let tmp2 = block[i];
        let tmp = block[i] ^ block[i + 1] ^ block[i + 2] ^ block[i + 3];

        block[i + 0] ^= tmp ^ xtime(block[i + 0] ^ block[i + 1]);
        block[i + 1] ^= tmp ^ xtime(block[i + 1] ^ block[i + 2]);
        block[i + 2] ^= tmp ^ xtime(block[i + 2] ^ block[i + 3]);
        block[i + 3] ^= tmp ^ xtime(block[i + 3] ^ tmp2);
    }

    /*
     * AddRoundKey
     */
    for i in 0..16 {
        block[i] ^= round_key[i];
    }
}

pub fn next_aes128_round_key(prev_key: &[u8], next_key: &mut [u8], round: usize) {
    next_key[0] = prev_key[0] ^ SBOX[prev_key[13] as usize] ^ RC[round];
    next_key[1] = prev_key[1] ^ SBOX[prev_key[14] as usize];
    next_key[2] = prev_key[2] ^ SBOX[prev_key[15] as usize];
    next_key[3] = prev_key[3] ^ SBOX[prev_key[12] as usize];

    for i in 4..16 {
        next_key[i] = prev_key[i] ^ next_key[i - 4];
    }
}

pub fn prev_aes128_round_key(next_key: &[u8], prev_key: &mut [u8], round: usize) {
    for i in (4..16).rev() {
        prev_key[i] = next_key[i] ^ next_key[i - 4];
    }
    prev_key[0] = next_key[0] ^ SBOX[prev_key[13] as usize] ^ RC[round];
    prev_key[1] = next_key[1] ^ SBOX[prev_key[14] as usize];
    prev_key[2] = next_key[2] ^ SBOX[prev_key[15] as usize];
    prev_key[3] = next_key[3] ^ SBOX[prev_key[12] as usize];
}

/*
 * Encrypt @block with @key over @nrounds. If @lastfull is true, the last round includes MixColumn, otherwise it doesn't.
 * @nrounds <= 10
 */
pub fn aes128_enc(
    block: &mut [u8; AES_BLOCK_SIZE],
    key: [u8; AES_128_KEY_SIZE],
    nrounds: usize,
    lastfull: bool,
) {
    let mut prev_key: [u8; 16] = [0; 16];
    let mut next_key: [u8; 16] = [0; 16];

    for i in 0..16 {
        block[i] ^= key[i];
        prev_key[i] = key[i];
    }

    next_aes128_round_key(&prev_key, &mut next_key, 0);

    let mut pk = 0;
    let mut nk: usize = 16;
    let mut turn = true;

    for i in 1..nrounds {
        if turn {
            aes_round(block, &mut next_key, 0);
        } else {
            aes_round(block, &mut prev_key, 0);
        }

        pk = (pk + 16) & 0x10;
        nk = (nk + 16) & 0x10;
        turn = !turn;
        if !turn {
            next_aes128_round_key(&mut next_key, &mut prev_key, i);
        } else {
            next_aes128_round_key(&mut prev_key, &mut next_key, i);
        }
    }
    if lastfull && turn {
        aes_round(block, &mut next_key, 0);
    } else if lastfull && !turn {
        aes_round(block, &mut prev_key, 0);
    } else if turn {
        aes_round(block, &mut next_key, 16);
    } else {
        aes_round(block, &mut prev_key, 16);
    }
}

