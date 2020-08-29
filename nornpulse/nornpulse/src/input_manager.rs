use callengine::call_engine;

#[repr(C, packed)]
pub struct InputManager {}


impl InputManager {

  #[call_engine(0x0055eef0)]
  pub unsafe fn sys_add_mouse_down_event(&self, p1: i32, p2: i32, p3: i32);

  #[call_engine(0x0055f330)]
  pub unsafe fn sys_add_mouse_move_event(&self, p1: i32, p2: i32);

  #[call_engine(0x0055f110)]
  pub unsafe fn sys_add_mouse_up_event(&self, p1: i32, p2: i32, p3: i32);

  #[call_engine(0x0055f550)]
  pub unsafe fn sys_add_mouse_wheel_event(&self, p1: i32, p2: i32, p3: i32);
}