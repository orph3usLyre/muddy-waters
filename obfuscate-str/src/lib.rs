#![warn(clippy::pedantic)]

mod lazy_str;

pub use lazy_str::LazyStr;
pub use obfuscate_str_proc::{obfuscate_init, obfuscate_str, obfuscate_strs};
// renaming
pub use obfuscate_str_proc::{o, obfuscate_str as hide, obfuscate_strs as hide_all};

// re-exports
pub use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng},
    consts::U32,
    ChaCha20Poly1305, Key, Nonce,
};
pub use once_cell::sync::Lazy;
