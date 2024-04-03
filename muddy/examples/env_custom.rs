// Compile this example
// At buildtime, the provided key variable will be used.
// This key must be set at runtime to deobfuscate the strings or the program will panic
// MY_KEY_NAME='2819EC204DFD150583BAE69CA99F679F4F6CADC87724B899FC160AF70E67679C' ./target/debug/example/simple
//
// Grep the binary for 'obfuscated':
// $ strings ./target/debug/examples/env_custom | grep obfuscated
//

use muddy::{muddy_all, muddy_init};

muddy_init!("env", "MY_KEY_NAME");

static MY_NON_OBFUSCATED_TEXT: &str = "My non obfuscated static str - ripgrep 'obfuscated'";

muddy_all! {
    static MY_FIRST_STR: &str = "My first obfuscated static str";
    static MY_SECOND_STR: &str = "My second obfuscated static str";
}

fn main() {
    println!("{}", MY_FIRST_STR);
    println!("{}", MY_NON_OBFUSCATED_TEXT);
    println!("{}", MY_SECOND_STR);
}
