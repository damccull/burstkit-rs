//! This module contains structs and enums representing various types of entities in Burstcoin.
//! Burst accounts, transactions, etc.

use std::{convert::TryFrom, usize};
use thiserror::Error;

const INITIAL_CODEWORD: [usize; 17] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
const GEXP: [usize; 32] = [
    1, 2, 4, 8, 16, 5, 10, 20, 13, 26, 17, 7, 14, 28, 29, 31, 27, 19, 3, 6, 12, 24, 21, 15, 30, 25,
    23, 11, 22, 9, 18, 1,
];
const GLOG: [usize; 32] = [
    0, 0, 1, 18, 2, 5, 19, 11, 3, 29, 6, 27, 20, 8, 12, 23, 4, 10, 30, 17, 7, 22, 28, 26, 21, 25,
    9, 16, 13, 14, 24, 15,
];
const CODEWORD_MAP: [usize; 17] = [3, 2, 1, 0, 7, 6, 5, 4, 13, 14, 15, 16, 12, 8, 9, 10, 11];
const ALPHABET: &str = "23456789ABCDEFGHJKLMNPQRSTUVWXYZ";
const BASE32_LENGTH: usize = 13;
const BURST_PREFIX: &str = "BURST-";

/// Represents a Burstcoin account address. _Example: BURST-B982-YTG4-ZS2F-2C55D_
#[derive(Debug, PartialEq)]
pub struct BurstAddress {
    address: String,
}
impl BurstAddress {
    /// Creates a new [`BurstAddress`] from a string.
    ///
    /// # Input
    /// A string in the format: BURST-XXXX-XXXX-XXXX-XXXXX
    ///
    /// # Output
    /// A valid [`BurstAddress`]
    ///
    /// # Example:
    /// ```rust
    /// use burstkit_rs::entity::BurstAddress;
    ///
    /// let burst_address = BurstAddress::new("BURST-B982-YTG4-ZS2F-2C55D".to_string());
    /// ```
    pub fn new(address: String) -> Self {
        BurstAddress { address }
    }

    /// Get a reference to the burst address's address.
    pub fn address(&self) -> &String {
        &self.address
    }
}
impl From<BurstId> for BurstAddress {
    /// Creates a [`BurstAddress`] from a valid [`BurstId`].
    fn from(burst_id: BurstId) -> Self {
        let mut codeword_length = 0;
        let mut codeword: [usize; INITIAL_CODEWORD.len()] = [0; INITIAL_CODEWORD.len()];

        let (mut digits, mut length) = u64_to_digit_array(burst_id.id);

        loop {
            let mut new_length = 0;
            let mut digit32: usize = 0;

            for i in 0..length {
                digit32 = digit32 * 10 + digits[i];
                if digit32 >= 32 {
                    digits[new_length] = digit32 >> 5;
                    digit32 &= 31;
                    new_length += 1;
                } else if new_length > 0 {
                    digits[new_length] = 0;
                    new_length += 1;
                }
            }

            length = new_length;
            codeword[codeword_length] = digit32;
            codeword_length += 1;

            if length == 0 {
                break;
            }
        }

        let mut p: [usize; 4] = [0; 4];
        for i in (0..(BASE32_LENGTH - 1)).rev() {
            let fb = codeword[i] ^ p[3];
            p[3] = p[2] ^ gmult(30, fb);
            p[2] = p[1] ^ gmult(6, fb);
            p[1] = p[0] ^ gmult(9, fb);
            p[0] = gmult(17, fb);
        }

        // Copy these calculated values into the codeword array at specific locations
        // No idea why these locations or what this does. It's magic.
        codeword[13] = p[0];
        codeword[14] = p[1];
        codeword[15] = p[2];
        codeword[16] = p[3];

        let mut outstring = String::from(BURST_PREFIX);

        for (i, e) in CODEWORD_MAP.iter().cloned().enumerate() {
            let codeword_index = e;

            let alphabet_index = codeword[codeword_index];
            outstring.push_str(&ALPHABET[alphabet_index..(alphabet_index + 1)]);
            if (i & 3) == 3 && i < 13 {
                outstring.push('-');
            }
        }
        Self::new(outstring)
    }
}

/// Represents a Burstcoin account Numeric ID number. _Example: 399812073269533888_
#[derive(Debug, PartialEq)]
pub struct BurstId {
    id: u64,
}
impl BurstId {
    /// Creates a new [`BurstId`] from a string.
    ///
    /// # Input
    /// A u64 in the format: 399812073269533888
    ///
    /// # Output
    /// A valid [`BurstId`]
    ///
    /// # Example:
    /// ```rust
    /// use burstkit_rs::entity::BurstId;
    ///
    /// let burst_id = BurstId::new(399812073269533888_u64);
    /// ```
    pub fn new(id: u64) -> Self {
        BurstId { id }
    }

    /// Get a reference to the burst id's id.
    pub fn id(&self) -> &u64 {
        &self.id
    }
}
impl TryFrom<BurstAddress> for BurstId {
    type Error = BurstAddressConversionError;

