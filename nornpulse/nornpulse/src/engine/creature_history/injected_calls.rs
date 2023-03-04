use std::ptr::read_unaligned;

use super::*;
use crate::utils::cpp_adapter::CppString;

#[repr(C, packed)]
pub struct FFILifeEvent {
    event_type: u32,
    _unknown: [u8; 16],
    moniker_a: CppString,
    moniker_b: CppString,
    string_3: CppString,
    photo_target: CppString,
    world_name: CppString,
    world_moniker: CppString,
    string_7: CppString,
    _string_buffer: [u8; 72], // TODO: Create type + destructor.
}

fn to_life_event(le: &FFILifeEvent) -> LifeEvent {
    let e: EventType = match le.event_type {
        0x00 => EventType::Conceived {
            mother: {
                let unaligned = std::ptr::addr_of!(le.moniker_a);
                unsafe { read_unaligned(unaligned).to_string() }
            },
            father: {
                let unaligned = std::ptr::addr_of!(le.moniker_b);
                unsafe { read_unaligned(unaligned).to_string() }
            }
        },
        0x01 => EventType::Spliced {
            first_source: {
                let unaligned = std::ptr::addr_of!(le.moniker_a);
                unsafe { read_unaligned(unaligned).to_string() }
            },
            second_source: {
                let unaligned = std::ptr::addr_of!(le.moniker_b);
                unsafe { read_unaligned(unaligned).to_string() }
            }
        },
        0x02 => EventType::Engineered {
            genome_file: {
                let unaligned = std::ptr::addr_of!(le.moniker_b);
                unsafe { read_unaligned(unaligned).to_string() }
            }
        },
        0x03 => EventType::Born {
            mother: {
                let unaligned = std::ptr::addr_of!(le.moniker_a);
                unsafe { read_unaligned(unaligned).to_string() }
            },
            father: {
                let unaligned = std::ptr::addr_of!(le.moniker_b);
                unsafe { read_unaligned(unaligned).to_string() }
            }
        },
        0x04 => EventType::Aged,
        0x05 => EventType::Exported,
        0x06 => EventType::Imported,
        0x07 => EventType::Died,
        0x08 => EventType::BecamePregnant {
            child: {
                let unaligned = std::ptr::addr_of!(le.moniker_a);
                unsafe { read_unaligned(unaligned).to_string() }
            },
            father: {
                let unaligned = std::ptr::addr_of!(le.moniker_b);
                unsafe { read_unaligned(unaligned).to_string() }
            }
        },
        0x09 => EventType::Impregnated {
            child: {
                let unaligned = std::ptr::addr_of!(le.moniker_a);
                unsafe { read_unaligned(unaligned).to_string() }
            },
            mother: {
                let unaligned = std::ptr::addr_of!(le.moniker_b);
                unsafe { read_unaligned(unaligned).to_string() }
            }
        },
        0x0A => EventType::ChildBorn {
            child: {
                let unaligned = std::ptr::addr_of!(le.moniker_a);
                unsafe { read_unaligned(unaligned).to_string() }
            },
            father: {
                let unaligned = std::ptr::addr_of!(le.moniker_b);
                unsafe { read_unaligned(unaligned).to_string() }
            }
        },
        0x0B => EventType::LaidByMother {
            mother: {
                let unaligned = std::ptr::addr_of!(le.moniker_a);
                unsafe { read_unaligned(unaligned).to_string() }
            }
        },
        0x0C => EventType::LaidAnEgg {
            child: {
                let unaligned = std::ptr::addr_of!(le.moniker_a);
                unsafe { read_unaligned(unaligned).to_string() }
            }
        },
        0x0D => EventType::PhotoTaken {
            target: {
                let unaligned = std::ptr::addr_of!(le.photo_target);
                unsafe { read_unaligned(unaligned).to_string() }
            }
        },
        0x0E => EventType::Cloned {
            source: {
                let unaligned = std::ptr::addr_of!(le.moniker_a);
                unsafe { read_unaligned(unaligned).to_string() }
            }
        },
        0x0F => EventType::CloneSource {
            clone: {
                let unaligned = std::ptr::addr_of!(le.moniker_a);
                unsafe { read_unaligned(unaligned).to_string() }
            }
        },
        0x10 => EventType::WarpedOut,
        0x11 => EventType::WarpedIn,
        _ => EventType::Custom {
            event_id: le.event_type,
            moniker_a: {
                let unaligned = std::ptr::addr_of!(le.moniker_a);
                unsafe { read_unaligned(unaligned).to_string() }
            },
            moniker_b: {
                let unaligned = std::ptr::addr_of!(le.moniker_b);
                unsafe { read_unaligned(unaligned).to_string() }
            }
        },
    };
    LifeEvent {
        world_moniker: {
            let unaligned = std::ptr::addr_of!(le.world_moniker);
            unsafe { read_unaligned(unaligned).to_string() }
        },
        world_name: {
            let unaligned = std::ptr::addr_of!(le.world_name);
            unsafe { read_unaligned(unaligned).to_string() }
        },
        event: e,
    }
}

impl std::fmt::Debug for FFILifeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let et = self.event_type;
        write!(f, "LifeEvent {}", et)
        // write!(
        //     f,
        //     "LifeEvent {{ type: \"{}\",
        //    |  female_parent: {},
        //    |  male_parent: {},
        //    |  3: {},
        //    |  photo_target: {},
        //    |  world_name: {}, 
        //    |  world_moniker: {}, 
        //    |  7: {}, 
        //    }}",
        //     et,
        //     self.moniker_a,
        //     self.moniker_b,
        //     self.string_3,
        //     self.photo_target,
        //     self.world_name,
        //     self.world_moniker,
        //     self.string_7
        // )
    }
}

pub unsafe fn inject_calls() {
    // Creature History
    //replace_call!(0x0044f990, creature_present_for_event);
    //replace_call!(0x0041ad80, creature_history::copy_test);
    //replace_call!(0x0041b0c0, lifeevent_drop);
}

// C2E:0x0044f990
pub extern "thiscall" fn creature_present_for_event(this: &FFILifeEvent) -> bool {
    let life_event = to_life_event(&this);
    life_event.event.creature_present_for_event()
}

// C2E:0x0041ad80
pub extern "thiscall" fn lifeevent_drop(this: *mut FFILifeEvent) {
    log::debug!("FFI PTR: {:?}", this);
    unsafe { std::ptr::drop_in_place(this) };
}
