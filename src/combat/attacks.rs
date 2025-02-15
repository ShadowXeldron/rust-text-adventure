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
#[derive(Copy, Clone, PartialEq, PartialOrd)]
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
	
	// Caveats
	OnlyHitWeak // Prevents the attack from doing anything if the target does not take super effective damage from the attack.
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
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
	Break, // Sets AC to 0
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
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
	pub effect: Option<&'a [MoveEffect]>,
	pub power: u8,                // Base power of the attack
	pub hit_rate: u8, // Value up to 100 that affects how likely the attack is to connect. Numbers higher than 100 will ignore accuracy checks to always hit
	pub target: TargetClass,
}

// Physical neutral

pub const ATTACK_TACKLE: Attack<'_> = Attack {
	name: "Tackle",
	desc: "A weak physical attack.",
	cost: 0,
	category: AttackCategory::Physical,
	element: Element::Neutral,
	effect: None,
	power: 1,
	hit_rate: 70,
	target: TargetClass::Foe,
};

pub const ATTACK_SUPER_PUNCH: Attack<'_> = Attack {
	name: "Super Punch",
	desc: "Unarmed technique. Strikes the target physically for heavy damage",
	cost: 5,
	category: AttackCategory::Physical,
	element: Element::Neutral,
	effect: None,
	power: 65,
	hit_rate: 40,
	target: TargetClass::Foe,
};

// Physical slash

pub const ATTACK_SLASH: Attack<'_> = Attack {
	name: "Slash",
	desc: "A weak slashing attack",
	cost: 0,
	category: AttackCategory::Physical,
	element: Element::Slash,
	effect: None,
	power: 1,
	hit_rate: 80,
	target: TargetClass::Foe,
};

// Sword

// Level 1
pub const ATTACK_SLICE: Attack<'_> = Attack {
	name: "Slice",
	desc: "Sword technique wherein the user performs a clean cut with a two-handed diagonal slice. Has a 20% chance to make the target bleed.",
	cost: 0,
	category: AttackCategory::Physical,
	element: Element::Slash,
	effect: Some(&[MoveEffect::InflictVStatus(VolatileStatus::Bleeding, 20)]),
	power: 3,
	hit_rate: 90,
	target: TargetClass::Foe,
};

// Level 3
pub const ATTACK_HEAVY_SLASH: Attack<'_> = Attack {
	name: "Heavy Slash",
	desc: "A powerful sword technique that thrusts the whole weight of the user's weapon down onto the opponent. Has a 15% chance to deal critical damage",
	cost: 5,
	category: AttackCategory::Physical,
	element: Element::Slash,
	effect: Some(&[MoveEffect::Critical(15)]),
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Axe

// Level 1
pub const ATTACK_CLEAVE: Attack<'_> = Attack {
	name: "Cleave",
	desc: "The user holds their axe sideways and attacks with a heavy horizontal chop. Has a 20% chance to make the target bleed.",
	cost: 0,
	category: AttackCategory::Physical,
	element: Element::Slash,
	effect: Some(&[MoveEffect::InflictVStatus(VolatileStatus::Bleeding, 20)]),
	power: 4,
	hit_rate: 80,
	target: TargetClass::Foe,
};

// Level 2
pub const ATTACK_AXE_BOOMERANG: Attack<'_> = Attack {
	name: "Axe Boomerang",
	desc: "Axe technique. The user throws their weapon to strike an enemy from afar.",
	cost: 0,
	category: AttackCategory::Physical,
	element: Element::Slash,
	effect: None,
	power: 6,
	hit_rate: 80,
	target: TargetClass::Foe,
};

// Level 3
pub const ATTACK_BERSERK: Attack<'_> = Attack {
	name: "Berserk",
	desc: "Axe technique. The user pours all their energy into performing an extremely powerful attack. Powerful, but highly innacurate.",
	cost: 0,
	category: AttackCategory::Physical,
	element: Element::Slash,
	effect: None,
	power: 40,
	hit_rate: 65,
	target: TargetClass::Foe,
};

// Physical impact

pub const ATTACK_BASH: Attack<'_> = Attack {
	name: "Bash",
	desc: "A weak impact attack",
	cost: 0,
	category: AttackCategory::Physical,
	element: Element::Impact,
	effect: None,
	power: 1,
	hit_rate: 80,
	target: TargetClass::Foe,
};

// Club

