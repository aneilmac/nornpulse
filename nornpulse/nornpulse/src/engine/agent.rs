#[repr(C, packed)]
pub struct Agent {
    pub _unknown0: [u8; 2940], // 0 - 2939
    pub is_garbage: bool,      //2940
    pub _unknown1: [u8; 263],  // 2941 - 3203
    pub handle_count: u32,     //3204 - 3207
    pub _agent_type_mask: u8,  //3208
}

enum AgentTypeMask {
    Agent = 0x01,
    SimpleAgent = 0x02,
    PointerAgent = 0x04,
    CompoundAgent = 0x08,
    Vehicle = 0x10,
    Creature = 0x20,
    SkeletalCreature = 0x40,
}
