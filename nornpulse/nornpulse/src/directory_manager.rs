use callengine::call_engine;

#[repr(C, packed)]
pub struct DirectoryManager {}

impl DirectoryManager {
    #[call_engine(0x0055d0d0)]
    pub unsafe fn read_from_configuration_files(&self) -> bool;
}

pub fn get_directory_manager() -> &'static mut DirectoryManager {
    unsafe { return std::mem::transmute(0x0062df30) }
}
