use std::ops::Deref;

use crate::{KeyMode, Mode, NonObfuscatedText, ENCRYPTION, OBFUSCATION_KEY};
use chacha20poly1305::{
    aead::{Aead, AeadCore, OsRng},
    ChaCha20Poly1305, Nonce,
};
use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use rand::RngCore;

pub(crate) fn build_static_obfuscation(
    non_obfuscated_text: NonObfuscatedText,
) -> Result<TokenStream, chacha20poly1305::Error> {
    let mut full_output = TokenStream::new();
    let (encrypted, nonce) = encrypt_string_literals(non_obfuscated_text.text.value())?;
    let variable_name = non_obfuscated_text.variable_name;
    let visibility = non_obfuscated_text.visibility;

    let output = quote! {
        #visibility static #variable_name: crate::obfuscate_internal::LazyStr = crate::obfuscate_internal::LazyStr::new(|| {
            let obfuscated_string: String = crate::obfuscate_internal::decrypt(#encrypted, #nonce);
            Box::leak(obfuscated_string.into_boxed_str())
        }
    );};

    full_output.extend(TokenStream::from(output));
    Ok(full_output)
}

fn encrypt_string(plaintext: String) -> Result<(Vec<u8>, Nonce), chacha20poly1305::Error> {
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = ENCRYPTION.encrypt(&nonce, plaintext.as_ref())?;
    Ok((ciphertext, nonce))
}

fn encrypt_string_literals(
    plaintext: String,
) -> Result<(Literal, Literal), chacha20poly1305::Error> {
    encrypt_string(plaintext).map(|(ciphertext, nonce)| {
        (
            Literal::byte_string(ciphertext.deref()),
            Literal::byte_string(nonce.deref()),
        )
    })
}

pub(crate) fn encrypt_string_tokens(
    plaintext: String,
) -> Result<TokenStream, chacha20poly1305::Error> {
    encrypt_string_literals(plaintext).map(|(cipher_lit, nonce_lit)| {
        quote! {
            obfuscate_internal::decrypt(#cipher_lit, #nonce_lit)
        }
        .into()
    })
}

pub(crate) fn build_obfuscation_imports() -> proc_macro2::TokenStream {
    quote! {
        use obfuscate_str::{GenericArray, KeyInit, Lazy, ChaCha20Poly1305, Key, Aead, U32, Nonce};
        pub use obfuscate_str::{LazyStr, FromHex};
        use std::os::unix::ffi::OsStrExt;
    }
}

fn build_embedded_cipher_block(obfus_key: Literal, junk: Literal) -> proc_macro2::TokenStream {
    quote! {
        static OBFUSCATION_KEY: &'static [u8; 32] = #obfus_key;
        static JUNK: &'static [u8; 32] = #junk;
        static CIPHER: Lazy<ChaCha20Poly1305> = Lazy::new(|| {
            let mut key = OBFUSCATION_KEY.clone();
            key.iter_mut().zip(JUNK.iter()).for_each(|(b, k)| *b ^= k);
            ChaCha20Poly1305::new(Key::from_slice(&key))
        });
    }
}

fn build_env_cipher_block() -> proc_macro2::TokenStream {
    // let var = std::env::var_os("OBFUSCATION_KEY").expect("OBFUSCATION_KEY is not set");
    quote! {
        static OBFUSCATION_KEY: Lazy<Key> = Lazy::new(|| {
            let var = std::env::var_os("OBFUSCATION_KEY")
                        .expect("OBFUSCATION_KEY is not set");
            let bytes = <[u8; 32]>::from_hex(var.as_bytes()).unwrap();

            Key::clone_from_slice(&bytes)
            // Key::clone_from_slice(var.as_bytes())
        });
        static CIPHER: Lazy<ChaCha20Poly1305> = Lazy::new(|| {
            let key: &Key = Lazy::force(&OBFUSCATION_KEY);
            ChaCha20Poly1305::new(key)
        });
    }
}

pub(crate) fn build_obfuscation_mod(keymode: KeyMode) -> TokenStream {
    let cipher_block = match keymode.mode() {
        Mode::None | Mode::Embedded => {
            let mut other = [0u8; 32];
            OsRng.fill_bytes(&mut other);

            let junk = other;
            let key = OBFUSCATION_KEY.as_slice();

            // XOR the key with the random arr
            other.iter_mut().zip(key.iter()).for_each(|(b, k)| *b ^= k);

            let cipher_block = build_embedded_cipher_block(
                Literal::byte_string(&other),
                Literal::byte_string(&junk),
            );
            cipher_block
        }
        Mode::Env => {
            let key = OBFUSCATION_KEY
                .as_slice()
                .iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<String>();
            // .collect::<Vec<_>>();
            eprintln!("Len: {}, Key: {:?}", key.len(), key);
            build_env_cipher_block()
        }
    };
    // let mut other = [0u8; 32];
    // OsRng.fill_bytes(&mut other);
    //
    // let junk = other;
    // let key = OBFUSCATION_KEY.as_slice();
    //
    // // XOR the key with the random arr
    // other.iter_mut().zip(key.iter()).for_each(|(b, k)| *b ^= k);
    //
    // let cipher_block =
    //     build_embedded_cipher_block(Literal::byte_string(&other), Literal::byte_string(&junk));

    let decrypt_block = quote! {
        pub fn decrypt(encrypted: &[u8], nonce: &[u8]) -> String {
                let nonce = Nonce::from_slice(nonce);
                let plaintext = CIPHER.decrypt(nonce, encrypted).unwrap();
                String::from_utf8(plaintext).unwrap()
        }
    };
    let imports_block = build_obfuscation_imports();
    let output = quote! {
        pub mod obfuscate_internal {
            #imports_block

            #cipher_block

            #decrypt_block
        }
    };

    TokenStream::from(output)
}
