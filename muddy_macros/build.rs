use std::env;

fn main() {
    // Retrieve the target family directly using CARGO_CFG_TARGET_FAMILY.
    let target_family =
        env::var("CARGO_CFG_TARGET_FAMILY").expect("CARGO_CFG_TARGET_FAMILY is not set");
    // Set an environment variable that can be accessed by the proc-macro.
    println!("cargo:rustc-env=TARGET_FAMILY={}", target_family);
    let target = env::var("TARGET").expect("TARGET is not set");
    println!("cargo:rustc-env=TARGET={}", target);

    println!("cargo:rerun-if-env-changed=MUDDY");
}
