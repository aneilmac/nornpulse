use crate::engine::app::App;
use crate::engine::directory_manager::DirectoryManager;
use crate::windowing::icon;
use crate::windowing::input_convert::{keycode_to_vk_keycode, map_mouse_btn};
use std::sync::mpsc;
use winapi::shared::windef::HWND;

mod global {
    use super::*;
    pub const HWND: *mut HWND = unsafe { std::mem::transmute(0x0060ebe0) };
}

const WINDOW_TITLE: &str = "nornpulse";
const WINDOW_WIDTH: u32 = 0x326;
const WINDOW_HEIGHT: u32 = 0x271;

struct TickGameStepEvent {}

struct GameLoop {
    sdl_context: sdl2::Sdl,
    _window: sdl2::video::Window,
    event_subsystem: sdl2::EventSubsystem,
}

impl GameLoop {
    fn setup(cmd_line: &str) -> Result<GameLoop, String> {
        App::get_mut().process_command_line(cmd_line);
        init_configuration()?;

        log::debug!("Setting up SDL2.");
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = init_sdl2_window(&video_subsystem)?;
        sdl_context.mouse().show_cursor(false);

        // Here would go code to start up the connection to an external interface. This was shared
        // memory used for inter-process communication. Used `OpenFileMapping`/`OpenMutex`/`OpenEvent`
        // to achieve this.
        //
        // That is beyond our scope. We can address inter-process communication later using a more
        // modern standard.
        log::debug!("Skipping starting external interface.");

        let event_subsystem = sdl_context.event()?;
        event_subsystem.register_custom_event::<TickGameStepEvent>()?;

        Ok(Self {
            sdl_context: sdl_context,
            event_subsystem: event_subsystem,
            _window: window,
        })
    }

    fn execute(&self) -> Result<(), String> {
        log::debug!("Starting message loop.");

        let timer_subsystem = self.sdl_context.timer()?;
        let (rx, _timer) = make_timer_pair(&timer_subsystem);

        let mut event_pump = self.sdl_context.event_pump()?;

        // TODO, find out why this can not be called before event-pump.
        // It's affecting event_pump init in unexpected ways.
        log::debug!("Calling App init");
        App::get_mut().init()?;

        'game_loop: loop {
            for event in event_pump.poll_event() {
                self.process_event(event);
            }

            let continue_game = attempt_app_update(&rx);
            if !continue_game {
               break 'game_loop;
            }
        }

        log::debug!("Shutdown message loop.");
        unsafe { App::get_mut().shut_down() }

