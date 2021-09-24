use std::{env, process::Command};

fn main()
{
    if env::var_os("CARGO_FEATURE_WEB").is_some() {
        println!("cargo:rerun-if-changed=web");

        let status = Command::new("yarn")
            .arg("build")
            .current_dir("web")
            .status()
            .expect("Failed to build frontend.");

        if status.code().unwrap() != 0 {
            panic!("Building frontend failed.");
        }
    }

    // This currently isn't possible because cargo doc requires a lock on the Cargo.lock file that
    // can't be provided

    // if env::var_os("CARGO_FEATURE_DOCS").is_some() {
    //     println!("cargo:rerun-if-changed=src");

    //     let status = Command::new(env::var("CARGO").unwrap())
    //         .args(["doc", "--no-deps"])
    //         .status()
    //         .expect("Failed to build docs.");

    //     if status.code().unwrap() != 0 {
    //         panic!("Failed to build docs.");
    //     }
    // }
}
