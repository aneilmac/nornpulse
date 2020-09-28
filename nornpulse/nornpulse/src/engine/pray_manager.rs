use crate::utils::cpp_adapter::CppString;
use callengine::call_engine;

#[repr(C, packed)]
pub struct PrayManager {}

impl PrayManager {
    #[call_engine(0x005711a0)]
    #[rustfmt::skip]
    pub unsafe fn get() -> &'static mut PrayManager;

    pub fn set_language(&mut self, lang: &str) {
        let s = CppString::from(lang);
        unsafe { _set_language(self, s) }
    }
}

#[call_engine(0x0054bfc0, "thiscall")]
#[rustfmt::skip]
unsafe fn _set_language(this: *mut PrayManager, lang: CppString);
