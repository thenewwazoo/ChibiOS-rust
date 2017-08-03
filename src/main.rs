#![feature(lang_items,asm,start,compiler_builtins_lib)]
#![no_std]
#![crate_type="staticlib"]

extern crate compiler_builtins;

#[lang="eh_personality"]
extern "C" fn eh_personality() {}

#[lang="panic_fmt"]
pub extern "C" fn panic_fmt(_fmt: &core::fmt::Arguments,
                                    _file_line: &(&'static str, usize))
                                    -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() -> () {
    loop {}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() -> () {
    loop {}
}

extern "C" {
    fn _estack();
}

#[link_section=".vectors"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static ISRVectors: [Option<unsafe extern "C" fn()>; 16] = [Some(_estack), // Stack pointer
                                                               Some(startup), // Reset
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
                                                               None, // Reserved
];

#[start]
fn lang_start(_: isize, _: *const *const u8) -> isize {
    unsafe {
        startup();
    }
    0
}

pub unsafe extern "C" fn startup() {
    main();
}

pub fn main() {
    loop {
    }
}
