
#![allow(non_upper_case_globals)]

#![feature(lang_items,asm,start,compiler_builtins_lib)]
#![no_std]
#![crate_type="staticlib"]

extern crate compiler_builtins;

mod chibios;
mod app;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
pub extern "C" fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! {
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

// These handlers are declared in os/common/startup/ARMCMx/compilers/GCC/vectors.c
extern "C" {
    fn __main_stack_end__();
    fn Reset_Handler();
    fn NMI_Handler();
    // fn HardFault_Handler();
    // fn MemManage_Handler();
    // fn BusFault_Handler();
    // fn UsageFault_Handler();
    #[cfg(feature="thumbv7m")]
    fn SVC_Handler();
    // fn DebugMon_Handler();
    #[cfg(feature="thumbv7m")]
    fn PendSV_Handler();
// fn SysTick_Handler(); // see e.g. demos/various/NIL-ARMCM0-GENERIC/main.c, CH_IRQ_HANDLER(SysTick_Handler)
}

#[link_section = ".vectors"]
#[no_mangle]
pub static ISRVectors: [Option<unsafe extern "C" fn()>; 16] = [
    Some(__main_stack_end__), // Stack pointer
    Some(Reset_Handler),
    Some(NMI_Handler),
    None, // HardFault_Handler
    None, // MemManage_Handler
    None, // BusFault_Handler
    None, // UsageFault_Handler
    None, // Reserved
    None, // Reserved
    None, // Reserved
    None, // Reserved
    #[cfg(feature = "thumbv7m")]
    Some(SVC_Handler),
    #[cfg(not(feature = "thumbv7m"))]
    None, // SVC_Handler
    None, // DebugMon_Handler
    None, // Reserved
    None, // PendSV_Handler if CORTEX_ALTERNATE_SWITCH is set in chconf.h
    None, // SysTick_Handler
];


#[start]
fn main(_: isize, _: *const *const u8) -> isize {
    app::user_main();
    0
}
