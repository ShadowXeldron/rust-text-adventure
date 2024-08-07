use crate::combat::*;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Attack<'a> {
    pub name: &'a str, // Brief name used for the attack
    pub desc: &'a str, // Description of the attack
    pub cost: usize, // MP cost of the attack
    pub category: usize, // Whether the attack is Physical, Magical or Status.
    pub element: usize, // Elemental type of the attack, used in damage calculation
    pub power: usize, // Base power of the attack
    pub hit_rate: usize, // Value up to 100 that affects how likely the attack is to connect. Numbers higher than 100 will ignore accuracy checks to always hit
    pub target: usize
}

pub static ATTACK_TACKLE: Attack<'_> = Attack {
    name: "Tackle",
    desc: "Strikes the target physically for heavy damage",
    cost: 0,
    category: MOVE_PHYSICAL,
    element: TYPE_NEUTRAL,
    power: 1,
    hit_rate: 70,
    target: TARGET_FOE
};

pub static ATTACK_DEVOUR: Attack<'_> = Attack {
    name: "Devour",
    desc: "Gobbles up the target in one hit",
    cost: 0,
    category: MOVE_INSTANT_KILL,
    element: TYPE_NEUTRAL,
    power: 99999,
    hit_rate: 101,
    target: TARGET_FOE
};

pub static ATTACK_SUPER_PUNCH: Attack<'_> = Attack {
    name: "Super Punch",
    desc: "Strikes the target physically for heavy damage",
    cost: 5,
    category: MOVE_PHYSICAL,
    element: TYPE_NEUTRAL,
    power: 65,
    hit_rate: 40,
    target: TARGET_FOE
};

pub static ATTACK_HEAVY_SLASH: Attack<'_> = Attack {
    name: "Heavy Slash",
    desc: "A powerful sword technique",
    cost: 5,
    category: MOVE_PHYSICAL,
    element: TYPE_SLASH,
    power: 20,
    hit_rate: 85,
    target: TARGET_FOE
};

pub static ATTACK_CRUSHER: Attack<'_> = Attack {
    name: "Crusher",
    desc: "Attacks with a crushing blow",
    cost: 5,
    category: MOVE_PHYSICAL,
    element: TYPE_IMPACT,
    power: 20,
    hit_rate: 85,
    target: TARGET_FOE
};

pub static ATTACK_ARROW_RAID: Attack<'_> = Attack {
    name: "Arrow Raid",
    desc: "Attacks with myriad arrows",
    cost: 5,
    category: MOVE_PHYSICAL,
    element: TYPE_PIERCE,
    power: 20,
    hit_rate: 85,
    target: TARGET_FOE
};

pub static ATTACK_MAGIC_MISSILE: Attack<'_> = Attack {
    name: "Magic Missile",
    desc: "Launches a magic projectile that always hits its target",
    cost: 5,
    category: MOVE_MAGIC,
    element: TYPE_NEUTRAL,
    power: 20,
    hit_rate: 101,
    target: TARGET_FOE
};

pub static ATTACK_FIREBLAST: Attack<'_> = Attack {
    name: "Fireblast",
    desc: "Shoots a fireball",
    cost: 5,
    category: MOVE_MAGIC,
    element: TYPE_FIRE,
    power: 20,
    hit_rate: 85,
    target: TARGET_FOE
};

pub static ATTACK_ICE_BEAM: Attack<'_> = Attack {
    name: "Ice Beam",
    desc: "Fires a freezing laser",
    cost: 5,
    category: MOVE_MAGIC,
    element: TYPE_ICE,
    power: 20,
    hit_rate: 85,
    target: TARGET_FOE
};

pub static ATTACK_THUNDER: Attack<'_> = Attack {
    name: "Thunder",
    desc: "Calls down a lightning bolt",
    cost: 5,
    category: MOVE_MAGIC,
    element: TYPE_ELECTRIC,
    power: 20,
    hit_rate: 85,
    target: TARGET_FOE
};

pub static ATTACK_WIND_BLAST: Attack<'_> = Attack {
    name: "Wind Blast",
    desc: "Attacks with a gust of wind",
    cost: 5,
    category: MOVE_MAGIC,
    element: TYPE_WIND,
    power: 20,
    hit_rate: 85,
    target: TARGET_FOE
};

pub static ATTACK_QUAKE: Attack<'_> = Attack {
    name: "Quake",
    desc: "Causes a mini earthquake",
    cost: 5,
    category: MOVE_MAGIC,
    element: TYPE_GROUND,
    power: 20,
    hit_rate: 85,
    target: TARGET_FOE
};

pub static ATTACK_DARK_FORCE: Attack<'_> = Attack {
    name: "Dark Force",
    desc: "Attacks with a dark power",
    cost: 5,
    category: MOVE_MAGIC,
    element: TYPE_DARK,
    power: 20,
    hit_rate: 85,
    target: TARGET_FOE
};

pub static ATTACK_RADIANCE: Attack<'_> = Attack {
    name: "Radiance",
    desc: "Attacks with a shining light",
    cost: 5,
    category: MOVE_MAGIC,
    element: TYPE_LIGHT,
    power: 20,
    hit_rate: 85,
    target: TARGET_FOE
};

pub static ATTACK_HEAL: Attack<'_> = Attack {
    name: "Heal",
    desc: "Restores HP equal to the user's spirit stat",
    cost: 5,
    category: MOVE_STATUS,
    element: TYPE_NEUTRAL,
    power: 1,
    hit_rate: 101,
    target: TARGET_SELF
};
