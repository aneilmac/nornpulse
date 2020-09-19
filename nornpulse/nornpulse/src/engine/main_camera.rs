use callengine::call_engine;

#[repr(C, packed)]
struct MainCamera {
}

impl MainCamera {
  #[call_engine(0x0057e730)]
  pub fn get() -> &'static MainCamera;

  #[call_engine(0x0057e6d0)]
  pub fn toggle_map_image(&mut self);
}