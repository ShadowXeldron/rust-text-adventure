// Zone file
// Due to necessary recursive borrowing, the zones need to be static which means they'll be floating around in memory throughout the program's runtime. I don't like that.

use crate::Mob;
mod encounters;
use crate::zones::encounters::*;
pub mod shop;
use crate::zones::shop::*;

#[derive(Copy, Clone)]
pub struct Zone<'a> {
	pub name: &'a str,
	pub text: &'a str,
	pub npcs: Option<&'a [NPC<'a>]>,
	pub objects: Option<&'a [&'a str]>, // Replace this with a proper item check
	pub directions: &'a Connections<'a>,
	pub encounter_rate: u8, // base encounter rate value out of 100
	pub random_encounters: Option<&'a [NPC<'a>]>,
	// script: (area-specific stuff for example taking damage in a poison area)
}

impl Zone<'_> {
	pub fn talk_npc(&self, npc: &str) {
		// Iterate through NPC array
		let npc_list: &[NPC<'_>] = self.npcs.expect("NPC list empty!");

		for counter in npc_list {
			if counter.name == npc {
				println!("{}", counter.dialogue);
				return;
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
	pub name: &'a str,     // Enemy name used in the parser
	pub dialogue: &'a str, // Text that always shows when you talk to the NPC
	pub fight_table: Option<&'a [Mob<'a>]>, // Table of random encounters, used for the attack command or NPCs spawned randomly
	                                        // Should add an FnMut for running custom events
	                                        //  pub event_talk
	                                        //  pub event_attack
}

impl<'a> NPC<'a> {
	pub fn get_exp_from_encounters(&self) -> u16 {
		let mut exp_drop: u16 = 0;
		// Iterate through table and reward EXP for all foes (to account for when support for enemy formations is properly added)
		for counter in self.fight_table.unwrap() {
			exp_drop += counter.exp_reward
		}

		exp_drop
	}
}

// The Castle

// First floor

// Throne Room
pub static CASTLE_1F_THRONE_ROOM: Zone = Zone {
    name: "Castle 1F: Throne Room",
    text: "You stand in the throne room of the royal palace. Upon the throne sits the land's portly ruler, draped in a majestic red cloak and wearing a ruby-encrusted crown of solid gold upon his head.\nA distressed-looking royal guard stands near to the king. He is wearing ornately decorated armour and carries a gleaming sword and shield.\nAt the southern end of the room is a doorway that leads into a hallway that leads to the rest of the castle.",
    encounter_rate: 0,

    // Behold, maximum dumbness
    npcs: Some(&[

        NPC {name: "king",
            dialogue: "You stand before the obese monarch of the land and he begins to talk in a somewhat unimpressed tone. \"Didn't I tell you to go and kill that giant? If you don't get a move on I- uh, WE will not ne able to have dinner!\"",
            fight_table: None},

        NPC {name: "guard",
            dialogue: "The guard turns toward you and begins to speak.\n\"Please hurry up, if you don't I'll never hear the end of it. You should go to the armoury: we have some equipment in there that will be indespensible on your quest. It might also be visiting the court mage on top of the northwest tower as well, since he knows all about magic.\"", fight_table: None},

        NPC {name: "queen", dialogue: "You see no queen in the vicinity and proceed to call the king maidenless. He retorts by stating that Elden Ring memes aren't funny anymore.", fight_table: None}
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
    },

    random_encounters: None
};

pub static CASTLE_1F_THRONE_ROOM_CORRIDOR: Zone = Zone {
    name: "Castle 1F: Throne Room Corridor",
    text: "You stand in a great hallway decorated in paintings and tapestries depicting the land's many victories. A row of shiny armoured suits lines the corridor.\nTo your north is the entrance to the castle's throne room, and to south is the southern wing of the castle.",
    encounter_rate: 0, // Temporary feature for battle testing
    npcs: None,
    objects: None,

    directions: &Connections {
        north: Some(&CASTLE_1F_THRONE_ROOM),
        south: Some(&CASTLE_1F_SOUTH_CORRIDOR),
        east: None,
        west: None,
        up: None,
        down: None,
    },

    random_encounters: None
};

// Corridors
pub static CASTLE_1F_SOUTH_CORRIDOR: Zone = Zone {
    name: "Castle 1F: South Corridor",
    text: "You are in the south wing.\nTo your north is the entrance to the castle's throne room, to your east is the southeast tower and to your west is the southwest tower.",
    encounter_rate: 0,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: Some(&CASTLE_1F_THRONE_ROOM_CORRIDOR),
        south: None,
        east: Some(&CASTLE_1F_SOUTHEAST_TOWER),
        west: Some(&CASTLE_1F_SOUTHWEST_TOWER),
        up: None,
        down: None,
    },

    random_encounters: None
};

pub static CASTLE_1F_NORTH_CORRIDOR: Zone = Zone {
    name: "Castle 1F: North Corridor",
    text: "You are in the north wing.\nTo your east is the northeast tower, and to your west is the northwest tower.",
    encounter_rate: 0,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: None,
        south: None,
        east: Some(&CASTLE_1F_NORTHEAST_TOWER),
        west: Some(&CASTLE_1F_NORTHWEST_TOWER),
        up: None,
        down: None,
    },

    random_encounters: None
};

pub static CASTLE_1F_EAST_CORRIDOR: Zone = Zone {
    name: "Castle 1F: East Corridor",
    text: "You are in the east wing.\nTo your north is the northeast tower, and to your south is the southeast tower.",
    encounter_rate: 0,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: Some(&CASTLE_1F_NORTHEAST_TOWER),
        south: Some(&CASTLE_1F_SOUTHEAST_TOWER),
        east: None,
        west: None,
        up: None,
        down: None,
    },

    random_encounters: None
};

