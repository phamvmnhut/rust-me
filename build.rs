// build.rs

use std::env;
use std::fs;
use std::path::Path;
// use std::process::Command;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    fs::write(
        &dest_path,
        "pub fn message() -> &'static str {
            \"Hello, World! in Hello.rs\"
        }
        ",
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");

    // let out_dir = env::var("OUT_DIR").unwrap();

    // // Note that there are a number of downsides to this approach, the comments
    // // below detail how to improve the portability of these commands.
    // Command::new("gcc").args(&["src/hello.c", "-c", "-fPIC", "-o"])
    //                    .arg(&format!("{}/hello.o", out_dir))
    //                    .status().unwrap();
    // Command::new("ar").args(&["crus", "libhello.a", "hello.o"])
    //                   .current_dir(&Path::new(&out_dir))
    //                   .status().unwrap();

    // println!("cargo:rustc-link-search=native={}", out_dir);
    // println!("cargo:rustc-link-lib=static=hello");
    // println!("cargo:rerun-if-changed=src/hello.c");

    cc::Build::new().file("src/hello.c").compile("hello");
    println!("cargo:rerun-if-changed=src/hello.c");

    // pkg_config::Config::new().probe("zlib").unwrap();
    // println!("cargo:rerun-if-changed=build.rs");
}
