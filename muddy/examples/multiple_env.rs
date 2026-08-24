use muddy::muddy as m;

// Example for running a program with multiple env variables:
// `cargo build --example multiple_env` should output:
//
// ```
// MUDDY='564CEBC3BB2AC902EAD4E602A8780F3A474DF62123C85C21F7E75C852F6C5E4E'
// MY_ENV='5951A941D5A0F92C60F41E8CCA1C71359F4F3A2C9D5596088AFB2DC350FB6308'
// ```
//
// Then run the example:
// ```
// MUDDY='564CEBC3BB2AC902EAD4E602A8780F3A474DF62123C85C21F7E75C852F6C5E4E' \
//  MY_ENV='5951A941D5A0F92C60F41E8CCA1C71359F4F3A2C9D5596088AFB2DC350FB6308' \
//  cargo run --example multiple_env
// ```
//
//

fn main() {
    let a = m!("this is a test");
    let b = m!(env, "another test");
    let c = m!(env, "third test!");
    let d = m!(env = "MY_ENV", "fourth test!!!!");
    let e = m!(env = "MY_ENV", "FIFTH TEST!!!!");
    eprintln!("{a}");
    eprintln!("{b}");
    eprintln!("{c}");
    eprintln!("{d}");
    eprintln!("{e}");
}
