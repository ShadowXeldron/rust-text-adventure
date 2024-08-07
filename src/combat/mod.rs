// Start by offloading content into other files
pub mod attacks; use crate::attacks::*;
pub mod mobs; //use crate::mobs::*;
use crate::*;
use crate::dicerolls::*;

// Load externals
use std::cmp::Reverse;
use rand::prelude::*;
use read_input::shortcut::input;

// These are probably redundant but I've included them to make modification easier.

pub const BATTLE_RESULT_VICTORY: u8 = 0; //
pub const BATTLE_RESULT_FAILURE: u8 = 1; // Total party kill. This is considered a fail state.
pub const BATTLE_RESULT_ESCAPE: u8 = 2; // Battle canceled by player escaping
pub const BATTLE_RESULT_TRUCE: u8 = 3; // Peaceful resolution from negotiation

const MOVE_PHYSICAL: usize = 0;
const MOVE_MAGIC: usize = 1;
const MOVE_STATUS: usize = 2;
const MOVE_INSTANT_KILL: usize = 3;

const TARGET_FOE: usize = 0;
const TARGET_SELF: usize = 1;

const TYPE_NEUTRAL: usize = 0;
const TYPE_SLASH: usize = 1;
const TYPE_PIERCE: usize = 2;
const TYPE_IMPACT: usize = 3;

const TYPE_FIRE: usize = 4;
const TYPE_ICE: usize = 5;
const TYPE_ELECTRIC: usize = 6;
const TYPE_WIND: usize = 7;
const TYPE_GROUND: usize = 8;
const TYPE_DARK: usize = 9;
const TYPE_LIGHT: usize = 10;

// Set up globals


/*
// Weapon type constants, for later

const WEAPON_NONE: usize = 0;
// Slash
const WEAPON_SWORD: usize = 1;
const WEAPON_GREATSWORD: usize = 2;
const WEAPON_AXE: usize = 3;
const WEAPON_GREATAXE: usize = 4;
// Pierce
const WEAPON_SPEAR: usize = 5;
const WEAPON_POLEARM: usize = 6;
const WEAPON_BOW: usize = 8;
// Impact
const WEAPON_CLUB: usize = 9;
const WEAPON_HAMMER: usize = 10;
const WEAPON_STAFF: usize = 11;
// Unique
const WEAPON_WAND: usize = 12;
const WEAPON_KNIFE: usize = 13;

// Array for checking weapon damage types, although for autogen knives will do slash or pierce damage depending on what's more effective
static WEAPON_DAMAGE_TYPES: [usize; 13] = [TYPE_NEUTRAL /* Barehand */, TYPE_SLASH /* Sword */, TYPE_SLASH /* Greatsword */, TYPE_SLASH /* Axe */, TYPE_SLASH /* Greataxe */, TYPE_PIERCE /* Spear */, TYPE_PIERCE /* Polearm */, TYPE_PIERCE /* Bow */, TYPE_IMPACT /* Club */, TYPE_IMPACT /* Hammer */, TYPE_IMPACT /* Staff */, TYPE_NEUTRAL /* Wand */, TYPE_PIERCE /* Knife */];
*/
// Constant array of string names for the elements
static ELEMENT_NAMES: &[&str] = &["Neutral", "Slash", "Pierce", "Impact", "Fire", "Ice", "Electric", "Wind", "Ground", "Dark", "Light"];


// Add targeting stuff

/* This function returns a different number depending on the result:
    0. All enemies dead, players win and experience given
    1. All players dead, considered a fail state
    2. Battle terminated by player escape
    3. Battle resolved peacefully by negotiation
*/

