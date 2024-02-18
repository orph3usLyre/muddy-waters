#![warn(clippy::pedantic)]
#![feature(proc_macro_diagnostic)]

use chacha20poly1305::{
    aead::{KeyInit, OsRng},
    ChaCha20Poly1305, Key,
};
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};

mod internal;
mod meta;

use internal::*;
use meta::*;

/// Used to generate the key at build time
/// Kept seperately to embed in the target binary
pub(crate) static OBFUSCATION_KEY: Lazy<Key> =
    Lazy::new(|| ChaCha20Poly1305::generate_key(&mut OsRng));

/// Used to generate text encryptions at build time
pub(crate) static ENCRYPTION: Lazy<ChaCha20Poly1305> =
    Lazy::new(|| ChaCha20Poly1305::new(&OBFUSCATION_KEY));

#[proc_macro]
pub fn obfuscate_init(_input: TokenStream) -> TokenStream {
    let mut output: TokenStream = TokenStream::new();
    output.extend(build_obfuscation_mod());

    output
}

#[proc_macro]
pub fn o(input: TokenStream) -> TokenStream {
    let text = parse_macro_input!(input as LitStr).value();
    let Ok(out) = encrypt_string_tokens(text) else {
        panic!("Encountered encryption error");
    };
    out
}

#[proc_macro_attribute]
pub fn obfuscate_str(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as NonObfuscatedText);
    let span = input.text.span().clone();
    let Ok(out) = build_static_obfuscation(input) else {
        span.unwrap().error("Encountered encryption error").emit();
        return TokenStream::new();
    };
    out
}

#[proc_macro]
pub fn obfuscate_strs(input: TokenStream) -> TokenStream {
    let NonObfuscatedTexts { texts } = parse_macro_input!(input as NonObfuscatedTexts);
    let mut output: TokenStream = TokenStream::new();
    let Ok(iter): Result<Vec<TokenStream>, chacha20poly1305::Error> =
        texts.into_iter().map(build_static_obfuscation).collect()
    else {
        panic!("Encountered encryption error");
    };
    output.extend(iter);
    output
}
