use callengine::call_engine;

#[repr(C, packed)]
pub struct MainCamera {}

impl MainCamera {
    #[call_engine(0x0057e730)]
  #[rustfmt::skip]
    pub unsafe fn get() -> &'static mut MainCamera;

  #[call_engine(0x0057e6d0)]
  #[rustfmt::skip]
    pub unsafe fn toggle_map_image(&mut self);

  #[call_engine(0x0057dcd0)]
  #[rustfmt::skip]
    pub unsafe fn enable(&mut self);
}
