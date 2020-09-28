use crate::engine::handler_function::HandlerFunction;
use crate::engine::op_spec::OpSpec;
use crate::utils::cpp_adapter::CppString;
use callengine::call_engine;

#[repr(C, packed)]
pub struct CAOSDescription {}

impl CAOSDescription {
    pub unsafe fn get() -> &'static mut CAOSDescription {
        std::mem::transmute(0x0060fa08)
    }

    #[call_engine(0x004c3860)]
    #[rustfmt::skip]
    pub unsafe fn load_default_tables(&mut self);

    pub fn save_syntax(&mut self, syntax_folder: &str) -> Result<(), String> {
        let mut s = CppString::from(syntax_folder);
        let result = unsafe { _save_syntax(self, &mut s) };
        match result {
            true => Ok(()),
            false => Err(s.to_string()),
        }
    }

    #[rustfmt::skip]
    pub fn push_table(
        &mut self,
        f: HandlerFunction,
        i: i32,
        name: &str,
        op_specs: &[OpSpec],
    ) {
        let name = CppString::from(name);
        unsafe {
            _push_table(
                self,
                f,
                i,
                &name,
                op_specs.as_ptr(),
                op_specs.len()
            );
        }
    }
}

#[call_engine(0x004bf6f0, "thiscall")]
#[rustfmt::skip]
pub unsafe fn _push_table(
    this: *mut CAOSDescription,
    f: HandlerFunction,
    i: i32,
    name: *const CppString,
    op_spec: *const OpSpec,
    spec_count: usize,
);

#[call_engine(0x004bfc50, "thiscall")]
#[rustfmt::skip]
unsafe fn _save_syntax(this: &mut CAOSDescription, s: *mut CppString) -> bool;
