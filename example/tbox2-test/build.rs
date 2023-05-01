use std::os::unix::fs::symlink;

fn main() {
    println!("cargo:rustc-flags= -L./lib/ -ltermbox2");

    match symlink("../../lib/libtermbox2.so", "./target/release/libtermbox2.so.2") {
        Ok(_) => println!("Soft linked with libtermbox2"),
        Err(why) => eprintln!("Soft linked failed, error: {why}"),
    }
}
