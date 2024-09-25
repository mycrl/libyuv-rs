use std::{env, fs, path::Path, process::Command};

use anyhow::anyhow;
use which::which;

fn join(root: &str, next: &str) -> anyhow::Result<String> {
    Ok(Path::new(root)
        .join(next)
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to path into string."))?
        .to_string())
}

fn exists(dir: &str) -> bool {
    fs::metadata(dir).is_ok()
}

fn exec(command: &str, work_dir: &str) -> anyhow::Result<String> {
    let output = Command::new(if cfg!(target_os = "windows") {
        "powershell"
    } else {
        "bash"
    })
    .arg(if cfg!(target_os = "windows") {
        "-command"
    } else {
        "-c"
    })
    .arg(if cfg!(target_os = "windows") {
        format!("$ProgressPreference = 'SilentlyContinue';{}", command)
    } else {
        command.to_string()
    })
    .current_dir(work_dir)
    .output()?;
    if !output.status.success() {
        Err(anyhow!("{}", unsafe {
            String::from_utf8_unchecked(output.stderr)
        }))
    } else {
        Ok(unsafe { String::from_utf8_unchecked(output.stdout) })
    }
}

fn library_name(complete: bool) -> String {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let ext = if cfg!(target_os = "windows") { "lib" } else { "a" };
    let flag = if cfg!(target_os = "windows") { "" } else { "lib" };
    let name = format!("yuv-{}-{}", os, arch);
    if complete {
        format!("{}{}.{}", flag, name, ext)
    } else {
        name
    }
}

fn get_download_executable() -> anyhow::Result<&'static str> {
    if which("wget").is_ok() {
        Ok("wget")
    } else if which("curl").is_ok() {
        Ok("curl --remote-name")
    } else {
        Err(anyhow!("Neither wget nor curl found on the system to download precompiled binaries"))
    }
}

fn main() -> anyhow::Result<()> {
    println!("cargo:cargo:rerun-if-env-changed=./src");

    let repository = env::var("CARGO_PKG_REPOSITORY").unwrap();
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let output = env::var("OUT_DIR").unwrap();

    if !exists(&join(&output, &library_name(true))?) {
        let url = format!("{}/releases/download/v{}/{}", repository, version, library_name(true));
        if cfg!(target_os = "windows") {
            exec(&format!("Invoke-WebRequest -Uri {} -OutFile {}", url, library_name(true)),
                 &output)
        } else {
            let download_executable = get_download_executable()?;
            exec(&format!("{} {}", download_executable, url), &output)
        }.expect("There is no precompiled binary library file in git \
                releases, please try to compile it yourself according to the \
                README, see https://github.com/colourful-rtc/libyuv-rs");
    }

    println!("cargo:rustc-link-search=all={}", output);
    println!("cargo:rustc-link-lib={}", library_name(false));
    Ok(())
}
