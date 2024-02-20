// Compile this example and grep the binary for 'obfuscated':
// $ strings ./target/debug/examples/simple | grep obfuscated

extern crate obfuscate_str;

use obfuscate_str::{o, obfuscate_init};

obfuscate_init!();

fn main() {
    println!("{}", o!("My highly obfuscated text"));
    println!("{}", "My non obfuscated static str - ripgrep me");
}
