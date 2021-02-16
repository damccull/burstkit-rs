//! # Burstcoin Toolkit for rustlang
//! 
//! ## What is this?
//! This kit is intended to provide burstcoin-specific utility code in a library form
//! for rust applications.
//! 
//! ## What isn't this?
//! This kit does not implement anything that can be found in the rust ecosystem already, such as
//! various types of cryptography.
//! 
//! For Elliptical Curve Diffie Hellman (ECDH), [x25519-dalek] is recommended.
//! 
//! 
//! 
//! 
//! [x25519-dalek]: https://github.com/dalek-cryptography/x25519-dalek "x25519-dalek"

// pub mod crypto;
pub mod entity;
// pub mod service; //TODO: Check if this is actually needed
pub mod util;
pub mod error;


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
