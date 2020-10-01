use super::*;

pub unsafe fn inject_calls() {
    replace_call!(0x0055f7c0, get_mouse_vx);
    replace_call!(0x0055f7f0, get_mouse_vy);
    //replace_call!(0xDEADBEEF, get_event_count);
}

extern "thiscall" fn get_mouse_vx(input_manager: &InputManager) -> f32 {
    input_manager.mouse_vx()
}

extern "thiscall" fn get_mouse_vy(input_manager: &InputManager) -> f32 {
    input_manager.mouse_vy()
}

extern "thiscall" fn get_event_count(input_manager: &InputManager) -> usize {
    input_manager.events.len()
}
