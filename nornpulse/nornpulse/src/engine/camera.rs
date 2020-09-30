use callengine::call_engine;

pub struct Camera {}

impl Camera {
    #[call_engine(0x0057dd60)]
  #[rustfmt::skip]
    pub unsafe fn update(&mut self, p1: bool, p2: bool);
}
