/// Holds an account passphrase. Will zero out its memory on drop.
#[derive(Debug)]
pub struct Passphrase {
    value: String,
}
impl Passphrase {
    fn new(passphrase: &str) -> Self {
        Self {
            value: passphrase.to_string(),
        }
    }

    /// Get a reference to the passphrase's key.
    pub fn value(&self) -> &str {
        &self.value
    }
}
impl Drop for Passphrase {
    fn drop(&mut self) {
        let inner = std::mem::take(&mut self.value);
        let mut vec = inner.into_bytes();
        // vec[..].fill(0);
        for v in vec.iter_mut() {
            *v = 0_u8;
        }
    }
}

#[derive(Debug)]
pub struct PrivateKey {
    value: [u8],
}
impl PrivateKey {
    /// Get a reference to the private key's value.
    pub fn value(&self) -> &[u8] {
        &self.value
    }
}
// impl Drop for PrivateKey {
//     fn drop(&mut self) {
//         let inner = std::mem::take(&mut self.value);
//         let mut vec = inner.into_bytes();
//         // vec[..].fill(0);
//         for v in vec.iter_mut() {
//             *v = 0_u8;
//         }
//     }
// }

#[derive(Debug)]
pub struct PublicKey {
    value: String,
}
impl PublicKey {
    /// Get a reference to the public key's value.
    pub fn value(&self) -> &String {
        &self.value
    }
}
impl Drop for PublicKey {
    fn drop(&mut self) {
        let inner = std::mem::take(&mut self.value);
        let mut vec = inner.into_bytes();
        // vec[..].fill(0);
        for v in vec.iter_mut() {
            *v = 0_u8;
        }
    }
}