    /// Attempts to create a [`BurstId`] from a valid [`BurstAddress']
    /// Throws a [`BurstAddressConversionError`] on failure.
    fn try_from(value: BurstAddress) -> Result<Self, Self::Error> {
        let burst_address = value.address.replace(BURST_PREFIX, "");
        println!("DEBUG:::::{}", burst_address);
        let mut codeword = INITIAL_CODEWORD;
        let mut codeword_length = 0;

        for elem in burst_address.chars() {
            let position_in_alphabet = match ALPHABET.chars().position(|s| s == elem) {
                // Skip this iteration if position is none
                None => continue,
                Some(pos) => pos,
            };

            if codeword_length > 16 {
                return Err(BurstAddressConversionError::CodeWordTooLong(codeword));
            }

            let codeword_index = CODEWORD_MAP[codeword_length];
            codeword[codeword_index] = position_in_alphabet;
            codeword_length += 1;
        }

        if codeword_length != 17 || !is_codeword_valid(codeword) {
            return Err(BurstAddressConversionError::CodeWordInvalid(codeword));
        }

        let mut length = BASE32_LENGTH;
        let mut cyper_string_32: Vec<usize> = Vec::new();
        for i in 0..length {
            cyper_string_32.push(codeword[length - i - 1]);
        }

        // Since we're not using a string like BRS, the digits come out of this loop in reverse
        // b_vec is a temporary repository to hold them so we can reverse and add them later
        let mut b_vec: Vec<usize> = Vec::new();

        loop {
            let mut new_length = 0;
            let mut digit_10 = 0;

            for i in 0..length {
                digit_10 = digit_10 * 32 + cyper_string_32[i];

                if digit_10 >= 10 {
                    cyper_string_32[new_length] = digit_10 / 10;
                    digit_10 %= 10;
                    new_length += 1;
                } else if new_length > 0 {
                    cyper_string_32[new_length] = 0;
                    new_length += 1
                }
            }

            length = new_length;

            b_vec.push(digit_10);

            if length == 0 {
                break;
            }
        }

        let mut b_id: u64 = 0;

        // Reverse b_vec to put the digits in the right order, then multiply b_vec by 10
        // to shift the total to the left by a single place each time, and add the new 1's digit
        for i in b_vec.iter().rev().cloned() {
            b_id = b_id * 10 + i as u64;
        }

        Ok(Self::new(b_id))
    }
}

/// Represents errors that can occur when converting [`BurstAddress`] to [`BurstId`].
#[derive(Debug, Error)]
pub enum BurstAddressConversionError {
    #[error("the code word was too long: `{0:?}`")]
    CodeWordTooLong([usize; 17]),
    #[error("the code word was invalid: `{0:?}`")]
    CodeWordInvalid([usize; 17]),
    #[error("unknown Burst address conversion error")]
    Unknown,
}

/// Convert a u64 into a [u8;20]
/// Returns a tuple containing the 20-element array of u8's and a usize representing the
/// number of digits in the supplied id.
fn u64_to_digit_array(number: u64) -> ([usize; 20], usize) {
    let mut n = number; // Get a mutable copy

    let mut num_vec: Vec<usize> = Vec::new(); // Vec is needed for numbers with less than 20 digits
    let mut num_array: [usize; 20] = [0; 20]; // u64::MAX is 20 digits, so no need to be bigger

    // Fill the ved with all the digits
    while n != 0 {
        num_vec.push((n % 10) as usize); // Cast is ok because it will always be inside u8 range
        n /= 10;
    }

    //Reverse the vec and fill the array from 0 to vec.len(), leaving 0s to pad the rest
    for (i, item) in num_vec.iter().rev().enumerate() {
        num_array[i] = item.to_owned();
    }

    (num_array, num_vec.len())
}

fn gmult(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return 0;
    }

    let index = (GLOG[a] + GLOG[b]) % 31;

    GEXP[index]
}

fn is_codeword_valid(codeword: [usize; 17]) -> bool {
    let mut sum = 0;
    for i in 1..5 {
        let mut t = 0;

        for j in 0..31 {
            if j > 12 && j < 27 {
                continue;
            }

            let mut position = j;
            if j > 26 {
                position -= 14;
            }

            t ^= gmult(codeword[position], GEXP[(i * j) % 31]);
        }

        sum |= t;
    }
    sum == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn burst_id_new_creates_new_burst_id() {
        let burst_id = BurstId::new(399812073269533888);
        assert_eq!(
            burst_id,
            BurstId {
                id: 399812073269533888
            }
        );
    }

    #[test]
    fn burst_address_new_creates_new_burst_address() {
        let burst_address = BurstAddress::new("BURST-B982-YTG4-ZS2F-2C55D".to_string());
        assert_eq!(
            burst_address,
            BurstAddress {
                address: "BURST-B982-YTG4-ZS2F-2C55D".to_string()
            }
        );
    }

    #[test]
    fn burst_address_from_converts_burst_id() {
        let burst_id = BurstId::new(399812073269533888);
        let burst_address = BurstAddress::new("BURST-B982-YTG4-ZS2F-2C55D".to_string());

        let from_burst_id = BurstAddress::from(burst_id);

        assert_eq!(from_burst_id, burst_address);
    }

    #[test]
    fn burst_id_tryfrom_converts_burst_address() {
        let burst_id = BurstId::new(399812073269533888);
        let burst_address = BurstAddress::new("BURST-B982-YTG4-ZS2F-2C55D".to_string());

        let from_burst_address = BurstId::try_from(burst_address).unwrap();

        assert_eq!(from_burst_address, burst_id);
    }

    #[test]
    fn u64_to_digit_array_converts_properly() {
        let arr: [usize; 20] = [3, 9, 9, 8, 1, 2, 0, 7, 3, 2, 6, 9, 5, 3, 3, 8, 8, 8, 0, 0];
        let (result, length) = u64_to_digit_array(399812073269533888);
        assert_eq!(arr, result);
        assert_eq!(length, 18);
    }
}
