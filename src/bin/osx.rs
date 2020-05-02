#![feature(rustc_private)]
#![no_std]
#![no_main]

// mod io {
//     extern "C" {
//         pub fn putchar(c: i8) -> i8;
//     }

//     pub fn print_char(input: i8) {
//         unsafe {
//             let _ = putchar(input);
//         }
//     }

//     pub fn print_ln(input: &str) {
//         unsafe {
//             for ch in input.chars() {
//                 putchar(ch as i8);
//             }
//             putchar('\n' as i8);
//         }
//     }
// }

use core::slice;
use heapless::consts::*;
use heapless::String;

extern crate libc;

static mut ARGS: String<U128> = String(heapless::i::String::new());

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let exit_code: i32;

    // // Since we are passing a C string the final null character is mandatory.
    #[cfg(not(target_os = "macos"))]
    {
        exit_code = 0;
    }
    #[cfg(target_os = "macos")]
    unsafe {
        let args_slice = slice::from_raw_parts(_argv, _argc as usize);
        for arg in args_slice.iter().skip(1) {
            let slice: &[u8] = slice::from_raw_parts(*arg, libc::strlen(*arg as *const i8));
            let s: &str = core::str::from_utf8_unchecked(slice);
            ARGS.push_str(s).unwrap();
            ARGS.push(' ').unwrap();
        }
        exit_code = libc::system(ARGS.as_ptr() as *const i8);
    }
    exit_code as isize
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
