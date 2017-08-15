
ChibiOS is pretty cool. Rust is pretty cool.

This is an attempt to get ChibiOS useful via Rust. Or maybe it's to make Rust useful on ChibiOS.

Setup
-----

```bash
$ rustup override set nightly
$ cargo install xargo
$ brew install llvm # per https://rust-lang-nursery.github.io/rust-bindgen/requirements.html
```

Update the `memory.x` file to reflect the layout of your hardware.

Build
-----

Selecing your desired hardware is kind of an ugly process at the moment, and not very well
fleshed-out. ChibiOS requires a few flags to specify the port (i.e. CPU type), which we need
to pass in to the build script. As such, each CPU will need to get its own feature which will
gate compilation of `libchibios.a`, manage `bindgen`, as well as set handlers in `main.rs`.

See [`Cargo.toml`](Cargo.toml) to dive into it. If you're not adding your own chip, the following
are currently supported:

* `stm32f407xg`
* `stm32f051x8`

Pass them to Cargo using the `--features` flag, e.g.

```bash
$ xargo build --target thumbv7m-none-eabi --features stm32f407xg
```

Use
---

I like openocd and gdb.

```bash
$ openocd -f board/stm32f0discovery.cfg -c '$_TARGETNAME configure -rtos auto' -c 'gdb_port 3333' # or whatever
```
