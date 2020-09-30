use crate::utils::cpp_adapter::CppVector;
use callengine::{call_engine, CheckStructAlign};

/*
 TODO: Add vector + mouse history array (size 4) + InputEvent struct
                  + flush
*/

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct InputEvent {
    mask: usize,
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
            x_history: CppVector::empty(),
            y_history: CppVector::empty(),
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

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    // pub fn get_mouse_vx(&self) -> f32 {
    // }

    // pub fn get_mouse_vy(&self) -> f32 {
    // }

    // pub fn get_pending_mask(&self) -> f32 {
    // }

    // pub fn get_translated_char_event(&self) -> f32 {
    // }

    // pub fn is_key_down() -> bool {
    // }

    // pub fn send_text_to_clipboard() {
    // }

    // pub fn set_mouse_position() {
    // }

    // pub fn set_translated_char_target() {
    // }

    pub fn sys_add_key_down_event(&mut self, key: i32) {
        self.events.push(InputEvent {
            mask: 1,
            param_1: key,
            param_2: 0,
            param_3: 0,
        });
        self.pending_mask |= 1;
    }

    pub fn sys_add_key_up_event(&mut self, key: i32) {
        self.events.push(InputEvent {
            mask: 2,
            param_1: key,
            param_2: 0,
            param_3: 0,
        });
        self.pending_mask |= 2;
    }

    pub fn sys_add_mouse_down_event(&mut self, x: i32, y: i32, button: i32) {
        self.events.push(InputEvent {
            mask: 8,
            param_1: x,
            param_2: y,
            param_3: button,
        });
        self.mouse_x = x;
        self.mouse_y = y;
        self.pending_mask |= 8;
    }

    pub fn sys_add_mouse_move_event(&mut self, x: i32, y: i32) {
        self.events.push(InputEvent {
            mask: 4,
            param_1: x,
            param_2: y,
            param_3: 0,
        });
        self.mouse_x = x;
        self.mouse_y = y;
        self.pending_mask |= 4;
    }

    pub fn sys_add_mouse_up_event(&mut self, x: i32, y: i32, button: i32) {
        self.events.push(InputEvent {
            mask: 0x10,
            param_1: x,
            param_2: y,
            param_3: button,
        });
        self.mouse_x = x;
        self.mouse_y = y;
        self.pending_mask |= 0x10;
    }

    pub fn sys_add_mouse_wheel_event(&mut self, x: i32, y: i32, button: i32) {
        self.events.push(InputEvent {
            mask: 0x20,
            param_1: x,
            param_2: y,
            param_3: button,
        });
        self.pending_mask |= 0x20;
    }

    pub fn sys_add_translated_char_event(&mut self, char: i32) {
        self.events.push(InputEvent {
            mask: 0x40,
            param_1: char,
            param_2: 0,
            param_3: 0,
        });
        self.pending_mask |= 0x40;
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
