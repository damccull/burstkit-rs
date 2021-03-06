use std::usize;

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

/// Represents a Burstcoin account address.
#[derive(Debug, PartialEq)]
pub struct BurstAddress {
    address: String,
}
impl BurstAddress {
    pub fn new(address: String) -> Self {
        BurstAddress { address }
    }
}
impl From<BurstId> for BurstAddress {
    fn from(burst_id: BurstId) -> Self {
        let mut codeword_length = 0;
        let mut codeword: [usize; 17] = [0; 17];

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
        Self { address: outstring }
    }
}

/// Represents a Burstcoin account Numeric ID number.
#[derive(Debug, PartialEq)]
pub struct BurstId {
    id: u64,
}
impl BurstId {
    pub fn new(id: u64) -> Self {
        BurstId { id }
    }
}
impl From<BurstAddress> for BurstId {
    fn from(burst_address: BurstAddress) -> Self {
        //TODO: Run conversion
        todo!();
    }
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
    fn burst_id_from_converts_burst_adress() {
        let burst_id = BurstId::new(399812073269533888);
        let burst_address = BurstAddress::new("BURST-B982-YTG4-ZS2F-2C55D".to_string());

        let from_burst_address = BurstId::from(burst_address);

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
