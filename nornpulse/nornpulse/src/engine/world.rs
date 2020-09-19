use callengine::call_engine;

#[repr(C, packed)]
struct World {
}

impl World {
  #[call_engine(0x005200b0)]
  pub fn get_previous_meta_room(&mut self);

  #[call_engine(0x00520040)]
  pub fn get_next_meta_room(&mut self);

}