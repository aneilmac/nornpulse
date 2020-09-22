use super::*;
use crate::utils::cpp_adapter::CppString;

pub unsafe fn inject_calls() {
    replace_call!(0x0041cfd0, new_configurator);
    replace_call!(0x00539f90, bind_to_file);
    replace_call!(0x0041d150, exists);
    replace_call!(0x0054a4c0, flush);
    replace_call!(0x0053a7f0, get_overload_13);
    replace_call!(0x0053a910, get_overload_23);
    replace_call!(0x0053aa90, get_overload_33);
    replace_call!(0x0053ab50, set_overload_12);
    replace_call!(0x0053ace0, set_overload_22);
    replace_call!(0x0041d050, configurator_destructor);
}

pub extern "thiscall" fn new_configurator(
    dst: *mut Configurator,
) -> *mut Configurator {
    unsafe { std::ptr::write(dst, Configurator::new()); }
    dst
}


extern "thiscall" fn bind_to_file(configurator: &mut Configurator, file: &CppString) -> bool {
    let file = file.to_string();
    let result = configurator.bind_to_file(file);
    result.is_ok()
}

extern "thiscall" fn exists(configurator: &Configurator, key: &CppString) -> bool {
    let key = key.to_string();
    configurator.exists(&key)
}

extern "thiscall" fn flush(configurator: &mut Configurator) -> bool {
    let result = configurator.flush();
    result.is_ok()
}

extern "thiscall" fn get_overload_13(
    configurator: &Configurator,
    storage_pointer: *mut CppString,
    key: &CppString,
) -> *mut CppString {
    get_overload_23(configurator, key, storage_pointer);
    storage_pointer
}

extern "thiscall" fn get_overload_23(
    configurator: &Configurator,
    key: &CppString,
    out: *mut CppString,
) {
    let key = key.to_string();
    let value = configurator.get(&key);

    unsafe {
        std::ptr::write(
            out,
            match value {
                Some(value) => CppString::from(value.clone()),
                _ => CppString::empty(),
            },
        );
    }
}

extern "thiscall" fn get_overload_33(configurator: &Configurator, key: &CppString, out: *mut i32) {
    let key = key.to_string();
    let value = configurator.get(&key);

    let mut out_i = 0;
    if let Some(value) = value {
        if let Ok(i) = value.parse::<i32>() {
            out_i = i;
        }
    }

    unsafe {
        std::ptr::write(out, out_i);
    }
}

extern "thiscall" fn set_overload_12(
    configurator: &mut Configurator,
    key: &CppString,
    value: &CppString,
) {
    let key = key.to_string();
    let value = value.to_string();
    configurator.set(key, value);
}

extern "thiscall" fn set_overload_22(configurator: &mut Configurator, key: &CppString, value: i32) {
    let key = key.to_string();
    let value = value.to_string();
    configurator.set(key, value);
}

extern "thiscall" fn configurator_destructor(configurator: *mut Configurator) {
    unsafe { std::ptr::drop_in_place(configurator) };
}
