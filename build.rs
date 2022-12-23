use std::env;
use std::path::*;
use std::process::*;

fn join(a: &str, b: &str) -> PathBuf {
    Path::new(a).join(b)
}

fn split(path: &Path) -> (String, String) {
    let name = path.file_stem().unwrap().to_str().unwrap().to_string();
    let dir = path.parent().unwrap().to_str().unwrap().to_string();

    (dir,
     name.starts_with("lib")
         .then(|| name.replacen("lib", "", 1))
         .unwrap_or(name))
}

fn get_lib_name(key: &str, long: bool) -> String {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let ext = if cfg!(windows) { "lib" } else { "a" };
    let flag = if cfg!(windows) { "" } else { "lib" };
    let name = format!("{}-{}-{}", key, os, arch);
    if long {
        format!("{}{}.{}", flag, name, ext)
    } else {
        name
    }
}

fn download(name: &str) -> (String, String) {
    let repository = env::var("CARGO_PKG_REPOSITORY").unwrap();
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let output = env::var("OUT_DIR").unwrap();

    let lib_name = get_lib_name(name, true);
    let path = join(&output, &lib_name);
    if !path.exists() {
        Command::new("curl").arg("-f")
                            .arg("-L")
                            .arg("-o")
                            .arg(path.to_str().unwrap())
                            .arg(&format!("{}/releases/download/v{}/{}",
                                          repository, version, lib_name))
                            .output()
                            .expect(
                                    "There is no precompiled binary library file in git \
                releases, please try to compile it yourself according to the \
                README, see https://github.com/colourful-rtc/libyuv-rs",
        );
    }

    split(&path)
}

fn main() {
    for name in ["YUV_LIBRARY_PATH"] {
        println!("cargo:cargo:rerun-if-env-changed={}", name);
        if let Ok(path) = env::var(name) {
            println!("cargo:rerun-if-changed={}", path);
        }
    }

    let (yuv_lib_path, yuv_lib_name) =
        env::var("YUV_LIBRARY_PATH").map(|path| split(Path::new(&path)))
                                    .unwrap_or_else(|_| download("yuv"));

    println!("cargo:rustc-link-lib={}", yuv_lib_name);
    println!("cargo:rustc-link-search=all={}", yuv_lib_path);
}
