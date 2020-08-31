use callengine::call_engine;

#[repr(C, packed)]
pub struct InputManager {
  _unknown0: [u8; 4],
  pub _unknown1: usize,
  pub _unknown2: usize,
  _unknown3: usize,
  pub pending_mask: usize,
  pub mouse_x: i32,
  pub mouse_y: i32,
}

impl InputManager {
  #[call_engine(0x0055e8d0)]
  pub unsafe fn sys_add_key_down_event(&self, p1: i32, p2: i32, p3: i32);

  #[call_engine(0x0055ead0)]
  pub unsafe fn sys_add_key_up_event(&self, p1: i32, p2: i32, p3: i32);

  #[call_engine(0x0055eef0)]
  pub unsafe fn sys_add_mouse_down_event(&self, p1: i32, p2: i32, p3: usize);

  #[call_engine(0x0055f330)]
  pub unsafe fn sys_add_mouse_move_event(&self, p1: i32, p2: i32);

  #[call_engine(0x0055f110)]
  pub unsafe fn sys_add_mouse_up_event(&self, p1: i32, p2: i32, p3: i32);

  #[call_engine(0x0055f550)]
  pub unsafe fn sys_add_mouse_wheel_event(&self, p1: i32, p2: i32, p3: usize);

  #[call_engine(0x0055ece0)]
  pub unsafe fn sys_add_translated_char_event(&self, p1: i32);
}