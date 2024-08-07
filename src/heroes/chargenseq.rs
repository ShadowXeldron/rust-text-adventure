use crate::heroes::*;
use read_input::shortcut::input;

pub fn generate_character(character_name: &str) -> Hero {
    //println!("Would you like to use a prebuilt character or assign stats yourself (y/n)?");
    //let characterPrompt: usize = input::<usize>().get();

    let mut character_strength: usize = 5;
    let mut character_dexterity: usize = 5;
    let mut character_constitution: usize = 5;
    let mut character_intelligence: usize = 5;
    let mut character_spirit: usize = 5;

    // This should ask a question regarding whether or not to use a prebuilt character

    if true
    {
        let mut compare_stat: usize = 0;
        let mut stat_name: &str = "PLACEHOLDER";
        let mut bonus_points: usize = 10;

        while bonus_points != 0
        {
            // Very bad code alert
            println!("
            1) STRENGTH - {} - [{}]
            2) DEXTERITY - {} - [{}]
            3) CONSTITUTION - {} - [{}]
            4) INTELLIGENCE - {} - [{}]
            5) SPIRIT - {} - [{}]

            You have {} bonus point(s) remaining.
            TIP: If you want to remove points that you applied to a stat, enter a minus number. This is the only time you can do this!
            ",

            // Due to how line ends work in Rust, I can do this for the sake of readability
            character_strength, "*".repeat(character_strength),
            character_dexterity, "*".repeat(character_dexterity),
            character_constitution, "*".repeat(character_constitution),
            character_intelligence, "*". repeat(character_intelligence),
            character_spirit, "*". repeat(character_spirit),

            bonus_points);
        
            print!("Choose a stat number (1-5): ");
            let chosen_stat: u8 = input::<u8>().get();
                
            // Case switching because I have PTSD from elseif chains (see my Sonic mods). I don't care if these are fundamentally identical to elseif chains I'd rather not have to deal with them
            // TODO: Find a way to only have to do this in one case switch
            match chosen_stat
            {
                1 =>
                {
                    compare_stat = character_strength;
                    stat_name = "Strength";
                }

                2 =>
                {
                    compare_stat = character_dexterity;
                    stat_name = "Dexterity";
                }

                3 =>
                {
                    compare_stat = character_constitution;
                    stat_name = "Constitution";
                }

                4 =>
                {
                    compare_stat = character_intelligence;
                    stat_name = "Intelligence";
                }

                5 =>
                {
                    compare_stat = character_spirit;
                    stat_name = "Spirit";
                }

                _ => println!("Invalid stat!")
            }

            print!("How many bonus points do you want to spend on this {stat_name}? (You have {bonus_points} bonus point(s) remaining): ");
            let points_to_use: usize = input().get();

            // WARNING! This features elseifs in elseifs in elseifs which will most likely cause you to throw up            
               
            if points_to_use > bonus_points // If you try to use more bonus points than you have
            {
                println!("You do not have that many bonus points!");
            }
                // TODO: Add negotiation to use your remaining bonus points instead

            else if points_to_use == 0
            {
                break; // Do nothing
            }

            else if points_to_use < 0 // if you are REMOVING stat points
            {
                if (compare_stat + points_to_use) > compare_stat // Minus numbers are weird
                {
                    println!("You cannot lower a stat below 1");
                }

                else
                {
                    print!("Are you sure you want to remove {} from {stat_name}? (y/n): ", points_to_use);
                    
                    if input::<String>().get() == "y"
                    {
                        // Lower the stat and increase bonus points. This blantantly exploits the bizzare quirks of minus numbers.
                        compare_stat += points_to_use;
                        bonus_points -= points_to_use;
                                    
                        match chosen_stat
                        {
                            1 =>
                                character_strength -= points_to_use,
                            2 =>
                                character_dexterity -= points_to_use,
                            3 =>
                                character_constitution -= points_to_use,
                            4 =>
                                character_intelligence -= points_to_use,
                            5 =>
                                character_spirit -= points_to_use,
                            _ => panic!("Invalid stat number!")
                        }

                        println!("{} decreased!", stat_name);
                        // Originally, there was a limit of 5 points for "stat dumps" (lowering a stat below 5), but I decided to omit that limitation because it was a pain in the arse to implement 
                    }
                }
            }

                        
            // Now for actually adding stat points
            else {
                print!("Are you sure you want to invest {points_to_use} in {stat_name}? (y/n): ");
                if input::<String>().get() == "y" {
                    bonus_points -= points_to_use;

                    // Now apply stat changes
                    match chosen_stat {
                        1 =>
                            character_strength += points_to_use,
                        2 =>
                            character_dexterity += points_to_use,
                        3 =>
                            character_constitution += points_to_use,
                        4 =>
                            character_intelligence += points_to_use,
                        5 =>
                            character_spirit += points_to_use,
                        _ => panic!("Invalid stat number!")
                    }
                }
            }
        }
    }

    let character_hp: usize = (character_constitution * 1) + ((character_strength / 2) + 1);
    let character_mp: usize = (character_intelligence * 1) + character_spirit + 1;

    // Return the hero
    println!("Generated hero!");
    return Hero {
        name: character_name,
        max_hp: character_hp,
        hp: character_hp,
        max_mp: character_mp,
        mp: character_mp,

        stats: Stats {
            level: 1,
            constitution: character_constitution,
            strength: character_strength,
            dexterity: character_dexterity,
            intelligence: character_intelligence,
            spirit: character_spirit,
            ac: 0, // Armour Class - applied directly on top of defence
            mr: 0,
            wp: 1,
            sp: 1
        },

        elements: ElementalEffects {
            weak: None,
            resist: None,
            immune: None,
            heal: None, // Some(&[TYPE_SLASH])
            reflect: None,
            avoid: None,
        },

        exp: 0,
        movelist: &[]

    };

    // Absolute fallback in case things go haywire
    //else
    //{
    //    println!("Invalid stat number!")
    //}
}
