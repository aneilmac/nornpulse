use crate::engine::app::App;
use crate::engine::directory_manager::DirectoryManager;
use crate::windowing::icon;
use crate::windowing::input_convert::{keycode_to_vk_keycode, map_mouse_btn};
use callengine::call_engine;
use std::ffi::CStr;
use winapi::shared::windef::HWND;

mod global {
    use super::*;
    pub const HWND: *mut HWND = unsafe { std::mem::transmute(0x0060ebe0) };
    pub const GAME_RUNNING: *mut bool = unsafe { std::mem::transmute(0x0060eba8) };
}

const WINDOW_TITLE: &str = "Nornpulse";
const WINDOW_WIDTH: u32 = 0x326;
const WINDOW_HEIGHT: u32 = 0x271;

struct TickGameStepEvent {}

static mut EVENT_PENDING: bool = false;
static mut EVENT_SUBSYSTEM: *const sdl2::EventSubsystem = std::ptr::null();

pub fn startup(cmd_line: &CStr) -> Result<(), String> {
    unsafe {
        *global::GAME_RUNNING = true;
    }

    log::debug!("Processing command line.");
    let success = unsafe { App::get().process_command_line(cmd_line) };
    if !success {
        return Err(String::from(
            "Could not parse command-line arguments. Aborting",
        ));
    }

    init_app()?;

    log::debug!("Setting up SDL2.");
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;


    let _window = init_sdl2_window(&video_subsystem)?;
    sdl_context.mouse().show_cursor(false);

    log::debug!("Calling startup.");
    let result = unsafe { do_startup(*global::HWND) };
    if !result {
        return Err(String::from("Could not initialize app."));
    }

    let event_subsystem = sdl_context.event()?;
    event_subsystem.register_custom_event::<TickGameStepEvent>()?;

    unsafe {
        EVENT_SUBSYSTEM = &event_subsystem;
    }

    let mut event_pump = sdl_context.event_pump()?;
    log::debug!("Starting message loop.");
    for event in event_pump.wait_iter() {
        let continue_game = process_event(&event_subsystem, event);

        if !continue_game {
            break;
        }
    }
    log::debug!("Shutdown message loop.");

    unsafe {
        message_destroy_called();
    }

    Ok(())
}

fn init_sdl2_window(video_subsystem: &sdl2::VideoSubsystem) -> Result<sdl2::video::Window, String> {
    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .resizable()
        .build();

    if window.is_err() {
        return Err(String::from("Could not build window."));
    }

    let mut window = window.unwrap();
    window.set_icon(icon::ds_icon_bmp());

    unsafe { // TODO: Remove. We don't want a global HWND variable.
        let mut info = std::mem::zeroed();
        sdl2_sys::SDL_GetWindowWMInfo(window.raw(), &mut info);
        assert_eq!(info.subsystem, sdl2_sys::SDL_SYSWM_TYPE::SDL_SYSWM_WINDOWS);
        let mut hwnd_addr: [u8; 4] = Default::default();
        hwnd_addr.clone_from_slice(&info.info.dummy[0..4]);
        *global::HWND = std::mem::transmute(hwnd_addr);
    }

    Ok(window)
}

fn init_app() -> Result<(), String> {
    let success = unsafe { App::get().init_config_files() };
    if !success {
        return Err(String::from("Could not init config files."));
    }

    let success = unsafe { DirectoryManager::get().read_from_configuration_files() };
    if !success {
        return Err(String::from("Could not read config files."));
    }

    let success = unsafe { App::get().init_localization() };
    if !success {
        return Err(String::from("Could not init localization."));
    }

    Ok(())
}

