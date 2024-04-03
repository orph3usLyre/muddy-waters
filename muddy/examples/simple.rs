/// Compile this example and grep the binary for 'obfuscated':
/// $ strings ./target/debug/examples/simple | grep obfuscated
///
extern crate muddy;

use muddy::{m, muddy_init};

muddy_init!();

fn main() {
    println!("{}", m!("My highly obfuscated text"));
    println!("My non obfuscated static str - ripgrep me");
}