// Level 1
pub const ATTACK_SLAM: Attack<'_> = Attack {
	name: "Slam",
	desc: "Bashes the opponent on the head with a blunt weapon. Has a 20% chance to inflict confusion.",
	cost: 0,
	category: AttackCategory::Physical,
	element: Element::Impact,
	effect: Some(&[MoveEffect::InflictVStatus(VolatileStatus::Confusion, 10)]),
	power: 3,
	hit_rate: 80,
	target: TargetClass::Foe,
};


// Level 3
pub const ATTACK_CRUSHER: Attack<'_> = Attack {
	name: "Crusher",
	desc: "Attacks with a crushing blow. Has a 15% chance to inflict the Break status.",
	cost: 5,
	category: AttackCategory::Physical,
	element: Element::Impact,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Break, 15)]),
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Staff
pub const ATTACK_STRIKE: Attack<'_> = Attack {
	name: "Strike",
	desc: "Staff technique. Swipes with an overhead attack.",
	cost: 5,
	category: AttackCategory::Physical,
	element: Element::Impact,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Break, 15)]),
	power: 5,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Physical pierce

pub const ATTACK_STAB: Attack<'_> = Attack {
	name: "Stab",
	desc: "A weak piercing attack",
	cost: 0,
	category: AttackCategory::Physical,
	element: Element::Pierce,
	effect: None,
	power: 3,
	hit_rate: 80,
	target: TargetClass::Foe,
};

// Spear attacks

// Level 1
pub const ATTACK_LUNGE: Attack<'_> = Attack {
	name: "Lunge",
	desc: "Thrusts with a long-range polearm. Has a 10% chance to inflict critical damage.",
	cost: 0,
	category: AttackCategory::Physical,
	element: Element::Pierce,
	effect: Some(&[MoveEffect::Critical(10)]),
	power: 3,
	hit_rate: 80,
	target: TargetClass::Foe,
};

// Bow attacks

// Level 1
pub const ATTACK_SEEKER_SHOT: Attack<'_> = Attack {
	name: "Seeker Shot",
	desc: "Shoots a magically enhanced arrow that ignores accuracy checks to always hit.",
	cost: 5,
	category: AttackCategory::Physical,
	element: Element::Pierce,
	effect: None,
	power: 10,
	hit_rate: 101,
	target: TargetClass::Foe,
};

pub const ATTACK_ARROW_RAID: Attack<'_> = Attack {
	name: "Arrow Raid",
	desc: "Fires a volley of arrows.",
	cost: 5,
	category: AttackCategory::Physical,
	element: Element::Pierce,
	effect: None,
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_POISON_ARROW: Attack<'_> = Attack {
	name: "Poison Arrow",
	desc: "Attacks with an arrow coated in venom. Poisons the target.",
	cost: 5,
	category: AttackCategory::Physical,
	element: Element::Pierce,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Poison, 100)]),
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_FLAMING_ARROW: Attack<'_> = Attack {
	name: "Flaming Arrow",
	desc: "Fires an arrow that has been lit on fire. Has a 10% chance to burn the target.",
	cost: 5,
	category: AttackCategory::Physical,
	element: Element::Fire,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Burn, 10)]),
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Neutral magic

// Level 1

pub const ATTACK_FORCE_BOLT: Attack<'_> = Attack {
	name: "Force Bolt",
	desc: "Fires a weak pulse of magical energy.",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Neutral,
	effect: None,
	power: 5,
	hit_rate: 95,
	target: TargetClass::Foe,
};

// Level 3

pub const ATTACK_MAGIC_MISSILE: Attack<'_> = Attack {
	name: "Magic Missile",
	desc: "Launches a magic projectile that always hits its target",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Neutral,
	effect: None,
	power: 20,
	hit_rate: 101,
	target: TargetClass::Foe,
};

// Level 8

pub const ATTACK_ARMAGEDDON: Attack<'_> = Attack {
	name: "Armageddon", // Megidolaon with the serial numbers filed off
	desc: "Calls forth the destructive might of the apocalpse to destroy the enemy",
	cost: 100,
	category: AttackCategory::Magic,
	element: Element::Neutral,
	effect: None,
	power: 140,
	hit_rate: 70,
	target: TargetClass::Foe,
};

// Fire magic

// Level 2
pub const ATTACK_BURN: Attack<'_> = Attack {
	name: "Burn",
	desc: "Sets the opponent on fire. Inflicts Burn.",
	cost: 10,
	category: AttackCategory::StatusEffect,
	element: Element::Fire,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Burn, 100)]),
	power: 1,
	hit_rate: 75,
	target: TargetClass::Foe,
};

