// Prefabricated encounter tables

use crate::combat::mobs::*;
//use crate::zones::NPC;

// Basic enemy tables

/*pub static RANDOM_ENCOUNTER_SAMPLE: &[NPC] = &[
    NPC {
        name: "pebble", // Not really needed for these but still
        dialogue: "Suddenly, an aggressive rock with googly eyes shows up and attacks you!",
        fight_table: Some(ENCTABLE_SINGLE_PEBBLE)
    },
    
    NPC {
        name: "goblin", // Not really needed for these but still
        dialogue: "A goblin sneaks up and attacks you!",
        fight_table: Some(ENCTABLE_SINGLE_GOBLIN)
    }
];*/
    

pub static ENCTABLE_SINGLE_PEBBLE: &[Mob] = &[MOB_PEBBLE];
pub static ENCTABLE_SINGLE_GOBLIN: &[Mob] = &[MOB_GOBLIN];
//pub static ENCTABLE_TRIPLE_PEBBLE: &[Mob] = &[MOB_PEBBLE, MOB_PEBBLE, MOB_PEBBLE];
//pub static ENCTABLE_GRUE: &[Mob] = &[MOB_GRUE];
