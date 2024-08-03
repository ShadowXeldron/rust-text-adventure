println!("Would you like to use a prebuilt character or assign stats yourself (y/n)?");
    let characterPrompt: usize = input::<usize>().get();

    // Character templates

    if true
    {
        while player_bonusPoints != 0
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
            ", player_Strength, "*".repeat(player_Strength), player_Dexterity, "*".repeat(player_Dexterity), player_Constitution, "*".repeat(player_Constitution), player_Intelligence, "*".repeat( player_Intelligence), player_Spirit, "*".repeat(player_Intelligence), player_bonusPoints);
        
            print!("Choose a stat number (1-5): ");
            let mut chosenStat: usize = input::<usize>().get();
                
            // Case switching because I have PTSD from elseif chains (see my Sonic mods). I don't care if these are fundamentally identical to elseif chains I'd rather not have to deal with them

            let mut compareStat: usize;
            let mut statName: String;

            // TODO: Find a way to only have to do this in one case switch
            match chosenStat
            {
                1 =>
                {
                    compareStat = player_Strength;
                    statName = "Strength".to_string();
                }

                2 =>
                {
                    compareStat = player_Dexterity;
                    statName = "Dexterity".to_string();
                }

                3 =>
                {
                    compareStat = player_Constitution;
                    statName = "Constitution".to_string();
                }

                4 =>
                {
                    compareStat = player_Intelligence;
                    statName = "Intelligence".to_string();
                }

                5 =>
                {
                    compareStat = player_Spirit;
                    statName = "Spirit".to_string();
                }
            }

            print!("How many bonus points do you want to spend on this {statName}? (You have {player_bonusPoints} bonus point(s) remaining): ");
            let pointsToUse: isize = input.get();

            // WARNING! This features elseifs in elseifs in elseifs which will most likely cause you to throw up            
               
            if pointsToUse.try_into().unwrap() > player_bonusPoints // If you try to use more bonus points than you have
            {
                println!("You do not have that many bonus points!");
            }
                // TODO: Add negotiation to use your remaining bonus points instead

            else if pointsToUse == 0
            {
                break; // Do nothing
            }

            else if pointsToUse < 0 // if you are REMOVING stat points
            {
                if (compareStat + pointsToUse.try_into().unwrap()) < 1 // Minus numbers are weird
                {
                    println!("You cannot lower a stat below 1");
                }         

                else
                {
                    print!("Are you sure you want to remove {} from {statName}? (y/n): ", -pointsToUse);
                    
                    if input.get() == "y"
                    {
                        // Lower the stat and increase bonus points. This blantantly exploits the bizzare quirks of minus numbers.
                        compareStat += pointsToUse;
                        player_bonusPoints -= pointsToUse;
                                    
                        match chosenStat
                        {
                            1 =>
                                player_Strength += pointsToUse,
                            2 =>
                                player_Dexterity += pointsToUse,
                            3 =>
                                player_Constitution += pointsToUse,
                            4 =>
                                player_Intelligence += pointsToUse,
                            5 =>
                                player_Spirit += pointsToUse,
                        }

                        println!("{} decreased!", statName);
                        // Originally, there was a limit of 5 points for "stat dumps" (lowering a stat below 5), but I decided to omit that limitation because it was a pain in the arse to implement 
                    }
                }
            }

                        
            // Now for actually adding stat points
            else
                {
                if input::<str>().get("Are you sure you want to invest {pointsToUse} in {statName}? (y/n): ") == "y"
                {
                    player_bonusPoints -= pointsToUse;

                    // Now apply stat changes
                    match chosenStat
                    {
                        1 =>
                            player_Strength += pointsToUse,
                        2 =>
                            player_Dexterity += pointsToUse,
                        3 =>
                            player_Constitution += pointsToUse,
                        4 =>
                            player_Intelligence += pointsToUse,
                        5 =>
                            player_Spirit += pointsToUse,
                    }
                }
            }
        }
    }

        // Absolute fallback in case things go haywire
        //else
        //{
        //    println!("Invalid stat number!")
        //}
    

    if false
    {
        println!("

        1) Knight - A melee-focused character with high strength and constituion
        2) Wizard - Magically orietated character with high
        3) Ranger - A dexterity-focused character with just enough strength to weild some of the better early bows  
        4) Barbarian - Mixed-melee character that focuses on strength, constitution and dexterity
        5) Monk - Focused on Spirit and Strength with a bit of Intelligence
        6) Bard - Mixed attacker which mainly focuses on Dexterity and Intelligence with a bit of strength
        7) Thief - Strength and Dexterity 

        X) Freelancer - A character with evenly distributed stats across the board
        0) Wretch - An absolute meme of a character with 1 in every stat. Additionally, choosing the Wretch will deny you the missing bonus points. Only choose this if you hate fun.

        ")
    }

    println!("\nInsert long-winded backstory here")

    //input("Enable hardcore mode? This hides the necessary stat thresholds for stat checks (y/n): ") // TODO

    //castle_1F_throneRoom()