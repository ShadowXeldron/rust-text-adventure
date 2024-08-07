// This script is loosely based on this tutorial: https://python-text-adventure.readthedocs.io/en/latest/index.html

// Import libraries
pub use read_input::prelude::*;
pub use clearscreen::clear;
pub use std::str::SplitWhitespace;
use rand::Rng;

// Include files
pub mod zones;
use crate::zones::*;

pub mod combat;
pub use crate::combat::*;
pub use crate::combat::mobs::*;

pub mod dicerolls;
pub use crate::dicerolls::*;

pub mod items;
pub use crate::items::*;

pub mod heroes;
pub use crate::heroes::*;
pub use crate::heroes::chargenseq::*;

// All areas in the game are defined as Zones

// Constants
// Global level cap for characters
pub const LEVEL_CAP: usize = 99;

// Define structs
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Stats {
    // Universal character stats
    pub level: usize, // Affects all stats
    pub constitution: usize, // Used to calculate base HP and natural damage reduction
    pub strength: usize, // Used to calculate physical attack power and base HP
    pub dexterity: usize, // Used to calculate accuracy, evasion and initiative
    pub intelligence: usize, // Used to calculate magic damage and base MP
    pub spirit: usize, // Used to calculate magic resistance and base MP
    pub ac: usize, // Armour Class - used to calculate physical damage reduction
    pub mr: usize, // Magic Resistance - used to calculate magical damage reduction
    pub wp: usize, // Weapon Power - used to calculate physical attack damage
    pub sp: usize, // Spell Power - used to calculate magic attack damage
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct ElementalEffects<'a> {
    pub weak: Option<&'a [usize]>, // Slice
    pub resist: Option<&'a [usize]>,
    pub immune: Option<&'a [usize]>,
    pub heal: Option<&'a [usize]>,
    pub reflect: Option<&'a [usize]>,
    pub avoid: Option<&'a [usize]>, // Will always dodge this element unless it is forced to hit
}

// Actual code begins here
fn main() {
    //let player.title: String = "Newbie".to_string();
    let player_alignment: i8 = 0;

    //clear();
    println!("Really basic Rust (was python) text adventure");
    //println!("\nAsk the player whether they want to load a save");

    // To start with, generate a character
    print!("Enter player name: ");
    let binding = input::<String>().get();
    let player_name: &str = binding.as_str();
    let player = generate_character(player_name); // Temporary immutability while I test combat mechanics

    // New game startup sequence;

    clear().expect("failed to clear screen");
    println!("{}, be ready to die miserably", player.name);

    show_stat_row(player, get_title(player_alignment, player.stats.level));
    player_action(CASTLE_1F_THRONE_ROOM, player);

    //exit(0) // Close the program peacefully when the game ends.
}

// Functions and whatnot. I already miss Python.

// This code is probably bad... yet part of me thinks this is the right way to do it?
fn show_stat_row(player: Hero, title: String) {
    println!(
        "
        {} the {title}
    Level {},     EXP: {}/{} ({} until next level)
    HP: {}/{}     MP: {}/{}    Status: Normal
    Area: Castle 1F: Throne Room
        "
    , player.name, player.stats.level, player.exp, (5 * player.stats.level + player.stats.level), player.get_remaining_exp(), player.hp, player.max_hp, player.mp, player.max_mp);

}

// TODO: Spin these off into a seperate file, preferably not as functions

/*fn castle_1f_throne_room() {
    println!("You stand in the throne room of the royal palace. Upon the throne sits the land's portly ruler, draped in a majestic red cloak and wearing a ruby-encrusted crown of solid gold upon his head.");
    println!("A distressed-looking royal guard stands near to the king. He is wearing ornately decorated armour and carries a gleaming sword and shield.");
    println!("At the southern end of the room is a doorway that leads into a hallway that heads to the rest of the castle.");


    player_action(vec!["n", "s"], vec!["king", "guard", "queen"])
}*/