pub static CASTLE_1F_WEST_CORRIDOR: Zone = Zone {
    name: "Castle 1F: West Corridor",
    text: "You are in the west wing.\nTo your north is the northwest tower, and to your south is the southwest tower.",
    encounter_rate: 0,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: Some(&CASTLE_1F_NORTHWEST_TOWER),
        south: Some(&CASTLE_1F_SOUTHWEST_TOWER),
        east: None,
        west: None,
        up: None,
        down: None,
    },

    random_encounters: None
};

// Towers
pub static CASTLE_1F_SOUTHEAST_TOWER: Zone = Zone {
    name: "Castle 1F: Southeast Tower",
    text: "You are in the southeast tower.\nTo your west is the south wing, and to your north is the east wing.\nYou can go up and down the stairs.",
    encounter_rate: 0,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: Some(&CASTLE_1F_EAST_CORRIDOR),
        south: None,
        east: None,
        west: Some(&CASTLE_1F_SOUTH_CORRIDOR),
        up: Some(&CASTLE_2F_SOUTHEAST_TOWER),
        down: Some(&CASTLE_GF_SOUTHEAST_TOWER),
    },

    random_encounters: None
};

pub static CASTLE_1F_SOUTHWEST_TOWER: Zone = Zone {
    name: "Castle 1F: Southwest Tower",
    text: "You are in the southwest tower.\nTo your east is the south wing, and to your north is the west wing.\nYou can go up and down the stairs.",
    encounter_rate: 0,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: Some(&CASTLE_1F_WEST_CORRIDOR),
        south: None,
        east: Some(&CASTLE_1F_SOUTH_CORRIDOR),
        west: None,
        up: Some(&CASTLE_2F_SOUTHWEST_TOWER),
        down: Some(&CASTLE_GF_SOUTHWEST_TOWER),
    },

    random_encounters: None
};

pub static CASTLE_1F_NORTHEAST_TOWER: Zone = Zone {
    name: "Castle 1F: Northeast Tower",
    text: "You are in the northeast tower.\nTo your west is the north wing, and to your south is the east wing.\nYou can go up and down the stairs.",
    encounter_rate: 0,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: None,
        south: Some(&CASTLE_1F_EAST_CORRIDOR),
        east: None,
        west: Some(&CASTLE_1F_NORTH_CORRIDOR),
        up: Some(&CASTLE_2F_NORTHEAST_TOWER),
        down: Some(&CASTLE_GF_NORTHWEST_TOWER),
    },

    random_encounters: None
};

