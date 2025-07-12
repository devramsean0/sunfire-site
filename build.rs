use std::process::Command;
use std::path::Path;
use std::{io, fs};

fn main() {
    let out_dir = "./build";
    std::fs::create_dir_all(out_dir).expect("Failed to create output directory");
    
    println!("cargo::warning=Building Astro project to {}", out_dir);
    let output = Command::new("bun")
        .arg("astro")
        .arg("build")
        .current_dir("./src-web")
        .output()
        .expect("Failed to execute command");
    copy_dir_all("./src-web/dist", out_dir).expect("Failed to copy build output");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src-web");
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}