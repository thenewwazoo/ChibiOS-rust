
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![feature(lang_items,asm,start,compiler_builtins_lib)]
#![no_std]
#![crate_type="staticlib"]

extern crate cty;
extern crate compiler_builtins;

#[cfg(feature="cmsis_os")]
include!(concat!(env!("OUT_DIR"), "/cmsis_os.rs"));

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
    fn __process_stack_end__();
    fn Reset_Handler();
}

#[link_section=".vectors"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static ISRVectors: [Option<unsafe extern "C" fn()>; 16] = [Some(__process_stack_end__), // Stack pointer
                                                               Some(Reset_Handler),
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
fn main(_: isize, _: *const *const u8) -> isize {
    user_main();
    0
}

fn user_main() {
    loop {}
}