pub static CASTLE_1F_NORTHWEST_TOWER: Zone = Zone {
    name: "Castle 1F: Northwest Tower",
    text: "You are in the northwest tower.\nTo your east is the north wing, and to your south is the east wing.\nYou can go up and down the stairs.",
    encounter_rate: 0,
    npcs: None,
    objects: None,

    directions: &Connections {
        north: None,
        south: Some(&CASTLE_1F_EAST_CORRIDOR),
        east: Some(&CASTLE_1F_NORTH_CORRIDOR),
        west: None,
        up: Some(&CASTLE_2F_NORTHWEST_TOWER),
        down: Some(&CASTLE_GF_NORTHWEST_TOWER),
    },

    random_encounters: None
};

// Castle 2F Towers
pub static CASTLE_2F_SOUTHEAST_TOWER: Zone = Zone {
    name: "Castle 2F: Southeast Tower",
    text: "You are up the southeast tower.\nA stern archer is looking out of the window with an arrow knocked to his bow.\nYou can go down the stairs.",
    encounter_rate: 0,
    npcs: Some(&[
        NPC {name: "archer",
            dialogue: "The archer beings talking but does not turn to face you.\n\"You really shouldn't barge into here and distract me like that. I've got to snipe any potentially dangerous threats that are coming towards the castle, and if I'm distracted I might not be able to do my job.\"\nSuddenly, screaming erupts from the town centre. While you can barely see past the archer's head, you faintly make out a hand carrying away what looks like a shipment of grain.\n\"See what I mean!?\"", fight_table: None},
    ]),

    objects: None,

    directions: &Connections {
        north: None,
        south: None,
        east: None,
        west: None,
        up: None,
        down: Some(&CASTLE_1F_SOUTHEAST_TOWER),
    },

    random_encounters: None
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
	},

	random_encounters: None,
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
	},

	random_encounters: None,
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
	},

	random_encounters: None,
};

// Third floor
pub static CASTLE_3F_NORTHWEST_TOWER: Zone = Zone {
    name: "Castle 3F: Northwest Tower",
    text: "You are on top the northwest tower.\nA wizard stands over a crystal ball.\nYou can down the stairs.",
    encounter_rate: 0,
    npcs: Some(&[
        NPC {name: "wizard",
            dialogue: "The wizard stops staring into the crystal ball and turns up to face you.\n\"I don't do anything yet, leave me alone!\"\nThe wizard goes back to looking at his crystal ball.", fight_table: None}
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
    },

    random_encounters: None
};

// Ground Floor

// Ground Floor Towers
pub static CASTLE_GF_NORTHWEST_TOWER: Zone = Zone {
	name: "Castle Ground Floor: Northwest Tower",
	text: "You are at the bottom of the northwest tower.\nTo your south is the wext corridor and ti your west is the north corridor.\nYou can go up the stairs.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: None,
		south: Some(&CASTLE_GF_WEST_CORRIDOR),
		east: None,
		west: Some(&CASTLE_GF_NORTH_CORRIDOR),
		up: Some(&CASTLE_1F_NORTHWEST_TOWER),
		down: None,
	},

	random_encounters: None,
};

pub static CASTLE_GF_NORTHEAST_TOWER: Zone = Zone {
	name: "Castle Ground Floor: Northeast Tower",
	text: "You are at the bottom of the northeast tower.\nTo your south is the east corridor and to your west is the north corridor\nYou can go up the stairs.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: None,
		south: Some(&CASTLE_GF_EAST_CORRIDOR),
		east: Some(&CASTLE_1F_NORTH_CORRIDOR),
		west: None,
		up: Some(&CASTLE_1F_NORTHEAST_TOWER),
		down: None,
	},

	random_encounters: None,
};

pub static CASTLE_GF_SOUTHWEST_TOWER: Zone = Zone {
	name: "Castle Ground Floor: Southwest Tower",
	text: "You are at the bottom of the southwest tower.\nTo your east is the entry hall and to your north is the west corridor.\nYou can go up the stairs.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_GF_WEST_CORRIDOR),
		south: None,
		east: Some(&CASTLE_GF_ENTRANCE),
		west: None,
		up: Some(&CASTLE_1F_SOUTHWEST_TOWER),
		down: None,
	},

	random_encounters: None,
};

