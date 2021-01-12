use crate::engine::camera::Camera;
use callengine::call_engine;

pub const OUR_ALREADY_DONE_SET_CURRENT_BOUNDS_THIS_TICK: *mut bool =
    unsafe { std::mem::transmute(0x0060ecba) };

#[repr(C, packed)]
pub struct MainCamera {}

impl MainCamera {
    #[call_engine(0x0057e730)]
    #[rustfmt::skip]
    pub unsafe fn get() -> &'static MainCamera;

    #[call_engine(0x0057e730)]
    #[rustfmt::skip]
    pub unsafe fn get_mut() -> &'static mut MainCamera;

    #[call_engine(0x0057e6d0)]
    #[rustfmt::skip]
    pub unsafe fn toggle_map_image(&mut self);

    #[call_engine(0x0057dcd0)]
    #[rustfmt::skip]
    pub unsafe fn enable(&mut self);

    #[call_engine(0x0054e8a7)]
    #[rustfmt::skip]
    pub unsafe fn disable(&mut self);

    #[call_engine(0x0057e6c0)]
    #[rustfmt::skip]
    pub unsafe fn enable_map_image(&mut self);

    #[call_engine(0x0057e6a0)]
    #[rustfmt::skip]
    pub unsafe fn disable_map_image(&mut self);

    #[call_engine(0x0057dfc0)]
    #[rustfmt::skip]
    pub unsafe fn render(&mut self);

    pub fn make_the_entity_handler_reset_bounds_properly(&self) {
        unsafe {
            *OUR_ALREADY_DONE_SET_CURRENT_BOUNDS_THIS_TICK = false;
        }
    }

    pub fn as_camera(&self) -> &Camera {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_camera_mut(&mut self) -> &mut Camera {
        unsafe { std::mem::transmute(self) }
    }
}
