extern crate cortex_m_rt;

/// Macro for wrapping an unsafe C function for use as an exception.
///
/// This macro depends upon the `cortex-m` crate's `exception!` macro, which takes
/// a Rust function (`fn()`). The ChibiOS handlers we want to use, however, are
/// of type `unsafe extern "C" fn()`.
///
/// This macro was copied from [japaric's
/// cortex-m-rt](https://github.com/japaric/cortex-m-rt/blob/master/src/lib.rs) version, and (very)
/// slightly modified.
#[macro_export]
macro_rules! c_exception {
    ($NAME:ident, $path:path, locals: {
        $($lvar:ident:$lty:ident = $lval:expr;)+
    }) => {
        #[allow(non_snake_case)]
        mod $NAME {
            pub struct Locals {
                $(
                    pub $lvar: $lty,
                )+
            }
        }

        #[allow(non_snake_case)]
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern "C" fn $NAME() {
            // check that the handler exists
            let _ = cortex_m_rt::Exception::$NAME;

            static mut LOCALS: self::$NAME::Locals = self::$NAME::Locals {
                $(
                    $lvar: $lval,
                )*
            };

            // type checking
            let f: unsafe extern "C" fn(&mut self::$NAME::Locals) = $path;
            f(&mut LOCALS);
        }
    };
    ($NAME:ident, $path:path) => {
        #[allow(non_snake_case)]
        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern "C" fn $NAME() {
            // check that the handler exists
            let _ = cortex_m_rt::Exception::$NAME;

            // type checking
            let f: unsafe extern "C" fn() = $path;
            f();
        }
    }
}

/// Build a null-terminated C string
///
/// Thread name arguments must be C strings, i.e. null-terminated. This is a utility
/// macro that will permit convenient use of Rust string literals when constructing
/// them.
///
/// Cribbed directly from libstd.
#[macro_export]
macro_rules! szstr {
    ($s:expr) => (
        concat!($s, "\0")
    )
}