pub static CASTLE_GF_SOUTHEAST_TOWER: Zone = Zone {
	name: "Castle Ground Floor: Southeast Tower",
	text: "You are at the bottom of the southeast tower.\nTo your north is the east corridor and to your west is the entrance hall.\nYou can go up the stairs.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_GF_EAST_CORRIDOR),
		south: None,
		east: None,
		west: Some(&CASTLE_GF_ENTRANCE),
		up: Some(&CASTLE_1F_SOUTHEAST_TOWER),
		down: None,
	},

	random_encounters: None,
};

// Ground Floor Corridors
pub static CASTLE_GF_EAST_CORRIDOR: Zone = Zone {
	name: "Castle Ground Floor: East Corridor",
	text: "You are in the castle's eastern corridor.\nTo your north is the northeast tower and to your south is the southeast tower.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_GF_NORTHEAST_TOWER),
		south: Some(&CASTLE_GF_SOUTHEAST_TOWER),
		east: None,
		west: None,
		up: None,
		down: None,
	},

	random_encounters: None,
};

pub static CASTLE_GF_WEST_CORRIDOR: Zone = Zone {
	name: "Castle Ground Floor: West Corridor",
	text: "You are in the castle's western corridor.\nTo your north is the northwest tower and to your south is the southwest tower. To your east is the kitchen.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_GF_NORTHWEST_TOWER),
		south: Some(&CASTLE_GF_SOUTHWEST_TOWER),
		east: Some(&CASTLE_GF_KITCHEN),
		west: None,
		up: None,
		down: None,
	},

	random_encounters: None,
};

pub static CASTLE_GF_ENTRANCE: Zone = Zone {
	name: "Castle Ground Floor: Entrance Hall",
	text: "You are in the castle's lavishly-decorated entrance hallway. \n To your east is the southeast tower, to your north is the banquet hall and to your south is the castle town.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_GF_BANQUET_HALL),
		south: Some(&CASTLE_COURTYARD),
		east: Some(&CASTLE_GF_SOUTHEAST_TOWER),
		west: Some(&CASTLE_GF_SOUTHWEST_TOWER),
		up: None,
		down: None,
	},

	random_encounters: None,
};

// Castle Ground Floor Rooms
pub static CASTLE_GF_KITCHEN: Zone = Zone {
	name: "Castle Ground Floor: Kitchen",
	text: "You are in the castle's kitchen.\nTo your west is the west corridor and to your east is the banquet hall.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: None,
		south: None,
		east: Some(&CASTLE_GF_BANQUET_HALL),
		west: Some(&CASTLE_GF_WEST_CORRIDOR),
		up: None,
		down: None,
	},

	random_encounters: None,
};

pub static CASTLE_GF_BANQUET_HALL: Zone = Zone {
	name: "Castle Ground Floor: Banquet Hall",
	text: "You are in the castle's banquet hall.\nTo your south is the entrance hall, to your west is the kitchen and to your north is the north corridor.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_GF_NORTH_CORRIDOR),
		south: Some(&CASTLE_GF_ENTRANCE),
		east: None,
		west: Some(&CASTLE_GF_KITCHEN),
		up: None,
		down: None,
	},

	random_encounters: None,
};

pub static CASTLE_GF_NORTH_CORRIDOR: Zone = Zone {
	name: "Castle Ground Floor: North Corridor",
	text: "You are in the castle's northern corridor.\nTo your west is the southwest tower, to your south is the banquet hall and to your east is the southeast tower.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_GF_BANQUET_HALL),
		south: None,
		east: Some(&CASTLE_GF_SOUTHEAST_TOWER),
		west: Some(&CASTLE_GF_SOUTHWEST_TOWER),
		up: None,
		down: None,
	},

	random_encounters: None,
};

// Castle Basement, currently unused
/*
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
	},

	random_encounters: None,
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
	},

	random_encounters: None,
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
	},

	random_encounters: None,
};
*/

// Castle Town

