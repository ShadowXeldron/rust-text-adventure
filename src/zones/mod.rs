#[derive(Copy, Clone)]
pub struct Zone<'a> {
    pub name: &'a str,
    pub text: &'a str,
    pub npcs: Option<&'a [NPC<'a>]>,
    pub objects: Option<&'a [&'a str]>, // Replace this with a proper item check
    pub directions: &'a Connections<'a>,
    pub encounter_rate: u8, // base encounter rate value out of 100
    // encounter_table: (a vector full of enemy definitions)
    // script: (area-specific stuff for example taking damage in a poison area)
}

impl Zone<'_> {
    pub fn talk_npc(&self, npc: &str) {
        // Iterate through NPC array
        let npc_list = self.npcs.expect("NPC list empty!");

        for counter in 0..npc_list.len() {
            if npc_list[counter].name == npc {
                println!("{}", npc_list[counter].dialogue);
                return
            }
        }

        // Otherwise, return nothing
        println!("There isn't a {} here.", npc)
    }
}

// Struct for setting up map connections
pub struct Connections<'a> {
    pub north: Option<&'a Zone<'a>>,
    pub south: Option<&'a Zone<'a>>,
    pub east: Option<&'a Zone<'a>>,
    pub west: Option<&'a Zone<'a>>,
    pub up: Option<&'a Zone<'a>>,
    pub down: Option<&'a Zone<'a>>,
}

#[derive(Copy, Clone)]
pub struct NPC<'a> {
    pub name: &'a str,
    pub dialogue: &'a str,
    // Should add an FnMut for running custom events
    //  pub event_talk
    //  pub event_attack
    // Should also include an encounter group for in case you attack the NPC
}

// The Castle

// First floor

// Throne Room
pub static CASTLE_1F_THRONE_ROOM: Zone = Zone {
    name: "Castle 1F: Throne Room",
    text: "You stand in the throne room of the royal palace. Upon the throne sits the land's portly ruler, draped in a majestic red cloak and wearing a ruby-encrusted crown of solid gold upon his head.\nA distressed-looking royal guard stands near to the king.He is wearing ornately decorated armour and carries a gleaming sword and shield.\nAt the southern end of the room is a doorway that leads into a hallway that leads to the rest of the castle.",
    encounter_rate: 0,

    // Behold, maximum dumbness
    npcs: Some(&[

        NPC {name: "king",
            dialogue: "You stand before the obese monarch of the land and he begins to talk in a somewhat unimpressed tone. \"Didn't I tell you to go and kill that giant? If you don't get a move on I- uh, WE will not ne able to have dinner!\""},

        NPC {name: "guard",
            dialogue: "The guard turns toward you and begins to speak.\n\"Please hurry up, if you don't I'll never hear the end of it. You should go to the armoury: we have some equipment in there that will be indespensible on your quest. It might also be visiting the court mage on top of the northwest tower as well, since he knows all about magic.\""},

        NPC {name: "queen", dialogue: "You see no queen in the vicinity and proceed to call the king maidenless. He retorts by stating that Elden Ring memes aren't funny anymore."}
    ]),
    objects: Some(&["torch"]),

    // Wish there was a better way to do this...
    directions: &Connections {
        north: None,
        south: Some(&CASTLE_1F_THRONE_ROOM_CORRIDOR),
        east: None,
        west: None,
        up: None,
        down: None,
    }
};

pub static CASTLE_1F_THRONE_ROOM_CORRIDOR: Zone = Zone {
    name: "Castle 1F: Throne Room Corridor",
    text: "You stand in a great hallway decorated in paintings and tapestries depicting the land's many victories. A row of shiny armoured suits lines the corridor.\nTo your north is the entrance to the castle's throne room, and to south is the southern wing of the castle.",
    encounter_rate: 255,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: Some(&CASTLE_1F_THRONE_ROOM),
        south: Some(&CASTLE_1F_SOUTH_CORRIDOR),
        east: None,
        west: None,
        up: None,
        down: None,
    }
};

// Corridors
pub static CASTLE_1F_SOUTH_CORRIDOR: Zone = Zone {
    name: "Castle 1F: South Corridor",
    text: "You are in the south wing.\nTo your north is the entrance to the castle's throne room, to your east is the southeast tower and to your west is the southwest tower.",
    encounter_rate: 20,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: Some(&CASTLE_1F_THRONE_ROOM_CORRIDOR),
        south: None,
        east: Some(&CASTLE_1F_SOUTHEAST_TOWER),
        west: Some(&CASTLE_1F_SOUTHWEST_TOWER),
        up: None,
        down: None,
    }
};

