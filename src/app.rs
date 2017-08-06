
use chibios;

extern "C" fn thread_one(arg: chibios::RawOSArg) {
    loop {}
}

pub fn user_main() {
    let null_mut: chibios::RawOSArgMut = 0 as chibios::RawOSArgMut;

    let thread_one_def = chibios::OsThreadDef::new(
        thread_one,
        chibios::osPriority::osPriorityNormal,
        0x400,
        "thread_one",
    );

    unsafe {
        chibios::osKernelInitialize();
    }

    let thread_one_id = unsafe { chibios::osThreadCreate(&thread_one_def.into(), null_mut) };

    let kernel_status = unsafe { chibios::osKernelStart() };

    loop {}
}