pub fn battle_start(players: &mut [Hero], baddies: &mut [Mob] /*slice filled with encounter data*/ ) -> u8 {
    const DMG_HURT: u8 = 1;
    const DMG_REPEL: u8 = 2;
    const DMG_ABSORB: u8 = 3;

    let mut damage: usize;


    // Establish baseline point values for each character
    for counter in 0..players.len() {
        players[counter].max_hp = (players[counter].stats.constitution * players[counter].stats.level) + ((players[counter].stats.strength / 2) + players[counter].stats.level) + 10;
        players[counter].hp = players[counter].max_hp;
        players[counter].max_mp = (players[counter].stats.intelligence * players[counter].stats.level) + players[counter].stats.spirit + players[counter].stats.level;
        players[counter].mp = players[counter].max_mp;
    }

    for counter in 0..baddies.len() {
        baddies[counter].max_hp = (baddies[counter].stats.constitution * baddies[counter].stats.level) + ((baddies[counter].stats.strength / 2) + baddies[counter].stats.level);
        baddies[counter].hp = baddies[counter].max_hp;
        baddies[counter].max_mp = (baddies[counter].stats.intelligence * baddies[counter].stats.level) + baddies[counter].stats.spirit + baddies[counter].stats.level;
        baddies[counter].mp = baddies[counter].max_mp;
    }

    let mut dmg_mode: u8; // Used for deciding whether damage is dealt nomrmally, reflected or absorbed
    let mut attack: Attack; // Attack to use
    let mut command: u8; // Used to select the player's command
    let mut enable_player_commands: bool = false;

    // Roll for initiative

    println!("{} draws near. Go forth, {}!", baddies[0].name, players[0].name);

    let mut turn_count: usize = 0; // Increment at the start of each turn

    // Start battle loop
    loop { // Break once a battle end command is signaled
        enable_player_commands = false;
        // Roll for initiative, start by rolling for each player
        // We use an SMT3 styled system where it goes player phase -> enemy phase since it's easier to implement
        players.sort_by_key(|item| Reverse(item.stats.dexterity + item.stats.level));
        baddies.sort_by_key(|item| Reverse(item.stats.dexterity + item.stats.level));

        //players[0].init = players[0].stats.dexterity + players[0].stats.level;
        //baddies[0].init = baddies[0].stats.dexterity + baddies[0].stats.level;
        //println!("Player initiative: {}\nMob initiative: {}", players[0].init, baddies[0].init);


        // preturn command loop

        loop {

            println!("Enter pre-turn command.\n1) Fight\n2) Negotiate\n3) Flee");

            command = input::<u8>().get();
            match command {
                1 => {
                    enable_player_commands = true;
                    break
                }

                2 => {
                    println!("You started trying to talk to the enemy.");
                    if start_negotiation(players[0], baddies[0]) {
                        println!("Negotiation was successful!");
                        return BATTLE_RESULT_TRUCE
                    } else {
                        println!("Negotiation failed!");
                        break
                    }
                }

                3 => {
                        println!("Your team attempted to get away...");

                        if do_saving_throw(players[0].stats.dexterity, 20, ROLL_DISADVANTAGE) {
                            println!("...and did!");
                            return BATTLE_RESULT_ESCAPE
                        } else {
                            println!("...but they couldn't escape in time!")
                        }

                        break
                    }
                _ => {println!("Invalid command!");}
            }
        }

        println!("\nTurn {} start!\n", turn_count);
        turn_count += 1;


        // Iterate through all players
        // TODO: Negotiate initiative for baddies as well

        if enable_player_commands {
            for counter in 0..players.len() {
                // Before starting, make sure the enemy has not been KO'd. We only need to check for enemy HP because it's the player's turn
                if baddies[0].hp == 0 {break}

                // Otherwise have the player select a command
                loop {
                    println!("HP: {} / {}", players[counter].hp, players[counter].max_hp);
                    println!("MP: {} / {}", players[counter].mp, players[counter].max_mp);

                    println!("{}", baddies[0].name);
                    println!("Enemy HP: {} / {}", baddies[0].hp, baddies[0].max_hp);
                    println!("Enemy MP: {} / {}", baddies[0].mp, baddies[0].max_mp);

                    println!("Player turn.");
                    println!("What should {} do?\n1) Normal Attack\n2) Use ability from spells list\n3) Use item from inventory\n4) Guard", players[counter].name);

                    command = input::<u8>().get();

                    match command {
                        1 => {
                            // Physical attack
                            println!("{} attacks", players[counter].name);

                            // Aw crud we're back to the drawing board
                            attack = generate_weapon_attack(TYPE_SLASH, 75);
                            break
                        }

                        2 => {
                            // List spells
                            if players[counter].movelist.is_empty() {println!("{} doesn't know any spells.", players[counter].name)} else {

                                println!("Abilities:");
                                for (counter, attack) in players[counter].movelist.iter().enumerate() {
                                    println!("{} - {}", counter + 1, attack.name);
                                }

                                // Cast attack from spells list
                                print!("\nEnter spell number:");
                                let spell: usize = input::<usize>().get();

                                if spell > players[counter].movelist.len() {println!("Spell outside attack range")}
                                else {
                                    show_attack_info(players[counter].movelist[spell - 1]);
                                    // Add a "use spell? (y/n)" here
                                    attack = players[counter].movelist[spell - 1];

                                    if players[counter].mp >= attack.cost {
                                        // Magic Attack

                                        players[counter].mp -= attack.cost;
                                        println!("Player used {}!", attack.name);
                                        break
                                    }
                                    else {println!("Player does not have enough MP to use {}", attack.name);}
                                }
                            }
                        }

                        3 => {println!("I cannot think of a funny quip related to items");}

                        4 => {println!("Stop right there, criminal scum! You thought you could yse a feature that hasn't been added yet?");}

                        _ => {println!("Invalid command!");}
                    }
                }

                // Now use the attack
                 let target: usize;

                if baddies.len() == 0 {target = 0}
                else {target = 0} // This should prompt the player to choose an enemy

                // Perform elemental checks
                if baddies[target].elements.heal.is_some() && baddies[0].elements.heal.expect("Invalid target element").contains(&attack.element) {dmg_mode = DMG_ABSORB;}

                else if baddies[target].elements.reflect.is_some() && baddies[0].elements.reflect.expect("Invalid target element").contains(&attack.element) {dmg_mode = DMG_REPEL;}

                else {dmg_mode = DMG_HURT;}

                // Inflict damage at the end of the player's turn
                match dmg_mode {
                    DMG_ABSORB => {
                        // Force attack to hit no matter what
                        attack.hit_rate = 101;

                        println!("But the target absorbs {}!", ELEMENT_NAMES[attack.element]);
                        damage = calculate_damage(attack, players[0].stats, baddies[0].stats, baddies[0].elements) / 2;
                        println!("{} recovered {} HP!", baddies[0].name, damage);
                        baddies[0].hp += damage;
                        if baddies[0].hp > baddies[0].max_hp {baddies[0].hp = baddies[0].max_hp}
                    }

                    DMG_REPEL => {
                        // Force attack to hit no matter what
                        attack.hit_rate = 101;

                        println!("But the target reflects {}!", ELEMENT_NAMES[attack.element]);
                        damage = calculate_damage(attack, players[0].stats, players[0].stats, baddies[0].elements);

                        println!("You took {damage} damage!");
                        if damage > players[0].hp {players[0].hp = 0;}
                        else {baddies[0].hp -= damage;}
                    }

                    _ => {
                        damage = calculate_damage(attack, players[0].stats, baddies[0].stats, baddies[0].elements);


                        if damage > 0 {
                            println!("The attack did {} damage to the {}.", damage, baddies[0].name);

                            // To prevent underflowing when your damage output exceeds the enemy's HP
                            if damage > baddies[0].hp {baddies[0].hp = 0;}
                            else {baddies[0].hp -= damage;}
                        }
                    }
                }

            }
        }


        if baddies[0].hp > 0 && players[0].hp > 0 {
            println!("\nEnemy Turn\n");

            // Used for iterating
            //let mut enemy_moved = false;

            // Needs handling for if an enemy has no 0 MP attacks
            loop {
                attack = baddies[0].movelist[rand::thread_rng().gen_range(0..baddies[0].movelist.len())];

                if baddies[0].mp >= attack.cost {
                    break;
                }

            }

            println!("The enemy used {}!", attack.name);
            baddies[0].mp -= attack.cost;

            match attack.target
            {
                TARGET_FOE => {

                    damage = calculate_damage(attack, baddies[0].stats, players[0].stats, baddies[0].elements);
                    if damage > 0 {println!("You took {damage} damage!");
                        // To prevent underflowing when your damage output exceeds the player's HP
                        if damage > players[0].hp {players[0].hp = 0;}
                        else {players[0].hp -= damage;}
                    }

                }

                TARGET_SELF => {
                    if attack.power > 0 {
                        damage = baddies[0].stats.spirit + baddies[0].stats.level + attack.power;
                        baddies[0].hp += damage;
                        if baddies[0].hp > baddies[0].max_hp {baddies[0].hp = baddies[0].max_hp}
                        println!("{} restored {} HP", baddies[0].name, damage)
                    }
                }

                _ => {
                    panic!("Invalid targeting type")

                }
            }
        }
        // If it all ends miserably, end the fight
        if baddies[0].hp == 0 || players[0].hp == 0 {break}
    }

    // If the loop breaks, check for hit points. If the enemy team was annihlated, return 0 to signal a victory. But if the player team was defeated, return 1 to signal a loss.
    if baddies[0].hp == 0 {BATTLE_RESULT_VICTORY}
    else {BATTLE_RESULT_FAILURE}
}


