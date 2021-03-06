//! *Library for encrypting and decryping age messages*
//!
//! age is a simple, secure, and modern encryption tool with small explicit keys, no
//! config options, and UNIX-style composability.
//!
//! The age specification is available in a Google Doc here: [A simple file encryption tool & format](https://age-encryption.org/v1).
//!
//! *Caution*: all crate versions prior to 1.0 are beta releases for **testing purposes only**.
//!
//! # Examples
//!
//! ## Key-based encryption
//!
//! ```
//! use std::io::{Read, Write};
//!
//! # fn run_main() -> Result<(), age::Error> {
//! let key = age::SecretKey::generate();
//! let pubkey = key.to_public();
//!
//! let plaintext = b"Hello world!";
//!
//! // Encrypt the plaintext to a ciphertext...
//! let encrypted = {
//!     let encryptor = age::Encryptor::Keys(vec![pubkey]);
//!
//!     let mut encrypted = vec![];
//!     let mut writer = encryptor.wrap_output(&mut encrypted, age::Format::Binary)?;
//!     writer.write_all(plaintext)?;
//!     writer.finish()?;
//!
//!     encrypted
//! };
//!
//! // ... and decrypt the obtained ciphertext to the plaintext again.
//! let decrypted = {
//!     let decryptor = age::Decryptor::with_identities(vec![key.into()]);
//!
//!     let mut decrypted = vec![];
//!     let mut reader = decryptor.trial_decrypt(&encrypted[..])?;
//!     reader.read_to_end(&mut decrypted);
//!
//!     decrypted
//! };
//!
//! assert_eq!(decrypted, plaintext);
//! # Ok(())
//! # }
//!
//! # run_main().unwrap();
//! ```
//!
//! ## Passphrase-based encryption
//!
//! ```
//! use secrecy::Secret;
//! use std::io::{Read, Write};
//!
//! # fn run_main() -> Result<(), age::Error> {
//! let plaintext = b"Hello world!";
//! let passphrase = "this is not a good passphrase";
//!
//! // Encrypt the plaintext to a ciphertext using the passphrase...
//! let encrypted = {
//!     let encryptor = age::Encryptor::Passphrase(Secret::new(passphrase.to_owned()));
//!
//!     let mut encrypted = vec![];
//!     let mut writer = encryptor.wrap_output(&mut encrypted, age::Format::Binary)?;
//!     writer.write_all(plaintext)?;
//!     writer.finish()?;
//!
//!     encrypted
//! };
//!
//! // ... and decrypt the ciphertext to the plaintext again using the same passphrase.
//! let decrypted = {
//!     let decryptor = age::Decryptor::with_passphrase(Secret::new(passphrase.to_owned()));
//!
//!     let mut decrypted = vec![];
//!     let mut reader = decryptor.trial_decrypt(&encrypted[..])?;
//!     reader.read_to_end(&mut decrypted);
//!
//!     decrypted
//! };
//!
//! assert_eq!(decrypted, plaintext);
//! # Ok(())
//! # }
//! # run_main().unwrap();
//! ```

// Catch documentation errors caused by code changes.
#![deny(intra_doc_link_resolution_failure)]
#![deny(missing_docs)]

/// Format of output
pub enum Format {
    /// age binary format
    Binary,
    /// ascii armor
    AsciiArmor,
}

mod error;
mod format;
pub mod keys;
mod openssh;
mod primitives;
mod protocol;
mod util;

pub use error::Error;
pub use keys::SecretKey;
pub use primitives::stream::StreamReader;
pub use protocol::{Callbacks, Decryptor, Encryptor};

#[cfg(feature = "cli-common")]
pub mod cli_common;

/// Helper for fuzzing the Header parser and serializer.
#[cfg(fuzzing)]
pub fn fuzz_header(data: &[u8]) {
    if let Ok(header) = format::Header::read(data) {
        let mut buf = Vec::with_capacity(data.len());
        if let Ok(_) = header.write(&mut buf) {
            assert_eq!(buf, data);
        }
    }
}
