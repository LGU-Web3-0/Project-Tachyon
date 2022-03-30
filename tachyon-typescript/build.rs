#![feature(exit_status_error)]

fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=gulpfile.js");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=tsconfig.json");
    println!("cargo:rerun-if-changed=webpack.config.js");
    std::process::Command::new("yarn")
        .arg("install")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .exit_ok()
        .unwrap();
    std::process::Command::new("gulp")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .exit_ok()
        .unwrap();
}