mod gf256;

use gf256::mul;

const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

const RSBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

pub const MIX_COLUMNS_MATRIX: [[u8; 4]; 4] =
    [[2, 3, 1, 1], [1, 2, 3, 1], [1, 1, 2, 3], [3, 1, 1, 2]];

const MIX_COLUMNS_MATRIX_INV: [[u8; 4]; 4] = [
    [14, 11, 13, 9],
    [9, 14, 11, 13],
    [13, 9, 14, 11],
    [11, 13, 9, 14],
];

fn add_round_key(block: &mut [u8], key: &[u8]) {
    for (x, k) in block.iter_mut().zip(key) {
        *x ^= k;
    }
}

pub fn encrypt_round(block: &mut [u8], round_key: &[u8]) {
    sub_bytes(block, &SBOX);
    shift_rows_left(block);
    mix_columns(block, &MIX_COLUMNS_MATRIX);
    add_round_key(block, round_key);
}

pub fn decrypt_round(block: &mut [u8], round_key: &[u8]) {
    add_round_key(block, round_key);
    mix_columns(block, &MIX_COLUMNS_MATRIX_INV);
    shift_rows_right(block);
    sub_bytes(block, &RSBOX);
}

fn sub_bytes(block: &mut [u8], sbox: &[u8; 256]) {
    for x in block.iter_mut() {
        *x = sbox[*x as usize];
    }
}

fn shift_rows_left(block: &mut [u8]) {
    let tmp = block[1];
    block[1] = block[4 + 1];
    block[4 + 1] = block[2 * 4 + 1];
    block[2 * 4 + 1] = block[3 * 4 + 1];
    block[3 * 4 + 1] = tmp;

    block.swap(2, 2 * 4 + 2);
    block.swap(4 + 2, 3 * 4 + 2);

    let tmp = block[3 * 4 + 3];
    block[3 * 4 + 3] = block[2 * 4 + 3];
    block[2 * 4 + 3] = block[4 + 3];
    block[4 + 3] = block[3];
    block[3] = tmp;
}

fn shift_rows_right(block: &mut [u8]) {
    let tmp = block[3 * 4 + 1];
    block[3 * 4 + 1] = block[2 * 4 + 1];
    block[2 * 4 + 1] = block[4 + 1];
    block[4 + 1] = block[1];
    block[1] = tmp;

    block.swap(2, 2 * 4 + 2);
    block.swap(4 + 2, 3 * 4 + 2);

    let tmp = block[3];
    block[3] = block[4 + 3];
    block[4 + 3] = block[2 * 4 + 3];
    block[2 * 4 + 3] = block[3 * 4 + 3];
    block[3 * 4 + 3] = tmp;
}

pub fn mix_columns(block: &mut [u8], matrix: &[[u8; 4]; 4]) {
    let mut result = [0u8; 16];

    for i in 0..4 {
        for j in 0..4 {
            let mut value = 0;

            for k in 0..4 {
                value ^= mul(block[i * 4 + k], matrix[j][k])
            }
            result[i * 4 + j] = value;
        }
    }

    block.copy_from_slice(&result);
}

// Test data used comes from here: http://www.herongyang.com/Cryptography/AES-Example-Vector-of-AES-Encryption.html
#[test]
fn test_encrypt_round() {
    use hex_literal::hex;

    let mut block: [u8; 16] = hex!("00102030405060708090a0b0c0d0e0f0");

    let key: [u8; 16] = hex!("d6aa74fdd2af72fadaa678f1d6ab76fe");

    let result: [u8; 16] = hex!("89d810e8855ace682d1843d8cb128fe4");

    encrypt_round(&mut block, &key);

    assert_eq!(&block, &result)
}

#[test]
fn test_decrypt_round() {
    // This test is lazy and based on the assumption that it encryption is correct and you can decrypt, it's good
    use hex_literal::hex;

    let mut block: [u8; 16] = hex!("00102030405060708090a0b0c0d0e0f0");

    let key: [u8; 16] = hex!("d6aa74fdd2af72fadaa678f1d6ab76fe");

    let result: [u8; 16] = hex!("89d810e8855ace682d1843d8cb128fe4");

    let plaintext = block.clone();

    encrypt_round(&mut block, &key);

    assert_eq!(&block, &result);

    decrypt_round(&mut block, &key);

    assert_eq!(&block, &plaintext);
}

#[test]
fn test_add_round_key() {
    use hex_literal::hex;

    let mut block: [u8; 16] = hex!("00112233445566778899aabbccddeeff");

    let key: [u8; 16] = hex!("000102030405060708090a0b0c0d0e0f");

    let result: [u8; 16] = hex!("00102030405060708090a0b0c0d0e0f0");

    add_round_key(&mut block, &key);

    assert_eq!(&block, &result)
}

#[test]
fn test_sub_bytes() {
    use hex_literal::hex;

    let mut block: [u8; 16] = hex!("00102030405060708090a0b0c0d0e0f0");

    let result: [u8; 16] = hex!("63cab7040953d051cd60e0e7ba70e18c");

    let initial = block.clone();

    sub_bytes(&mut block, &SBOX);

    assert_eq!(&block, &result);

    sub_bytes(&mut block, &RSBOX);

    assert_eq!(&block, &initial);
}

#[test]
fn test_shift_rows() {
    use hex_literal::hex;

    let mut block: [u8; 16] = hex!("63cab7040953d051cd60e0e7ba70e18c");

    let result: [u8; 16] = hex!("6353e08c0960e104cd70b751bacad0e7");

    let initial = block.clone();

    shift_rows_left(&mut block);

    assert_eq!(&block, &result);

    shift_rows_right(&mut block);

    assert_eq!(&block, &initial);
}

#[test]
fn test_mix_columns() {
    use hex_literal::hex;

    let mut block: [u8; 16] = hex!("6353e08c0960e104cd70b751bacad0e7");
    let result: [u8; 16] = hex!("5f72641557f5bc92f7be3b291db9f91a");

    mix_columns(&mut block, &MIX_COLUMNS_MATRIX);

    assert_eq!(&block, &result)
}