pub static CASTLE_COURTYARD: Zone = Zone {
	name: "Castle Courtyard",
	text: "You are in the castle's courtyard.\nTo your north is the castle's interior and to your south is the castle drawbridge.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_GF_ENTRANCE),
		south: Some(&CASTLE_DRAWBRIDGE),
		east: None,
		west: None,
		up: None,
		down: None,
	},

	random_encounters: None,
};

// TODO: Make the drawbridge area inacessible during the night. Also disable resting on the drawbridge.
pub static CASTLE_DRAWBRIDGE: Zone = Zone {
	name: "Castle Drawbridge",
	text: "You are on the castle's drawbridge.\nTo your north is the castle's drawbridge and to your south is the Royal Square in Castle Town.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_COURTYARD),
		south: Some(&CASTLE_TOWN_ROYAL_SQUARE),
		east: None,
		west: None,
		up: None,
		down: None,
	},

	random_encounters: None,
};

pub static CASTLE_TOWN_ROYAL_SQUARE: Zone = Zone {
	name: "Castle Town - Royal Square",
	text: "You are in the busy Royal Square at Castle Town.\nTo your south are the city gates, to your west is the Merchant District, to your east is the Military District and to your north is the castle.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_DRAWBRIDGE),
		south: Some(&CASTLE_TOWN_CITY_GATES),
		east: Some(&CASTLE_TOWN_MILITARY_DISTRICT),
		west: Some(&CASTLE_TOWN_MERCHANT_DISTRICT),
		up: None,
		down: None,
	},

	random_encounters: None,
};

// TODO: These areas need a proper shop system integrated into the Connections struct
// There should be a General Store here
pub static CASTLE_TOWN_MERCHANT_DISTRICT: Zone = Zone {
	name: "Castle Town - Merchant District",
	text: "You are in the bustling central buisness area that is Castle Town's Merchant District.\nTo your east is the Royal Square and to your north is the Theatre District.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_TOWN_THEATRE_DISTRICT),
		south: None,
		east: Some(&CASTLE_TOWN_ROYAL_SQUARE),
		west: None,
		up: None,
		down: None,
	},

	random_encounters: None,
};

// There should be a Magic Shop here
pub static CASTLE_TOWN_THEATRE_DISTRICT: Zone = Zone {
	name: "Castle Town - Theatre District",
	text: "You are in Castle Town's Theatre District.\nTo your south is the Merchant District and to your north is the Cathedral District.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_TOWN_CATHEDRAL_DISTRICT),
		south: Some(&CASTLE_TOWN_MERCHANT_DISTRICT),
		east: None,
		west: None,
		up: None,
		down: None,
	},

	random_encounters: None,
};

// There should obviously be a Cathedral here which functions as a Law-aligned chapel. There should also be an Infirmary here.
pub static CASTLE_TOWN_CATHEDRAL_DISTRICT: Zone = Zone {
	name: "Castle Town - Cathedral District",
	text: "You are in Castle Town's Cathedral District.\nTo your east is the Luxury District and to your south is the Theatre District.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: None,
		south: Some(&CASTLE_TOWN_CATHEDRAL_DISTRICT),
		east: Some(&CASTLE_TOWN_LUXURY_DISTRICT),
		west: None,
		up: None,
		down: None,
	},

	random_encounters: None,
};

pub static CASTLE_TOWN_LUXURY_DISTRICT: Zone = Zone {
	name: "Castle Town - Luxury District",
	text: "You are in Castle Town's Luxury District, which is where all the rich people live.\nTo your west is the Workers' District and to your east is the Cathedral District.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: None,
		south: None,
		east: Some(&CASTLE_TOWN_CATHEDRAL_DISTRICT),
		west: Some(&CASTLE_TOWN_WORKERS_DISTRICT),
		up: None,
		down: None,
	},

	random_encounters: None,
};

// The Tavern should be here
pub static CASTLE_TOWN_WORKERS_DISTRICT: Zone = Zone {
	name: "Castle Town - Workers' District",
	text: "You are in Castle Town's Workers' District which is where everything is made.\nTo your west is the Luxury District and to your south is the Peasant Area.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: None,
		south: Some(&CASTLE_TOWN_PEASANT_AREA),
		east: None,
		west: Some(&CASTLE_TOWN_LUXURY_DISTRICT),
		up: None,
		down: None,
	},

	random_encounters: None,
};

