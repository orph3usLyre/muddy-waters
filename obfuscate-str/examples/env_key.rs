// Compile this example
// At buildtime, a key will be printed to the terminal
// This key must be set at runtime to deobfuscate the strings or the program will panic
// OBFUSCATE_KEY='' ./target/debug/example/simple
//
// Grep the binary for 'obfuscated':
// $ strings ./target/debug/examples/simple | grep obfuscated
extern crate obfuscate_str;

use obfuscate_str::{hide_all, obfuscate_init};

obfuscate_init!("env");

hide_all! {
    static MY_FIRST_STR: &'static str = "My first obfuscated static str";
    static MY_SECOND_STR: &'static str = "My second obfuscated static str";
}

fn main() {
    println!("{}", MY_FIRST_STR);
    println!("{}", "My non obfuscated static str - ripgrep 'obfuscated'");
    println!("{}", MY_SECOND_STR);
}
