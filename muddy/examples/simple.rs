use muddy::muddy;

// build with `cargo b --example simple`
// then search the binary for embedded strings:
// `strings target/debug/examples/simple | grep supersecret`
//
// only the non-obfuscated text will show up
fn main() {
    let non_obfuscated = "notsupersecret9001";
    let obfuscated = muddy!("supersecret42");
    println!("{}", non_obfuscated);
    println!("{}", obfuscated);
}
