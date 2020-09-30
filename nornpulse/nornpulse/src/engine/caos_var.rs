use callengine::call_engine;

#[repr(C, packed)]
pub struct CAOSVar {
    _unknown: [u8; 30],
}

impl CAOSVar {
    #[call_engine(0x0041a9a0)]
    #[rustfmt::skip]
    pub unsafe fn set_integer(&self, value: i32);

    #[call_engine(0x0041a770)]
    #[rustfmt::skip]
    pub unsafe fn integer(&self) -> i32;

    pub fn empty() -> Self {
        Self {
            _unknown: Default::default(),
        }
    }
}
