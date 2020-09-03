use callengine::call_engine;

#[repr(C, packed)]
pub struct DirectoryManager {}

impl DirectoryManager {
    #[call_engine(0x0055d0d0)]
    pub unsafe fn read_from_configuration_files(&mut self) -> bool;

    pub unsafe fn get() -> &'static mut DirectoryManager {
        std::mem::transmute(0x0062df30)
    }
}
