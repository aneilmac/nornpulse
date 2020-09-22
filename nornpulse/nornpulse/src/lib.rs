#![feature(abi_thiscall)]
#![feature(c_variadic)]
#![feature(maybe_uninit_ref)]
#[rustfmt::skip::attributes(call_engine)]
#[macro_use]
mod utils;

#[macro_use]
extern crate pest_derive;

mod engine;
mod windowing;

use std::ffi::CStr;
use winapi::shared::minwindef::HINSTANCE;
use winapi::um::winnt::LPSTR;

/// This function has the same function signature of `WinMain` in Win32
/// applications and has become the new entry point for `WinMain` in Docking
/// Station. See:
/// https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-winmain
#[no_mangle]
pub extern "stdcall" fn nornpulse_main(
    _h_instance: HINSTANCE,
    _h_prev_instance: HINSTANCE,
    cmd_line: LPSTR,
    _show_cmd: i32,
) -> i32 {
    log::set_logger(&c2ers::flight_recorder::FLIGHT_RECORDER)
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .expect("Failed to set the FlightLogger. Aborting.");

    // Inject engine.exe with callback to functions defined in nornpulse.dll.
    unsafe {
        windowing::inject_calls();
        engine::inject_calls();
    }

    let cmd_line = unsafe { CStr::from_ptr(cmd_line) };
    let cmd_line = cmd_line.to_str().unwrap_or("");
    let exit_code = windowing::startup(cmd_line);
    match exit_code {
        Ok(_) => 0,
        Err(s) => {
            log::error!("{}", s);
            1
        }
    }
}
