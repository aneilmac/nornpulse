use crate::utils::cpp_adapter::CppString;
use callengine::call_engine;

#[repr(C, packed)]
pub struct SharedGallery {
    gallery_string: CppString,
}

impl SharedGallery {
    pub fn get() -> &'static mut SharedGallery {
        unsafe { std::mem::transmute(0x0060ed58) }
    }

    pub fn set_creature_gallery_folder(&mut self, folder: &str) {
        unsafe {
            std::ptr::write(&mut self.gallery_string, CppString::from(folder));
        }
    }

    #[call_engine(0x004871e0)]
    #[rustfmt::skip]
    pub unsafe fn clean_creature_gallery_folder(&mut self);
}
