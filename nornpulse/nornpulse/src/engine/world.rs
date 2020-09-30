use callengine::call_engine;

#[repr(C, packed)]
pub struct World {}

impl World {
    #[call_engine(0x005200b0)]
  #[rustfmt::skip]
    pub unsafe fn get_previous_meta_room(&mut self);

  #[call_engine(0x00520040)]
  #[rustfmt::skip]
    pub unsafe fn get_next_meta_room(&mut self);

  #[call_engine(0x00521ad0)]
  #[rustfmt::skip]
    pub unsafe fn save(&mut self) -> bool;

  #[call_engine(0x0051f630)]
  #[rustfmt::skip]
    pub unsafe fn task_switcher(&mut self);
}
