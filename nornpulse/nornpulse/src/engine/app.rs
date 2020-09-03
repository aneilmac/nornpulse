use crate::utils::cpp_adapter::CppString;
use crate::engine::input_manager::InputManager;
use std::ffi::CStr;
use callengine::call_engine;

#[repr(C, packed)]
pub struct App {
    _unknown1: [u8; 184], // 0 - 183
    pub input_manager: InputManager, // 184 - 211
    pub _0xd4: usize, // 212 - 215
    pub _0xd8: usize, // 216 - 219,
    pub _0xdc: usize, // 220 - 223
    pub _0xe0: usize, // 224 - 227
    pub _0xe4: usize, // 228 - 231
    pub _0xe8: usize, // 232 - 235
    pub _0xec: usize, // 236 - 239
    _unknown2: [u8;77] // 240 - 316
}

impl App {
    pub fn process_command_line(&mut self, cmd_line: &CStr) -> bool {
        type CommandLineFn = unsafe extern "thiscall" fn(app: &App, param1: CppString) -> bool;
        const PROCESS_COMMAND_LINE: u32 = 0x005511f0;

        let process_command_line: CommandLineFn =
            unsafe { std::mem::transmute(PROCESS_COMMAND_LINE) };

        unsafe { process_command_line(self, CppString::from_c_str(cmd_line.as_ptr())) }
    }

    #[call_engine(0x0054cc50, "cdecl")]
    pub unsafe fn get() -> &'static mut App;

    #[call_engine(0x0054e000)]
    pub unsafe fn update(&mut self);

    #[call_engine(0x05578b0)]
    pub unsafe fn init_config_files(&mut self) -> bool;

    #[call_engine(0x0054f210)]
    pub unsafe fn init_localization(&mut self) -> bool;

    #[call_engine(0x0041d270)]
    pub unsafe fn get_input_manager(&self) -> *mut InputManager;

    #[call_engine(0x0054e8d0)]
    pub unsafe fn window_has_moved(&mut self);

    #[call_engine(0x0054e8e0)]
    pub unsafe fn window_has_resized(&mut self);

    #[call_engine(0x00557fa0)]
    pub unsafe fn is_full_screen(&self) -> bool;

    #[call_engine(0x0054ec50)]
    pub unsafe fn toggle_full_screen_mode(&mut self) -> bool;
}

pub unsafe fn inject_calls() {
    use injected_calls::*;
    replace_call!(0x00556030, check_for_cd);
}

mod injected_calls {
    use super::*;

    // C2E:0x00556030
    pub extern "thiscall" fn check_for_cd(_app: &App) -> bool {
        true // Where we're going, we don't need CDs!
    }
}