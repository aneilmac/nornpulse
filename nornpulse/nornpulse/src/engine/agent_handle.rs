mod injected_calls;

pub use injected_calls::inject_calls;

use crate::engine::agent::Agent;

use std::cmp::Ordering;

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
