use std::env;
use std::path::PathBuf;

extern crate bindgen;
extern crate gcc;

/// In order to build ChibiOS and generate bindings, ChibiOS must know about its chip type,
/// its device type, and its port. Each of these things controls slightly different aspects
/// of compilation. The port influences what files are compiled. The device and the chip
/// are passed into ChibiOS as flags.
///
/// The port files are consistent across device architectures, e.g. thumbv6m, thumbv7m, etc.
/// You can make your device selection depend upon its port (read: architecture) to include
/// the port files for compilation.
///
/// The device
///
/// In order to determine this, the easiest way is to inspect a demo Makefile and familiarize
/// yourself with the flags that are passed there, and how files are selected for compilation.
/// Once you've done that, you can inspect the device _type_'s `cmparams.h` file for a list
/// of supported chips.
fn main() {

    #[cfg(not(any(feature="port_thumbv6m",feature="port_thumbv7m")))]
    compile_error!("You must specify at least one target CPU feature. See Cargo.toml.");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut builder = gcc::Config::new();

    let bindings = bindgen::Builder::default()
        .header("./ChibiOS/os/common/abstractions/cmsis_os/cmsis_os.h")
        .ctypes_prefix("cty")
        .use_core()
        .trust_clang_mangling(false);

    builder.include("./ChibiOS/os/common/abstractions/cmsis_os");

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

    builder.file("./ChibiOS/os/common/abstractions/cmsis_os/cmsis_os.c");

    #[cfg(feature="port_thumbv7m")]
    let port_src_files = [
        "./ChibiOS/os/common/ports/ARMCMx/chcore.c",
        "./ChibiOS/os/common/ports/ARMCMx/chcore_v7m.c",
        "./ChibiOS/os/common/ports/ARMCMx/compilers/GCC/chcoreasm_v7m.S",
    ];

    #[cfg(feature="port_thumbv6m")]
    let port_src_files = [
        "./ChibiOS/os/common/ports/ARMCMx/chcore.c",
        "./ChibiOS/os/common/ports/ARMCMx/chcore_v6m.c",
        "./ChibiOS/os/common/ports/ARMCMx/compilers/GCC/chcoreasm_v6m.S",
    ];

    for port_src_file in port_src_files.iter() {
        builder.file(port_src_file);
    }

    let flags = [
        "-mno-thumb-interwork", // CFLAGS, because USB_THUMB is set
        "-ffunction-sections", // from os/common/startup/ARMCMx/compilers/GCC/rules.mk
        "-fdata-sections", // from os/common/startup/ARMCMx/compilers/GCC/rules.mk
        "-fno-common", // from os/common/startup/ARMCMx/compilers/GCC/rules.mk
        "-fomit-frame-pointer",
        "-falign-functions=16",
    ];

    for flag in flags.iter() {
        builder.flag(flag);
    }

    let include_dirs = [
        "./",                               // for chconf.h
        "./ChibiOS/os/license",
        "./ChibiOS/os/various",
        "./ChibiOS/os/rt/include",            // KERNINC, from os/rt/rt.mk
        "./ChibiOS/os/common/oslib/include",  // KERNINC, from os/rt/rt.mk
        "ChibiOS/os/common/ports/ARMCMx",                  // PORTINC, from os/common/ports/ARMCMx/compilers/GCC/mk/port_v?m.mk
        "ChibiOS/os/common/ports/ARMCMx/compilers/GCC",    // PORTINC, from os/common/ports/ARMCMx/compilers/GCC/mk/port_v?m.mk
        "ChibiOS/os/common/ext/CMSIS/include",                 // STARTUPINC, from os/common/startup/ARMCMx/compilers/GCC/mk/startup_*.mk
    ];

    for include_dir in include_dirs.iter() {
        builder.include(include_dir);
    }

    let bindings = bindings.clang_args(include_dirs.iter().map(|d| format!("-I{}", d)));

    #[cfg(feature="device_stm32f4xx")]
    let device_include_dirs = [
        "ChibiOS/os/common/startup/ARMCMx/devices/STM32F4xx",  // STARTUPINC, from os/common/startup/ARMCMx/compilers/GCC/mk/startup_stm32f4xx.mk
        "ChibiOS/os/common/ext/CMSIS/ST/STM32F4xx",            // STARTUPINC, from os/common/startup/ARMCMx/compilers/GCC/mk/startup_stm32f4xx.mk
    ];

    #[cfg(feature="device_stm32f0xx")]
    let device_include_dirs = [
        "ChibiOS/os/common/startup/ARMCMx/devices/STM32F0xx",   // STARTUPINC, from os/common/startup/ARMCMx/compilers/GCC/mk/startup_stm32f0xx.mk
        "ChibiOS/os/common/ext/CMSIS/ST/STM32F0xx",             // STARTUPINC, from os/common/startup/ARMCMx/compilers/GCC/mk/startup_stm32f0xx.mk
    ];

    #[cfg(feature="device_lpc177x_8x")]
    let device_include_dirs = [
        "ChibiOS/os/common/startup/ARMCMx/devices/LPC177x_8x/",
        "ChibiOS/os/common/ext/CMSIS/NXP/LPC177x_8x/",
    ];

    for include_dir in device_include_dirs.iter() {
        builder.include(include_dir);
    }

    let bindings = bindings.clang_args(device_include_dirs.iter().map(|d| format!("-I{}", d)));

    // These may require parameterization
    let defines = [
        ("THUMB_PRESENT", None),                              // CFLAGS, because USE_THUMB is set, rules.mk
        ("THUMB_NO_INTERWORKING", None),                      // CFLAGS, because USB_THUMB is set, rules.mk
    ];

    for &(def_key, def_val) in defines.iter() {
        builder.define(def_key, def_val);
    }

    #[cfg(feature="stm32f407xg")]
    let port_defines = [
        ("STM32F407xx", None), // UDEFS from RT-ARMCM4-GENERIC demo Makefile
        ("THUMB", None),
    ];

    #[cfg(feature="stm32f051x8")]
    let port_defines = [
        ("STM32F051x8", None), // UDEFS from RT-ARMCM0-GENERIC demo Makefile
        ("THUMB", None),
    ];

    #[cfg(feature="lpc1788fbd208")]
    let port_defines = [
        ("THUMB", None),
    ];

    for &(def_key, def_val) in port_defines.iter() {
        builder.define(def_key, def_val);
    }

    // These defines mirror those above; these are for bindgen, and the above are for
    // building libchibios. Unfortunately, because bindings::clang_arg() returns Self
    // and not &Self like gcc::define does, we cannot loop, and so must set each flag
    // "manually".
    #[cfg(feature="stm32f407xg")]
    let bindings = bindings.clang_arg("-DSTM32F407xx");
    #[cfg(feature="stm32f051x8")]
    let bindings = bindings.clang_arg("-DSTM32F051x8");
    //#[cfg(feature="lpc1788fbd208")]
    // no defines needed for this part

    builder.pic(false);
    builder.archiver("arm-none-eabi-ar");
    builder.compile("libchibios.a");

    #[cfg(feature="cortex_alternate_switch")]
    let bindings = bindings.clang_arg("-DCORTEX_ALTERNATE_SWITCH=TRUE");
    bindings
        .generate()
        .expect("unable to generate cmsis bindings")
        .write_to_file(out_dir.join("cmsis_os.rs"))
        .expect("unable to write cmsis bindings");

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.to_str().unwrap()
    );
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=chconf.h");
}
