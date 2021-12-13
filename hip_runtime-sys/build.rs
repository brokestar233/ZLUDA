use std::env::VarError;
use std::{env, path::PathBuf};

fn main() -> Result<(), VarError> {
    println!("cargo:rustc-link-lib=dylib=amdhip64");
    if cfg!(windows) {
        let env = env::var("CARGO_CFG_TARGET_ENV")?;
        if env == "msvc" {
            let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
            path.push("lib");
            println!("cargo:rustc-link-search=native={}", path.display());
        } else {
            println!("cargo:rustc-link-search=native=C:\\Windows\\System32");
        };
    } else {
        //println!("cargo:rustc-link-search=native=/opt/rocm/lib/");
        println!("cargo:rustc-link-search=native=/home/ubuntu/hipamd/build/lib");
        println!("cargo:rustc-link-search=native=/home/vosen/hipamd/build/lib");
    }
    Ok(())
}
