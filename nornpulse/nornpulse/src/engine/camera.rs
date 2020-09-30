use callengine::call_engine;

pub struct Camera {}

impl Camera {
    #[call_engine(0x0057dd60)]
    #[rustfmt::skip]
    pub unsafe fn update(&mut self, p1: bool, p2: bool);

    #[call_engine(0x0057c900)]
    #[rustfmt::skip]
    pub unsafe fn set_loading(&mut self, is_loading: bool);
}
