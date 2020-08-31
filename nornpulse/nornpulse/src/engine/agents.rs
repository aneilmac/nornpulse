use std::mem;
use c2ers::agents as ca;

pub unsafe fn inject_calls() {
    use injected_calls::*;
    replace_call!(0x0047fe50, new_agenthandle);
    replace_call!(0x0049c7c0, new_agenthandle_from);
    replace_call!(0x0049c7e0, new_agenthandle_from);
    replace_call!(0x0049c810, agenthandle_clone);
    //replace_call!(0x0049c840, agents::agenthandle_drop);
    replace_call!(0x0049c9b0, agenthandle_neq);
    replace_call!(0x0049c990, agenthandle_eq);
    replace_call!(0x0049c9d0, agenthandle_lessthan);
}

mod injected_calls {
    use super::*;

    // C2E:0x0047fe50
    pub extern "thiscall" fn new_agenthandle(
        dst: &mut mem::MaybeUninit<ca::AgentHandle>,
    ) -> &ca::AgentHandle {
        *dst = mem::MaybeUninit::new(ca::AgentHandle::default());
        unsafe { dst.get_ref() }
    }
    // C2E:0x0049c7c0,0x0049c7e0
    pub extern "thiscall" fn new_agenthandle_from(
        dst: &mut mem::MaybeUninit<ca::AgentHandle>,
        agent: *mut ca::Agent,
    ) -> &ca::AgentHandle {
        *dst = mem::MaybeUninit::new(ca::AgentHandle::from_agent(agent));
        unsafe { dst.get_ref() }
    }

    // C2E:0x0049c810
    pub extern "thiscall" fn agenthandle_clone<'a>(
        dst: &'a mut mem::MaybeUninit<ca::AgentHandle>,
        src: &ca::AgentHandle,
    ) -> &'a ca::AgentHandle {
        *dst = mem::MaybeUninit::new(src.clone());
        unsafe { dst.get_ref() }
    }

    // C2E:0x0049c840
    pub extern "thiscall" fn agenthandle_drop(dst: *mut ca::AgentHandle) {
        unsafe { std::ptr::drop_in_place(dst) };
    }

    // C2E:0x0049c9b0
    pub extern "thiscall" fn agenthandle_neq(this: &ca::AgentHandle, other: &ca::AgentHandle) -> bool {
        this != other
    }

    // C2E:0x0049c990
    pub extern "thiscall" fn agenthandle_eq(this: &ca::AgentHandle, other: &ca::AgentHandle) -> bool {
        this == other
    }

    // C2E:0x0049c9d0
    pub extern "thiscall" fn agenthandle_lessthan(
        this: &ca::AgentHandle,
        other: &ca::AgentHandle,
    ) -> bool {
        this < other
    }
}