// Level 3
pub const ATTACK_FIREBALL: Attack<'_> = Attack {
	name: "Flame",
	desc: "Launches a small orb of blazing fire. Has a 30% chance to inflict Burn.",
	cost: 15,
	category: AttackCategory::StatusEffect,
	element: Element::Fire,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Burn, 30)]),
	power: 10,
	hit_rate: 80,
	target: TargetClass::Foe,
};

// Level 5
pub const ATTACK_FIREBLAST: Attack<'_> = Attack {
	name: "Fireblast",
	desc: "Shoots an explosive fireball. Has a 40% chance to inflict Burn.",
	cost: 25,
	category: AttackCategory::Magic,
	element: Element::Fire,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Burn, 40)]),
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Level 7
pub const ATTACK_HELLFIRE: Attack<'_> = Attack {
	name: "Hellfire",
	desc: "Traps the opponent in an unnending inferno. Inflict Burns.",
	cost: 25,
	category: AttackCategory::Magic,
	element: Element::Fire,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Burn, 100)]),
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Level 9
pub const ATTACK_ATOMIC_BLAZE: Attack<'_> = Attack {
	name: "Atomic Blaze",
	desc: "The ultimate fire magic. Creates an almighty explosion to obliterate the target from existence.",
	cost: 300,
	category: AttackCategory::Magic,
	element: Element::Fire,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Burn, 100)]),
	power: 215,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Ice magic

// Level 3
pub const ATTACK_FROST: Attack<'_> = Attack {
	name: "Frost",
	desc: "Attacks with a gust of icy wind. Has a 30% chance to inflict Chill.",
	cost: 15,
	category: AttackCategory::StatusEffect,
	element: Element::Ice,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Chill, 30)]),
	power: 10,
	hit_rate: 80,
	target: TargetClass::Foe,
};

pub const ATTACK_FREEZE: Attack<'_> = Attack {
	name: "Freeze",
	desc: "Attempts to trap an enemy inside of an icy prison. Inflicts Freeze.",
	cost: 20,
	category: AttackCategory::StatusEffect,
	element: Element::Ice,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Freeze, 100)]),
	power: 1,
	hit_rate: 45,
	target: TargetClass::Foe,
};

// Level 4
pub const ATTACK_FLURRY: Attack<'_> = Attack {
	name: "Flurry",
	desc: "Attacks with all enemies with freezing winds. Has a 30% chance to inflict Chill.",
	cost: 30,
	category: AttackCategory::StatusEffect,
	element: Element::Ice,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Chill, 30)]),
	power: 10,
	hit_rate: 80,
	target: TargetClass::Foe, // Note to self: add a boolean for spread targeting
};

// Level 5
pub const ATTACK_ICE_BEAM: Attack<'_> = Attack {
	name: "Ice Beam",
	desc: "Fires a blast of freezing energy. Has a 20% chance to inflict Freeze.",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Ice,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Freeze, 20)]),
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Level 9
pub const ATTACK_ICE_AGE: Attack<'_> = Attack {
	name: "Ice Age",
	desc: "The ultimate Ice magic. Obliterates all enemies with the coldest of temperatures. Inflicts Freeze.",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Ice,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Freeze, 100)]),
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Electric magic

// Level 3
pub const ATTACK_ZAP: Attack<'_> = Attack {
	name: "Zap",
	desc: "Zaps the target with a jolt of electricty. Has a 25% chance to inflict Stun.",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Electric,
	effect: Some(&[MoveEffect::InflictVStatus(VolatileStatus::Stun, 25)]),
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_PARALYSE: Attack<'_> = Attack {
	name: "Paralyse",
	desc: "Paralyses an enemy with electromagnetism. Inflicts Stun.",
	cost: 5,
	category: AttackCategory::StatusEffect,
	element: Element::Electric,
	effect: Some(&[MoveEffect::InflictVStatus(VolatileStatus::Stun, 100)]),
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};


// Level 5
pub const ATTACK_THUNDER: Attack<'_> = Attack {
	name: "Thunder",
	desc: "Calls down a lightning bolt. Has a 20% chance to inflict Stun.",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Electric,
	effect: Some(&[MoveEffect::InflictVStatus(VolatileStatus::Stun, 20)]),
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Level 9
pub const ATTACK_TERAVOLTAGE: Attack<'_> = Attack {
	name: "Teravoltage",
	desc: "Zapfries an enemy with a humonguous blast of energy equal to a trilion volts of electricity. Inflicts Stun.",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Electric,
	effect: Some(&[MoveEffect::InflictVStatus(VolatileStatus::Stun, 100)]),
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Air magic

