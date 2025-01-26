#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    rustdoc::broken_intra_doc_links,
    missing_docs
)]
#![no_std]

//! # muddy
//!
//! muddy is a literal string obfuscation library, designed to provide an easy way of avoiding simple static binary analysis tools such as `strings` or YARA rules.
//! It functions by encrypting texts at build time, and embedding an in-place decrypter that is evaluated at runtime.
//!
//!
//! ## Usage & Examples
//!
//! ```rust
//! // examples/simple.rs
//! use muddy::muddy;
//!
//! let non_obfuscated = "notsupersecret9001";
//! let obfuscated = muddy!("supersecret42");
//! println!("{}", non_obfuscated);
//! println!("{}", obfuscated);
//! ```  
//!     
//!    
//! > Compile this example and grep the binary for `obfuscated`:  
//! >
//! > `cargo b --example simple`  
//! >
//! > `strings ./target/debug/examples/simple | grep obfuscated`    
//! > Only the second nonobfuscated line should appear.
//! >  
//!   
//!   
//! `muddy` primarily provides the exported `muddy!()` and `muddy_unchecked!()`, macros, which
//! each take a literal text as input, encrypt it at buildtime, and generate
//! an in-place decrypter which is evaluated to the plaintext `&'static str` at runtime.
//!
//!
//! By default, these macros will encrypt literal strings with the [`chacha20poly1305`] implementation
//! and embed the key inside the binary.  
//!
//! #### Runtime-provided decryption
//!
//! If the env argument is provided to the macro invocation, the deobfuscation key
//! will not be embedded into the binary. Instead, it will be generated at buildtime
//! and must be provided at runtime.
//!
//! ```should_panic
//! use muddy::muddy;
//!
//! let obfuscated = muddy!(env, "supersecret42");
//! println!("{}", obfuscated);
//! ```  
//!
//!
//! Running `cargo b` will print out `MUDDY='<SOME_KEY>'` to stderr.
//!                                                                                              
//! This env will then need to be set at runtime, otherwise the program will panic: `MUDDY='<SOME_KEY>' cargo r`
//!
//! > You may also set your own key identifiers: `muddy!(env = "MY_KEY_NAME", "supersecret42")`
//! >
//!
//! ### `muddy_unchecked!()`
//!
//! The difference between `muddy!()` and `muddy_unchecked!()` is that the `muddy!()` macro
//! checks that the macro invocation is not evaluated multiple times.
//! Opt for `muddy_unchecked!()` if you can uphold this guarantee.
//!
//! ```should_panic
//! use muddy::muddy_unchecked;
//!
//! fn f() -> &'static str {
//!   muddy!(env, "supersecret1")
//! }
//!
//! fn f2() -> &'static str {
//!   muddy_unchecked!(env, "supersecret42")
//! }
//!
//! fn f3() -> &'static str {
//!   muddy_unchecked!(env, "supersecret9001")
//! }
//!
//! for _ in 0..2 {
//!   println!("{}", f()); // <----- fine, since `muddy!()` provides checks against multiple evaluations
//! }
//!
//! for _ in 0..2 {
//!   println!("{}", f2()); // <---- panics at the second evaluation
//! }
//!
//! for _ in 0..2 {
//!   std::thread::spawn(|| {
//!     println!("{}", f3()); // <-  panics at the second evaluation
//!   });
//! }
//! ```
//!
//! Alternatively:
//! ```rust
//! use muddy::muddy_unchecked;
//!
//! // only evaluated once
//! let plaintext = muddy_unchecked!("supersecret1337");
//! for _ in 0..2 {
//!   println!("{}", plaintext); // <--- fine
//! }
//!
//! for _ in 0..2 {
//!   std::thread::spawn(move || {
//!     println!("{}", plaintext); // <- also fine
//!   });
//! }
//! ```  
//!
//!
//! ### Note on obfuscation and encryption
//!
//! This crate does not provide any form of real encryption. It only makes the task of understanding strings
//! in your binary more difficult. [Obfuscation is not security](https://cwe.mitre.org/data/definitions/656.html).
//!
//! This crate also _does not_ obfuscate any debug symbols you may have.
//! Profile settings such as  
//! ```toml
//! # inside Cargo.toml
//!
//! [profile]
//! strip = true
//! panic = "abort"
//! # ...
//! ```  
//! and more can be found in the [cargo reference](https://doc.rust-lang.org/cargo/reference/profiles.html).
//!
//! ### Macro expansion
//!
//! To check what this macro expands to:
//! - install [cargo expand](https://github.com/dtolnay/cargo-expand)
//! - run: `cargo expand -p muddy --example simple`
//!
//!
//! #### Unstable API
//!
//! This crate is still very much a work-in-progress. Expect breaking changes between minor
//! releases.
//!
//!

#[cfg(feature = "env")]
extern crate std;

#[cfg(feature = "env")]
pub use muddy_macros::muddy;

pub use muddy_macros::muddy_unchecked;

// re-exports
pub use aead;
pub use arrayvec;
pub use chacha20poly1305;
#[cfg(feature = "env")]
pub use const_hex;