// There should be a secret Chaos-aligned chapel here as well as an entrance to the Sewers (which itself is the secret hideout of the Theives' Guild).
pub static CASTLE_TOWN_PEASANT_AREA: Zone = Zone {
	name: "Castle Town - Peasant Area",
	text: "You cross through the quarantine checkpoint into the slumlike Peasant Area. Already one of the most malnourished parts of the city, a confirmed outbreak of the plague in the area means that few people willingly set foot in the area.\nTo your north is the Workers's District and to your south is the Military District.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_TOWN_WORKERS_DISTRICT),
		south: Some(&CASTLE_TOWN_MILITARY_DISTRICT),
		east: None,
		west: None,
		up: None,
		down: None,
	},

	random_encounters: None,
};

// The Armoury should be here
pub static CASTLE_TOWN_MILITARY_DISTRICT: Zone = Zone {
	name: "Castle Town - Military District",
	text: "You are in Castle Town's Military District\nTo your west is the Royal Square and to your north is the Peasant Area.",
	encounter_rate: 0,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_TOWN_PEASANT_AREA),
		south: None,
		east: None,
		west: Some(&CASTLE_TOWN_ROYAL_SQUARE),
		up: None,
		down: None,
	},

	random_encounters: None,
};

pub static CASTLE_TOWN_CITY_GATES: Zone = Zone {
	name: "Castle Town - City Gates",
	text: "You stand in the City Gates of Castle Town.\nA stern-looking guard stands on duty. He's wearing steel armout and holds a shield and spear.\nTo your south is the Plain Plains and to your north is the Royal Square.",
	encounter_rate: 0,
	npcs: Some(&[
        NPC {name: "guard", // Ideally this character should take a more aggressive tone if you're Chaotic. He should also instantly catch you if you try to leave without any equipment.
            dialogue: "You catch the guard's attention and he begins speaking.\n\"Off to go kill a giant? You really should make sure that you are properly equipped before heading out!At the very least, you should make sure that your weapons and armour are up to scratch before heading out. You won't be going anywhere near that giant if the monsters living out on Plain Plains get you first!\"", fight_table: None},
    ]),
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_TOWN_ROYAL_SQUARE),
		south: Some(&PLAIN_PLAINS_CASTLE_TOWN_GATES),
		east: None,
		west: Some(&CASTLE_TOWN_ROYAL_SQUARE),
		up: None, // Perhaps there should be an upper level here?
		down: None,
	},

	random_encounters: None,
};

// Plain Plains
// Because this is an overworld region, random encounters should be enabled in every area.
pub static PLAIN_PLAINS_CASTLE_TOWN_GATES: Zone = Zone {
	name: "Plain Plains - Castle Town Gates",
	text: "You stand outside the City Gates of Castle Town.",
	encounter_rate: 10,
	npcs: None,
	objects: None,

	directions: &Connections {
		north: Some(&CASTLE_TOWN_CITY_GATES),
		south: None,
		east: None,
		west: None,
		up: None, // Perhaps there should be an upper level here?
		down: None,
	},

	random_encounters: Some(&[
		// I probably need hostility indicators for random encounters. And some prefabricated encounter tables for the sake of readability - I tried making them once but I failed miserably. Ehehehe.
		NPC {
			name: "pebble", // Not really needed for these but still
			dialogue: "Suddenly, an aggressive rock with googly eyes shows up and attacks you!",
			fight_table: Some(ENCTABLE_SINGLE_PEBBLE),
		},
		NPC {
			name: "goblin", // Not really needed for these but still
			dialogue: "A goblin sneaks up and attacks you!",
			fight_table: Some(ENCTABLE_SINGLE_GOBLIN),
		},
		NPC {
			name: "goblins", // Not really needed for these but still
			dialogue: "You were ganged up on by some goblins and a rock",
			fight_table: Some(ENCTABLE_GOBLIN_GANG),
		},
		NPC {
			name: "pebbles", // Not really needed for these but still
			dialogue: "A herd of aggressive pebbles approached rapidly!",
			fight_table: Some(ENCTABLE_TRIPLE_PEBBLE),
		},
	]),
};
