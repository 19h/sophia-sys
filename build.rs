use std::process::Command;

fn main() {
    let output =
        Command::new("make")
        .current_dir("deps/sophia")
        .output().unwrap();

    if !output.status.success() {
        panic!("Could not build sophia correctly.");
    }

    println!("cargo:rustc-flags=-l sophia -L deps/sophia");
}
