use super::*;

use crate::engine::agent::Agent;

pub unsafe fn inject_calls() {
    use injected_calls::*;
    // replace_call!(0x0047fe50, new_agenthandle);
    // replace_call!(0x0049c7c0, new_agenthandle_from);
    // replace_call!(0x0049c7e0, new_agenthandle_from);
    // replace_call!(0x0049c810, agenthandle_clone);
    // //replace_call!(0x0049c840, agents::agenthandle_drop);
    // replace_call!(0x0049c9b0, agenthandle_neq);
    // replace_call!(0x0049c990, agenthandle_eq);
    // replace_call!(0x0049c9d0, agenthandle_lessthan);
}

// C2E:0x0047fe50
pub extern "thiscall" fn new_agenthandle(dst: *mut AgentHandle) -> *mut AgentHandle {
    unsafe {
        std::ptr::write(dst, AgentHandle::default());
    }
    dst
}

// C2E:0x0049c7c0,0x0049c7e0
pub extern "thiscall" fn new_agenthandle_from(
    dst: *mut AgentHandle,
    agent: *mut Agent,
) -> *mut AgentHandle {
    unsafe {
        std::ptr::write(dst, AgentHandle::default());
    }
    dst
}

// C2E:0x0049c810
pub extern "thiscall" fn agenthandle_clone<'a>(
    dst: *mut AgentHandle,
    src: &AgentHandle,
) -> *mut AgentHandle {
    unsafe {
        std::ptr::write(dst, src.clone());
    }
    dst
}

// C2E:0x0049c840
pub extern "thiscall" fn agenthandle_drop(dst: *mut AgentHandle) {
    unsafe { std::ptr::drop_in_place(dst) };
}

// C2E:0x0049c9b0
pub extern "thiscall" fn agenthandle_neq(this: &AgentHandle, other: &AgentHandle) -> bool {
    this != other
}

// C2E:0x0049c990
pub extern "thiscall" fn agenthandle_eq(this: &AgentHandle, other: &AgentHandle) -> bool {
    this == other
}

// C2E:0x0049c9d0
pub extern "thiscall" fn agenthandle_lessthan(this: &AgentHandle, other: &AgentHandle) -> bool {
    this < other
}
