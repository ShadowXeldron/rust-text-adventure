// This script is loosely based on this tutorial: https://python-text-adventure.readthedocs.io/en/latest/index.html

// Import libraries
use read_input::prelude::*;
use clearscreen::clear;
use rand::prelude::*;
use std::str::SplitWhitespace;

// Include files
//mod zones;

// All areas in the game are defined as Zones

#[derive(Copy, Clone)]
pub struct Zone<'a> {
    name: &'a str,
    text: &'a str,
    npcs: Option<&'a [NPC<'a>]>,
    objects: Option<&'a [&'a str]>, // Replace this with a proper item check
    directions: &'a Connections<'a>,
    // encounter_rate: usize // base encounter rate value out of 100
    // encounter_table: (a vector full of enemy definitions)
    // script: (area-specific stuff for example taking damage in a poison area)
}

// Struct for setting up map connections
pub struct Connections<'a> {
    north: Option<&'a Zone<'a>>,
    south: Option<&'a Zone<'a>>,
    east: Option<&'a Zone<'a>>,
    west: Option<&'a Zone<'a>>,
    up: Option<&'a Zone<'a>>,
    down: Option<&'a Zone<'a>>,
}

#[derive(Copy, Clone)]
pub struct NPC<'a> {
    name: &'a str,
    intro: Option<&'a str>,
    dialogue: &'a str, // Printed in speech marks
}


pub static castle_1f_throne_room: Zone = Zone {
    name: "Castle 1F: Throne Room",
    text: "You stand in the throne room of the royal palace. Upon the throne sits the land's portly ruler, draped in a majestic red cloak and wearing a ruby-encrusted crown of solid gold upon his head.\nA distressed-looking royal guard stands near to the king.He is wearing ornately decorated armour and carries a gleaming sword and shield.\nAt the southern end of the room is a doorway that leads into a hallway that heads to the rest of the castle.",
    // Behold, maximum dumbness
    npcs: Some(&[
        
    NPC {name: "king", 
        intro:  Some("You stand before the obese monarch of the land and he begins to talk in a somewhat unimpressed tone."),
        dialogue: "Didn't I tell you to go and kill that giant? If you don't get a move on I- mean, WE will not ne able to have dinner!"}, 
    
    NPC {name: "guard", 
        intro: Some("The guard turns toward you and begins to speak."), 
        dialogue: "Please hurry up, if you don't I'll never hear the end of it. You should go to the armoury: we have some equipment in there that will be indespensible on your quest. It might also be visiting the court mage on top of the northwest tower as well"},
    
    NPC {name: "queen", intro: None, dialogue: "You see no queen in the vicinity and proceed to call the king maidenless. He retorts by stating that Elden Ring memes aren't funny anymore."}
    ]),
    objects: None,

    // Wish there was a better way to do this...
    directions: &Connections {
        north: None,
        south: Some(&castle_1f_throne_room_corridor),
        east: None,
        west: None,
        up: None,
        down: None,
    }
};

pub static castle_1f_throne_room_corridor: Zone = Zone {
    name: "Castle 1F: Throne Room Corridor",
    text: "You stand in a great hallway decorated in paintings and tapestries depicting the land's many victories. A row of shiny armoured suits lines the corridor.\nTo your north is the entrance to the castle's throne room, and to south is the southern wing of the castle.",
    npcs: None,
    objects: None,

    directions: &Connections {
        north: Some(&castle_1f_throne_room),
        south: None,
        east: None,
        west: None,
        up: None,
        down: None,
    }

    //directions: Connections {
    //    north: castle_1f_throne_room.into()
    //},
};

// MOVE THESE BACK INTO zones.rs EVENTUALLY!

fn main() {
    let player_name: String;
    //let player_title: String = "Newbie".to_string();
    let mut player_alignment: i8 = 0;

    let mut player_level: usize = 1;
    let mut player_hp: usize = 20;
    let mut player_mp: usize = 10;
    let mut player_exp: usize = 0;
    let mut player_bonus_points: usize = 15;

    // Character Stats
    let mut player_strength: usize = 5;
    let mut player_dexterity: usize = 5;
    let mut player_constitution: usize = 5;
    let mut player_intelligence: usize = 5;
    let mut player_spirit: usize = 5;

    //clear();
    println!("Hello, world!");
    println!("Really basic Rust (was python) text adventure");

    println!("\nAsk the player whether they want to load a save");

    // New game startup sequence;

    print!("Enter player name: ");
    player_name = input::<String>().get();
    clear();
    println!("{}, be ready to die miserably", player_name);

    println!("Add the character generation sequence here");
    println!("Insert overly drawn out backstory here");

    clear();
    show_stat_row(player_name, get_title(player_alignment, player_level), player_level, player_hp, player_mp, player_mp);
    player_action(castle_1f_throne_room);

}

// Functions and whatnot. I already miss Python.

// This code is probably bad... yet part of me thinks this is the right way to do it?
fn show_stat_row(name: String, title: String, level: usize, exp: usize, health: usize, mana: usize) {
    println!(
        "
        {name} the {title}
    Level {level},     EXP: {exp}/XX (XX until next level)
    HP: {health}/XX     MP: {mana}/XX    Status: Normal
    Area: Castle 1F: Throne Room
        "
    );
}

