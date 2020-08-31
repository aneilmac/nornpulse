use crate::engine::app::{get_the_app, App};
use crate::engine::directory_manager::get_directory_manager;
use callengine::call_engine;
use std::ffi::CStr;
use winapi::shared::minwindef::HINSTANCE;
use winapi::shared::minwindef::LPARAM;
use winapi::shared::minwindef::LRESULT;
use winapi::shared::minwindef::UINT;
use winapi::shared::minwindef::WPARAM;
use winapi::shared::windef::HWND;

fn init_instance(h_instance: HINSTANCE) -> bool {
    let app = unsafe { get_the_app() };
    log::debug!("Init config files");
    let success = unsafe { (*app).init_config_files() };

    if !success {
        log::debug!("Init failed.");
        return false;
    }

    log::debug!("Setting up directory manager");

    let d = unsafe { get_directory_manager() };

    let success = unsafe { d.read_from_configuration_files() };
    if !success {
        log::error!("Failed to read configuration files.");
        return false;
    }

    log::debug!("Loading catalogs.");

    let success = unsafe { app.init_localization() };
    if !success {
        log::error!("Loading catalogs failed.");
        return false;
    }

    // Starting from 0x0047779f something happens here I can't fathom. Something
    // to do with testing an unhappy path of the configuration. We will ignore.

    log::debug!("Mousehweel support");

    use winapi::um::winuser;
    let engine_cstr = CStr::from_bytes_with_nul(b"engine\0").unwrap();

    unsafe {
        log::debug!("Loading icon.");

        let icon_name = CStr::from_bytes_with_nul(b"Docking Station.ico\0").unwrap();
        let icon = winuser::LoadImageA(
            std::ptr::null_mut(),
            icon_name.as_ptr(),
            winuser::IMAGE_ICON,
            0,
            0,
            winuser::LR_LOADFROMFILE,
        );

        let class_a = winuser::WNDCLASSA {
            style: winuser::CS_VREDRAW | winuser::CS_HREDRAW,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hCursor: winuser::LoadCursorA(std::ptr::null_mut(), std::ptr::null()),
            hbrBackground: winapi::um::wingdi::GetStockObject(
                winapi::um::wingdi::BLACK_BRUSH as i32,
            ) as winapi::shared::windef::HBRUSH,
            lpszClassName: engine_cstr.as_ptr(),
            lpszMenuName: std::ptr::null(),
            hIcon: icon as winapi::shared::windef::HICON,
            lpfnWndProc: Some(window_proc),
            hInstance: h_instance,
        };

        log::debug!("Registering window class");

        let atom = winuser::RegisterClassA(&class_a);
        if atom == 0 {
            log::error!("Could not register class.");
            return false;
        }

        log::debug!("Creating window");

        let hwnd = winuser::CreateWindowExA(
            winuser::WS_EX_LEFTSCROLLBAR,
            engine_cstr.as_ptr(),
            engine_cstr.as_ptr(),
            winuser::WS_CAPTION
                | winuser::WS_VISIBLE
                | winuser::WS_SYSMENU
                | winuser::WS_SIZEBOX
                | winuser::WS_TABSTOP
                | winuser::WS_GROUP,
            0,
            0,
            0x326,
            0x271,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            h_instance,
            std::ptr::null_mut(),
        );

        if hwnd.is_null() {
            log::error!("Could not generate window");
            return false;
        }

        *std::mem::transmute::<u32, *mut HWND>(0x0060ebe0) = hwnd;
    }

    true
}

pub fn startup(h_instance: HINSTANCE, cmd_line: &CStr) -> Result<i32, i32> {
    unsafe {
        *std::mem::transmute::<u32, *mut u32>(0x0060eba8) = 1;
        winapi::um::timeapi::timeBeginPeriod(1);
    }

    let app = unsafe { get_the_app() };

    let success = app.process_command_line(cmd_line);

    if !success {
        log::error!("Could not parse command-line arguments. Aborting");
        return Err(1);
    }

    let success = init_instance(h_instance);

    if !success {
        log::error!("Could not initialize app. Aborting");
        return Err(1);
    }

    log::info!("Entering main message loop.");

    let mut our_quit: bool = false;

    while !our_quit {
        use winapi::um::winuser;

        //let _ = unsafe { winuser::WaitMessage() };

        unsafe {
            let quit_action: *mut bool = std::mem::transmute(0x0060ebd8);
            if *quit_action {
                winuser::PostMessageA(
                    *std::mem::transmute::<u32, *mut HWND>(0x0060ebe0),
                    winuser::WM_USER + 1,
                    0,
                    0,
                );
                *quit_action = false;
            }
        }

        let result = update_app(winuser::WM_NULL, 0x3ff, false)
            .and_then(|()| update_app(winuser::WM_USER, winuser::WM_USER, true))
            .and_then(|()| update_app(winuser::WM_USER + 2, winuser::WM_USER + 2, false))
            .and_then(|()| update_app(winuser::WM_USER + 1, winuser::WM_USER + 1, false))
            .and_then(|()| update_app(winuser::WM_USER + 3, 0x7fff, false))
            .and_then(|()| update_app(winuser::WM_APP, 0xffffffff, false));

        if result.is_none() {
            our_quit = true;
        }

        unsafe {
            let our_terminate: *mut bool = std::mem::transmute(0x0060eb59);
            if *our_terminate {
                let s = winuser::DestroyWindow(*std::mem::transmute::<u32, *mut HWND>(0x0060ebe0));
                log::debug!("Terminate window triggered. Success: {}", s);
                our_quit = true;
            }
        }
    }

    log::debug!("Message loop has triggered.");
    unsafe {
        winapi::um::timeapi::timeEndPeriod(1);
    }
    Ok(0)
}