fn show_attack_info(attack: Attack) {
    println!("\n{}", attack.name);
    println!("MP Cost: {}", attack.cost);
    println!("Element: {}   Base Power: {}   Hit Rate: {}%", ELEMENT_NAMES[attack.element], attack.power, attack.hit_rate);
    println!("\n{}", attack.desc);
}

fn do_accuracy_check(attack: Attack, user_stats: Stats, target_stats: Stats, avoid_elements: Option<&[usize]>) -> bool {
    // Base hit rates higher than 100 are garunteed hits
    if attack.hit_rate > 100 {return true};

    // Account for avoided elements
    if avoid_elements.is_some() && avoid_elements.expect("Invalid avoid element").contains(&attack.element) {
        println!("This target always avoids {}", ELEMENT_NAMES[attack.element]);
        return false
    }


    // Otherwise, calculate hit rate

    // Okay BODMAS, don't betray me now
    let accuracy = (attack.hit_rate + user_stats.dexterity / 2 + user_stats.level) - (target_stats.dexterity + target_stats.level) / 2;
    println!("{}", accuracy);

    // Roll a random number between 1 and 100
    let hit_roll = rand::thread_rng().gen_range(1..100);
    println!("{}", hit_roll);

    // If the hit roll is higher than the accuracy calculation

    hit_roll <= accuracy
}

