// let mut big_arg = String::new();
// for arg in std::env::args().skip(1) {
//     big_arg.push_str(&arg);
//     big_arg.push(' ');
// }
// std::process::Command::new("bash")
//     .arg("-c")
//     .arg(big_arg)
//     .status();
#![feature(rustc_private)]
#![no_std]
#![no_main]

mod io {
    extern "C" {
        pub fn putchar(c: i8) -> i8;
    }

    pub fn print_char(input: i8) {
        unsafe {
            let _ = putchar(input);
        }
    }

    pub fn print_ln(input: &str) {
        unsafe {
            for ch in input.chars() {
                putchar(ch as i8);
            }
            putchar('\n' as i8);
        }
    }
}
//use core::mem::size_of_val;
use core::slice;
use heapless::consts::*;
use heapless::String;

extern crate libc;

static mut ARGS: String<U128> = String(heapless::i::String::new());

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // io::print_char('A' as i8);
    // io::print_char('l' as i8);
    // io::print_char('y' as i8);
    // io::print_char('s' as i8);
    // io::print_char('\n' as i8);
    // io::print_char(((_argc as i8) + 48) as i8);
    // io::print_char('\n' as i8);
    let exit_code: i32;

    // // Since we are passing a C string the final null character is mandatory.
    #[cfg(target_os = "macos")]
    unsafe {
        let args_slice = slice::from_raw_parts(_argv, _argc as usize);
        for arg in args_slice.iter().skip(1) {
            let slice: &[u8] = slice::from_raw_parts(*arg, libc::strlen(*arg as *const i8));

            let s: &str = core::str::from_utf8_unchecked(slice);
            //io::print_ln(s);
            ARGS.push_str(s).unwrap();
            ARGS.push(' ').unwrap();
        }
        //ARGS.push('\0').unwrap();
        //io::print_ln(&ARGS);
        exit_code = libc::system(ARGS.as_ptr() as *const i8);
    }
    exit_code as isize
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
