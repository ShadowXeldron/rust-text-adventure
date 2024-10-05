// Dice rolls
use rand::Rng;

// Rolling constants - these are from D&D
pub const ROLL_NORMAL: u8 = 0; // Roll one die
pub const ROLL_ADVANTAGE: u8 = 1; // Roll two die, accept the higher value
pub const ROLL_DISADVANTAGE: u8 = 2; // Roll two die, accept the lower value

pub fn do_saving_throw(stat: u8, dc: u8, advantage: u8) -> bool {
	// Start by rolling a dice if the encounter rate value is higher than 0
	let mut roll = rand::thread_rng().gen_range(1..20);
	println!("Roll 1d20, outcome is {}", stat);

	// Roll a second dice if you're at an advantage or disadvantage
	if advantage != ROLL_NORMAL {
		let roll2 = rand::thread_rng().gen_range(1..20);
		println!("Second roll, outcome is {}", roll2);

		if advantage == ROLL_ADVANTAGE && roll2 > roll {
			println!("Advantageous roll and second is higher. Discard initial value.");
			roll = roll2
		} else if advantage == ROLL_DISADVANTAGE && roll2 < roll {
			println!("Disadvantageous roll and second is lower. Discard initial value.");
			roll = roll2
		};
	};

	// TODO: Add ability modifiers here
	// I think an actual ability modifier system in place of this would be better.
	roll += stat / 3;

	// If the dice roll is equal to or exceeds the difficulty class, return true. Otherwise return false.
	roll > dc
}

/*
"So what are these for?"

During negotiation or on the overworld, you will be asked to perform a saving throw. This is a mechanic from Dungeons and Dragons.
Whenever a saving throw happens, the program will simulate rolling a d20

Keep in mind that this game is not Dungeons and Dragons. It's very likely that early game characters

If the roll fails, it'll return false.

*/