// I apologise for how horribly long this function is and my life would be so much easier if Rust allowed global variables
fn calculate_damage(attack: Attack, user_stats: Stats, target_stats: Stats, target_elements: ElementalEffects) -> usize {

    // Before doing anything, calculate accuracy and immunity
    if target_elements.immune.is_some() && target_elements.immune.expect("Invalid immune element").contains(&attack.element) {
        println!("This target blocks {}", ELEMENT_NAMES[attack.element]);
        return 0
    }

    else if !do_accuracy_check(attack, user_stats, target_stats, target_elements.avoid) {
        println!("The attack missed!");
        return 0
    }

    // If all goes well, calculate attack and defence
    let atk: usize;
    let def: usize;

    if attack.category == MOVE_MAGIC {
        atk = attack.power + user_stats.sp + user_stats.intelligence + user_stats.level;
        def = (target_stats.mr + target_stats.spirit + target_stats.level) / 2;
    } else {
        atk = attack.power + user_stats.wp + user_stats.strength + user_stats.level;
        def = (target_stats.ac + target_stats.constitution + target_stats.level) / 2;
    }

    // Establish damage since we'll need it later
    let mut damage: usize;

    // If defence exceeds attack, set attack to 1 to prevent integer overflows and always deal at least 1 damage
    if def >= atk {damage = 1;}
    // Otherwise
    else {damage = atk - def;}

    // Damage variance
    let variance: usize = rand::thread_rng().gen_range(0..40);
    println!("{damage} {variance} ");

    damage = ((100 + variance) * damage) / 100;
    println!("{damage}");

    // Absolute final fallback to ensure that you always deal at least 1 damage

    if target_elements.weak.is_some() && target_elements.weak.expect("Invalid weakness element").contains(&attack.element) {
        println!("{} is super effective against this target!", ELEMENT_NAMES[attack.element]);
        damage *= 2;
    }

    else if target_elements.resist.is_some() && target_elements.resist.expect("Invalid resistance element").contains(&attack.element) {
        println!("{} isn't very effective against this target...", ELEMENT_NAMES[attack.element]);
        damage /= 2;
    }

    if damage < 1 {damage = 1;}

    damage
}