pub static CASTLE_1F_NORTH_CORRIDOR: Zone = Zone {
    name: "Castle 1F: North Corridor",
    text: "You are in the north wing.\nTo your east is the northeast tower, and to your west is the northwest tower.",
    encounter_rate: 20,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: None,
        south: None,
        east: Some(&CASTLE_1F_NORTHEAST_TOWER),
        west: Some(&CASTLE_1F_NORTHWEST_TOWER),
        up: None,
        down: None,
    }
};

pub static CASTLE_1F_EAST_CORRIDOR: Zone = Zone {
    name: "Castle 1F: East Corridor",
    text: "You are in the east wing.\nTo your north is the northeast tower, and to your south is the southeast tower.",
    encounter_rate: 20,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: Some(&CASTLE_1F_NORTHEAST_TOWER),
        south: Some(&CASTLE_1F_SOUTHEAST_TOWER),
        east: None,
        west: None,
        up: None,
        down: None,
    }
};

pub static CASTLE_1F_WEST_CORRIDOR: Zone = Zone {
    name: "Castle 1F: West Corridor",
    text: "You are in the west wing.\nTo your north is the northwest tower, and to your south is the southwest tower.",
    encounter_rate: 20,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: Some(&CASTLE_1F_NORTHWEST_TOWER),
        south: Some(&CASTLE_1F_SOUTHWEST_TOWER),
        east: None,
        west: None,
        up: None,
        down: None,
    }
};

// Towers
pub static CASTLE_1F_SOUTHEAST_TOWER: Zone = Zone {
    name: "Castle 1F: Southeast Tower",
    text: "You are in the southeast tower.\nTo your west is the south wing, and to your north is the east wing.\nYou can go up and down the stairs.",
    encounter_rate: 20,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: Some(&CASTLE_1F_EAST_CORRIDOR),
        south: None,
        east: None,
        west: Some(&CASTLE_1F_SOUTH_CORRIDOR),
        up: Some(&CASTLE_2F_SOUTHEAST_TOWER),
        down: None,
    }
};

pub static CASTLE_1F_SOUTHWEST_TOWER: Zone = Zone {
    name: "Castle 1F: Southwest Tower",
    text: "You are in the southwest tower.\nTo your east is the south wing, and to your north is the west wing.\nYou can go up and down the stairs.",
    encounter_rate: 20,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: Some(&CASTLE_1F_WEST_CORRIDOR),
        south: None,
        east: Some(&CASTLE_1F_SOUTH_CORRIDOR),
        west: None,
        up: Some(&CASTLE_2F_SOUTHWEST_TOWER),
        down: None,
    }
};

pub static CASTLE_1F_NORTHEAST_TOWER: Zone = Zone {
    name: "Castle 1F: Northeast Tower",
    text: "You are in the northeast tower.\nTo your west is the north wing, and to your south is the east wing.\nYou can go up and down the stairs.",
    encounter_rate: 20,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: None,
        south: Some(&CASTLE_1F_EAST_CORRIDOR),
        east: None,
        west: Some(&CASTLE_1F_NORTH_CORRIDOR),
        up: Some(&CASTLE_2F_NORTHEAST_TOWER),
        down: None,
    }
};

pub static CASTLE_1F_NORTHWEST_TOWER: Zone = Zone {
    name: "Castle 1F: Northwest Tower",
    text: "You are in the northwest tower.\nTo your east is the north wing, and to your south is the east wing.\nYou can go up and down the stairs.",
    encounter_rate: 20,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: None,
        south: Some(&CASTLE_1F_EAST_CORRIDOR),
        east: Some(&CASTLE_1F_NORTH_CORRIDOR),
        west: None,
        up: Some(&CASTLE_2F_SOUTHWEST_TOWER),
        down: None,
    }
};

// Castle 2F Towers
pub static CASTLE_2F_SOUTHEAST_TOWER: Zone = Zone {
    name: "Castle 2F: Southeast Tower",
    text: "You are up the southeast tower.\nA stern archer is looking out of the window with an arrow knocked to his bow.\nYou can go down the stairs.",
    encounter_rate: 0,
    npcs: Some(&[
        NPC {name: "archer",
            dialogue: "The archer beings talking but does not turn to face you.\n\"You really shouldn't barge into here and distract me like that. I've got to snipe any potentially dangerous threats that are coming towards the castle, and if I'm distracted I might not be able to do my job.\"\nSuddenly, screaming erupts from the town centre. While you can barely see past the archer's head, you faintly make out a hand carrying away what looks like a shipment of grain.\n\"See what I mean!?\""},
    ]),

    objects: None,

    directions: &Connections {
        north: None,
        south: None,
        east: None,
        west: None,
        up: None,
        down: Some(&CASTLE_1F_SOUTHEAST_TOWER),
    }
};

