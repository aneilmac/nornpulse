#[derive(Debug)]
/// All histories begin with one of the following four events.
/// * `Conceived`
/// * `Spliced`
/// * `Engineered`
/// * `Cloned`
///
///  All other events happen during a creature's life.
pub enum EventType {
    /// A natural start to life, associated monikers are the mother's and father's.
    Conceived { mother: String, father: String },
    ///  Created using GENE CROS to crossover the two associated monikers.
    Spliced {
        first_source: String,
        second_source: String,
    },
    ///  From a human made genome with GENE LOAD, the String is the filename.
    Engineered { genome_file: String },
    /// Triggered by the BORN command, associated monikers are the parents.
    Born { mother: String, father: String },
    /// Reached the next life stage, either naturally from the ageing loci or with AGES.
    Aged,
    /// Emigrated to another world.
    Exported,
    /// Immigrated back again.
    Imported,
    ///  Triggered naturally with the death trigger locus, or by the DEAD command.
    Died,
    /// The first associated moniker is the child, and the second the father.
    BecamePregnant { child: String, father: String },
    /// First associated moniker is the child, second the mother
    Impregnated { child: String, mother: String },
    ///  First moniker is the child, second the other parent.
    ChildBorn { child: String, father: String },
    ///  An egg has been laid. Moniker is the mother.
    LaidByMother { mother: String },
    ///  An egg has been laid. Moniker is the unborn child.
    LaidAnEgg { child: String },
    ///  Picture taken. Moniker is the creature pictured.
    PhotoTaken { target: String },
    ///  Such as when importing a creature that already exists in the world and
    /// reallocating the new moniker, when TWINing or GENE CLONing;
    /// associated moniker is who we were cloned from
    Cloned { source: String },
    /// someone was cloned from you, first moniker is whom.
    CloneSource { clone: String },
    /// Exported through a worm hole with NET: EXPO
    WarpedOut,
    /// Imported through a worm hole
    WarpedIn,
    /// Custom event. First value is the event code, followed by two associated
    /// monikers (if applicable -- can be empty strings).
    ///  Start with numbers 100 and above, as events below that are reserved for
    /// the engine.
    Custom {
        event_id: u32,
        moniker_a: String,
        moniker_b: String,
    },
}

impl EventType {
    pub fn creature_present_for_event(&self) -> bool {
        match self {
            EventType::Conceived { .. } => true,
            EventType::Spliced { .. } => true,
            EventType::Engineered { .. } => true,
            EventType::Born { .. } => true,
            EventType::Aged => true,
            EventType::Exported => true,
            EventType::Imported => true,
            EventType::Died => true,
            EventType::BecamePregnant { .. } => true,
            EventType::Impregnated { .. } => true,
            EventType::Cloned { .. } => true,
            EventType::WarpedOut => true,
            EventType::WarpedIn => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct LifeEvent {
    pub world_moniker: String,
    pub world_name: String,
    pub event: EventType,
}

struct Moniker {
    generation: u16,
    species: String,
    genome: String,
}
