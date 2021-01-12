use crate::engine::caos_var::CAOSVar;
use crate::engine::macro_script::MacroScript;
use crate::utils::cpp_adapter::CppString;
use callengine::call_engine;

#[repr(C, packed)]
pub struct CAOSMachine {}

impl CAOSMachine {
    #[call_engine(0x004c62c0, "cdecl")]
    #[rustfmt::skip]
    pub unsafe fn initialize_handler_tables();

    pub fn fetch_string_rv(&mut self) -> CppString {
        let mut s = CppString::empty();
        unsafe { _fetch_string_rv(self, &mut s) };
        s
    }

    pub fn fetch_integer_rv(&mut self) -> i32 {
        unsafe { _fetch_integer_rv(self) }
    }

    pub fn fetch_float_rv(&mut self) -> f32 {
        unsafe { _fetch_float_rv(self) }
    }

    pub fn fetch_generic_rv(&mut self) -> CAOSVar {
        let mut cv = CAOSVar::empty();
        unsafe { _fetch_generic_rv(self, &mut cv) };
        cv
    }

    pub fn fetch_variable(&mut self) {
        unsafe { _fetch_variable(self) }
    }

    #[call_engine(0x00449900)]
    #[rustfmt::skip]
    pub unsafe fn set_ip(&mut self, ip: isize);

    #[call_engine(0x00449910)]
    #[rustfmt::skip]
    pub unsafe fn ip(&self) -> isize;

    #[call_engine(0x0044a180)]
    #[rustfmt::skip]
    pub unsafe fn is_blocking(&self) -> bool;

    #[call_engine(0x00449e40)]
    #[rustfmt::skip]
    pub unsafe fn pop(&mut self) -> i32;


    #[rustfmt::skip]
    pub fn script(&self) -> &MacroScript
    {
        unsafe {
            let s = _script(self);
             &*s
        }
    }
}

#[call_engine(0x004c8780, "thiscall")]
#[rustfmt::skip]
pub unsafe fn _fetch_string_rv(machine: *mut CAOSMachine, s: &mut CppString);

#[call_engine(0x004c8710, "thiscall")]
#[rustfmt::skip]
pub unsafe fn _fetch_integer_rv(machine: *mut CAOSMachine) -> i32;

#[call_engine(0x004c8750, "thiscall")]
#[rustfmt::skip]
pub unsafe fn _fetch_float_rv(machine: *mut CAOSMachine) -> f32;

#[call_engine(0x0044a230, "thiscall")]
#[rustfmt::skip]
pub unsafe fn _fetch_variable(machine: *mut CAOSMachine);

#[call_engine(0x004c8d00, "thiscall")]
#[rustfmt::skip]
pub unsafe fn _fetch_generic_rv(
    machine: *mut CAOSMachine,
    storage_ptr: *mut CAOSVar
) -> *mut CAOSVar;

#[call_engine(0x004497a0, "thiscall")]
#[rustfmt::skip]
pub unsafe fn _script(this: *const CAOSMachine) -> *const MacroScript;
