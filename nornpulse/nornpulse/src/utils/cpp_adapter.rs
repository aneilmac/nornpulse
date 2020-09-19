use std::os::raw::c_char;
use callengine::call_engine;

/// Calls C++'s `delete` upon the given pointer. Segfaults will
/// trigger if applied to any pointer than those allocated by `operator_new`.
#[call_engine(0x005870cc, "cdecl")]
unsafe fn operator_delete(ptr: *mut libc::c_void);

/// Given a size, returns a newly allocated pointer using C++'s `new`.
/// Segfaults will trigger if this is deallocated with anything other than
/// `operator_delete`.
#[call_engine(0x005870d2, "cdecl")]
unsafe fn operator_new(size: usize) -> *mut libc::c_void;

/// Representation of a C++ string. Contains C++
#[repr(C, packed)]
pub struct CppString {
    _unknown: [u8; 4],
    data: *const c_char,
    length: usize,
    capacity: usize,
}

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
    pub unsafe fn from_c_str(c_str_ptr: *const c_char) -> Self {
        let c_str = std::ffi::CStr::from_ptr(c_str_ptr);
        let len = c_str.to_bytes().len();
        let cpp_str_ptr = operator_new(len + 2) as *mut c_char;
         // length + null terminator.
        std::ptr::copy_nonoverlapping(c_str_ptr, cpp_str_ptr.offset(1), len + 1);
        *cpp_str_ptr = 0; // Ref count.
        CppString {
            _unknown: [0, 0, 0, 0],
            data: cpp_str_ptr,
            length: len,
            capacity: len,
        }
    }

    pub const fn empty() -> Self {
        CppString {
            _unknown: [0, 0, 0, 0],
            data: std::ptr::null(),
            length: 0,
            capacity: 0,
        }
    }
}

#[repr(C, packed)]
pub struct CppVector<T> {
    _allocator: u8,
    _padding: [u8;3],
    start_ptr: *mut T,
    length_end_ptr: *mut T,
    capacity_end_ptr: *mut T,
}

impl<T> CppVector<T> {
    pub const fn empty() -> Self {
        Self {
            _allocator: 104, // Always seems to be the value for raw allocator.
            _padding: [0, 0, 0],
            start_ptr: std:: ptr::null_mut(),
            length_end_ptr: std::ptr::null_mut(),
            capacity_end_ptr: std::ptr::null_mut(),
        }
    }

    pub fn clear(&mut self) {
        self.length_end_ptr = self.start_ptr;
    }

    fn len_bytes(&self) -> usize {
        if self.start_ptr.is_null() {
            0
        } else {
            let k = self.length_end_ptr as i32 - self.start_ptr as i32;
            if k < 0 { 0 } else { k as usize }
        }
    }

    pub fn len(&self) -> usize {
        self.len_bytes() /  std::mem::size_of::<T>()
    }

    pub fn push(&mut self, val: T) {
        unsafe {
            if !self.length_end_ptr.is_null() && self.length_end_ptr < self.capacity_end_ptr {
                std::ptr::write_unaligned(self.length_end_ptr, val);
                self.length_end_ptr = self.length_end_ptr.offset(1);
            } else {
                let len_bytes = self.len_bytes();
                let capacity_bytes = len_bytes + std::mem::size_of::<T>();
                let start_ptr = operator_new(capacity_bytes) as *mut T;
                if len_bytes > 0 {
                    std::ptr::copy_nonoverlapping(self.start_ptr, start_ptr, self.len());
                    operator_delete(self.start_ptr as *mut libc::c_void);
                }
                self.start_ptr = start_ptr;
                self.length_end_ptr = (start_ptr as usize + len_bytes) as *mut T;
                self.capacity_end_ptr = (start_ptr as usize + capacity_bytes) as *mut T;
                self.push(val);
            }
        }
    }
}

impl<T> core::ops::Index<usize> for CppVector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &(*self.start_ptr.offset(index as isize)) }
    }
}

impl<T> core::ops::IndexMut<usize> for CppVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut(*self.start_ptr.offset(index as isize)) }
    }
}

impl<T> Clone for CppVector<T> {
    fn clone(&self) -> Self {
        let start_ptr;
        let end_ptr;
        unsafe {
            let len_bytes = self.len_bytes();
            start_ptr = operator_new(len_bytes) as *mut T;
            end_ptr = (start_ptr as usize + len_bytes) as *mut T;
            std::ptr::copy_nonoverlapping(self.start_ptr, start_ptr, self.len());
        }
        Self {
            _allocator: self._allocator,
            _padding: self._padding,
            start_ptr: start_ptr,
            length_end_ptr: end_ptr,
            capacity_end_ptr: end_ptr
        }
    }
}

impl<T: Copy> Into<Vec<T>> for CppVector<T> {
    fn into(self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.len());
        let mut current = self.start_ptr;
        while current != self.length_end_ptr {
            unsafe { 
                v.push(std::ptr::read_unaligned(current));
                current = current.offset(1);
            }
        }
        v
    }
}

impl<T> Drop for CppVector<T> {
    fn drop(&mut self) {
        unsafe { operator_delete(self.start_ptr as *mut libc::c_void); }
        self.start_ptr =  std::ptr::null_mut();
        self.capacity_end_ptr =  std::ptr::null_mut();
        self.length_end_ptr =  std::ptr::null_mut();
    }
}

impl<T: Copy + std::fmt::Debug> std::fmt::Debug for CppVector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v : Vec<T> = self.clone().into();
        std::fmt::Debug::fmt(&v, f)
    }
}