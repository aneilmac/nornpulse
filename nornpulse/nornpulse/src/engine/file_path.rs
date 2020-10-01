use crate::utils::cpp_adapter::CppString;

use callengine::call_engine;

pub struct FilePath {
    _unknown: [u8; 20],
}

impl FilePath {
    pub fn new(s: &str, i: i32, a: bool, b: bool) -> Self {
        let s = CppString::from(s);
        unsafe {
            let mut fp: FilePath = std::mem::zeroed();
            _constructor(&mut fp, s, i, a, b);
            fp
        }
    }

    pub fn full_path(&self) -> String {
        let mut s = CppString::empty();
        unsafe {
            _full_path(self, &mut s);
        }
        s.to_string()
    }
}

#[call_engine(0x0054a360, "thiscall")]
#[rustfmt::skip]
unsafe fn _constructor(
    storage: *mut FilePath, 
    str: CppString, 
    i: i32, 
    a: bool, 
    b: bool
) -> *mut FilePath;

#[call_engine(0x0054a470, "thiscall")]
#[rustfmt::skip]
unsafe fn _full_path(
    this: &FilePath, 
    str: *mut CppString, 
) ->  *mut CppString;
