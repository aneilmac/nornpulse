use std::cmp::Ordering;

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

#[repr(C, packed)]
#[derive(Eq)]
pub struct AgentHandle {
    agent: *mut Agent,
}

impl Default for AgentHandle {
    fn default() -> Self {
        AgentHandle {
            agent: std::ptr::null::<Agent>() as _,
        }
    }
}

impl Clone for AgentHandle {
    fn clone(&self) -> Self {
        AgentHandle::from_agent(self.agent)
    }
}

impl Drop for AgentHandle {
    fn drop(&mut self) {
        if !self.agent.is_null() {
            unsafe {
                if (*self.agent).handle_count > 0 {
                    (*self.agent).handle_count -= 1;
                }
            }
        }
    }
}

impl PartialEq for AgentHandle {
    fn eq(&self, other: &Self) -> bool {
        self.agent == other.agent
    }
}

impl PartialOrd for AgentHandle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = self.agent;
        let b = other.agent;
        a.partial_cmp(&b)
    }
}

impl Ord for AgentHandle {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.agent;
        let b = other.agent;
        a.cmp(&b)
    }
}

impl AgentHandle {
    pub fn from_agent(a: *mut Agent) -> Self {
        AgentHandle {
            agent: {
                if a.is_null() || unsafe { (*a).is_garbage } {
                    std::ptr::null::<Agent>() as *mut _
                } else {
                    unsafe {
                        (&mut *a).handle_count += 1;
                    }
                    a
                }
            },
        }
    }

    pub fn is_compound_agent(&self) {}
}
