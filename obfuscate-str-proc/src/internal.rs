use crate::{NonObfuscatedText, ENCRYPTION, OBFUSCATION_KEY};
use chacha20poly1305::{
    aead::{Aead, AeadCore, OsRng},
    ChaCha20Poly1305, Nonce,
};
use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;

pub(crate) fn build_static_obfuscation(non_obfuscated_text: NonObfuscatedText) -> TokenStream {
    let mut full_output = TokenStream::new();
    let (encrypted, nonce) = encrypt_string_literals(non_obfuscated_text.text.value());

    let variable_name = non_obfuscated_text.variable_name;
    let output = quote! {
        static #variable_name: obfuscate_internal::LazyStr = obfuscate_internal::LazyStr::new(|| {
            // leak it
            let obfuscated_string: String = obfuscate_internal::decrypt(#encrypted, #nonce);
            Box::leak(obfuscated_string.into_boxed_str())
        }
    );};

    full_output.extend(TokenStream::from(output));
    full_output
}

fn encrypt_string(plaintext: String) -> Result<(Vec<u8>, Nonce), chacha20poly1305::Error> {
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = ENCRYPTION.encrypt(&nonce, plaintext.as_ref())?;
    Ok((ciphertext, nonce))
}

fn encrypt_string_literals(plaintext: String) -> (Literal, Literal) {
    let (ciphertext, nonce) = encrypt_string(plaintext).expect("Unable to encrypt string");
    (
        Literal::byte_string(&ciphertext),
        Literal::byte_string(&nonce),
    )
}

pub(crate) fn encrypt_string_tokens(plaintext: String) -> TokenStream {
    let (encrypted, nonce) = encrypt_string_literals(plaintext);
    quote! {
        obfuscate_internal::decrypt(#encrypted, #nonce)
    }
    .into()
}

// fn build_obfuscation_imports() -> TokenStream {
//     quote! { use obfuscate::LazyStr; }.into()
// }

pub(crate) fn build_obfuscation_mod() -> TokenStream {
    let key = Literal::byte_string(OBFUSCATION_KEY.as_slice());
    let output = quote! {
        pub mod obfuscate_internal {
            // TODO: refactor
            // imports
            use obfuscate_str::{GenericArray, KeyInit, Lazy, ChaCha20Poly1305, Key, Aead, U32, Nonce};
            pub use obfuscate_str::LazyStr;

            static OBFUSCATION_KEY: &'static [u8; 32] = #key;
            static CIPHER: Lazy<ChaCha20Poly1305> = Lazy::new(|| {
                let key = Key::from_slice(OBFUSCATION_KEY);
                ChaCha20Poly1305::new(key)
            });
            pub fn decrypt(encrypted: &[u8], nonce: &[u8]) -> String {
                    let nonce = Nonce::from_slice(nonce);
                    let plaintext = CIPHER.decrypt(nonce, encrypted).unwrap();
                    String::from_utf8(plaintext).unwrap()
            }
        }
    };

    TokenStream::from(output)
}
