#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    rustdoc::broken_intra_doc_links,
    missing_docs
)]

//! # `muddy_macro`
//!
//! This crate provides macros that are reexported by the [muddy](https://github.com/orph3usLyre/muddy-waters/tree/main/muddy) crate and should **not** be used
//! outside the context of that crate.
//!
//!

use chacha20poly1305::{
    aead::{KeyInit, OsRng},
    ChaCha20Poly1305, Key,
};
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use rand::{
    distributions::{Alphanumeric, DistString},
    Rng,
};
use syn::{parse_macro_input, LitStr};

mod internal;
mod meta;

use internal::{build_obfuscation_mod, build_static_obfuscation, encrypt_string_tokens};
use meta::{KeyMode, NonObfuscatedText, NonObfuscatedTexts};

/// Used to generate the key at build time
///
/// Kept separately to embed in the target binary
pub(crate) static KEY: Lazy<Key> = Lazy::new(|| {
    use const_hex::FromHex;
    std::env::var_os("MUDDY_KEY").map_or_else(|| ChaCha20Poly1305::generate_key(&mut OsRng), |muddy_key| {
         let Ok(bytes) = <[u8; 32]>::from_hex(muddy_key.as_encoded_bytes()) else {
             panic!("Can't get bytes from 'MUDDY_KEY' env variable, secret key needs to be a hex string with 64 symbols length.");
         };
         Key::clone_from_slice(&bytes)
     })
});

/// Used to generate text encryptions at build time
pub(crate) static ENCRYPTION: Lazy<ChaCha20Poly1305> = Lazy::new(|| ChaCha20Poly1305::new(&KEY));

/// Random string identifiers for the key and cipher
pub(crate) static IDENTS: Lazy<(String, String)> = Lazy::new(|| {
    static A: u8 = b'A';
    static Z: u8 = b'Z';
    (
        format!(
            "{}{}",
            OsRng.gen_range(A..=Z) as char,
            Alphanumeric.sample_string(&mut OsRng, 16).to_uppercase()
        ),
        format!(
            "{}{}",
            OsRng.gen_range(A..=Z) as char,
            Alphanumeric.sample_string(&mut OsRng, 16).to_uppercase()
        ),
    )
});

static ERROR_MESSAGE: &str = "Encountered encryption error";
static DEFAULT_ENV: &str = "MUDDY";

pub(crate) type Result<T> = std::result::Result<T, chacha20poly1305::Error>;

// NOTE: All doc tests in this crate are marked with `ignore` since the `muddy_macros` cannot work
// without the `muddy` crate as a wrapper

#[proc_macro]
/// Initialization macro that must be called at the crate root.
/// This sets up the scaffolding for the lazy decryption at runtime.
///
/// # Modes
///
/// Optional values that can be provided are:
/// - "embed" (default)
/// - "env"
///
/// ### "embed"
/// If the "embed" mode is chosen, the key will be embedded in the binary with minor obfuscation
/// (`XORed` with a random array).
///
/// ### "env"
/// If "env" is provided, the key will not be embedded in the binary.
/// An additional value may be provided to set the env key identifier:
///
/// ```ignore
///
/// muddy_init!("env", "MY_KEY");
/// ```
///
/// Or at build time:
///
/// ```ignore
///
/// muddy_init!("env");
///
/// // run with `MUDDY='MY_KEY_NAME' cargo b`
///
/// ```
///
///
pub fn muddy_init(input: TokenStream) -> TokenStream {
    let keymode: KeyMode = parse_macro_input!(input as KeyMode);
    let key_ident = Ident::new(&IDENTS.0, Span::call_site());
    let cipher_ident = Ident::new(&IDENTS.1, Span::call_site());
    build_obfuscation_mod(&key_ident, &cipher_ident, keymode)
}

#[proc_macro]
/// Obfuscates a literal text.
/// (aka [`m!`])
///
/// # Example
///
/// ```ignore
/// println!("{}", muddy_str!("my text"));
/// ```
///
pub fn muddy_str(input: TokenStream) -> TokenStream {
    let text = parse_macro_input!(input as LitStr);
    let Ok(out) = encrypt_string_tokens(&text.value()) else {
        syn::Error::new(text.span(), ERROR_MESSAGE).to_compile_error();
        return TokenStream::new();
    };
    out
}

#[proc_macro]
/// Obfuscates a literal text.
/// (aka [`muddy_str!`])
///
/// # Example
///
/// ```ignore
/// println!("{}", m!("my text"));
/// ```
///
pub fn m(input: TokenStream) -> TokenStream {
    muddy_str(input)
}

#[proc_macro_attribute]
/// Obfuscates a static string expression as an attribute macro.
///
/// # Example
///
/// ```ignore
/// #[muddy]
/// static MY_STR: &str = "my text";
/// ```
///
pub fn muddy(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as NonObfuscatedText);
    let Ok(out) = build_static_obfuscation(&input) else {
        syn::Error::new(input.text.span(), ERROR_MESSAGE).to_compile_error();
        return TokenStream::new();
    };
    out
}

#[proc_macro]
/// Obfuscates multiple static string expressions.
///
/// # Example
///
/// ```ignore
/// muddy_all! {
///     pub static MY_FIRST_STR: &str = "my text";
///     static MY_SECOND_STR: &str = "my second text";
/// }
/// ```
///
pub fn muddy_all(input: TokenStream) -> TokenStream {
    let non_obfuscated = parse_macro_input!(input as NonObfuscatedTexts);
    let mut output: TokenStream = TokenStream::new();
    let Ok(iter): Result<Vec<TokenStream>> = non_obfuscated
        .texts
        .iter()
        .map(build_static_obfuscation)
        .collect()
    else {
        syn::Error::new(Span::call_site(), ERROR_MESSAGE).to_compile_error();
        return TokenStream::new();
    };
    output.extend(iter);
    output
}
