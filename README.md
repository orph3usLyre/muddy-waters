<!-- cargo-rdme start -->

# muddy

muddy is a literal string obfuscation library, designed to provide an easy way of avoiding simple static binary analysis tools such as `strings` or YARA rules.
It functions by encrypting texts at build time, and embedding an in-place decrypter that is evaluated at runtime.


## Usage & Examples

```rust
// examples/simple.rs
use muddy::muddy;

let non_obfuscated = "notsupersecret9001";
let obfuscated = muddy!("supersecret42");
println!("{}", non_obfuscated);
println!("{}", obfuscated);
```  
    
   
> Compile this example and grep the binary for `obfuscated`:  
>
> `cargo b --example simple`  
>
> `strings ./target/debug/examples/simple | grep obfuscated`    
> Only the second nonobfuscated line should appear.
>  
  
  
`muddy` primarily provides the exported `muddy!()` and `muddy_unchecked!()`, macros, which
each take a literal text as input, encrypt it at buildtime, and generate
an in-place decrypter which is evaluated to the plaintext `&'static str` at runtime.


By default, these macros will encrypt literal strings with the [`chacha20poly1305`] implementation
and embed the key inside the binary.  

#### Runtime-provided decryption

If the env argument is provided to the macro invocation, the deobfuscation key
will not be embedded into the binary. Instead, it will be generated at buildtime
and must be provided at runtime.

```rust
use muddy::muddy;

let obfuscated = muddy!(env, "supersecret42");
println!("{}", obfuscated);
```  


Running `cargo b` will print out `MUDDY='<SOME_KEY>'` to stderr.
                                                                                             
This env will then need to be set at runtime, otherwise the program will panic: `MUDDY='<SOME_KEY>' cargo r`

> You may also set your own key identifiers: `muddy!(env = "MY_KEY_NAME", "supersecret42")`
>

### `muddy_unchecked!()`

The difference between `muddy!()` and `muddy_unchecked!()` is that the `muddy!()` macro
checks that the macro invocation is not evaluated multiple times.
Opt for `muddy_unchecked!()` if you can uphold this guarantee.

```rust
use muddy::muddy_unchecked;

fn f() -> &'static str {
  muddy!(env, "supersecret1")
}

fn f2() -> &'static str {
  muddy_unchecked!(env, "supersecret42")
}

fn f3() -> &'static str {
  muddy_unchecked!(env, "supersecret9001")
}

for _ in 0..2 {
  println!("{}", f()); // <----- fine, since `muddy!()` provides checks against multiple evaluations
}

for _ in 0..2 {
  println!("{}", f2()); // <---- panics at the second evaluation
}

for _ in 0..2 {
  std::thread::spawn(|| {
    println!("{}", f3()); // <-  panics at the second evaluation
  });
}
```

Alternatively:
```rust
use muddy::muddy_unchecked;

// only evaluated once
let plaintext = muddy_unchecked!("supersecret1337");
for _ in 0..2 {
  println!("{}", plaintext); // <--- fine
}

for _ in 0..2 {
  std::thread::spawn(move || {
    println!("{}", plaintext); // <- also fine
  });
}
```  


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
- run: `cargo expand -p muddy --example simple`


#### Unstable API

This crate is still very much a work-in-progress. Expect breaking changes between minor
releases.


<!-- cargo-rdme end -->

### Migrating from previous versions

Previous versions of this crate provided obfuscation for static strings. This behavior may be achieved with the current API by using a [`once_cell::Lazy`](https://docs.rs/once_cell/latest/once_cell/sync/struct.Lazy.html):
```
use once_cell::sync::Lazy;
use muddy::muddy_unchecked;

static MY_STRING: Lazy<&'static str> = Lazy::new(|| muddy_unchecked!("some text"));
```

### Next steps:
- [ ]  check proc macro testing suites

### Disclaimer
This library is developed with the sole intention of providing a tool to challenge and educate cybersecurity professionals. It is not intended for any malicious or unlawful activities. The creators and contributors of this library do not endorse, encourage, or support the use of this tool for any illegal purposes.

### Shoutouts
- thanks to [@blastrock](https://github.com/blastrock) for his advice on `muddy v0.3.0`

### Similar/related projects
- [cryptify](https://github.com/dronavallipranav/rust-obfuscator/tree/main/cryptify)
- [litcrypt](https://github.com/anvie/litcrypt.rs)
- [include-crypt-bytes](https://github.com/breakpointninja/include-crypt-bytes)
- [obfstr](https://github.com/CasualX/obfstr)
- [Interesting related article by vrls.ws](https://vrls.ws/posts/2023/06/obfuscating-rust-binaries-using-llvm-obfuscator-ollvm/)

### License

Dual-licensed under Apache 2.0 and MIT terms.
