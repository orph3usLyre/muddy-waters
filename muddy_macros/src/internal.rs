use crate::{KeyMode, NonObfuscatedText, Result, ENCRYPTION, KEY};
use chacha20poly1305::{
    aead::{Aead, AeadCore, OsRng},
    ChaCha20Poly1305, Nonce,
};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal};
use quote::quote;
use rand::RngCore;
use std::fmt::Write;

/// Creates the internal `muddy_internal` mod along with the appropriate imports, cipher, and
/// decryption function
pub(crate) fn build_obfuscation_mod(
    key_ident: &Ident,
    cipher_ident: &Ident,
    keymode: KeyMode,
) -> TokenStream {
    let cipher_block = match keymode {
        KeyMode::Embedded => {
            let mut other = [0u8; 32];
            OsRng.fill_bytes(&mut other);

            let junk = other;
            let key = KEY.as_slice();

            other.iter_mut().zip(key.iter()).for_each(|(b, k)| *b ^= k);

            build_embedded_cipher_block(
                key_ident,
                cipher_ident,
                &Literal::byte_string(&other),
                &Literal::byte_string(&junk),
            )
        }
        KeyMode::Env(env) => {
            let key = KEY.as_slice().iter().fold(String::new(), |mut out, c| {
                let _ = write!(out, "{c:02X}");
                out
            });
            let env_name = match env {
                Some(ref s) => s.as_str(),
                _ => "MUDDY",
            };
            eprintln!("Please, set {env_name} env variable with content:");
            #[cfg(windows)]
            // language=cmd
            eprintln!(r#"set "{env_name}={key}""#);
            #[cfg(not(windows))]
            // language=sh
            eprintln!(r#"{env_name}="{key}""#);
            build_env_cipher_block(key_ident, cipher_ident, env_name)
        }
    };

    let decrypt_block = quote! {
        pub fn decrypt(encrypted: &[u8], nonce: &[u8]) -> String {
            let nonce = Nonce::from_slice(nonce);
            let plaintext = #cipher_ident.decrypt(nonce, encrypted).unwrap();
            String::from_utf8(plaintext).unwrap()
        }
    };
    let imports_block = build_obfuscation_imports();
    let output = quote! {
        pub mod muddy_internal {
            #imports_block

            #cipher_block

            #decrypt_block
        }
    };

    TokenStream::from(output)
}

/// Creates the inports for the `muddy_internal` mod
fn build_obfuscation_imports() -> proc_macro2::TokenStream {
    #[cfg(unix)]
    let use_os_str_ext = quote! {
        use std::os::unix::ffi::OsStrExt;
    };
    #[cfg(target_os = "wasi")]
    let use_os_str_ext = quote! {
        use std::os::wasi::ffi::OsStrExt;
    };
    #[cfg(windows)]
    let use_os_str_ext = quote! {
        use std::os::windows::ffi::OsStrExt;
    };
    quote! {
        use muddy::{GenericArray, KeyInit, Lazy, ChaCha20Poly1305, Key, Aead, U32, Nonce};
        pub use muddy::{LazyStr, FromHex};
        #use_os_str_ext
    }
}

/// Creates the cipher block with an embedded key
fn build_embedded_cipher_block(
    key_ident: &Ident,
    cipher_ident: &Ident,
    obfus_key: &Literal,
    junk: &Literal,
) -> proc_macro2::TokenStream {
    quote! {
        static #key_ident: &'static [u8; 32] = #obfus_key;
        static JUNK: &'static [u8; 32] = #junk;
        static #cipher_ident: Lazy<ChaCha20Poly1305> = Lazy::new(|| {
            let mut key = #key_ident.clone();
            key.iter_mut().zip(JUNK.iter()).for_each(|(b, k)| *b ^= k);
            ChaCha20Poly1305::new(Key::from_slice(&key))
        });
    }
}

fn build_env_cipher_block(
    key_ident: &Ident,
    cipher_ident: &Ident,
    env_ident: &str,
) -> proc_macro2::TokenStream {
    quote! {
        static #key_ident: Lazy<Key> = Lazy::new(|| {
            let Some(var) = std::env::var_os(#env_ident) else {
                #[cfg(debug_assertions)]
                panic!("Need to set {} env variable", #env_ident);
                #[cfg(not(debug_assertions))]
                panic!();
            };
            let Ok(bytes) = <[u8; 32]>::from_hex(var.as_encoded_bytes()) else {
                #[cfg(debug_assertions)]
                panic!("Can't get bytes from {} env variable, secret key needs to be a hex string with 64 symbols length.", #env_ident);
                #[cfg(not(debug_assertions))]
                panic!();
            };

            Key::clone_from_slice(&bytes)
        });
        static #cipher_ident: Lazy<ChaCha20Poly1305> = Lazy::new(|| {
            let key: &Key = Lazy::force(&#key_ident);
            ChaCha20Poly1305::new(key)
        });
    }
}

/// Recreates the static text as a [`super::Lazy<&'static str>`]
/// by decrypting the text at first call, and leaking the [String]
pub(crate) fn build_static_obfuscation(
    non_obfuscated_text: &NonObfuscatedText,
) -> Result<TokenStream> {
    let (encrypted, nonce) = encrypt_string_literals(&non_obfuscated_text.text.value())?;
    let variable_name = &non_obfuscated_text.variable_name;
    let visibility = &non_obfuscated_text.visibility;

    let output = quote! {
        #visibility static #variable_name: crate::muddy_internal::LazyStr = crate::muddy_internal::LazyStr::new(|| {
            let obfuscated_string: String = crate::muddy_internal::decrypt(#encrypted, #nonce);
            obfuscated_string.leak()
        }
    );};

    Ok(TokenStream::from(output))
}

fn encrypt_string(plaintext: &str) -> Result<(Vec<u8>, Nonce)> {
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = ENCRYPTION.encrypt(&nonce, plaintext.as_ref())?;
    Ok((ciphertext, nonce))
}

fn encrypt_string_literals(plaintext: &str) -> Result<(Literal, Literal)> {
    encrypt_string(plaintext).map(|(ciphertext, nonce)| {
        (
            Literal::byte_string(&ciphertext),
            Literal::byte_string(&nonce),
        )
    })
}

/// Recreates the static as a function call to decrypt it from the lazy cipher
pub(crate) fn encrypt_string_tokens(plaintext: &str) -> Result<TokenStream> {
    encrypt_string_literals(plaintext).map(|(cipher_lit, nonce_lit)| {
        quote! {
            muddy_internal::decrypt(#cipher_lit, #nonce_lit)
        }
        .into()
    })
}
