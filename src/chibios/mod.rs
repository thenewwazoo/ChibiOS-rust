
extern crate cty;

#[allow(dead_code)]
mod bindings;

pub use self::bindings::{osKernelInitialize, osKernelStart, osThreadCreate, osPriority};

pub type RawOSArg = *const cty::c_void;
pub type RawOSArgMut = *mut cty::c_void;
pub type OsPThread = unsafe extern "C" fn(RawOSArg);

pub trait OptPtr<T> {
    fn as_ptr<'a>(o: Option<T>) -> *const cty::c_void;
    //    fn as_mut_ptr<'a>(o: Option<&'a T>) -> *mut cty::c_void;
}

impl<T> OptPtr<T> for Option<T> {
    fn as_ptr<'a>(o: Option<T>) -> *const cty::c_void {
        match o {
            Some(p) => &p as *const _ as *const cty::c_void,
            None => 0 as *const cty::c_void,
        }
    }
}

pub struct OsThreadDef {
    pthread: OsPThread,
    priority: bindings::osPriority,
    stack_size: u32,
    name: &'static str,
}

impl OsThreadDef {
    pub fn new(
        thread_func: OsPThread,
        prio: bindings::osPriority,
        stack_size: u32,
        name: &'static str,
    ) -> Self {
        OsThreadDef {
            pthread: thread_func,
            priority: prio,
            stack_size: stack_size,
            name: name,
        }
    }
}

impl From<OsThreadDef> for bindings::osThreadDef_t {
    fn from(t: OsThreadDef) -> Self {
        bindings::osThreadDef_t {
            pthread: Some(t.pthread),
            tpriority: t.priority,
            stacksize: t.stack_size,
            name: t.name as *const _ as *const cty::c_char,
        }
    }
}