// For magic attacks, your intelligence stat is used to caclulate both accuracy and attack power

fn generate_weapon_attack(weapon_type: usize, weapon_accuracy: usize) -> Attack<'static> {
    // This automatically generates a flat attack based on your equipped weapon
    Attack {
        name: "Weapon Attack",
        desc: "Attack automatically generated from your equipped weapon",
        cost: 0,
        category: MOVE_PHYSICAL,
        element: weapon_type,
        power: 1,
        hit_rate: weapon_accuracy,
        target: TARGET_FOE
    }
}

// Returns true if the battle was resolved peacefully. Otherwise returns false if negotiations couldn't happen
fn start_negotiation(hero: Hero, mob: Mob) -> bool {
    let mut command: u8; // Used to select the player's command

    // This next bit should pull from an enemy's dialogue tree. This may have to be defined as a JSON or something.

    // Generic actions
    println!("1) Persuade (intelligence saving throw)\n2) Show Mercy (spirit saving throw)\n3) Threaten (strength saving throw)");
    command = input::<u8>().get();

    match command {
        1 => {
            println!("You tried to persuade the enemy to stop fighting. \n");

            if mob.hp == mob.max_hp {
                if do_saving_throw(hero.stats.intelligence, 15, ROLL_DISADVANTAGE) {
                    println!("\"Actually, you know what? I can't actually be bothered fighting you today.\"");
                    return true
                }
                else {
                    println!("\nHa ha, NO!\n");
                }
            }

            else if mob.hp < 11 {
                if do_saving_throw(hero.stats.intelligence, 5, ROLL_ADVANTAGE) {
                        println!("\"Y-you're sparing me? Thank the heavens!\"");
                        return true
                    } else {println!("\"I'll fight to the bitter end...\"")}
                }

            else {
                if do_saving_throw(hero.stats.intelligence, 10, ROLL_NORMAL) {
                    println!("\"...fine.\"");
                    return true
                } else {println!("\"No way!\"")}
            }
        },

        2 => {
            println!("You sheathed your weapon and showed the target mercy. \n");

            if mob.hp == mob.max_hp {
                if do_saving_throw(hero.stats.spirit, 15, ROLL_DISADVANTAGE) {
                    println!("\"Actually, you know what? I can't actually be bothered fighting you today.\"");
                    return true
                }
                else {println!("\nYou fool!\n")}
            }

            else if mob.hp < 11 {
                if do_saving_throw(hero.stats.spirit, 5, ROLL_ADVANTAGE) {
                        println!("\"O-o-okay!\"");
                        return true
                    } else {println!("\"Just finish me off, you coward...\"")}
                }

            else {
                if do_saving_throw(hero.stats.spirit, 10, ROLL_NORMAL) {
                    println!("\"I'm going home.\"");
                    return true
                } else {println!("\"You fool!\"")}
            }
        },

        3 => {
            println!("You pressed your weapon against the target's throat. \n");

            if mob.hp == mob.max_hp {
                if do_saving_throw(hero.stats.strength, 15, ROLL_DISADVANTAGE) {
                    println!("\"AAAAAAAAAAAH!!!!!!\"\nThey ran away!");
                    return true
                }
                else {println!("\nHow dare you!\n");}
            }

            else if mob.hp < 11 {
                if do_saving_throw(hero.stats.spirit, 5, ROLL_ADVANTAGE) {
                        println!("\"AAAAAAAAAAAH!!!!!!\"\nThey ran away!");
                        return true
                    } else {println!("\"Just finish me off, you coward...\"")}
                }

            else {
                if do_saving_throw(hero.stats.spirit, 10, ROLL_NORMAL) {
                    println!("\"AAAAAAAAAAAH!!!!!!\"\nThey ran away!");
                    return true
                } else {println!("\"Your taunts are meaningless!\"")}
            }
        }

        _ => {println!("The enemy didn't understand what you were trying to do.")}
    }

    false
}

