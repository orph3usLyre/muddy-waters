mod lazy_str;

pub use lazy_str::LazyStr;
pub use obfuscate_str_proc::*;

// reexports
pub use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng},
    consts::U32,
    ChaCha20Poly1305, Key, Nonce,
};
pub use once_cell::sync::Lazy;
