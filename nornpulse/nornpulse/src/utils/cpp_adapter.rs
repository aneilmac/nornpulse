use std::os::raw::c_char;
use callengine::call_engine;

/// Representation of a C++ string. Contains C++
#[repr(C, packed)]
pub struct CppString {
    _unknown: [u8; 4],
    data: *const c_char,
    length: usize,
    capacity: usize,
}

/// Calls C++'s `delete` upon the given pointer. Segfaults will
/// trigger if applied to any pointer than those allocated by `operator_new`.
#[call_engine(0x005870cc, "cdecl")]
unsafe fn operator_delete(ptr: *mut libc::c_void);

/// Given a size, returns a newly allocated pointer using C++'s `new`.
/// Segfaults will trigger if this is deallocated with anything other than
/// `operator_delete`.
#[call_engine(0x005870d2, "cdecl")]
unsafe fn operator_new(size: usize) -> *mut libc::c_void;

impl std::fmt::Display for CppString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str_copy = Vec::<u8>::with_capacity(self.length);

        unsafe {
            str_copy.set_len(self.length);
            std::ptr::copy(self.data, str_copy.as_mut_ptr() as *mut i8, self.length);
        }

        let wrapped_str =
            std::ffi::CString::new(str_copy).expect("C++ String has invalid internal \\0.");

        let output = wrapped_str.to_str().expect("C++ String has invalid UTF8.");

        write!(f, "{}", output)
    }
}

impl std::fmt::Debug for CppString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Drop for CppString {
    fn drop(&mut self) {
        self.length = 0;
        self.capacity = 0;
        if self.data != std::ptr::null() {
            unsafe {
                // Pre C++11 Strings are COW. In this particular implementation there is
                // one extra char allocated at the start of the string used for
                // reference counting.
                let start_ptr = self.data.offset(-1) as *mut i8;
                let ref_count = *start_ptr;
                if ref_count == -1 || ref_count == 0 {
                    operator_delete(start_ptr as *mut libc::c_void);
                } else {
                    *(start_ptr) -= 1;
                }
            }
            self.data = std::ptr::null();
        }
    }
}

impl Clone for CppString {
    fn clone(&self) -> Self {
        unsafe { CppString::from_c_str(self.data) }
    }
}

impl CppString {
    pub unsafe fn from_c_str(c_str_ptr: *const c_char) -> CppString {
        let c_str = std::ffi::CStr::from_ptr(c_str_ptr);
        let len = c_str.to_bytes().len();
        let cpp_str_ptr = operator_new(len + 2) as *mut c_char;
        std::ptr::copy(c_str_ptr, cpp_str_ptr.offset(1), len + 1); // length + null terminator.
        *cpp_str_ptr = 0; // Ref count.
        CppString {
            _unknown: [0, 0, 0, 0],
            data: cpp_str_ptr,
            length: len,
            capacity: len,
        }
    }

    pub fn empty() -> CppString {
        CppString {
            _unknown: [0, 0, 0, 0],
            data: std::ptr::null(),
            length: 0,
            capacity: 0,
        }
    }
}