/*
DAMAGE FORMULAS:

Physical
(((Attack Power + Weapon Power + (Strength / 2) + Level) - (Target's AC + ((Target's Constitution / 2) + (Target's Level / 2))) * Elemental Effectiveness) / (2 if guarding)

Magical
(Attack Power + Intelligence + Level) - (Target's MR + Spirit + (Target's Level / 2)) * Elemental Effectiveness

STAT CALCULATIONS

Max HP
(Constitution * Level) + ((Strength / 2) + Level)
Additionally, the player has an extra 10 HP

Max MP
(Intellect * Level) + Spirit + Level

Hit Rate
(Attack Accuracy + (Dexterity - (Target's Dexterity / 2) + (Level / 2)
Then roll a random number out of 100. If the random number is higher than the calculated hit rate, the attack misses.

Initiative
Dexterity + Level
For tie breaks, just compare level. If both are the same, flip a coin

If any value is not a whole number, ignore anything past the decimal point

Damage and hit rate calculations can also be done with an advantage. For advantages, the calculation will be rolled twice with the lowest value being discarded.
There are also disadvantages, which do the opposite.

These values can be altered to account for things like modifiers.

*/


// Weapon type should pull from the player's weapon data
/*
    WEAPON TYPES IN ORDER OF POWER
    (Strongest > Weakest)

    Slashing
    Greataxe > Greatsword > Axe > Sword > Knife

    Piercing
    Bow > Polearm -> Spear > Knife

    Impact
    Hammer -> Club -> Staff

    Knives normally do peircing damage, but they will do slash damage if the target is weak to it or takes less damage from pierce.
    Wands and unarmed combat do neutral damage. For unarmed, the weapon power is (Strength * Proficiency Level ) / 2. If you have no barehanded pro
*/

    /*
    If Player Strength < Weapon Weight {Weapon Accuracy /= 2};
    */

// Crunchy, munchy, so delicious! I love my food!

/*
EXAMPLE OF HOW A NEGOTATION CAN GO

Dealing with Beasts

    The first step of all negotiations is to check how much HP the enemy has left. Lots of enemies will plead for mercy if you're winning.

    Otherwise, if you try to negotiate when you're losing, the enemy might show you mercy under certain conditions.

    On equal footing:
    "The {ENEMY} is in an agressive stance."

    If you're winning
    "The enemy is quivering for its life..."

    If you don't have a Crest of Wood...
    Unfortunately, {ENEMY} is a wild animal and does not talk. Needless to say, it proceeds to attack you.

    But if you do...
    "Your Crest of Wood gives off a faint warmph."

    Beasts don't know what money is...
    "Grrr! What these? These "money?" Money junk!"

    ...but they do like food!
    "*sniff* *sniff* {ITEM} yummy!"

*/