fn process_event(
    event_subsystem: &sdl2::EventSubsystem,
    event: sdl2::event::Event,
) -> bool {
    use sdl2::keyboard::Keycode;
    use sdl2::keyboard::Mod;
    use sdl2::keyboard::Scancode;
    use sdl2::event::Event;

    match event {
        Event::MouseMotion { x, y, .. } => unsafe {
            App::get().input_manager.sys_add_mouse_move_event(x, y);
        },
        Event::MouseButtonDown {
            x, y, mouse_btn, ..
        } => unsafe {
            App::get()
                .input_manager
                .sys_add_mouse_down_event(x, y, map_mouse_btn(mouse_btn));
        },
        Event::MouseButtonUp {
            x, y, mouse_btn, ..
        } => unsafe {
            App::get()
                .input_manager
                .sys_add_mouse_up_event(x, y, map_mouse_btn(mouse_btn));
        },
        Event::MouseWheel { y, .. } => unsafe {
            App::get().input_manager.sys_add_mouse_wheel_event(0, 0, y);
        },
        Event::Window { win_event, .. } => {
            use sdl2::event::WindowEvent;
            match win_event {
                WindowEvent::Moved(..) => unsafe { App::get().window_has_moved() },
                WindowEvent::Resized(..) => unsafe { App::get().window_has_resized() },
                _ => (),
            }
        }
        Event::Quit { timestamp } => {
            let _ = event_subsystem.push_event(Event::KeyDown {
                timestamp: timestamp,
                window_id: 0,
                keycode: Some(Keycode::Escape),
                scancode: Some(Scancode::Escape),
                keymod: Mod::empty(),
                repeat: false,
            });

            let _ = event_subsystem.push_event(Event::KeyUp {
                timestamp: timestamp,
                window_id: 0,
                keycode: Some(Keycode::Escape),
                scancode: Some(Scancode::Escape),
                keymod: Mod::empty(),
                repeat: false,
            });
        }
        Event::KeyDown { keycode, .. } => unsafe {
            // TODO Break Key. (CTRL+PAUSE). Shows a dialog saying
            // "This will quit creatures without saving." On yes the game
            // quits. Otherwise no-op.

            // TODO: SHIFT+Pause key pressed. Perform debug pause, no ticking
            // events sent

            // TODO debug event SHIFT + SPACE force a tick in debug mode.

            match keycode {
                Some(k) => {
                    let vk_k = keycode_to_vk_keycode(k);
                    match vk_k {
                    Some(v) => { 
                        App::get().input_manager.sys_add_key_down_event(v);

                        // TODO: input_manager handles this all
                        // as a TextEdit event.
                        if k == Keycode::Return || k == Keycode::Backspace || k == Keycode::Escape {
                            App::get().input_manager.sys_add_translated_char_event(v);
                        }
                    },
                    None => log::debug!("Unknown key: {:?}", keycode),
                    }
                }
                None => (),
            }
        },
        Event::KeyUp { keycode: Some(Keycode::Return), keymod: Mod::LALTMOD, .. } => unsafe {
                App::get().toggle_full_screen_mode();
        },
        Event::KeyUp { keycode, .. } => unsafe {
            match keycode {
                Some(k) => {
                    App::get().input_manager.sys_add_key_up_event(k as i32);
                }
                None => (),
            }
        },
        Event::TextInput { text, .. } => unsafe {
            for c in text.chars() {
                // 2 is big enough to encode any char as u16.
                let mut char_buffer: [u16; 2] = Default::default();
                let slice = c.encode_utf16(&mut char_buffer);
                for c16 in slice {
                    App::get()
                    .input_manager
                    .sys_add_translated_char_event(*c16 as i32);
                }
            }
        },
        Event::User{ .. } => {
            let tick_event = event.as_user_event_type::<TickGameStepEvent>();
            match tick_event {
                Some(_) => return do_app_update(),
                _ => log::warn!("Uncaught user event {:?}", event),
            }
        }
        // TODO 0x402 codes.
        _ => log::debug!("Uncaptured event: {:?}", event)
    }
    true
}

fn do_app_update() -> bool {
    unsafe {
        EVENT_PENDING = false;
        let break_after_step = !App::get().terminate_triggered;
        App::get().update();
        App::get().input_manager.sys_flush_event_buffer();
        break_after_step
    }
}

#[call_engine(0x00478b80, "cdecl")]
unsafe fn do_startup(hwnd: HWND) -> bool;

#[call_engine(0x00478c50)]
unsafe fn message_destroy_called();

pub unsafe fn inject_calls() {
    use injected_calls::*;
    replace_call!(0x00477d40, trigger_game_step);
}

mod injected_calls {
    use super::*;
    // C2E:0x00477d40
    pub unsafe extern "stdcall" fn trigger_game_step() {
        if !EVENT_PENDING {
            EVENT_PENDING = true;
            let _ = (*EVENT_SUBSYSTEM).push_custom_event(TickGameStepEvent{});
        }
    }
}

