//use crate::combat::*;
// Should probably rework this to use enums instead...

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum AttackCategory {
	Physical,
	Magic,
	Support,
	StatusEffect,
	Instakill,
	Field,
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum TargetClass {
	Foe,
	Ally,
	User,
}

// Secondary effects of attacks
pub enum MoveEffect {
	// Harmful-to-target effects
	Critical(u8), // u8% chance to deal 1.5x damage
	InflictNVStatus(NonVolatileStatus, u8), // Percentage chance to inflict the target with the set non-volatile status effect. You can only be afflicted with one NV status effect at a time.
	InflictVStatus(VolatileStatus, u8), // Percentage chance to inflict the target with the set volatile status effect. You can be inflicted with multiple volatile status effects at once.
	
	// Healing effects
	RestoreHP, // Normal HP recovery effect
	RestoreHPSet(u16), //
	RestoreHPPercentage(u8),
	CureSetNVStatus(NonVolatileStatus), // Removes the set non-volatile status effect from the target
	CureAllNVStatus,
	CureSetVStatus(VolatileStatus), // Removes the set volatile status effect from the target
	CureAllVStatus,
	CureAllStatus,
	
	// Beneficial-to-self effects
	DrainHP(u8), // Restores u8% of the user's HP
	
	// Harmful-to-self effects
	RecoilSet(u16), // Takes set damage if the attack connects
	RecoilPercentOfDamage(u8), // Takes set damage if the attack connects
}

pub enum NonVolatileStatus {
	Poison, // Takes (40 - Constitution)% of your max HP of damage every turn. After taking damage, performs a constitution saving throw that, if successful, cures the status effect
	Burn, // Takes 10% of the user's max HP every turn. After taking damage, performs a dexterity saving throw that, if successful, cures the status effect.
	Illness, // Takes (20 - (Constitution / 2))% of your max HP of damage every turn. Also treats Constitution as 0 for the sake of damage calculation and saving throws.
	Chill, // Treats Dexterity as if it is 0 for the sake of all calculations involving it.
	Freeze, // Prevents movement. Has a 20% chance to expire at the end of the turn. When it expires,
	Petrify, // Prevents movement. Gains a Slash resistance, but will die in one hit to an Impact attack.
	Lagomorph, // Treats Strength as if it is 0 for the sake of all calculations involving it. Prevents negotiation.
	Silence, // Prevents the afflicted character from using special attacks
	Weak, // Halves Strength and Constitution as well as the damage of all the user's physical attacks
	Broken, // Sets AC to 0
}

pub enum VolatileStatus {
	Charm, // Automatically moves during the player turn. Attacks allies.
	Confusion, // Randomly selects a target from anyone, including heroes
	Halt, // Prevents movement. Has a 15% chance to expire every turn.
	Bleeding, // Takes 5% of your max HP of damage every turn. Prevents healing.
	Stun, // Basically the same as Halt
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Element {
	// Physical element types
	Neutral,
	Slash,
	Pierce,
	Impact, // Strike in Ys and Persona, but I'm using the Wesnoth term here because that was my intoduction to this kind of damage type system. Have you played the Battle for Wesnoth? If yes, you're based
	// Magic element types
	Fire,
	Ice,
	Electric,
	Air,   // Or Wind
	Earth, // Or Ground
	Dark,
	Light,
}

impl Element {
	pub fn get_element_name(&self) -> &str // Returns the name of the element as a string
	{
		match &self {
			Element::Neutral => "Neutral",
			Element::Slash => "Slash",
			Element::Pierce => "Pierce",
			Element::Impact => "Neutral",
			Element::Fire => "Fire",
			Element::Ice => "Ice",
			Element::Electric => "Electric",
			Element::Air => "Air",
			Element::Earth => "Earth",
			Element::Dark => "Dark",
			Element::Light => "Light",
		}
	}
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Attack<'a> {
	pub name: &'a str,            // Brief name used for the attack
	pub desc: &'a str,            // Description of the attack
	pub cost: u16,                // MP cost of the attack
	pub category: AttackCategory, // Whether the attack is Physical, Magical or Status.
	pub element: Element,         // Elemental type of the attack, used in damage calculation
	pub power: u8,                // Base power of the attack
	pub hit_rate: u8, // Value up to 100 that affects how likely the attack is to connect. Numbers higher than 100 will ignore accuracy checks to always hit
	pub target: TargetClass,
}

pub const ATTACK_TACKLE: Attack<'_> = Attack {
	name: "Tackle",
	desc: "A weak physical attack.",
	cost: 0,
	category: AttackCategory::Physical,
	element: Element::Neutral,
	power: 1,
	hit_rate: 70,
	target: TargetClass::Foe,
};

pub const ATTACK_STAB: Attack<'_> = Attack {
	name: "Stab",
	desc: "A weak piercing attack",
	cost: 0,
	category: AttackCategory::Physical,
	element: Element::Pierce,
	power: 3,
	hit_rate: 80,
	target: TargetClass::Foe,
};

