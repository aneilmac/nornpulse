use crate::engine::caos_machine::CAOSMachine;
use crate::engine::caos_var::CAOSVar;
use crate::utils::cpp_adapter::CppString;
use c2ers::agents::AgentHandle;

type Call = extern "C" fn(&mut CAOSMachine) -> ();
type GetInt = extern "C" fn(&mut CAOSMachine) -> i32;
type GetFloat = extern "C" fn(&mut CAOSMachine) -> f32;
type GetString = extern "C" fn(&mut CAOSMachine, *mut CppString) -> ();
type GetHandle = extern "C" fn(&mut CAOSMachine) -> AgentHandle;
type GetVar = extern "C" fn(&mut CAOSMachine) -> *mut CAOSVar;

#[repr(C)]
pub struct HandlerFunction {
    call: Option<Call>,
    get_int: Option<GetInt>,
    get_float: Option<GetFloat>,
    get_string: Option<GetString>,
    get_handle: Option<GetHandle>,
    get_var: Option<GetVar>,
}

impl HandlerFunction {
    pub fn from_call(f: Call) -> Self {
        let mut s = Self::empty();
        s.call = Some(f);
        s
    }

    pub fn from_get_int(f: GetInt) -> Self {
        let mut s = Self::empty();
        s.get_int = Some(f);
        s
    }

    pub fn from_get_float(f: GetFloat) -> Self {
        let mut s = Self::empty();
        s.get_float = Some(f);
        s
    }

    pub fn from_get_string(f: GetString) -> Self {
        let mut s = Self::empty();
        s.get_string = Some(f);
        s
    }

    pub fn from_get_handle(f: GetHandle) -> Self {
        let mut s = Self::empty();
        s.get_handle = Some(f);
        s
    }

    pub fn from_get_var(f: GetVar) -> Self {
        let mut s = Self::empty();
        s.get_var = Some(f);
        s
    }

    pub fn empty() -> Self {
        Self {
            call: None,
            get_int: None,
            get_float: None,
            get_string: None,
            get_handle: None,
            get_var: None,
        }
    }
}
