#[cfg(target_os = "linux")]
pub fn main() {
    let mut big_arg = String::new();
    for arg in std::env::args().skip(1) {
        big_arg.push_str(&arg);
        big_arg.push(' ');
    }
    std::process::Command::new("bash")
        .arg("-c")
        .arg(big_arg)
        .status();
}

#[cfg(not(target_os = "linux"))]
pub fn main() {}
