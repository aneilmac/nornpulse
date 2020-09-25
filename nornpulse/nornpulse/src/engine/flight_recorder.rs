use c2ers::flight_recorder as cf;
use callengine::call_engine;
use log::Log;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(FromPrimitive)]
#[repr(u32)]
enum FlightCategory {
    FlightRecordDestruct = 0x00,
    FatalError = 0x01,
    Shutdown = 0x10,
    PrayViolation = 0x20,
    Initializing = 0x40,
}

fn category_to_level(c: u32) -> log::Level {
    match FromPrimitive::from_u32(c) {
        Some(FlightCategory::FlightRecordDestruct) => log::Level::Debug,
        Some(FlightCategory::FatalError) => log::Level::Error,
        Some(FlightCategory::Shutdown) => log::Level::Debug,
        Some(FlightCategory::PrayViolation) => log::Level::Warn,
        Some(FlightCategory::Initializing) => log::Level::Debug,
        None => log::Level::Info,
    }
}

pub unsafe fn inject_calls() {
    use injected_calls::*;
    replace_call!(0x00536ab0, get_flight_recorder);
    replace_call!(0x0054b550, f_log);
    replace_call!(0x0054b760, set_flight_categories);
}

mod injected_calls {
    use super::*;

    // C2E:0x0054b550
    pub unsafe extern "C" fn f_log(
        recorder_ptr: *mut cf::FlightRecorder,
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
    pub unsafe extern "C" fn get_flight_recorder() -> *const cf::FlightRecorder {
        return &cf::FLIGHT_RECORDER as *const _;
    }

    // C2E:0x0054b760
    pub unsafe extern "thiscall" fn set_flight_categories(
        _recorder_ptr: &mut cf::FlightRecorder,
        _flight_category_mask: u32,
    ) {
        log::debug!(
            target: "engine.exe",
            "FlightRecorder::SetCategories called. Ignored."
        );
    }
}

type SprintfFN = unsafe extern "C" fn(*mut c_char, *const c_char, ...);

/// The darkest of dark arts. From the lookup table grab vsprintf from
///  MSVCRT.dll. There is probably not an unsafer function in existence.
unsafe fn get_vsprintf() -> SprintfFN {
    type FF = *const SprintfFN;
    let op_address: FF = std::mem::transmute(0x0062fc40);
    *op_address
}
