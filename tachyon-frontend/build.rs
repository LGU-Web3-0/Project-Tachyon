#![feature(exit_status_error)]

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{PathBuf, Path};

struct CSSTarget {
    path: PathBuf,
}

fn list_css<P : AsRef<Path>>(dir: P) -> Vec<CSSTarget> {
    let mut res = Vec::new();
    for i in fs::read_dir(dir).unwrap().map(|x| x.unwrap()) {
        let meta = i.metadata().unwrap();
        let path = i.path();
        if meta.is_dir() {
            let mut vec = list_css(path.as_path());
            res.append(&mut vec);
        } else {
            res.push(CSSTarget {
                path,
            })
        }
    }
    res
}

fn generate_rust(items: Vec<CSSTarget>) {
    let file = File::create(".tmp/summary.rs").unwrap();
    let mut writer = std::io::BufWriter::new(file);

    write!(&mut writer, "pub const TARGETS: phf::Map<&'static str, &'static str> = ").unwrap();
    let mut map = &mut phf_codegen::Map::new();

    for i in &items {
        let data = std::fs::read_to_string(&i.path).unwrap();
        map = map.entry(i.path.to_str().unwrap().trim_start_matches("dist/"), &format!("{:?}", data));
    }

    writeln!(&mut writer, "{};", map.build()).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=dist");
    println!("cargo:rerun-if-changed=.tmp");
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
        .arg("default")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .exit_ok()
        .unwrap();
    std::process::Command::new("gulp")
        .arg("css")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .exit_ok()
        .unwrap();
    generate_rust(list_css("dist"));
}
