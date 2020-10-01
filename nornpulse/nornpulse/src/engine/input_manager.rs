mod injected_calls;

pub use injected_calls::inject_calls;

use crate::utils::cpp_adapter::CppVector;
use sdl2::keyboard::Mod;

mod mask {
    pub type MaskFlag = usize;
    pub const KEY_DOWN: MaskFlag = 0x1;
    pub const KEY_UP: MaskFlag = 0x2;
    pub const MOUSE_MOVE: MaskFlag = 0x4;
    pub const MOUSE_DOWN: MaskFlag = 0x8;
    pub const MOUSE_UP: MaskFlag = 0x10;
    pub const MOUSE_WHEEL: MaskFlag = 0x20;
    pub const TRANSLATED_CHAR: MaskFlag = 0x40;
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct InputEvent {
    mask: mask::MaskFlag,
    param_1: i32,
    param_2: i32,
    param_3: i32,
}

#[repr(C)]
#[derive(Debug)]
pub struct InputManager {
    events: CppVector<InputEvent>,
    pub pending_mask: usize,
    pub mouse_x: i32,
    pub mouse_y: i32,
    n_count: usize,
    x_history: CppVector<i32>,
    y_history: CppVector<i32>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            events: CppVector::empty(),
            pending_mask: 0,
            mouse_x: 0,
            mouse_y: 0,
            n_count: 0,
            x_history: {
                // vec![0;4]
                let mut v = CppVector::empty();
                v.push(0);
                v.push(0);
                v.push(0);
                v.push(0);
                v
            },
            y_history: {
                // vec![0;4]
                let mut v = CppVector::empty();
                v.push(0);
                v.push(0);
                v.push(0);
                v.push(0);
                v
            },
        }
    }

    pub fn fetch_text_from_clipboard() {}

    pub fn event(&self, index: usize) -> Option<InputEvent> {
        if index < 0 {
            None
        } else if index >= self.events.len() {
            None
        } else {
            Some(self.events[index])
        }
    }

    pub fn mouse_vx(&self) -> f32 {
        let vv = self.mouse_x - self.x_history[(self.n_count + 1) % 3];
        vv as f32 / 3.0
    }

    pub fn mouse_vy(&self) -> f32 {
        let vv = self.mouse_y - self.y_history[(self.n_count + 1) % 3];
        vv as f32 / 3.0
    }

    // pub fn get_pending_mask(&self) -> f32 {
    // }

    // pub fn get_translated_char_event(&self) -> f32 {
    // }

    pub fn is_mod_down(&self, modifier: Mod) -> bool {
        let v: Vec<InputEvent> = self.events.clone().into();
        v.iter()
            .find(|e| {
                e.mask == mask::KEY_DOWN
                    && Mod::from_bits_truncate(e.param_2 as u16).contains(modifier)
            })
            .is_some()
    }

    // pub fn send_text_to_clipboard() {
    // }

    // pub fn set_mouse_position() {
    // }

    // pub fn set_translated_char_target() {
    // }

    pub fn sys_add_key_down_event(&mut self, key: i32, modifier: Mod) {
        self.events.push(InputEvent {
            mask: mask::KEY_DOWN,
            param_1: key,
            param_2: modifier.bits() as i32,
            param_3: 0,
        });
        self.pending_mask |= mask::KEY_DOWN;
    }

    pub fn sys_add_key_up_event(&mut self, key: i32) {
        self.events.push(InputEvent {
            mask: mask::KEY_UP,
            param_1: key,
            param_2: 0,
            param_3: 0,
        });
        self.pending_mask |= mask::KEY_UP;
    }

    pub fn sys_add_mouse_down_event(&mut self, x: i32, y: i32, button: i32) {
        self.events.push(InputEvent {
            mask: mask::MOUSE_DOWN,
            param_1: x,
            param_2: y,
            param_3: button,
        });
        self.mouse_x = x;
        self.mouse_y = y;
        self.pending_mask |= mask::MOUSE_DOWN;
    }

    pub fn sys_add_mouse_move_event(&mut self, x: i32, y: i32) {
        self.events.push(InputEvent {
            mask: mask::MOUSE_MOVE,
            param_1: x,
            param_2: y,
            param_3: 0,
        });
        self.mouse_x = x;
        self.mouse_y = y;
        self.pending_mask |= mask::MOUSE_MOVE;
    }

    pub fn sys_add_mouse_up_event(&mut self, x: i32, y: i32, button: i32) {
        self.events.push(InputEvent {
            mask: mask::MOUSE_UP,
            param_1: x,
            param_2: y,
            param_3: button,
        });
        self.mouse_x = x;
        self.mouse_y = y;
        self.pending_mask |= mask::MOUSE_UP;
    }

    pub fn sys_add_mouse_wheel_event(&mut self, x: i32, y: i32, button: i32) {
        self.events.push(InputEvent {
            mask: mask::MOUSE_WHEEL,
            param_1: x,
            param_2: y,
            param_3: button,
        });
        self.pending_mask |= mask::MOUSE_WHEEL;
    }

    pub fn sys_add_translated_char_event(&mut self, char: i32) {
        self.events.push(InputEvent {
            mask: mask::TRANSLATED_CHAR,
            param_1: char,
            param_2: 0,
            param_3: 0,
        });
        self.pending_mask |= mask::TRANSLATED_CHAR;
    }

    pub fn sys_flush_event_buffer(&mut self) {
        self.n_count += 1;
        if 2 < self.n_count {
            self.n_count = 0;
        }

        self.x_history[self.n_count] = self.mouse_x;
        self.y_history[self.n_count] = self.mouse_y;
        self.events.clear();
        self.pending_mask = 0;
    }
}