        Ok(())
    }

    fn process_event(&self, event: sdl2::event::Event) {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;
        use sdl2::keyboard::Mod;
        use sdl2::keyboard::Scancode;

        match event {
            Event::MouseMotion { x, y, .. } => {
                App::get_mut().input_manager.sys_add_mouse_move_event(x, y);
            }
            Event::MouseButtonDown {
                x, y, mouse_btn, ..
            } => {
                App::get_mut().input_manager.sys_add_mouse_down_event(
                    x,
                    y,
                    map_mouse_btn(mouse_btn),
                );
            }
            Event::MouseButtonUp {
                x, y, mouse_btn, ..
            } => {
                App::get_mut()
                    .input_manager
                    .sys_add_mouse_up_event(x, y, map_mouse_btn(mouse_btn));
            }
            Event::MouseWheel { y, .. } => {
                App::get_mut()
                    .input_manager
                    .sys_add_mouse_wheel_event(0, 0, y);
            }
            Event::Window { win_event, .. } => {
                use sdl2::event::WindowEvent;
                match win_event {
                    WindowEvent::Moved(..) => App::get_mut().window_has_moved_flag = true,
                    WindowEvent::Resized(..) => App::get_mut().window_has_resized_flag = true,
                    _ => (),
                }
            }
            Event::Quit { timestamp } => {
                let _ = self.event_subsystem.push_event(Event::KeyDown {
                    timestamp: timestamp,
                    window_id: 0,
                    keycode: Some(Keycode::Escape),
                    scancode: Some(Scancode::Escape),
                    keymod: Mod::empty(),
                    repeat: false,
                });

                let _ = self.event_subsystem.push_event(Event::KeyUp {
                    timestamp: timestamp,
                    window_id: 0,
                    keycode: Some(Keycode::Escape),
                    scancode: Some(Scancode::Escape),
                    keymod: Mod::empty(),
                    repeat: false,
                });
            }
            Event::KeyDown {
                keycode, keymod, ..
            } => {
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
                                App::get_mut()
                                    .input_manager
                                    .sys_add_key_down_event(v, keymod);

                                // TODO: input_manager handles this all
                                // as a TextEdit event.
                                if k == Keycode::Return
                                    || k == Keycode::Backspace
                                    || k == Keycode::Escape
                                {
                                    App::get_mut()
                                        .input_manager
                                        .sys_add_translated_char_event(v);
                                }
                            }
                            None => log::debug!("Unknown key: {:?}", keycode),
                        }
                    }
                    None => (),
                }
            }
            Event::KeyUp {
                keycode: Some(Keycode::Return),
                keymod: Mod::LALTMOD,
                ..
            } => unsafe {
                App::get_mut().toggle_full_screen_mode();
            },
            Event::KeyUp { keycode, .. } => match keycode {
                Some(k) => {
                    App::get_mut().input_manager.sys_add_key_up_event(k as i32);
                }
                None => (),
            },
            Event::TextInput { text, .. } => {
                for c in text.chars() {
                    // 2 is guaranteed big enough to encode any char as u16.
                    let mut char_buffer: [u16; 2] = Default::default();
                    let slice = c.encode_utf16(&mut char_buffer);
                    for c16 in slice {
                        App::get_mut()
                            .input_manager
                            .sys_add_translated_char_event(*c16 as i32);
                    }
                }
            }
            _ => log::debug!("Uncaptured event: {:?}", event),
        }
    }
}

pub fn startup(cmd_line: &str) -> Result<(), String> {
    let game_loop = GameLoop::setup(cmd_line)?;
    game_loop.execute()
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

    unsafe {
        // TODO: Remove. We don't want a global HWND variable.
        let mut info = std::mem::zeroed();
        sdl2_sys::SDL_GetWindowWMInfo(window.raw(), &mut info);
        assert_eq!(info.subsystem, sdl2_sys::SDL_SYSWM_TYPE::SDL_SYSWM_WINDOWS);
        let mut hwnd_addr: [u8; 4] = Default::default();
        hwnd_addr.clone_from_slice(&info.info.dummy[0..4]);
        *global::HWND = std::mem::transmute(hwnd_addr);
    }

    Ok(window)
}

fn init_configuration() -> Result<(), String> {
    if let Err(e) = App::get_mut().init_config_files() {
        return Err(e.to_string());
    }

    let success = unsafe { DirectoryManager::get_mut().read_from_configuration_files() };
    if !success {
        return Err(String::from("Could not read config files."));
    }

    let success = unsafe { App::get_mut().init_localization() };
    if !success {
        return Err(String::from("Could not init localization."));
    }

    Ok(())
}

fn get_ticks() -> u32 {
    let app = App::get();
    if app.fastest_ticks {
        1
    } else {
        App::world_tick_interval()
    }
}

fn make_timer_pair<'a>(
    timer_subsystem: &'a sdl2::TimerSubsystem,
) -> (mpsc::Receiver<()>, sdl2::timer::Timer<'a, 'a>) {
    let (tx, rx) = mpsc::sync_channel::<()>(0);
    let timer = timer_subsystem.add_timer(
        5,
        Box::new(move || {
            // Desired behaviour is that send blocks until the reciever accepts the current
            // send, so multiple ticks can't trigger.
            let res = tx.send(());
            if res.is_ok() {
                get_ticks()
            } else {
                0
            }
        }),
    );
    (rx, timer)
}

fn attempt_app_update(rx: &mpsc::Receiver<()>) -> bool {
    use mpsc::TryRecvError;
    match rx.try_recv() {
        Ok(()) => do_app_update(),
        Err(TryRecvError::Empty) => true,
        Err(TryRecvError::Disconnected) => false,
    }
}

fn do_app_update() -> bool {
    let break_after_step = !App::get().terminate_triggered;
    App::get_mut().update();
    App::get_mut().input_manager.sys_flush_event_buffer();
    break_after_step
}