pub static CASTLE_2F_SOUTHWEST_TOWER: Zone = Zone {
    name: "Castle 2F: Southwest Tower",
    text: "You are up the southwest tower.\nYou can go down the stairs",
    encounter_rate: 0,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: None,
        south: None,
        east: None,
        west: None,
        up: None,
        down: Some(&CASTLE_1F_SOUTHWEST_TOWER),
    }
};

pub static CASTLE_2F_NORTHEAST_TOWER: Zone = Zone {
    name: "Castle 2F: Northeast Tower",
    text: "You are up the northeast tower.\nYou can go down the stairs.",
    encounter_rate: 0,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: None,
        south: None,
        east: None,
        west: None,
        up: None,
        down: Some(&CASTLE_1F_NORTHEAST_TOWER),
    }
};

pub static CASTLE_2F_NORTHWEST_TOWER: Zone = Zone {
    name: "Castle 2F: Northwest Tower",
    text: "You are up the northwest tower.\nYou can go up or down the stairs.",
    encounter_rate: 0,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: None,
        south: None,
        east: None,
        west: None,
        up: Some(&CASTLE_3F_NORTHWEST_TOWER),
        down: Some(&CASTLE_1F_NORTHWEST_TOWER),
    }
};

// Third floor
pub static CASTLE_3F_NORTHWEST_TOWER: Zone = Zone {
    name: "Castle 3F: Northwest Tower",
    text: "You are on top the northwest tower.\nA wizard stands over a crystal ball.\nYou can down the stairs.",
    encounter_rate: 0,
    npcs: Some(&[
        NPC {name: "wizard",
            dialogue: "The wizard stops staring into the crystal ball and turns up to face you.\n\"I don't do anything yet, leave me alone!\"\nThe wizard goes back to looking at his crystal ball."},
            /* This wizard should have an event where he has different dialogue on the first visit and gives you a free spell.

                First visit:
                "So you're that hotshot the king chose to deal with the giant, eh? You will almost certainly have to fight a lot of enemies on your quest, and many of those foes cannot be defeated with weapons alone. You should try using magic!"
                The wizard gets up and starts digging through his stuff, before he comes back with a small glowing ball.
                "This sphere contains the essence of a magic spell in a state that humans can use! As long as you have it junctioned, you will be able to use the Force Bolt spell. But using magic requires a lot of brainpower, so some of the more complex spells might be too confusing unless you're intelligent."

                Subsequent visits
                "If you've always wondered why wizards are always clever, it's because using the strongest spells requires a lot of brainpower. You'll need to be highly intelligent to use them."
            */
    ]),
    objects: None,

    directions: &Connections {
        north: None,
        south: None,
        east: None,
        west: None,
        up: None,
        down: Some(&CASTLE_2F_NORTHWEST_TOWER),
    }
};

// Ground Floor

pub static CASTLE_GF_NORTHWEST_TOWER: Zone = Zone {
    name: "Castle Ground Floor: Northwest Tower",
    text: "You are at the bottom of the northwest tower.\nYou can go up or down the stairs.",
    encounter_rate: 0,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: None,
        south: None,
        east: None,
        west: None,
        up: Some(&CASTLE_2F_NORTHWEST_TOWER),
        down: None,
    }
};


pub static CASTLE_GF_ENTRANCE: Zone = Zone {
    name: "Castle Ground Floor: Entrance Hall",
    text: "You are in the castle's lavishly-decorated entrance hallway.",
    encounter_rate: 0,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: None,
        south: None,
        east: None,
        west: None,
        up: None,
        down: None,
    }
};


// Basement

pub static CASTLE_B1F_HALLWAY: Zone = Zone {
    name: "Castle B1F: Hallway",
    text: "You are in the castle's basement hallway.",
    encounter_rate: 0,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: None,
        south: None,
        east: None,
        west: None,
        up: None,
        down: None,
    }
};

pub static CASTLE_B1F_DUNGEON: Zone = Zone {
    name: "Castle B1F: Dungeons",
    text: "You are in the castle's dungeon.",
    encounter_rate: 0,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: None,
        south: None,
        east: None,
        west: None,
        up: None,
        down: None,
    }
};

pub static CASTLE_B1F_TORTURE_CHAMBER: Zone = Zone {
    name: "Castle B1F: Torture Chamber",
    text: "You are in the castle's torture room.",
    encounter_rate: 0,
    npcs: None,
    objects: None, // Interract with the devices to torture yourself, usually for a stupid death.

    directions: &Connections {
        north: None,
        south: None,
        east: None,
        west: None,
        up: None,
        down: None,
    }
};






