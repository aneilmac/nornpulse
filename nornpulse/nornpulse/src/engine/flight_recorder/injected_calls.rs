use super::*;

pub unsafe fn inject_calls() {
    use injected_calls::*;
    replace_call!(0x00536ab0, get_flight_recorder);
    replace_call!(0x0054b550, f_log);
    replace_call!(0x0054b760, set_flight_categories);
}

// C2E:0x0054b550
pub unsafe extern "C" fn f_log(
    recorder_ptr: *mut FlightRecorder,
    flight_category: u32,
    message: *const c_char,
    args: ...
) {
    // v is used as a raw allocated memory block.
    let mut v = Vec::<i8>::with_capacity(4096);
    let vsprintf = get_vsprintf();
    vsprintf(v.as_mut_ptr(), message, args);

    let message = CStr::from_ptr(v.as_ptr());
    let message: &str = message.to_str().unwrap();

    (*recorder_ptr).log(
        &log::Record::builder()
            .args(format_args!("({}) {}", flight_category, message))
            .level(category_to_level(flight_category))
            .target("engine.exe")
            .build(),
    );
}

// C2E:0x00536ab0
pub unsafe extern "C" fn get_flight_recorder() -> *const FlightRecorder {
    return &FLIGHT_RECORDER as *const _;
}

// C2E:0x0054b760
pub unsafe extern "thiscall" fn set_flight_categories(
    _recorder_ptr: &mut FlightRecorder,
    _flight_category_mask: u32,
) {
    log::debug!(
        target: "engine.exe",
        "FlightRecorder::SetCategories called. Ignored."
    );
}

type SprintfFN = unsafe extern "C" fn(*mut c_char, *const c_char, ...);

/// The darkest of dark arts. From the lookup table grab vsprintf from
///  MSVCRT.dll. There is probably not an unsafer function in existence.
unsafe fn get_vsprintf() -> SprintfFN {
    type FF = *const SprintfFN;
    let op_address: FF = std::mem::transmute(0x0062fc40);
    *op_address
}
