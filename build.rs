use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let linker_script = PathBuf::from(manifest_dir).join("linker_maat.ld");
    println!("cargo:rerun-if-changed=linker_maat.ld");
    println!("cargo:rustc-link-arg=-T{}", linker_script.display());
}
