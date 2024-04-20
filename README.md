<!-- cargo-rdme start -->

# `muddy`

`muddy` is a static string obfuscation library, designed to provide an easy way of avoiding simple static binary analysis tools such as `strings` or YARA rules.
It functions by encrypting texts at buildtime, and decrypting them lazily at runtime.


## Usage & Examples

```rust
use muddy::{m, muddy_init};

muddy_init!();

println!("{}", m!("My highly obfuscated text"));
println!("{}", "My non obfuscated static str - ripgrep me");

```  
    
   
> Compile this example and grep the binary for `obfuscated`:  
>
> `cargo b --example simple`  
>
> `strings ./target/debug/examples/simple | grep obfuscated`    
> Only the second nonobfuscated line should appear.
>  
  
   
  
`muddy` primarily provides the exported `m!()`, `#[muddy]`, and `muddy_all! { }` macros, all which
take text as input and encrypt it. The `muddy_init!()` macro provides the scaffolding
for decrypting the strings at runtime and should be placed at the `root` of the user's crate.

These macros can be used as an in-place text replacement with `m!("my text")`:

```rust
use muddy::{muddy_init, m};

muddy_init!();

println!("{}", m!("my plaintext"));
```  

As an annotated `&'static str` with the [`muddy`] attribute:

```rust
use muddy::{muddy, muddy_init};

muddy_init!();

#[muddy]
static MY_STR: &str = "my plaintext";

```

Or as an invocation around multiple annotated `&'static str`s with [`muddy_all`]:

```rust
use muddy::{muddy_all, muddy_init};

muddy_init!();

muddy_all! {
   pub static MY_STR: &str = "my plaintext";
   pub static MY_SECOND_STR: &str = "my second plaintext";
   static MY_THIRD_STR: &str = "my module-specific third plaintext";
}

```

By default, `muddy` will encrypt the static strings with the [`chacha20poly1305`] implementation,
and embed the key inside the binary.  

To avoid hardcoding the deobfuscation key into your binary, you may use:

```rust

use muddy::{m, muddy_init};

muddy_init!("env");

// If you build your program with `muddy_init!("env")`,
// the deobfuscation key env variable and deobfuscation key
// will be printed out to stderr:  
// `MUDDY='D47A372C13DEFED74FD3B9B4C741C355F9CB2C23C43F98ADE2C02FD50CA55C3D'`

// This key needs to be provided at runtime else the program will panic.  
// `MUDDY='D47A372C13DEFED74FD3B9B4C741C355F9CB2C23C43F98ADE2C02FD50CA55C3D' ./target/debug/examples/env`
    println!("{}", m!("My highly obfuscated text"));
    println!("{}", "My non obfuscated static str - ripgrep me");
```  


If `muddy_init!("env")` is set, the `MUDDY` (by default) env variable will be checked at runtime for the key and the program will panic if it's not found.

You can also set your own env key identifier at buildtime through `MUDDY`:   
This: `MUDDY='MY_KEY_NAME_2' cargo b --example env`  
prints: `MY_KEY_NAME_2='FD5B85045B5278F5EDA567AD7C58EB56934BD8D7432C878B1AB6090052A64080'`  
  

### `muddy_init!`

`muddy_init!()` can take one of two values:
- `muddy_init!("embed")`
- `muddy_init!("env")`

If no value is provided, the macro defaults to the `"embed"` configuration.  
If `"env"` is provided, you may also set the env key identifier as the second field: `muddy_init!("env", "MY_KEY")`



### Note on obfuscation and encryption

This crate does not provide any form of real encryption. It only makes the task of understanding strings
in your binary more difficult. [Obfuscation is not security](https://cwe.mitre.org/data/definitions/656.html).

This crate also _does not_ obfuscate any debug symbols you may have.
Profile settings such as  
```toml
# inside Cargo.toml

[profile]
strip = true
panic = "abort"
# ...
```  
and more can be found in the [cargo reference](https://doc.rust-lang.org/cargo/reference/profiles.html).

### Macro expansion

To check what this macro expands to:
- install [cargo expand](https://github.com/dtolnay/cargo-expand)
- run: `cargo expand -p muddy --example env`



<!-- cargo-rdme end -->

### Next steps:
- [ ]  handle `#![no_std]` environments
- [ ]  check safety with leaking `mut` statics
- [ ]  check proc macro testing suites

### Disclaimer
This library is developed with the sole intention of providing a tool to challenge and educate cybersecurity professionals. It is not intended for any malicious or unlawful activities. The creators and contributors of this library do not endorse, encourage, or support the use of this tool for any illegal purposes.


### Similar/related projects
- [cryptify](https://github.com/dronavallipranav/rust-obfuscator/tree/main/cryptify)
- [litcrypt](https://github.com/anvie/litcrypt.rs)
- [include-crypt-bytes](https://github.com/breakpointninja/include-crypt-bytes)
- [obfstr](https://github.com/CasualX/obfstr)
- [Interesting related article by vrls.ws](https://vrls.ws/posts/2023/06/obfuscating-rust-binaries-using-llvm-obfuscator-ollvm/)

### License

Dual-licensed under Apache 2.0 and MIT terms.