// TODO: Spin these off into a seperate file, preferably not as functions

/*fn castle_1f_throne_room() {
    println!("You stand in the throne room of the royal palace. Upon the throne sits the land's portly ruler, draped in a majestic red cloak and wearing a ruby-encrusted crown of solid gold upon his head.");
    println!("A distressed-looking royal guard stands near to the king. He is wearing ornately decorated armour and carries a gleaming sword and shield.");
    println!("At the southern end of the room is a doorway that leads into a hallway that heads to the rest of the castle.");


    player_action(vec!["n", "s"], vec!["king", "guard", "queen"])
}*/

// Text parser. This might also be better off in its own file.
// Also, rework it to use structs
fn player_action(zone: Zone) {
    
    loop {
        let action: String;

        print!("What do you want to do? \n > ");
        action = input::<String>().get().to_lowercase(); // This is so it can be case insensitive
        
        // I **REALLY** NEED TO OPTIMISE THIS!
        let holder: SplitWhitespace = action.split_whitespace();
        let vec: Vec<&str> = holder.collect();
        let verb: &str = vec[0];
        let noun: &str;

        // crash prevention
        if vec.len() == 1 {noun = "EMPTY"} else {noun = vec[1]}

        //print!("{} {}", verb, noun);

        match verb {
            "info" =>
                println!("You are Name the Title\nYou are in the place you are in\nYou are aligned\nYou are conditioned"),
            
            "take" =>
                if zone.objects.clone().expect("Invalid Object").contains(&noun) {break}
                else {println!("You can't go {noun}")}
            
            &_ => println!("Unknown Command")
        }
    }
}

fn get_alignment(align: i8) -> String 
{
    let name: &str;

    // There might be a much better way to do this. Can I use case matching here?
    if align < -120 {
        name = "completely unfettered";
    }
    
    else if align < -90 {
        name = "anarchistic";
    }
    
    else if align < -70 {
        name = "rebelious";
    }

    else if align < -50 {
        name = "chaotic";
    }

    else if align > -20 && align < 20 {
        name = "neutral";
    }
    
    else if align > 120 {
        name = "devoutly monarchistic";
    }
    
    else if align > 90 {
        name = "patriotic";
    }
    
    else if align > 70 {
        name = "loyal";
    }

    else if align > 50 {
        name = "lawful";
    }

    // Absolute fallback
    else {
        name = "unaligned";
    }

    return name.to_string()
}

fn get_title(align: i8, level: usize) -> String
{
    let title: &str;
    // Name-based title easter eggs
    // If named Sonic, Amy, Shadow or Silver and you have at least 25 Dexterity
    // title = "Hedgehog"

    // If called Memphis Tenessse
    //  title = "Alpha Gamer"

    // If named Mario or Luigi and you have at least 15 Dexterity and intermediate proficiency in clubs
    // title = "Plumber"

    // If named Adol and you have Advanced proficiency in swords
    // title = "Red"

    // If named Arsene, Ren, Akira or Joker, have basic proficiency in knives and you have at least 10 Spirit and at least 15 Dexterity
    //  title = "Trickster"

    // If named Bruce or Batman and you're chaotic
    //  title = "Dark Knight"

    // If named Edward and you have at least one stat below 5
    //  title = "Spoony"

    // Negative stat based titles - always base these on the lowest stat
    
    // Strength
    // title = "Weak"

    // Constitution
    // title = "Sickly"

    // Dexterity
    // title = "Slow"

    // Intellect
    // title = "Dumb"
    // title = "Idiot"

    // Spirit
    // title = "Cursed"
    // title = "Haunted"

    //if all stats below 5
    // title = "Wretched"

    //if your total gold value - including all inventory items and gold in the bank - is below a certain amount
    //  title = "Poor"
    //  title = "Squalid"



    // Level based titles
    if level < 6
    {
        title = "Rookie"
    }
    
    // Alignment based titles
    else if align > 120
    {
        title = "Paragon"
    }
    
    else if align > -120
    {
        title = "Renegade"
        // if named Joker, read "Clown Prince" instead
    }

    
    // Proficiency based titles

    //if mastered all melee weapons
    //  title = "Master of Arms"

    //if mastered swords
    //  title = "Sword Saint"

    //if mastered axes
    //  title = "Berserker"

    //if mastered spears
    //  title = "Dragoon"

    //if mastered knives
    //  title = "Night Blade"

    //if mastered bows
    //  title = "Deadeye"

    //if mastered combat magic
    // title = "Archmage"

    //if mastered support magic
    //  title = "Great Healer"

    // Positive Stat based titles - unless always base these on the highest stat!
    // Strength stuff
    // title = "Strong"
    // title = "Unstoppable"

    // Constitution Stuff
    // title = "Healthy"
    // title = "Tough"
    // title = "Immovable"

    // Dexterity
    // title = "Agile"
    // title = "Swift"
    
    // Intelligence
    // title = "Smart"
    // title = "Wise"
    // title = "Genius"

    // Spirit
    // title = "Spiritual"
    // title = "Oracle"

    // Absolute fallback
    else
    {
        title = "Traveler"
    }

    return title.to_string()

}