// Text parser. This might also be better off in its own file.
// Also, it's constantly calling itself which I don't think is particularly good code.
fn player_action(zone: Zone, mut player: Hero) { // Hero parameter is temporary until I can figure out how to implement globals
    // Terminate the game if you have run out of health
    if player.hp == 0 {
        println!("\n \x1b[31;1;4mGAME OVER!\x1b[0m \n");
        return
    }

    println!("{}", zone.text);

    // Roll for random encounter
    let encroll: u8 = rand::thread_rng().gen_range(1..100);
    println!("Encounter roll: {}", encroll);
    if encroll < zone.encounter_rate {
        let encounters: &[NPC] = zone.random_encounters.unwrap();
        let foe: NPC = encounters[rand::thread_rng().gen_range(0..encounters.len())]; // Pick a random encounter from a table and throw an exception if the area doesn't have an encounter table
        println!("{}", foe.dialogue);
        match battle_start(&mut [player], &mut [MOB_PEBBLE]) {
            BATTLE_RESULT_VICTORY => {// Successful enemy kills
                println!("You stand victorious over your assailant. \nThe party gained {} experience points from the battle!", MOB_PEBBLE.exp_reward);
                player.gain_exp(foe.get_exp_from_encounters())
            },

            BATTLE_RESULT_FAILURE => {
                println!("You have perished upon the field of battle...");
                player.hp = 0;
                // Print "Game Over with ANSI codes"
                println!("\n \x1b[31;1;4mGAME OVER!\x1b[0m \n")
            },

            BATTLE_RESULT_ESCAPE =>
                println!("Successfully ran away from the battle."),
                // Subtract a random percentage of money.
                // println!("You dropped {} coins wile running away")

            BATTLE_RESULT_TRUCE =>
                println!("The enemy left to go and bother someone else."),

            _ =>
                println!("Suddenly, the enemy stopped existing.")

        } // Temporary functionality. If you lose the battle, it's all going to end in tears.

    }

    if player.hp != 0 {
            loop {
            print!("What do you want to do? \n > ");
            let action: String = input::<String>().get().to_lowercase(); // This is so it can be case insensitive

            // Find a way to convert inputs to lowercase at all times
            let holder: SplitWhitespace = action.split_whitespace();
            let vec: Vec<&str> = holder.collect();
            let verb: &str = vec[0];
            let noun: &str = if vec.len() == 1 {""} else {vec[1]};

            //print!("{} {}", verb, noun);

            match verb {
                "go" | "move" =>
                    // If no direction is entered
                    if noun.is_empty() {println!("What direction? You can go Up, Down, North, East, South or West")}
                    else if noun == "up" || noun == "down" ||  noun == "north" || noun == "south" || noun == "east" || noun == "west" {
                        // Not a very good implementation but I'm expecting Clippy to suggest an improvement for this
                        match noun {
                            "up" =>
                                if zone.directions.up.is_some() {
                                    // Due to how Rust works, we need to dereference the option with an asterisk and also run the "unwrap" function on the option.
                                    player_action(*zone.directions.up.unwrap(), player);
                                    break
                                }
                                else {println!("There is nothing above you.")},

                            "down" =>
                                if zone.directions.down.is_some() {
                                    player_action(*zone.directions.down.unwrap(), player);
                                    break
                                }
                                else {println!("There is nothing below you.")},

                            "north" =>
                                if zone.directions.north.is_some() {
                                    player_action(*zone.directions.north.unwrap(), player);
                                    break
                                }
                                else {println!("There is nothing to your north.")},

                            "south" =>
                                if zone.directions.south.is_some() {
                                    player_action(*zone.directions.south.unwrap(), player);
                                    break
                                }
                                else {println!("There is nothing to your south.")},

                            "east" =>
                                if zone.directions.east.is_some() {
                                    player_action(*zone.directions.east.unwrap(), player);
                                    break
                                }
                                else {println!("There is nothing to your east.")},

                            "west" =>
                                if zone.directions.west.is_some() {
                                    player_action(*zone.directions.west.unwrap(), player);
                                    break
                                }
                                else {println!("There is nothing to your west.")},

                            _ =>
                                panic!("Invalid direction! \"{}\" somehow got through the check?", noun)
                        }


                        //println!("That's correct")
                    }
                    else {println!("You can't go {noun}")}

                "info" => {
                    show_stat_row(player, get_title(0, player.stats.level));
                    println!("You are {}", get_alignment(0))},

                "take" | "get" =>
                    if zone.objects.is_some() {//zone.objects.expect("Invalid Object").contains(&noun) {break}
                        // Well then,
                        if zone.objects.expect("Invalid Object").contains(&noun) {println!("Got a {}!", noun)}
                        //println!("Somethings here but I don't know what")
                        else if noun.is_empty() {println!("You didn't take anything.")}
                        else {println!("That object isn't here.")}
                    }

                    else {
                        println!("There is nothing here that you can take.")
                    }

                "talk" | "chat" =>
                    if zone.npcs.is_some() {
                        if noun.is_empty() {println!("You talked to nobody.")}
                        else {zone.talk_npc(noun)}
                    }

                    else {
                        println!("Nobody else is here.")
                    }

                "look" => println!("{}", zone.text),

                "clear" => clear().expect("Couldn't clear screen"),

                "help" => println!("
                List of commands:

                go - Move to a connected area. You can go Up, Down, North, East, South or West

                info - Print information about the party
                help - Shows this text
                "),

                _ => println!("Unknown Command! Type \"help\" for more information.")
            }
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

    name.to_string()
}

fn get_title(align: i8, level: usize) -> String
{
    let title: &str;

    // Level based titles
    if level < 6 {
        title = "Rookie"
    }
    
    // Alignment based titles
    else if align > 120 {
        title = "Paragon"
    }
    
    else if align > -120 {
        title = "Renegade"
    }

    else {
        title = "Traveler"
    }

    title.to_string()
}












