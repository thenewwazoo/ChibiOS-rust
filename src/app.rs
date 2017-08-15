
use chibios;

extern "C" fn thread_one(_: chibios::RawOSArg) {
    loop {}
}

pub fn user_main() {
    let null_mut: chibios::RawOSArgMut = 0 as chibios::RawOSArgMut;

    let  thread_one_def: chibios::OsThreadDef = chibios::OsThreadDef::new(
        thread_one,
        chibios::osPriority::osPriorityNormal,
        0x400,
        szstr!("thread_one"),
        );

    unsafe {
        chibios::osKernelInitialize();
    }

    unsafe { chibios::osThreadCreate(&thread_one_def.into(), null_mut) };

    unsafe { chibios::osKernelStart() };

    loop {}
}
