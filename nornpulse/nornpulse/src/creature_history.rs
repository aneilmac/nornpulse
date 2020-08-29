use crate::cpp_adapter::CppString;
use c2ers::creature_history::{EventType, LifeEvent};

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
            mother: le.moniker_a.to_string(),
            father: le.moniker_b.to_string(),
        },
        0x01 => EventType::Spliced {
            first_source: le.moniker_a.to_string(),
            second_source: le.moniker_b.to_string(),
        },
        0x02 => EventType::Engineered {
            genome_file: le.moniker_b.to_string(),
        },
        0x03 => EventType::Born {
            mother: le.moniker_a.to_string(),
            father: le.moniker_b.to_string(),
        },
        0x04 => EventType::Aged,
        0x05 => EventType::Exported,
        0x06 => EventType::Imported,
        0x07 => EventType::Died,
        0x08 => EventType::BecamePregnant {
            child: le.moniker_a.to_string(),
            father: le.moniker_b.to_string(),
        },
        0x09 => EventType::Impregnated {
            child: le.moniker_a.to_string(),
            mother: le.moniker_b.to_string(),
        },
        0x0A => EventType::ChildBorn {
            child: le.moniker_a.to_string(),
            father: le.moniker_b.to_string(),
        },
        0x0B => EventType::LaidByMother {
            mother: le.moniker_a.to_string(),
        },
        0x0C => EventType::LaidAnEgg {
            child: le.moniker_a.to_string(),
        },
        0x0D => EventType::PhotoTaken {
            target: le.photo_target.to_string(),
        },
        0x0E => EventType::Cloned {
            source: le.moniker_a.to_string(),
        },
        0x0F => EventType::CloneSource {
            clone: le.moniker_a.to_string(),
        },
        0x10 => EventType::WarpedOut,
        0x11 => EventType::WarpedIn,
        _ => EventType::Custom {
            event_id: le.event_type,
            moniker_a: le.moniker_a.to_string(),
            moniker_b: le.moniker_b.to_string(),
        },
    };
    LifeEvent {
        world_moniker: le.world_moniker.to_string(),
        world_name: le.world_name.to_string(),
        event: e,
    }
}

impl std::fmt::Debug for FFILifeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let et = self.event_type;
        write!(
            f,
            "LifeEvent {{ type: \"{}\",
           |  female_parent: {},
           |  male_parent: {},
           |  3: {},
           |  photo_target: {},
           |  world_name: {}, 
           |  world_moniker: {}, 
           |  7: {}, 
           }}",
            et,
            self.moniker_a,
            self.moniker_b,
            self.string_3,
            self.photo_target,
            self.world_name,
            self.world_moniker,
            self.string_7
        )
    }
}


pub unsafe fn inject_calls() {
    use injected_calls::*;
    // Creature History
    replace_call!(0x0044f990, creature_present_for_event);
    //replace_call!(0x0041ad80, creature_history::copy_test);
    //replace_call!(0x0041b0c0, lifeevent_drop);
}

mod injected_calls {
    // C2E:0x0044f990
    pub extern "thiscall" fn creature_present_for_event(this: &super::FFILifeEvent) -> bool {
        let life_event = super::to_life_event(&this);
        life_event.event.creature_present_for_event()
    }

    // C2E:0x0041ad80
    pub extern "thiscall" fn lifeevent_drop(this: *mut super::FFILifeEvent) {
        log::debug!("FFI PTR: {:?}", this);
        unsafe { std::ptr::drop_in_place(this) };
    }
}