use std::env;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
extern crate gcc;

fn main() {

    let mut builder = gcc::Config::new();

    // from os/rt/rt.mk, KERNSRC
    let os_src_files = [
        "./ChibiOS/os/rt/src/chsys.c",
        "./ChibiOS/os/rt/src/chdebug.c",
        "./ChibiOS/os/rt/src/chtrace.c",
        "./ChibiOS/os/rt/src/chvt.c",
        "./ChibiOS/os/rt/src/chschd.c",
        "./ChibiOS/os/rt/src/chthreads.c",
        "./ChibiOS/os/rt/src/chtm.c",
        "./ChibiOS/os/rt/src/chstats.c",
        "./ChibiOS/os/rt/src/chregistry.c",
        "./ChibiOS/os/rt/src/chsem.c",
        "./ChibiOS/os/rt/src/chmtx.c",
        "./ChibiOS/os/rt/src/chcond.c",
        "./ChibiOS/os/rt/src/chevents.c",
        "./ChibiOS/os/rt/src/chmsg.c",
        "./ChibiOS/os/rt/src/chdynamic.c",
        "./ChibiOS/os/common/oslib/src/chmboxes.c",
        "./ChibiOS/os/common/oslib/src/chmemcore.c",
        "./ChibiOS/os/common/oslib/src/chheap.c",
        "./ChibiOS/os/common/oslib/src/chmempools.c",
    ];

    for os_src_file in os_src_files.iter() {
        builder.file(os_src_file);
    }

    #[cfg(feature="stm32f407xg")]
    let port_src_files = [
        "./ChibiOS/os/common/ports/ARMCMx/chcore.c",
        "./ChibiOS/os/common/ports/ARMCMx/chcore_v7m.c",
        "./ChibiOS/os/common/ports/ARMCMx/compilers/GCC/chcoreasm_v7m.S",
        "./ChibiOS/os/common/startup/ARMCMx/compilers/GCC/crt0_v7m.S",
        "./ChibiOS/os/common/startup/ARMCMx/compilers/GCC/crt1.c",
    ];

    #[cfg(feature="stm32f051x8")]
    let port_src_files = [
        "./ChibiOS/os/common/ports/ARMCMx/chcore.c",
        "./ChibiOS/os/common/ports/ARMCMx/chcore_v6m.c",
        "./ChibiOS/os/common/ports/ARMCMx/compilers/GCC/chcoreasm_v6m.S",
        "./ChibiOS/os/common/startup/ARMCMx/compilers/GCC/crt0_v6m.S",
        "./ChibiOS/os/common/startup/ARMCMx/compilers/GCC/crt1.c",
    ];

    for port_src_file in port_src_files.iter() {
        builder.file(port_src_file);
    }

    let flags = [
        "-mno-thumb-interwork", // CFLAGS, because USB_THUMB is set
        "-ffunction-sections",  // from os/common/startup/ARMCMx/compilers/GCC/rules.mk
        "-fdata-sections",      // from os/common/startup/ARMCMx/compilers/GCC/rules.mk
        "-fno-common",          // from os/common/startup/ARMCMx/compilers/GCC/rules.mk
        "-fomit-frame-pointer",
        "-falign-functions=16",
    ];

    for flag in flags.iter() {
        builder.flag(flag);
    }

    let include_dirs = [
        "./",                               // for chconf.h
        "ChibiOS/os/license",
        "ChibiOS/os/various",
        "ChibiOS/os/rt/include",            // KERNINC, from os/rt/rt.mk
        "ChibiOS/os/common/oslib/include",  // KERNINC, from os/rt/rt.mk
    ];

    for include_dir in include_dirs.iter() {
        builder.include(include_dir);
    }

    #[cfg(feature="stm32f407xg")]
    let port_include_dirs = [
        "ChibiOS/os/common/ports/ARMCMx",                  // PORTINC, from os/common/ports/ARMCMx/compilers/GCC/mk/port_v7m.mk
        "ChibiOS/os/common/ports/ARMCMx/compilers/GCC",    // PORTINC, from os/common/ports/ARMCMx/compilers/GCC/mk/port_v7m.mk
        "ChibiOS/os/common/startup/ARMCMx/compilers/GCC",      // STARTUPINC, from os/common/startup/ARMCMx/compilers/GCC/mk/startup_stm32f4xx.mk
        "ChibiOS/os/common/startup/ARMCMx/devices/STM32F4xx",  // STARTUPINC, from os/common/startup/ARMCMx/compilers/GCC/mk/startup_stm32f4xx.mk
        "ChibiOS/os/common/ext/CMSIS/include",                 // STARTUPINC, from os/common/startup/ARMCMx/compilers/GCC/mk/startup_stm32f4xx.mk
        "ChibiOS/os/common/ext/CMSIS/ST/STM32F4xx",            // STARTUPINC, from os/common/startup/ARMCMx/compilers/GCC/mk/startup_stm32f4xx.mk
    ];

    #[cfg(feature="stm32f051x8")]
    let port_include_dirs = [
        "ChibiOS/os/common/ports/ARMCMx",                  // PORTINC, from os/common/ports/ARMCMx/compilers/GCC/mk/port_v6m.mk
        "ChibiOS/os/common/ports/ARMCMx/compilers/GCC",    // PORTINC, from os/common/ports/ARMCMx/compilers/GCC/mk/port_v6m.mk
        "ChibiOS/os/common/startup/ARMCMx/compilers/GCC",       // STARTUPINC, from os/common/startup/ARMCMx/compilers/GCC/mk/startup_stm32f0xx.mk
        "ChibiOS/os/common/startup/ARMCMx/devices/STM32F0xx",   // STARTUPINC, from os/common/startup/ARMCMx/compilers/GCC/mk/startup_stm32f0xx.mk
        "ChibiOS/os/common/ext/CMSIS/include",                  // STARTUPINC, from os/common/startup/ARMCMx/compilers/GCC/mk/startup_stm32f0xx.mk
        "ChibiOS/os/common/ext/CMSIS/ST/STM32F0xx",             // STARTUPINC, from os/common/startup/ARMCMx/compilers/GCC/mk/startup_stm32f0xx.mk
    ];

    for include_dir in port_include_dirs.iter() {
        builder.include(include_dir);
    }

    let defines = [
        ("THUMB_PRESENT", None),                              // CFLAGS, because USE_THUMB is set, rules.mk
        ("THUMB_NO_INTERWORKING", None),                      // CFLAGS, because USB_THUMB is set, rules.mk
    ];

    for &(def_key, def_val) in defines.iter() {
        builder.define(def_key, def_val);
    }

    #[cfg(feature="stm32f407xg")]
    let port_defines = [
        ("STM32F407xx", None),      // UDEFS from RT-ARMCM4-GENERIC demo Makefile
        ("THUMB", None),
    ];

    #[cfg(feature="stm32f051x8")]
    let port_defines = [
        ("STM32F051x8", None),      // UDEFS from RT-ARMCM0-GENERIC demo Makefile
        ("THUMB", None),
    ];

    for &(def_key, def_val) in port_defines.iter() {
        builder.define(def_key, def_val);
    }

    builder.archiver("arm-none-eabi-ar");
    builder.compile("libchibios.a");

    // ld path from os/common/startup/ARMCMx/compilers/GCC/mk/startup_stm32f4xx.mk
    let ld_path = Path::new("./ChibiOS/os/common/startup/ARMCMx/compilers/GCC/ld/");

    #[cfg(feature="stm32f051x8")]
    let ld_file = "STM32F051x8.ld";

    #[cfg(feature="stm32f407xg")]
    let ld_file = "STM32F407xG.ld";

    fs::remove_file(ld_path.join("layout.ld")).expect("could not remove old linker file symlink");
    symlink(ld_file, ld_path.join("layout.ld")).expect("Could not create linker file symlink!");

    println!("cargo:rustc-link-search=native={}", ld_path.to_str().unwrap());
    println!("cargo:rustc-link-search=native={}", &env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=build.rs");
}
