#![feature(
    allow_internal_unstable,
    const_panic,
    const_fn,
    const_fn_union,
    const_raw_ptr_deref,
    const_if_match,
    intrinsics,
    lang_items,
    no_core,
    optin_builtin_traits,
    untagged_unions
)]
#![no_core] // https://blog.mgattozzi.dev/oxidizing-the-technical-interview/
//  #![no_main]//rustc_private,
//extern crate libc;
macro_rules! matches {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard: expr )?) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        }
    }
}
macro_rules! copy_clone_eq_impls {
    ($($t:ty)*) => {
      $(
      impl Copy for $t {}
      impl Clone for $t {
        fn clone(&self) -> Self {
           *self
        }
      }
      impl PartialEq for $t {
          fn eq(&self, other: &$t) -> bool {
              (*self) == (*other)
          }
      }
      )*
    }
  }

  #[allow(dead_code)] // It is actually needed! Zombie Ordering
  enum Ordering {
      Less = -1,
      Equal = 0,
      Greater = 1,
  }
  use Ordering::*;

#[lang = "sized"]
pub trait Sized {}
#[lang = "freeze"]
auto trait Freeze {}
extern "rust-intrinsic" {
    fn offset<T>(dst: *const T, offset: isize) -> *const T;
}
#[lang = "receiver"]
trait Receiver {}
#[lang = "index"]
trait Index<Idx: ?Sized> {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
}
impl<T, I> Index<I> for [T]
where
    I: SliceIndex<[T]>,
{
    type Output = I::Output;
    fn index(&self, index: I) -> &I::Output {
        index.index(self)
    }
}
trait SliceIndex<T: ?Sized> {
    type Output: ?Sized;
    fn get(self, slice: &T) -> Option<&Self::Output>;
    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;
    unsafe fn get_unchecked(self, slice: &T) -> &Self::Output;
    unsafe fn get_unchecked_mut(self, slice: &mut T) -> &mut Self::Output;
    fn index(self, slice: &T) -> &Self::Output;
    fn index_mut(self, slice: &mut T) -> &mut Self::Output;
}

trait Termination {
    fn report(self) -> i32;
}

#[cfg(target_os = "macos")]
#[link(name="System")]
extern "C" {
    pub fn printf(format: *const i32, ...) -> i8;
}

#[lang = "slice"]
impl<T> [T] {
    #[allow(unused_attributes)]
    #[allow_internal_unstable(const_fn_union)]
    const fn len(&self) -> usize {
        unsafe { Repr { rust: self }.raw.len }
    }

    fn as_ptr(&self) -> *const T {
        self as *const [T] as *const T
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut [T] as *mut T
    }
}

#[lang = "const_ptr"]
impl<T: ?Sized> *const T {
    unsafe fn add(self, count: usize) -> Self
    where
        T: Sized,
    {
        self.offset(count as isize)
    }

    unsafe fn offset(self, count: isize) -> *const T
    where
        T: Sized,
    {
        offset(self, count)
    }
}
#[lang = "mut_ptr"]
impl<T: ?Sized> *mut T {
    unsafe fn add(self, count: usize) -> *mut T
    where
        T: Sized,
    {
        self.offset(count as isize)
    }
    unsafe fn offset(self, count: isize) -> *mut T
    where
        T: Sized,
    {
        offset(self, count) as *mut T
    }
}
impl<T> SliceIndex<[T]> for usize {
    type Output = T;
    fn get(self, slice: &[T]) -> Option<&T> {
        if self < slice.len() {
            unsafe { Some(self.get_unchecked(slice)) }
        } else {
            None
        }
    }
    fn get_mut(self, slice: &mut [T]) -> Option<&mut T> {
        if self < slice.len() {
            unsafe { Some(self.get_unchecked_mut(slice)) }
        } else {
            None
        }
    }
    unsafe fn get_unchecked(self, slice: &[T]) -> &T {
        &*slice.as_ptr().add(self)
    }
    unsafe fn get_unchecked_mut(self, slice: &mut [T]) -> &mut T {
        &mut *slice.as_mut_ptr().add(self)
    }
    fn index(self, slice: &[T]) -> &T {
        &(*slice)[self]
    }
    fn index_mut(self, slice: &mut [T]) -> &mut T {
        &mut (*slice)[self]
    }
}

#[lang = "copy"]
trait Copy: Clone {}
trait Clone: Sized {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

#[lang = "eq"]
trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool {
        !self.eq(other)
    }
}

#[lang = "partial_ord"]
trait PartialOrd<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
    fn lt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }
    fn le(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Less) | Some(Equal))
    }
    fn gt(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }
    fn ge(&self, other: &Rhs) -> bool {
        matches!(self.partial_cmp(other), Some(Greater) | Some(Equal))
    }
}

#[allow(unconditional_recursion)]
impl PartialOrd for usize {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        self.partial_cmp(other)
    }
}

copy_clone_eq_impls!(usize u64 bool);

enum Option<T> {
    Some(T),
    None,
}
use crate::Option::*;
#[repr(C)]
pub(crate) union Repr<T> {
    pub(crate) rust: *const [T],
    rust_mut: *mut [T],
    pub(crate) raw: FatPtr<T>,
}
#[repr(C)]
pub(crate) struct FatPtr<T> {
    data: *const T,
    pub(crate) len: usize,
}

impl<T: ?Sized> Receiver for &T {}
impl<T: ?Sized> Receiver for &mut T {}

impl Not for bool {
    type Output = bool;
    fn not(self) -> bool {
        !self
    }
}

#[lang = "not"]
trait Not {
    type Output;

    fn not(self) -> Self::Output;
}
#[lang = "neg"]
trait Neg {
    type Output;
    fn neg(self) -> Self::Output;
}
impl Neg for isize {
    type Output = isize;

    fn neg(self) -> isize {
        -self
    }
}

#[lang = "add"]
trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

impl Add for usize {
    type Output = usize;
    fn add(self, rhs: usize) -> Self::Output {
        self + rhs
    }
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}

#[lang = "start"]
fn start<T: Termination + 'static>(main: fn() -> T, _: isize, _: *const *const u8) -> isize {
    main().report() as isize
}

//static mut ARGS: String<U128> = String(heapless::i::String::new());

//#[no_mangle]
pub fn main(
//    _argc: isize, _argv: *const *const u8
) -> () {//-> isize
    let exit_code: isize;

    // // Since we are passing a C string the final null character is mandatory.
   // #[cfg(not(target_os = "linux"))]
    {
        exit_code = 0;
    }

    // #[cfg(target_os = "linux")]
    // unsafe {
    //     let mut args: [u8;128] = [0; 128];
    //     let mut args_free = 0;
    //     let args_slice = slice::from_raw_parts(_argv, _argc as usize);
    //     for arg in args_slice.iter().skip(1) {
    //         let slice: &[u8] = slice::from_raw_parts(*arg, libc::strlen(*arg as *const i8));
    //         let s: &str = core::str::from_utf8_unchecked(slice);

    //         for ch in s.bytes() {
    //             args[args_free] = ch as u8;
    //             args_free += 1;
    //         }

    //         args[args_free] = ' ' as u8;
    //         args_free += 1;
    //     }
    //     args[args_free] = '\0' as u8;
    //     exit_code = libc::system(args.as_ptr() as *const i8);
    // }
//    exit_code as isize
}

// #[panic_handler]
// fn my_panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }
