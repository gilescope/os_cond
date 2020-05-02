#[cfg(target_os = "windows")]
pub fn main() {
    std::process::Command::new("cmd")
        .arg("/c")
        .args(std::env::args().skip(1))
        .status();
}

#[cfg(not(target_os = "windows"))]
pub fn main() {}