pub const ATTACK_DEVOUR: Attack<'_> = Attack {
	name: "Devour",
	desc: "Gobbles up the target in one hit",
	cost: 0,
	category: AttackCategory::Instakill,
	element: Element::Neutral,
	power: 255,
	hit_rate: 101,
	target: TargetClass::Foe,
};

pub const ATTACK_SUPER_PUNCH: Attack<'_> = Attack {
	name: "Super Punch",
	desc: "Strikes the target physically for heavy damage",
	cost: 5,
	category: AttackCategory::Physical,
	element: Element::Neutral,
	power: 65,
	hit_rate: 40,
	target: TargetClass::Foe,
};

pub const ATTACK_HEAVY_SLASH: Attack<'_> = Attack {
	name: "Heavy Slash",
	desc: "A powerful sword technique",
	cost: 5,
	category: AttackCategory::Physical,
	element: Element::Slash,
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_CRUSHER: Attack<'_> = Attack {
	name: "Crusher",
	desc: "Attacks with a crushing blow",
	cost: 5,
	category: AttackCategory::Physical,
	element: Element::Impact,
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_ARROW_RAID: Attack<'_> = Attack {
	name: "Arrow Raid",
	desc: "Attacks with myriad arrows",
	cost: 5,
	category: AttackCategory::Physical,
	element: Element::Pierce,
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_MAGIC_MISSILE: Attack<'_> = Attack {
	name: "Magic Missile",
	desc: "Launches a magic projectile that always hits its target",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Neutral,
	power: 20,
	hit_rate: 101,
	target: TargetClass::Foe,
};

pub const ATTACK_FIREBLAST: Attack<'_> = Attack {
	name: "Fireblast",
	desc: "Shoots a fireball",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Fire,
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_ICE_BEAM: Attack<'_> = Attack {
	name: "Ice Beam",
	desc: "Fires a freezing laser",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Ice,
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_THUNDER: Attack<'_> = Attack {
	name: "Thunder",
	desc: "Calls down a lightning bolt",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Electric,
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_WIND_BLAST: Attack<'_> = Attack {
	name: "Wind Blast",
	desc: "Attacks with a gust of wind",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Air,
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_QUAKE: Attack<'_> = Attack {
	name: "Quake",
	desc: "Causes a mini earthquake",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Earth,
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_DARK_FORCE: Attack<'_> = Attack {
	name: "Dark Force",
	desc: "Attacks with a dark power",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Dark,
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_RADIANCE: Attack<'_> = Attack {
	name: "Radiance",
	desc: "Attacks with a shining light",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Light,
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_HEAL: Attack<'_> = Attack {
	name: "Heal",
	desc: "Restores HP equal to the user's spirit stat",
	cost: 5,
	category: AttackCategory::Support,
	element: Element::Neutral,
	power: 1,
	hit_rate: 101,
	target: TargetClass::Ally,
};