// Level 3
pub const ATTACK_WIND_BLAST: Attack<'_> = Attack {
	name: "Wind Blast",
	desc: "Attacks with a gust of wind",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Air,
	effect: None,
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Level 5
pub const ATTACK_AIR_CUTTER: Attack<'_> = Attack {
	name: "Air Cutter",
	desc: "Summons a cutting blade of air.",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Air,
	effect: None,
	power: 30,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Level 7
pub const ATTACK_TORNADO: Attack<'_> = Attack {
	name: "Tornado",
	desc: "Creates a gigantic pillar of air.",
	cost: 50,
	category: AttackCategory::Magic,
	element: Element::Air,
	effect: None,
	power: 50,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Earth magic

// Level 3
pub const ATTACK_ROCKSLIDE: Attack<'_> = Attack {
	name: "Rockslide",
	desc: "Attacks by causing a rockslide. Has a 15% chance to inflict Break.",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Earth,
	effect:  Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Break, 15)]),
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Level 5
pub const ATTACK_QUAKE: Attack<'_> = Attack {
	name: "Quake",
	desc: "Causes a mini earthquake",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Earth,
	effect: None,
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_PETRIFY: Attack<'_> = Attack {
	name: "Petrify",
	desc: "Turns an enemy to stone. Inflicts Petrify.",
	cost: 5,
	category: AttackCategory::StatusEffect,
	element: Element::Earth,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Petrify, 100)]),
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Dark magic

// Level 1
pub const ATTACK_POISON: Attack<'_> = Attack {
	name: "Poison",
	desc: "Poisons an enemy with black magic.",
	cost: 5,
	category: AttackCategory::StatusEffect,
	element: Element::Dark,
	effect: Some(&[MoveEffect::InflictNVStatus(NonVolatileStatus::Poison, 100)]),
	power: 1,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Level 2
pub const ATTACK_DRAIN: Attack<'_> = Attack {
	name: "Drain",
	desc: "Absorbs some of an enemy's life force, restoring 50% of the damage dealt by the attack back to the user.",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Dark,
	effect: Some(&[MoveEffect::DrainHP(50)]),
	power: 5,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Level 5
pub const ATTACK_DARK_FORCE: Attack<'_> = Attack {
	name: "Dark Force",
	desc: "Attacks with a dark power",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Dark,
	effect: None,
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_REAPER: Attack<'_> = Attack {
	name: "Reaper",
	desc: "Steals the opponent's soul, instantly vanquishing them.",
	cost: 20,
	category: AttackCategory::Instakill,
	element: Element::Dark,
	effect: None,
	power: 1,
	hit_rate: 30,
	target: TargetClass::Foe,
};

// Light magic

// Level 2
pub const ATTACK_SHINE: Attack<'_> = Attack {
	name: "Holy Light", // Reference to the spell "Dia" from FF1, which is basically Turn Undead
	desc: "Calls holy sunlight to damage enemies. Only works on targets with a weakness to the Light element.",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Light,
	effect: Some(&[MoveEffect::OnlyHitWeak]),
	power: 4,
	hit_rate: 85,
	target: TargetClass::Foe,
};

// Level 5
pub const ATTACK_RADIANCE: Attack<'_> = Attack {
	name: "Radiance",
	desc: "Attacks with a shining light",
	cost: 5,
	category: AttackCategory::Magic,
	element: Element::Light,
	effect: None,
	power: 20,
	hit_rate: 85,
	target: TargetClass::Foe,
};

pub const ATTACK_EXORCISE: Attack<'_> = Attack {
	name: "Exorcise",
	desc: "Vanquishes an enemy with the power of light",
	cost: 5,
	category: AttackCategory::Instakill,
	element: Element::Light,
	effect: None,
	power: 1,
	hit_rate: 30,
	target: TargetClass::Foe,
};

// Healing

// Level 1
pub const ATTACK_HEAL: Attack<'_> = Attack {
	name: "Heal",
	desc: "Restores HP equal to the user's spirit stat",
	cost: 5,
	category: AttackCategory::Support,
	element: Element::Neutral,
	effect: Some(&[MoveEffect::RestoreHP]),
	power: 1,
	hit_rate: 101,
	target: TargetClass::Ally,
};

