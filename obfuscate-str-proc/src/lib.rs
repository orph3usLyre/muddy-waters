#![feature(proc_macro_diagnostic)]

use chacha20poly1305::{
    aead::{KeyInit, OsRng},
    ChaCha20Poly1305, Key,
};
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::LitStr;

mod internal;
use internal::*;

mod meta;
use meta::*;

// use https://docs.rs/chacha20poly1305/latest/chacha20poly1305/

/// Used to generate the key at build time
/// Kept in a separate static to recover the key and embed it into the binary
pub(crate) static OBFUSCATION_KEY: Lazy<Key> =
    Lazy::new(|| ChaCha20Poly1305::generate_key(&mut OsRng));

/// Used to generate the key at build time
pub(crate) static ENCRYPTION: Lazy<ChaCha20Poly1305> =
    Lazy::new(|| ChaCha20Poly1305::new(&OBFUSCATION_KEY));

#[proc_macro]
pub fn obfuscate_init(_input: TokenStream) -> TokenStream {
    let mut output: TokenStream = TokenStream::new();
    // output.extend(build_obfuscation_imports());
    output.extend(build_obfuscation_mod());

    output
}

#[proc_macro]
pub fn o(input: TokenStream) -> TokenStream {
    let text = parse_macro_input!(input as LitStr).value();
    encrypt_string_tokens(text)
}

#[proc_macro_attribute]
pub fn obfuscate_str(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as NonObfuscatedText);
    let full_output: TokenStream = build_static_obfuscation(input);
    TokenStream::from(full_output)
}

#[proc_macro]
pub fn obfuscate_strs(input: TokenStream) -> TokenStream {
    let NonObfuscatedTexts { texts } = parse_macro_input!(input as NonObfuscatedTexts);
    let mut output: TokenStream = TokenStream::new();
    let iter = texts.into_iter().map(build_static_obfuscation);
    output.extend(iter);
    output
}
