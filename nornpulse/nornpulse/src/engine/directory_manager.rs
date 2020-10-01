use crate::utils::cpp_adapter::CppString;
use callengine::call_engine;

#[repr(C, packed)]
pub struct DirectoryManager {}

impl DirectoryManager {
    #[call_engine(0x0055d0d0)]
    #[rustfmt::skip]
    pub unsafe fn read_from_configuration_files(&mut self) -> bool;

    pub unsafe fn get() -> &'static DirectoryManager {
        std::mem::transmute(0x0062df30)
    }

    pub unsafe fn get_mut() -> &'static mut DirectoryManager {
        std::mem::transmute(0x0062df30)
    }

    pub unsafe fn directory(&self, param: i32) -> String {
        let mut s = CppString::empty();
        type FF = extern "thiscall" fn(
            this: &DirectoryManager,
            s: *mut CppString,
            i: i32,
        ) -> *mut CppString;
        let op_address: FF = std::mem::transmute(0x0055e1a0);
        op_address(self, &mut s, param);
        s.to_string()
    }
}
