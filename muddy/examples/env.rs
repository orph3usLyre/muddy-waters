use muddy::muddy;

// running `cargo b --example env`
// will print out `MY_ENV='DF5C842962F76A25B7402524FC8B5C68174584BEC2B6318BBAC5EB1B83767CF0'`
// to the console
//
// This key will then need to be provided at runtime, otherwise the program will panic:
// `MY_ENV='DF5C842962F76A25B7402524FC8B5C68174584BEC2B6318BBAC5EB1B83767CF0' cargo r --example
// env`
//
fn main() {
    let text = muddy!(env = "MY_ENV", "supersecret123");
    println!("{}", text);
}
