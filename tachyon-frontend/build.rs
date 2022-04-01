#![feature(exit_status_error)]
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

struct CSSTarget {
    name: String,
    path: PathBuf,
}

fn list_css(dir: PathBuf, prefix: String) -> Vec<CSSTarget> {
    let mut res = Vec::new();
    for i in fs::read_dir(dir).unwrap().map(|x| x.unwrap()) {
        let meta = i.metadata().unwrap();
        let name = i.file_name().to_str().unwrap().to_string();
        let path = i.path();
        if meta.is_dir() {
            let mut vec = list_css(path, format!("{}_{}", prefix, name));
            res.append(&mut vec);
        } else {
            res.push(CSSTarget {
                name: format!(
                    "{}_{}",
                    prefix.to_uppercase(),
                    name.replace('.', "_").to_uppercase()
                )
                .trim_start_matches('_')
                .to_string(),
                path,
            })
        }
    }
    res
}

fn generate_rust(items: Vec<CSSTarget>) {
    let file = File::create(".tmp/summary.rs").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    for i in &items {
        writeln!(
            writer,
            r#"pub const {} : (&str, &str) = (include_str!("{}"), "{}");"#,
            i.name,
            i.path.canonicalize().unwrap().to_str().unwrap(),
            i.path.to_str().unwrap().trim_start_matches("dist/")
        )
        .unwrap();
    }
    writeln!(
        writer,
        r#"pub const TARGETS : [&(&str, &str); {}] = ["#,
        items.len()
    )
    .unwrap();
    for i in &items {
        writeln!(writer, "    &{},", i.name).unwrap();
    }
    writeln!(writer, "];",).unwrap();
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
    generate_rust(list_css("dist".parse().unwrap(), "".to_string()));
}
