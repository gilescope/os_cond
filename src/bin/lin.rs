#![feature(rustc_private)]
#![no_std]
#![no_main]
use core::slice;

extern crate libc;

//static mut ARGS: String<U128> = String(heapless::i::String::new());

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let exit_code: i32;

    // // Since we are passing a C string the final null character is mandatory.
    #[cfg(not(target_os = "linux"))]
    {
        exit_code = 0;
    }

    #[cfg(target_os = "linux")]
    unsafe {
        let mut args: [u8;128] = [0; 128];
        let mut args_free = 0;
        let args_slice = slice::from_raw_parts(_argv, _argc as usize);
        for arg in args_slice.iter().skip(1) {
            let slice: &[u8] = slice::from_raw_parts(*arg, libc::strlen(*arg as *const i8));
            let s: &str = core::str::from_utf8_unchecked(slice);

            for ch in s.bytes() {
                args[args_free] = ch as u8;
                args_free += 1;
            }

            args[args_free] = ' ' as u8;
            args_free += 1;
        }
        args[args_free] = '\0' as u8;
        exit_code = libc::system(args.as_ptr() as *const i8);
    }
    exit_code as isize
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
