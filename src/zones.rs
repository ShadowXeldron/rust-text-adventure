pub struct Zone<'a> {
    name: &'a str,
    text: &'a str,
    npcs: Option<Vec<&'a str>>,
    objects: Option<Vec<&'a str>>, // Replace this with a proper item check
    directions: Box<Connections<'a>>,
    // encounter_rate: u8 // base encounter rate value out of 100
    // encounter_table: (a vector full of enemy definitions)
    // script: (area-specific stuff for example taking damage in a poison area)
}

// Struct for setting up map connections
pub struct Connections<'a> {
    north: Option<Zone<'a>>,
    south: Option<Zone<'a>>,
    east: Option<Zone<'a>>,
    west: Option<Zone<'a>>,
    up: Option<Zone<'a>>,
    down: Option<Zone<'a>>,
}

pub const castle_1f_throne_room: Zone = Zone {
    name: "Castle 1F: Throne Room",
    text: "You stand in the throne room of the royal palace. Upon the throne sits the land's portly ruler, draped in a majestic red cloak and wearing a ruby-encrusted crown of solid gold upon his head.\nA distressed-looking royal guard stands near to the king.He is wearing ornately decorated armour and carries a gleaming sword and shield.\nAt the southern end of the room is a doorway that leads into a hallway that heads to the rest of the castle.",
    npcs: vec!["king", "guard", "queen"].into(),

    directions: Connections {
        south: castle_1f_throne_room_corridor.into()
    }.into()
};

pub const castle_1f_throne_room_corridor: Zone = Zone {
    name: "Castle 1F: Throne Room Corridor",
    text: "You stand in a great hallway decorated in paintings and tapestries depicting the land's many victories. A row of shiny armoured suits lines the corridor.\nTo your north is the entrance to the castle's throne room, and to south is the southern wing of the castle.",

    directions: Connections {
        north: castle_1f_throne_room.into()
    },
}