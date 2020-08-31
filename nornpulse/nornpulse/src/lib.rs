#![feature(abi_thiscall)]
#![feature(c_variadic)]
#![feature(maybe_uninit_ref)]

#[macro_use]
mod utils;

mod engine;
mod startup;

use startup::startup;
use std::ffi::CStr;
use winapi::shared::minwindef::HINSTANCE;
use winapi::um::winnt::LPSTR;


/// This function has the same function signature of `WinMain` in Win32
/// applications and has become the new entry point for `WinMain` in Docking
/// Station. See:
/// https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-winmain
#[no_mangle]
pub extern "stdcall" fn nornpulse_main(
    h_instance: HINSTANCE,
    _h_prev_instance: HINSTANCE,
    cmd_line: LPSTR,
    _show_cmd: i32,
) -> i32 {
    log::set_logger(&c2ers::flight_recorder::FLIGHT_RECORDER)
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .expect("Failed to set the FlightLogger. Aborting.");

    // Inject engine.exe with callback to functions defined in nornpulse.dll.
    unsafe { engine::inject_calls(); }

    let cmd_line = unsafe { CStr::from_ptr(cmd_line) };
    let exit_code = startup(h_instance, cmd_line);
    match exit_code {
        Ok(i) => i,
        Err(i) => i
    }
}



// let sdl_context = sdl2::init()
//   .expect("Unable to initialize SDL2");

// sdl_context.mouse().show_cursor(false);

// let video_subsystem = sdl_context.video()
//   .expect("Could not init video subsystem");

// let mut window = video_subsystem.window("Docking Station (Rust)", 800, 600)
//   .position_centered()
//   .opengl()
//   .build()
//   .map_err(|e| e.to_string()).expect("Could not convert error.");

// window.set_icon(c2ers::windowing::icon::ds_icon_bmp());

// let mut event_pump = sdl_context.event_pump()
//   .expect("Could not generate SDL2 event pump");

// for event in event_pump.poll_iter() {
//   match event {
//     sdl2::event::Event::Quit{..} => our_quit = true,
//     _ => ()
//   }
//   //app.handle_event(event);
// }