fn update_app(filter_min: u32, filter_max: u32, do_once: bool) -> Option<()> {
    use winapi::um::winuser;

    let mut has_messages: i32 = 0;

    while has_messages != -1
        && unsafe {
            let mut message = winuser::MSG::default();
            let b_val = winuser::PeekMessageA(
                &mut message,
                std::ptr::null_mut(),
                filter_min,
                filter_max,
                winuser::PM_NOREMOVE,
            );
            b_val != 0
        }
    {
        let mut message = winuser::MSG::default();
        unsafe {
            has_messages =
                winuser::GetMessageA(&mut message, std::ptr::null_mut(), filter_min, filter_max);
        }

        if has_messages == 0 {
            return None;
        } else {
            if has_messages != -1 {
                if message.message == winuser::WM_USER {
                    unsafe {
                        if *std::mem::transmute::<u32, *const u8>(0x0060eba8) != 0 {
                            *std::mem::transmute::<u32, *mut u8>(0x0060eb5a) = 0;
                            let bool_0060eb58: *mut bool = std::mem::transmute(0x0060eb58);
                            if false == *bool_0060eb58 {
                                *bool_0060eb58 = true;
                                let app: &mut App = get_the_app();
                                app.update();
                                do_post_update(app);
                                *bool_0060eb58 = false;
                            }
                        }
                    }
                } else {
                    unsafe {
                        winuser::TranslateMessage(&mut message);
                        winuser::DispatchMessageA(&mut message);
                    }
                }
            }
        }

        if do_once {
            break;
        }
    }
    Some(())
}

fn do_post_update(app: &mut App) {
    let i = app._0xd4 + 1;
    if 2 < i {
        app._0xd4 = 0;
    } else {
        app._0xd4 = i;
    }

    let input_manager = &mut app.input_manager;

    unsafe {
        *std::mem::transmute::<usize, *mut i32>(app._0xdc + app._0xd4 * 4) = input_manager.mouse_x;
        *std::mem::transmute::<usize, *mut i32>(app._0xec + app._0xd4 * 4) = input_manager.mouse_y;
    }

    input_manager._unknown2 = input_manager._unknown1;
    input_manager.pending_mask = 0;
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    u_msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    use winapi::um::winuser;

    let app = get_the_app();
    let input_manager = &app.input_manager;

    match u_msg {
        winuser::WM_CLOSE => {
            winuser::PostMessageA(hwnd, winuser::WM_KEYDOWN, winuser::VK_ESCAPE as usize, 0);
            winuser::PostMessageA(hwnd, winuser::WM_KEYUP, winuser::VK_ESCAPE as usize, 0);
            return 0;
        }
        winuser::WM_MOUSEMOVE => {
            input_manager.sys_add_mouse_move_event(
                winapi::shared::windowsx::GET_X_LPARAM(l_param),
                winapi::shared::windowsx::GET_Y_LPARAM(l_param)
            );
            return 0;
        }
        winuser::WM_LBUTTONDOWN => {
            input_manager.sys_add_mouse_down_event(
                winapi::shared::windowsx::GET_X_LPARAM(l_param),
                winapi::shared::windowsx::GET_Y_LPARAM(l_param),
                1,
            );
            return 0;
        }
        winuser::WM_LBUTTONUP => {
            input_manager.sys_add_mouse_up_event(
                winapi::shared::windowsx::GET_X_LPARAM(l_param),
                winapi::shared::windowsx::GET_Y_LPARAM(l_param),
                1,
            );
            return 0;
        }
        winuser::WM_RBUTTONDOWN => {
            input_manager.sys_add_mouse_down_event(
                winapi::shared::windowsx::GET_X_LPARAM(l_param),
                winapi::shared::windowsx::GET_Y_LPARAM(l_param),
                2,
            );
            return 0;
        }
        winuser::WM_RBUTTONUP => {
            input_manager.sys_add_mouse_up_event(
                winapi::shared::windowsx::GET_X_LPARAM(l_param),
                winapi::shared::windowsx::GET_Y_LPARAM(l_param),
                2,
            );
            return 0;
        }
        winuser::WM_MBUTTONDOWN => {
            input_manager.sys_add_mouse_down_event(
                winapi::shared::windowsx::GET_X_LPARAM(l_param),
                winapi::shared::windowsx::GET_Y_LPARAM(l_param),
                4,
            );
            return 0;
        }
        winuser::WM_MBUTTONUP => {
            input_manager.sys_add_mouse_up_event(
                winapi::shared::windowsx::GET_X_LPARAM(l_param),
                winapi::shared::windowsx::GET_Y_LPARAM(l_param),
                4,
            );
            return 0;
        }
        winuser::WM_MOUSEHWHEEL => {
            input_manager.sys_add_mouse_wheel_event(
                winapi::shared::windowsx::GET_X_LPARAM(l_param),
                winapi::shared::windowsx::GET_Y_LPARAM(l_param),
                w_param
            );
            return 0;
        }
        _ => return raw_window_proc(hwnd, u_msg, w_param, l_param)
    }
}

#[call_engine(0x00477d70, "system")]
unsafe fn raw_window_proc(hwnd: HWND, u_msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT;
