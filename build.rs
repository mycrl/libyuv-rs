use std::env;
use std::io::Error;
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

#[cfg(target_os = "windows")]
fn exec(cmd: &str, work_dir: &str) -> Result<ExitStatus, Error> {
    Command::new("powershell").args(["-command", cmd])
                              .current_dir(work_dir)
                              .status()
}

#[cfg(not(target_os = "windows"))]
fn exec(command: &str, work_dir: &str) -> Result<ExitStatus, Error> {
    Command::new("bash").arg("-c")
                        .arg(command)
                        .current_dir(work_dir)
                        .status()
}

fn download(name: &str) -> (String, String) {
    let repository = env::var("CARGO_PKG_REPOSITORY").unwrap();
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let output = env::var("OUT_DIR").unwrap();

    let lib_name = get_lib_name(name, true);
    let path = join(&output, &lib_name);
    if !path.exists() {
        let url = &format!("{}/releases/download/v{}/{}", repository, version, lib_name);
        let path = path.to_str().unwrap();
        if cfg!(windows) {
            exec(&format!("Invoke-WebRequest -Uri {} -OutFile {}", url, path),
                 &output)
        } else {
            exec(&format!("curl -f -L -o {} {}", path, url), &output)
        }.expect("There is no precompiled binary library file in git \
                releases, please try to compile it yourself according to the \
                README, see https://github.com/colourful-rtc/libyuv-rs");
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