pub const ATTACK_CURE: Attack<'_> = Attack {
	name: "Cure",
	desc: "Cures the Poison and Illness status effects from a single ally.",
	cost: 5,
	category: AttackCategory::Support,
	element: Element::Neutral,
	effect: Some(&[MoveEffect::CureSetNVStatus(NonVolatileStatus::Illness), MoveEffect::CureSetNVStatus(NonVolatileStatus::Poison)]),
	power: 1,
	hit_rate: 101,
	target: TargetClass::Ally,
};

pub const ATTACK_REVIVE: Attack<'_> = Attack {
	name: "Revive",
	desc: "Brings a dead ally back to life.",
	cost: 5,
	category: AttackCategory::Support,
	element: Element::Light,
	effect: Some(&[MoveEffect::RestoreHP]),
	power: 100,
	hit_rate: 101,
	target: TargetClass::Ally,
};

// Status magic

// Level 1
pub const ATTACK_CONFUSION: Attack<'_> = Attack {
	name: "Confusion",
	desc: "Casts a jinx to confuse an enemy.",
	cost: 5,
	category: AttackCategory::StatusEffect,
	element: Element::Neutral,
	effect: Some(&[MoveEffect::InflictVStatus(VolatileStatus::Confusion, 100)]),
	power: 1,
	hit_rate: 90,
	target: TargetClass::Foe,
};

// Field magic
pub const ATTACK_LIGHT_UP: Attack<'_> = Attack {
	name: "Light Up",
	desc: "Creates magical light, brightening up a dark area.",
	cost: 5,
	category: AttackCategory::Field,
	element: Element::Light,
	effect: None,
	power: 1,
	hit_rate: 101,
	target: TargetClass::User,
};

pub const ATTACK_TELEPORT: Attack<'_> = Attack {
	name: "Teleport",
	desc: "Manipulates space to return to the last used save point.",
	cost: 5,
	category: AttackCategory::Field,
	element: Element::Neutral,
	effect: None,
	power: 1,
	hit_rate: 101,
	target: TargetClass::User,
};

pub const ATTACK_HYPERSPACE: Attack<'_> = Attack {
	name: "Hyperspace",
	desc: "Slips into Hyperspace to warp over to any previously accessed area.",
	cost: 5,
	category: AttackCategory::Field,
	element: Element::Neutral,
	effect: None,
	power: 1,
	hit_rate: 101,
	target: TargetClass::User,
};

pub const ATTACK_FIX: Attack<'_> = Attack {
	name: "Fix",
	desc: "Alchemically bonds a base material to another item. This can be used to repair broken equipment.",
	cost: 5,
	category: AttackCategory::Field,
	element: Element::Neutral,
	effect: None,
	power: 1,
	hit_rate: 101,
	target: TargetClass::User,
};

pub const ATTACK_RECYCLE: Attack<'_> = Attack {
	name: "Recycle",
	desc: "Alchemically converts an item back into its base material More valuable items will produce more materials.",
	cost: 5,
	category: AttackCategory::Field,
	element: Element::Earth,
	effect: None,
	power: 1,
	hit_rate: 101,
	target: TargetClass::User,
};

pub const ATTACK_COOK: Attack<'_> = Attack {
	name: "Cook",
	desc: "Converts a raw food material into its cooked form. You will have to eat it on the spot so that the giant doesn't get it.",
	cost: 5,
	category: AttackCategory::Field,
	element: Element::Fire,
	effect: None,
	power: 1,
	hit_rate: 101,
	target: TargetClass::User,
};

pub const ATTACK_REFINE_MAGIC: Attack<'_> = Attack {
	name: "Refine Magic",
	desc: "Extracts magical essence from an item in an attempt to create a spell, destroying it in the process.",
	cost: 5,
	category: AttackCategory::Field,
	element: Element::Neutral,
	effect: None,
	power: 1,
	hit_rate: 101,
	target: TargetClass::User,
};

// Exclusive Attacks
// Grue's signature move
pub const ATTACK_DEVOUR: Attack<'_> = Attack {
	name: "Devour",
	desc: "Gobbles up the target in one hit",
	cost: 0,
	category: AttackCategory::Instakill,
	element: Element::Neutral,
	effect: None,
	power: 255,
	hit_rate: 101,
	target: TargetClass::Foe,
};
