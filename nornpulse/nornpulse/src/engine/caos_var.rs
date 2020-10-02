use callengine::call_engine;

#[repr(C, packed)]
pub struct CAOSVar {
    _unknown: [u8; 17],
}

impl CAOSVar {
    #[call_engine(0x0041a9a0)]
    #[rustfmt::skip]
    pub unsafe fn set_integer(&self, value: i32);

    #[call_engine(0x0041a770)]
    #[rustfmt::skip]
    pub unsafe fn integer(&self) -> i32;

    #[call_engine(0x0041a730)]
    #[rustfmt::skip]
    pub unsafe fn float(&self) -> f32;

    pub fn empty() -> Self {
        Self {
            _unknown: Default::default(),
        }
    }

    #[call_engine(0x0041a690)]
    #[rustfmt::skip]
    pub unsafe fn r#type(&self) -> i32;
}

impl Default for CAOSVar {
    fn default() -> Self {
        unsafe {
            let mut caos_var: CAOSVar = std::mem::zeroed();
            _constructor(&mut caos_var);
            caos_var
        }
    }
}

#[call_engine(0x0041a6a0, "thiscall")]
#[rustfmt::skip]
unsafe fn _constructor(storage: *mut CAOSVar) -> *mut CAOSVar;
