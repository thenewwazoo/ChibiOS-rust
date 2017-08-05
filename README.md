
This is a naive attempt to get ChibiOS useful in Rust.

It is ChibiOS and bindgen layered atop [bare-minimum-rust](https://github.com/thenewwazoo/bare-minimum-rust).

Setup
-----

```bash
$ rustup override set nightly
$ cargo install xargo
$ brew install llvm # per https://rust-lang-nursery.github.io/rust-bindgen/requirements.html
```

Build
-----

Alas! Selecting your preferred hardware is currently kind of an ugly process. You'll need to dig into the ChibiOS makefiles a bit. A good starting point is a ChibiOS demo project. You'll also want to mess with `chconf.h` probably.

If you want the CMSIS-RTOS bindings, also enable the `cmsis_os` feature.

```
$ git submodule update
$ xargo build -vv --target thumbv6m-none-eabi --features=stm32f051x8,cmsis_os
```
